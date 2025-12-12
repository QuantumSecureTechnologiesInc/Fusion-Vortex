use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSettings {
    #[serde(default = "default_model")]
    pub default_model: String,

    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,

    #[serde(default = "default_temperature")]
    pub temperature: f32,

    #[serde(default)]
    pub providers: AiProviders,

    #[serde(default)]
    pub safety: AiSafetySettings,
}

impl Default for AiSettings {
    fn default() -> Self {
        Self {
            default_model: default_model(),
            max_tokens: default_max_tokens(),
            temperature: default_temperature(),
            providers: AiProviders::default(),
            safety: AiSafetySettings::default(),
        }
    }
}

impl AiSettings {
    pub fn set_nested(&mut self, path: &[&str], value: toml::Value) -> Result<()> {
        match path.first() {
            Some(&"default_model") => {
                self.default_model = value.as_str().unwrap_or_default().to_string()
            }
            Some(&"max_tokens") => self.max_tokens = value.as_integer().unwrap_or(4096) as usize,
            Some(&"temperature") => self.temperature = value.as_float().unwrap_or(0.7) as f32,
            _ => anyhow::bail!("Unknown AI setting: {:?}", path),
        }
        Ok(())
    }

    pub fn merge(&mut self, other: &Self) {
        self.default_model = other.default_model.clone();
        self.max_tokens = other.max_tokens;
        self.temperature = other.temperature;
        self.providers.merge(&other.providers);
        self.safety.merge(&other.safety);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AiProviders {
    #[serde(default)]
    pub openai: OpenAIProvider,

    #[serde(default)]
    pub anthropic: AnthropicProvider,

    #[serde(default)]
    pub google: GoogleProvider,
}

impl AiProviders {
    pub fn merge(&mut self, other: &Self) {
        self.openai.merge(&other.openai);
        self.anthropic.merge(&other.anthropic);
        self.google.merge(&other.google);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpenAIProvider {
    pub api_key: Option<String>,
    pub organization: Option<String>,
    pub base_url: Option<String>,
}

impl OpenAIProvider {
    pub fn merge(&mut self, other: &Self) {
        if other.api_key.is_some() {
            self.api_key = other.api_key.clone();
        }
        if other.organization.is_some() {
            self.organization = other.organization.clone();
        }
        if other.base_url.is_some() {
            self.base_url = other.base_url.clone();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnthropicProvider {
    pub api_key: Option<String>,
}

impl AnthropicProvider {
    pub fn merge(&mut self, other: &Self) {
        if other.api_key.is_some() {
            self.api_key = other.api_key.clone();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoogleProvider {
    pub api_key: Option<String>,
}

impl GoogleProvider {
    pub fn merge(&mut self, other: &Self) {
        if other.api_key.is_some() {
            self.api_key = other.api_key.clone();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSafetySettings {
    #[serde(default = "default_true")]
    pub pii_detection: bool,

    #[serde(default = "default_true")]
    pub secret_detection: bool,

    #[serde(default = "default_true")]
    pub require_review_medium: bool,

    #[serde(default = "default_true")]
    pub offline_for_sensitive: bool,
}

impl Default for AiSafetySettings {
    fn default() -> Self {
        Self {
            pii_detection: true,
            secret_detection: true,
            require_review_medium: true,
            offline_for_sensitive: true,
        }
    }
}

impl AiSafetySettings {
    pub fn merge(&mut self, other: &Self) {
        self.pii_detection = other.pii_detection;
        self.secret_detection = other.secret_detection;
        self.require_review_medium = other.require_review_medium;
        self.offline_for_sensitive = other.offline_for_sensitive;
    }
}

/// GitHub configuration settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitHubSettings {
    pub token: Option<String>,
    pub default_owner: Option<String>,
    pub default_visibility: Option<String>,
}

impl GitHubSettings {
    pub fn set_nested(&mut self, path: &[&str], value: toml::Value) -> Result<()> {
        match path.first() {
            Some(&"token") => self.token = value.as_str().map(String::from),
            Some(&"default_owner") => self.default_owner = value.as_str().map(String::from),
            Some(&"default_visibility") => {
                self.default_visibility = value.as_str().map(String::from)
            }
            _ => anyhow::bail!("Unknown GitHub setting: {:?}", path),
        }
        Ok(())
    }

    pub fn merge(&mut self, other: &Self) {
        if other.token.is_some() {
            self.token = other.token.clone();
        }
        if other.default_owner.is_some() {
            self.default_owner = other.default_owner.clone();
        }
        if other.default_visibility.is_some() {
            self.default_visibility = other.default_visibility.clone();
        }
    }
}

/// Editor configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    #[serde(default = "default_editor")]
    pub default: String,

    #[serde(default = "default_terminal")]
    pub terminal: String,

    #[serde(default)]
    pub custom_commands: HashMap<String, String>,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            default: default_editor(),
            terminal: default_terminal(),
            custom_commands: HashMap::new(),
        }
    }
}

impl EditorSettings {
    pub fn set_nested(&mut self, path: &[&str], value: toml::Value) -> Result<()> {
        match path.first() {
            Some(&"default") => self.default = value.as_str().unwrap_or_default().to_string(),
            Some(&"terminal") => self.terminal = value.as_str().unwrap_or_default().to_string(),
            _ => anyhow::bail!("Unknown editor setting: {:?}", path),
        }
        Ok(())
    }

    pub fn merge(&mut self, other: &Self) {
        self.default = other.default.clone();
        self.terminal = other.terminal.clone();
        for (key, value) in &other.custom_commands {
            self.custom_commands.insert(key.clone(), value.clone());
        }
    }
}

/// Project management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    #[serde(default = "default_workspace_dir")]
    pub workspace_dir: String,

    #[serde(default = "default_true")]
    pub auto_save: bool,

    #[serde(default = "default_session_retention")]
    pub session_retention_days: usize,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            workspace_dir: default_workspace_dir(),
            auto_save: true,
            session_retention_days: default_session_retention(),
        }
    }
}

impl ProjectSettings {
    pub fn set_nested(&mut self, path: &[&str], value: toml::Value) -> Result<()> {
        match path.first() {
            Some(&"workspace_dir") => {
                self.workspace_dir = value.as_str().unwrap_or_default().to_string()
            }
            Some(&"auto_save") => self.auto_save = value.as_bool().unwrap_or(true),
            Some(&"session_retention_days") => {
                self.session_retention_days = value.as_integer().unwrap_or(30) as usize
            }
            _ => anyhow::bail!("Unknown project setting: {:?}", path),
        }
        Ok(())
    }

    pub fn merge(&mut self, other: &Self) {
        self.workspace_dir = other.workspace_dir.clone();
        self.auto_save = other.auto_save;
        self.session_retention_days = other.session_retention_days;
    }
}

/// MCP (Model Context Protocol) settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpSettings {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default)]
    pub servers: Vec<String>,

    #[serde(default)]
    pub custom_servers: HashMap<String, String>,
}

impl Default for McpSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            servers: vec!["filesystem".to_string(), "github".to_string()],
            custom_servers: HashMap::new(),
        }
    }
}

impl McpSettings {
    pub fn set_nested(&mut self, path: &[&str], value: toml::Value) -> Result<()> {
        match path.first() {
            Some(&"enabled") => self.enabled = value.as_bool().unwrap_or(true),
            _ => anyhow::bail!("Unknown MCP setting: {:?}", path),
        }
        Ok(())
    }

    pub fn merge(&mut self, other: &Self) {
        self.enabled = other.enabled;
        self.servers = other.servers.clone();
        for (key, value) in &other.custom_servers {
            self.custom_servers.insert(key.clone(), value.clone());
        }
    }
}

// Default value functions
fn default_model() -> String {
    "gpt-4-turbo-preview".to_string()
}

fn default_max_tokens() -> usize {
    4096
}

fn default_temperature() -> f32 {
    0.7
}

fn default_true() -> bool {
    true
}

fn default_editor() -> String {
    #[cfg(windows)]
    return "code".to_string();

    #[cfg(not(windows))]
    return "vim".to_string();
}

fn default_terminal() -> String {
    #[cfg(windows)]
    return "wt".to_string();

    #[cfg(not(windows))]
    return "gnome-terminal".to_string();
}

fn default_workspace_dir() -> String {
    "~/fusion-projects".to_string()
}

fn default_session_retention() -> usize {
    30 // days
}
