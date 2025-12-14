//! Secure mode enforcement
//!
//! Enhanced security with strict policy enforcement

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureMode {
    pub enabled: bool,
    pub respect_gitignore: bool,
    pub workspace_isolation: bool,
    pub url_allowlist: Vec<String>,
    pub url_denylist: Vec<String>,
}

impl SecureMode {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            respect_gitignore: enabled,
            workspace_isolation: enabled,
            url_allowlist: Vec::new(),
            url_denylist: Vec::new(),
        }
    }

    pub fn strict() -> Self {
        let mut mode = Self::new(true);
        mode.url_denylist = vec!["*".to_string()]; // Deny all by default
        mode
    }

    /// Check if path is within workspace
    pub fn is_path_allowed(&self, path: &PathBuf, workspace: &PathBuf) -> bool {
        if !self.workspace_isolation {
            return true;
        }

        path.starts_with(workspace)
    }

    /// Check if URL is allowed
    pub fn is_url_allowed(&self, url: &str) -> bool {
        if !self.enabled {
            return true;
        }

        // Check denylist first
        for pattern in &self.url_denylist {
            if pattern == "*" || url.contains(pattern) {
                return false;
            }
        }

        // If allowlist is empty, allow all (that aren't denied)
        if self.url_allowlist.is_empty() {
            return true;
        }

        // Check allowlist
        self.url_allowlist
            .iter()
            .any(|pattern| url.contains(pattern))
    }

    /// Force review for all terminal commands
    pub fn force_terminal_review(&self) -> bool {
        self.enabled
    }

    /// Force review for all browser actions
    pub fn force_browser_review(&self) -> bool {
        self.enabled
    }
}

impl Default for SecureMode {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_mode_default() {
        let mode = SecureMode::default();
        assert!(!mode.enabled);
        assert!(mode.is_url_allowed("https://example.com"));
    }

    #[test]
    fn test_strict_mode() {
        let mode = SecureMode::strict();
        assert!(mode.enabled);
        assert!(!mode.is_url_allowed("https://example.com"));
    }

    #[test]
    fn test_workspace_isolation() {
        let mode = SecureMode::new(true);
        let workspace = PathBuf::from("/workspace");
        let inside = PathBuf::from("/workspace/file.txt");
        let outside = PathBuf::from("/etc/passwd");

        assert!(mode.is_path_allowed(&inside, &workspace));
        assert!(!mode.is_path_allowed(&outside, &workspace));
    }

    #[test]
    fn test_url_allowlist() {
        let mut mode = SecureMode::new(true);
        mode.url_allowlist = vec!["github.com".to_string()];

        assert!(mode.is_url_allowed("https://github.com/repo"));
        assert!(!mode.is_url_allowed("https://evil.com"));
    }
}
