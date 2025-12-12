//! Workspace management for VS Code extension runtime

use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Workspace configuration
#[derive(Debug, Clone)]
pub struct Workspace {
    pub root: PathBuf,
    pub name: String,
    pub settings: WorkspaceSettings,
}

#[derive(Debug, Clone)]
pub struct WorkspaceSettings {
    pub exclude_patterns: Vec<String>,
    pub include_patterns: Vec<String>,
}

impl Default for WorkspaceSettings {
    fn default() -> Self {
        Self {
            exclude_patterns: vec![
                "**/node_modules/**".to_string(),
                "**/target/**".to_string(),
                "**/.git/**".to_string(),
            ],
            include_patterns: vec!["**/*".to_string()],
        }
    }
}

impl Workspace {
    pub fn new(root: PathBuf) -> Self {
        let name = root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("workspace")
            .to_string();

        Self {
            root,
            name,
            settings: WorkspaceSettings::default(),
        }
    }

    /// Find files in the workspace matching a pattern
    pub fn find_files(&self, pattern: &str) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.root)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let path = entry.path();

                // Check if path matches pattern
                if self.matches_pattern(path, pattern) {
                    files.push(path.to_path_buf());
                }
            }
        }

        Ok(files)
    }

    /// Check if a path matches a glob pattern
    fn matches_pattern(&self, path: &std::path::Path, pattern: &str) -> bool {
        // Simple pattern matching - could be enhanced with glob crate
        let path_str = path.to_string_lossy();

        if pattern == "**/*" {
            return true;
        }

        if pattern.starts_with("**/") {
            let suffix = &pattern[3..];
            return path_str.ends_with(suffix);
        }

        if pattern.ends_with("/**") {
            let prefix = &pattern[..pattern.len() - 3];
            return path_str.contains(prefix);
        }

        path_str.contains(pattern)
    }

    /// Get workspace root
    pub fn root_path(&self) -> &PathBuf {
        &self.root
    }

    /// Check if workspace contains a file
    pub fn contains_file(&self, relative_path: &str) -> bool {
        self.root.join(relative_path).exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_creation() {
        let workspace = Workspace::new(PathBuf::from("/test"));
        assert_eq!(workspace.name, "test");
    }

    #[test]
    fn test_pattern_matching() {
        let workspace = Workspace::new(PathBuf::from("/test"));
        assert!(workspace.matches_pattern(Path::new("/test/file.rs"), "**/*.rs"));
    }
}
