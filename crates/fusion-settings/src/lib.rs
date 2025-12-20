// Copyright (c) 2024 QuantumSecure Technologies Inc / Fusion Programming Language Team
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Fusion VSC CLI Coder

//! Fusion Settings
//!
//! Hierarchical settings management with precedence:
//! Enterprise > CLI > Local > Project > User

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod loader;

use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    // Core settings
    #[serde(default)]
    pub permissions: Option<PermissionSettings>,

    #[serde(default)]
    pub agent: Option<AgentSettings>,

    #[serde(default)]
    pub env: Option<HashMap<String, String>>,

    // Extended settings
    #[serde(default)]
    pub ai: AiSettings,
    #[serde(default)]
    pub github: GithubSettings,
    #[serde(default)]
    pub editor: EditorSettings,
    #[serde(default)]
    pub projects: ProjectSettings,
    #[serde(default)]
    pub mcp: McpSettings,
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AiSettings {
    #[serde(default = "default_model")]
    pub default_model: String,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default)]
    pub providers: AiProviders,
}

fn default_model() -> String {
    "gpt-4".to_string()
}
fn default_max_tokens() -> usize {
    4000
}
fn default_temperature() -> f32 {
    0.7
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AiProviders {
    #[serde(default)]
    pub openai: ApiKeyConfig,
    #[serde(default)]
    pub anthropic: ApiKeyConfig,
    #[serde(default)]
    pub google: ApiKeyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiKeyConfig {
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GithubSettings {
    pub token: Option<String>,
    pub default_owner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorSettings {
    #[serde(default = "default_editor")]
    pub default: String,
    #[serde(default = "default_terminal")]
    pub terminal: String,
}

fn default_editor() -> String {
    "code".to_string()
}
fn default_terminal() -> String {
    "cmd".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectSettings {
    #[serde(default)]
    pub workspace_dir: String,
    #[serde(default)]
    pub auto_save: bool,
    #[serde(default = "default_retention")]
    pub session_retention_days: u32,
}

fn default_retention() -> u32 {
    30
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpSettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub servers: Vec<String>,
}

// Implement Default for Settings manual to ensure defaults are populated
impl Default for Settings {
    fn default() -> Self {
        Self {
            permissions: None,
            agent: None,
            env: None,
            ai: AiSettings::default(),
            github: GithubSettings::default(),
            editor: EditorSettings::default(),
            projects: ProjectSettings::default(),
            mcp: McpSettings::default(),
            custom: HashMap::new(),
        }
    }
}

// Keep existing structs but update AgentSettings if needed
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentSettings {
    #[serde(default)]
    pub default_mode: String,
}

pub type FusionSettings = Settings;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PermissionSettings {
    #[serde(default)]
    pub allow: Vec<String>,
    #[serde(default)]
    pub ask: Vec<String>,
    #[serde(default)]
    pub deny: Vec<String>,
}

use serde_json::Value as JsonValue;
use std::fs;

impl Settings {
    pub fn save(&self) -> Result<()> {
        let path = Self::user_config_path()?;

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        // Basic validation
        if self.ai.max_tokens == 0 {
            anyhow::bail!("Max tokens must be > 0");
        }
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<toml::Value> {
        let json_val = serde_json::to_value(self).ok()?;

        let mut current = &json_val;
        for part in key.split('.') {
            current = current.get(part)?;
        }

        Self::json_to_toml(current.clone())
    }

    pub fn set(&mut self, key: &str, value: toml::Value) -> Result<()> {
        let mut json_val = serde_json::to_value(self.clone())?;

        Self::update_json_value(&mut json_val, key, value)?;

        *self = serde_json::from_value(json_val)?;
        Ok(())
    }

    fn update_json_value(root: &mut JsonValue, key: &str, value: toml::Value) -> Result<()> {
        let parts: Vec<&str> = key.split('.').collect();
        let mut current = root;

        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                if let JsonValue::Object(map) = current {
                    map.insert(part.to_string(), Self::toml_to_json(value.clone()));
                }
            } else {
                if let JsonValue::Object(map) = current {
                    if !map.contains_key(*part) {
                        map.insert(part.to_string(), JsonValue::Object(serde_json::Map::new()));
                    }
                    current = map.get_mut(*part).unwrap();
                }
            }
        }
        Ok(())
    }

    fn json_to_toml(v: JsonValue) -> Option<toml::Value> {
        match v {
            JsonValue::Null => None,
            JsonValue::Bool(b) => Some(toml::Value::Boolean(b)),
            JsonValue::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Some(toml::Value::Integer(i))
                } else if let Some(f) = n.as_f64() {
                    Some(toml::Value::Float(f))
                } else {
                    None
                }
            }
            JsonValue::String(s) => Some(toml::Value::String(s)),
            JsonValue::Array(arr) => {
                let mut t_arr = Vec::new();
                for item in arr {
                    if let Some(t) = Self::json_to_toml(item) {
                        t_arr.push(t);
                    }
                }
                Some(toml::Value::Array(t_arr))
            }
            JsonValue::Object(obj) => {
                let mut map = toml::map::Map::new();
                for (k, v) in obj {
                    if let Some(t) = Self::json_to_toml(v) {
                        map.insert(k, t);
                    }
                }
                Some(toml::Value::Table(map))
            }
        }
    }

    fn toml_to_json(v: toml::Value) -> JsonValue {
        // Simple conversion roughly
        match v {
            toml::Value::String(s) => JsonValue::String(s),
            toml::Value::Integer(i) => JsonValue::Number(serde_json::Number::from(i)),
            toml::Value::Float(f) => JsonValue::Number(
                serde_json::Number::from_f64(f).unwrap_or(serde_json::Number::from(0)),
            ),
            toml::Value::Boolean(b) => JsonValue::Bool(b),
            toml::Value::Datetime(d) => JsonValue::String(d.to_string()),
            toml::Value::Array(arr) => {
                JsonValue::Array(arr.into_iter().map(Self::toml_to_json).collect())
            }
            toml::Value::Table(table) => {
                let mut map = serde_json::Map::new();
                for (k, v) in table {
                    map.insert(k, Self::toml_to_json(v));
                }
                JsonValue::Object(map)
            }
        }
    }

    pub fn user_config_path() -> Result<PathBuf> {
        dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("No home directory"))
            .map(|p| p.join(".fusion-coder").join("settings.json"))
    }

    pub fn project_config_path() -> Result<PathBuf> {
        std::env::current_dir()
            .map(|p| p.join("fusion-coder.json"))
            .context("Failed to get current dir")
    }

    pub fn global_config_path() -> Result<PathBuf> {
        Ok(PathBuf::from("/etc/fusion/settings.json"))
    }
}

pub struct SettingsLoader {
    pub(crate) enterprise: Option<Settings>,
    pub(crate) cli_args: Option<Settings>,
    pub(crate) local: Option<Settings>,
    pub(crate) project: Option<Settings>,
    pub(crate) user: Option<Settings>,
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
}

impl Default for SettingsLoader {
    fn default() -> Self {
        Self::new()
    }
}
