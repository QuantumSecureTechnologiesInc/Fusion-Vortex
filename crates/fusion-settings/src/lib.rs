//! Fusion Settings
//!
//! Hierarchical settings management with precedence:
//! Enterprise > CLI > Local > Project > User

use anyhow::Result;
use fusion_review_policy::ReviewPolicy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    // Core settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSettings {
    pub allow: Vec<String>,
    pub ask: Vec<String>,
    pub deny: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSettings {
    pub default_mode: String,
}

pub struct SettingsLoader {
    enterprise: Option<Settings>,
    cli_args: Option<Settings>,
    local: Option<Settings>,
    project: Option<Settings>,
    user: Option<Settings>,
}

impl SettingsLoader {
    pub fn new() -> Self {
        Self {
            enterprise: None,
            cli_args: None,
            local: None,
            project: None,
            user: None,
        }
    }

    pub fn load() -> Result<Settings> {
        // TODO: Load from all sources and merge
        Ok(Settings::default())
    }
}
