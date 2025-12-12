use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Database interface for project state
pub struct ProjectDatabase {
    conn: Connection,
}

impl ProjectDatabase {
    /// Open or create database at path
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open database at {}", path.display()))?;

        let db = Self { conn };
        db.initialize_schema()?;

        Ok(db)
    }

    /// Initialize database schema
    fn initialize_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                path TEXT NOT NULL,
                created_at TEXT NOT NULL,
                last_accessed TEXT NOT NULL,
                metadata TEXT
            );

            CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                started_at TEXT NOT NULL,
                ended_at TEXT,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS conversations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id INTEGER NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                metadata TEXT,
                FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS changes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id INTEGER NOT NULL,
                file_path TEXT NOT NULL,
                change_type TEXT NOT NULL,
                diff TEXT NOT NULL,
                applied BOOLEAN NOT NULL DEFAULT 0,
                timestamp TEXT NOT NULL,
                provenance TEXT,
                FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_sessions_project ON sessions(project_id);
            CREATE INDEX IF NOT EXISTS idx_conversations_session ON conversations(session_id);
            CREATE INDEX IF NOT EXISTS idx_changes_session ON changes(session_id);
            CREATE INDEX IF NOT EXISTS idx_changes_applied ON changes(applied);
            "#,
        )?;

        Ok(())
    }

    /// Create a new project
    pub fn create_project(&self, name: &str, path: &Path) -> Result<i64> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO projects (name, path, created_at, last_accessed) VALUES (?1, ?2, ?3, ?4)",
            params![name, path.to_string_lossy(), now, now],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Get project by name
    pub fn get_project(&self, name: &str) -> Result<Option<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, path, created_at, last_accessed, metadata FROM projects WHERE name = ?1"
        )?;

        let mut rows = stmt.query(params![name])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                path: PathBuf::from(row.get::<_, String>(2)?),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)?.into(),
                last_accessed: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)?.into(),
                metadata: row
                    .get::<_, Option<String>>(5)?
                    .and_then(|s| serde_json::from_str(&s).ok()),
            }))
        } else {
            Ok(None)
        }
    }

    /// List all projects
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, path, created_at, last_accessed, metadata FROM projects ORDER BY last_accessed DESC"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                path: PathBuf::from(row.get::<_, String>(2)?),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap()
                    .into(),
                last_accessed: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .into(),
                metadata: row
                    .get::<_, Option<String>>(5)?
                    .and_then(|s| serde_json::from_str(&s).ok()),
            })
        })?;

        let mut projects = Vec::new();
        for row in rows {
            projects.push(row?);
        }

        Ok(projects)
    }

    /// Update project last accessed time
    pub fn touch_project(&self, project_id: i64) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "UPDATE projects SET last_accessed = ?1 WHERE id = ?2",
            params![now, project_id],
        )?;

        Ok(())
    }

    /// Delete project
    pub fn delete_project(&self, project_id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM projects WHERE id = ?1", params![project_id])?;
        Ok(())
    }

    /// Create a new session
    pub fn create_session(&self, project_id: i64) -> Result<i64> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO sessions (project_id, started_at) VALUES (?1, ?2)",
            params![project_id, now],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// End a session
    pub fn end_session(&self, session_id: i64) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "UPDATE sessions SET ended_at = ?1 WHERE id = ?2",
            params![now, session_id],
        )?;

        Ok(())
    }

    /// Add conversation message
    pub fn add_conversation(
        &self,
        session_id: i64,
        role: &str,
        content: &str,
        metadata: Option<&str>,
    ) -> Result<i64> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO conversations (session_id, role, content, timestamp, metadata) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![session_id, role, content, now, metadata],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Get conversation history for session
    pub fn get_conversation_history(&self, session_id: i64) -> Result<Vec<ConversationMessage>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, role, content, timestamp, metadata FROM conversations WHERE session_id = ?1 ORDER BY timestamp ASC"
        )?;

        let rows = stmt.query_map(params![session_id], |row| {
            Ok(ConversationMessage {
                id: row.get(0)?,
                role: row.get(1)?,
                content: row.get(2)?,
                timestamp: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap()
                    .into(),
                metadata: row
                    .get::<_, Option<String>>(4)?
                    .and_then(|s| serde_json::from_str(&s).ok()),
            })
        })?;

        let mut messages = Vec::new();
        for row in rows {
            messages.push(row?);
        }

        Ok(messages)
    }

    /// Add code change
    pub fn add_change(
        &self,
        session_id: i64,
        file_path: &str,
        change_type: &str,
        diff: &str,
        provenance: Option<&str>,
    ) -> Result<i64> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO changes (session_id, file_path, change_type, diff, timestamp, provenance) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![session_id, file_path, change_type, diff, now, provenance],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Mark change as applied
    pub fn mark_change_applied(&self, change_id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE changes SET applied = 1 WHERE id = ?1",
            params![change_id],
        )?;

        Ok(())
    }

    /// Get changes for session
    pub fn get_changes(&self, session_id: i64) -> Result<Vec<CodeChange>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, file_path, change_type, diff, applied, timestamp, provenance FROM changes WHERE session_id = ?1 ORDER BY timestamp ASC"
        )?;

        let rows = stmt.query_map(params![session_id], |row| {
            Ok(CodeChange {
                id: row.get(0)?,
                file_path: row.get(1)?,
                change_type: row.get(2)?,
                diff: row.get(3)?,
                applied: row.get(4)?,
                timestamp: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .into(),
                provenance: row
                    .get::<_, Option<String>>(6)?
                    .and_then(|s| serde_json::from_str(&s).ok()),
            })
        })?;

        let mut changes = Vec::new();
        for row in rows {
            changes.push(row?);
        }

        Ok(changes)
    }

    /// Clean up old sessions (older than retention days)
    pub fn cleanup_old_sessions(&self, retention_days: i64) -> Result<usize> {
        let cutoff = (Utc::now() - chrono::Duration::days(retention_days)).to_rfc3339();

        let deleted = self.conn.execute(
            "DELETE FROM sessions WHERE ended_at IS NOT NULL AND ended_at < ?1",
            params![cutoff],
        )?;

        Ok(deleted)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub path: PathBuf,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub id: i64,
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChange {
    pub id: i64,
    pub file_path: String,
    pub change_type: String,
    pub diff: String,
    pub applied: bool,
    pub timestamp: DateTime<Utc>,
    pub provenance: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_create_project() {
        let db = ProjectDatabase::open(Path::new(":memory:")).unwrap();
        let id = db
            .create_project("test-project", Path::new("/tmp/test"))
            .unwrap();
        assert!(id > 0);
    }

    #[test]
    fn test_get_project() {
        let db = ProjectDatabase::open(Path::new(":memory:")).unwrap();
        let id = db
            .create_project("test-project", Path::new("/tmp/test"))
            .unwrap();
        let project = db.get_project("test-project").unwrap();
        assert!(project.is_some());
        assert_eq!(project.unwrap().id, id);
    }

    #[test]
    fn test_session_lifecycle() {
        let db = ProjectDatabase::open(Path::new(":memory:")).unwrap();
        let project_id = db.create_project("test", Path::new("/tmp")).unwrap();
        let session_id = db.create_session(project_id).unwrap();
        assert!(session_id > 0);

        db.end_session(session_id).unwrap();
    }
}
