use anyhow::{Context, Result};

use std::path::{Path, PathBuf};

pub mod db;
pub mod state;

pub use db::{CodeChange, ConversationMessage, Project, ProjectDatabase};
pub use state::ProjectState;

/// Project workspace manager
pub struct ProjectWorkspace {
    db: ProjectDatabase,
    active_project: Option<i64>,
    active_session: Option<i64>,
}

impl ProjectWorkspace {
    /// Open workspace database
    pub fn open() -> Result<Self> {
        let db_path = Self::database_path()?;

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let db = ProjectDatabase::open(&db_path)?;

        Ok(Self {
            db,
            active_project: None,
            active_session: None,
        })
    }

    /// Get database path (~/.fusion/projects.db)
    fn database_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().context("Failed to determine config directory")?;
        Ok(config_dir.join("fusion").join("projects.db"))
    }

    /// Create a new project
    pub fn create_project(&mut self, name: &str, path: &Path) -> Result<i64> {
        // Validate path exists
        if !path.exists() {
            anyhow::bail!("Project path does not exist: {}", path.display());
        }

        let project_id = self.db.create_project(name, path)?;

        // Start initial session
        let session_id = self.db.create_session(project_id)?;

        self.active_project = Some(project_id);
        self.active_session = Some(session_id);

        Ok(project_id)
    }

    /// Open an existing project
    pub fn open_project(&mut self, name: &str) -> Result<()> {
        let project = self
            .db
            .get_project(name)?
            .with_context(|| format!("Project '{}' not found", name))?;

        // Update last accessed
        self.db.touch_project(project.id)?;

        // Start new session
        let session_id = self.db.create_session(project.id)?;

        self.active_project = Some(project.id);
        self.active_session = Some(session_id);

        Ok(())
    }

    /// Close active project
    pub fn close_project(&mut self) -> Result<()> {
        if let Some(session_id) = self.active_session {
            self.db.end_session(session_id)?;
        }

        self.active_project = None;
        self.active_session = None;

        Ok(())
    }

    /// Delete a project
    pub fn delete_project(&mut self, name: &str) -> Result<()> {
        let project = self
            .db
            .get_project(name)?
            .with_context(|| format!("Project '{}' not found", name))?;

        self.db.delete_project(project.id)?;

        // Close if it was active
        if self.active_project == Some(project.id) {
            self.active_project = None;
            self.active_session = None;
        }

        Ok(())
    }

    /// List all projects
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        self.db.list_projects()
    }

    /// Get active project
    pub fn active_project(&self) -> Result<Option<Project>> {
        if let Some(id) = self.active_project {
            // Get by ID
            let projects = self.db.list_projects()?;
            Ok(projects.into_iter().find(|p| p.id == id))
        } else {
            Ok(None)
        }
    }

    /// Add conversation message
    pub fn add_message(&self, role: &str, content: &str) -> Result<()> {
        let session_id = self.active_session.context("No active session")?;

        self.db.add_conversation(session_id, role, content, None)?;

        Ok(())
    }

    /// Get conversation history
    pub fn get_history(&self) -> Result<Vec<ConversationMessage>> {
        let session_id = self.active_session.context("No active session")?;

        self.db.get_conversation_history(session_id)
    }

    /// Record a code change
    pub fn record_change(&self, file_path: &str, change_type: &str, diff: &str) -> Result<i64> {
        let session_id = self.active_session.context("No active session")?;

        self.db
            .add_change(session_id, file_path, change_type, diff, None)
    }

    /// Mark change as applied
    pub fn apply_change(&self, change_id: i64) -> Result<()> {
        self.db.mark_change_applied(change_id)
    }

    /// Get all changes for current session
    pub fn get_changes(&self) -> Result<Vec<CodeChange>> {
        let session_id = self.active_session.context("No active session")?;

        self.db.get_changes(session_id)
    }

    /// Clean up old sessions
    pub fn cleanup(&self, retention_days: i64) -> Result<usize> {
        self.db.cleanup_old_sessions(retention_days)
    }

    /// Export project state
    pub fn export_project(&self, name: &str, output_path: &Path) -> Result<()> {
        let project = self
            .db
            .get_project(name)?
            .with_context(|| format!("Project '{}' not found", name))?;

        let state = ProjectState {
            project,
            sessions: vec![], // Could load sessions if needed
            conversations: vec![],
            changes: vec![],
        };

        let json = serde_json::to_string_pretty(&state)?;
        std::fs::write(output_path, json)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_workspace_creation() {
        let ws = ProjectWorkspace::open();
        assert!(ws.is_ok());
    }

    #[test]
    fn test_project_lifecycle() {
        let mut ws = ProjectWorkspace::open().unwrap();
        let temp_dir = env::temp_dir();

        // Create
        let project_id = ws.create_project("test-proj", &temp_dir).unwrap();
        assert!(project_id > 0);

        // List
        let projects = ws.list_projects().unwrap();
        assert!(!projects.is_empty());

        // Close
        ws.close_project().unwrap();

        // Delete
        ws.delete_project("test-proj").unwrap();
    }
}
