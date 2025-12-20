// Copyright (c) 2024 QuantumSecure Technologies Inc / Fusion Programming Language Team
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Fusion VSC CLI Coder

//! Settings hierarchy loader
//!
//! Implements Claude Code-style hierarchical settings with precedence

use crate::{Settings, SettingsLoader};
use anyhow::Result;
use std::path::Path;

impl SettingsLoader {
    /// Load settings from all sources with precedence
    pub fn load_all(workspace: &Path) -> Result<Settings> {
        let mut loader = Self::new();

        // Load from all sources (lowest to highest precedence)
        loader.user = Self::load_user_settings().ok();
        loader.project = Self::load_project_settings(workspace).ok();
        loader.local = Self::load_local_settings(workspace).ok();
        // CLI args and enterprise would be loaded externally

        // Merge with precedence
        Ok(loader.merge())
    }

    fn load_user_settings() -> Result<Settings> {
        let path = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("No home directory"))?
            .join(".fusion-coder")
            .join("settings.json");

        Self::load_from_file(&path)
    }

    fn load_project_settings(workspace: &Path) -> Result<Settings> {
        let path = workspace.join("fusion-coder.json");
        Self::load_from_file(&path)
    }

    fn load_local_settings(workspace: &Path) -> Result<Settings> {
        let path = workspace.join(".fusion-coder").join("settings.json");
        Self::load_from_file(&path)
    }

    fn load_from_file(path: &Path) -> Result<Settings> {
        if !path.exists() {
            return Ok(Settings::default());
        }

        let content = std::fs::read_to_string(path)?;
        let settings: Settings = serde_json::from_str(&content)?;
        Ok(settings)
    }

    fn merge(&self) -> Settings {
        let mut result = Settings::default();

        // Merge in order of precedence (lowest first)
        if let Some(user) = &self.user {
            result = Self::merge_settings(result, user.clone());
        }
        if let Some(project) = &self.project {
            result = Self::merge_settings(result, project.clone());
        }
        if let Some(local) = &self.local {
            result = Self::merge_settings(result, local.clone());
        }
        if let Some(cli) = &self.cli_args {
            result = Self::merge_settings(result, cli.clone());
        }
        if let Some(enterprise) = &self.enterprise {
            result = Self::merge_settings(result, enterprise.clone());
        }

        result
    }

    fn merge_settings(mut base: Settings, override_settings: Settings) -> Settings {
        // Merge permissions
        if let Some(override_perms) = override_settings.permissions {
            base.permissions = Some(override_perms);
        }

        // Merge agent settings
        if let Some(override_agent) = override_settings.agent {
            base.agent = Some(override_agent);
        }

        // Merge environment variables
        if let Some(override_env) = override_settings.env {
            let mut merged_env = base.env.unwrap_or_default();
            merged_env.extend(override_env);
            base.env = Some(merged_env);
        }

        base
    }
}

impl Settings {
    /// Load settings with automatic hierarchy
    pub fn load() -> Result<Settings> {
        let workspace = std::env::current_dir()?;
        SettingsLoader::load_all(&workspace)
    }

    /// Get permission for a command
    pub fn is_command_allowed(&self, command: &str) -> bool {
        if let Some(perms) = &self.permissions {
            // Check deny first
            if perms.deny.iter().any(|p| glob_match(p, command)) {
                return false;
            }

            // If allow list exists, command must match
            if !perms.allow.is_empty() {
                return perms.allow.iter().any(|p| glob_match(p, command));
            }

            // No allow list = allow all (except denied)
            true
        } else {
            true
        }
    }
}

fn glob_match(pattern: &str, value: &str) -> bool {
    if pattern.contains('*') {
        // Simple glob matching
        if pattern.ends_with('*') {
            let prefix = pattern.trim_end_matches('*');
            value.starts_with(prefix)
        } else if pattern.starts_with('*') {
            let suffix = pattern.trim_start_matches('*');
            value.ends_with(suffix)
        } else {
            // Contains wildcard in middle - basic support
            value.contains(pattern.trim_matches('*'))
        }
    } else {
        pattern == value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PermissionSettings;

    #[test]
    fn test_glob_matching() {
        assert!(glob_match("git*", "git status"));
        assert!(glob_match("cargo check*", "cargo check --all"));
        assert!(!glob_match("git*", "cargo build"));
    }

    #[test]
    fn test_permission_checking() {
        let settings = Settings {
            permissions: Some(PermissionSettings {
                allow: vec!["git*".to_string()],
                ask: Vec::new(),
                deny: vec!["rm".to_string()],
            }),
            agent: None,
            env: None,
            ai: crate::AiSettings::default(),
            github: crate::GithubSettings::default(),
            editor: crate::EditorSettings::default(),
            projects: crate::ProjectSettings::default(),
            mcp: crate::McpSettings::default(),
            custom: std::collections::HashMap::new(),
        };

        assert!(settings.is_command_allowed("git status"));
        assert!(!settings.is_command_allowed("rm file.txt"));
        assert!(!settings.is_command_allowed("cargo build"));
    }
}
