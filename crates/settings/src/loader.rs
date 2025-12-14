use crate::FusionSettings;
use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::Path;

/// Settings loader with environment variable expansion and multi-level precedence
pub struct SettingsLoader {
    global_path: Option<String>,
    user_path: Option<String>,
    project_path: Option<String>,
}

impl SettingsLoader {
    pub fn new() -> Self {
        Self {
            global_path: None,
            user_path: None,
            project_path: None,
        }
    }

    /// Load settings with precedence: project > user > global > defaults
    pub fn load(&self) -> Result<FusionSettings> {
        let mut settings = FusionSettings::default();

        // Load global settings (lowest precedence)
        if let Ok(global_path) = FusionSettings::global_config_path() {
            if global_path.exists() {
                if let Ok(global_settings) = self.load_from(&global_path) {
                    settings.merge(&global_settings);
                }
            }
        }

        // Load user settings (medium precedence)
        if let Ok(user_path) = FusionSettings::user_config_path() {
            if user_path.exists() {
                if let Ok(user_settings) = self.load_from(&user_path) {
                    settings.merge(&user_settings);
                }
            }
        }

        // Load project settings (highest precedence)
        if let Ok(project_path) = FusionSettings::project_config_path() {
            if project_path.exists() {
                if let Ok(project_settings) = self.load_from(&project_path) {
                    settings.merge(&project_settings);
                }
            }
        }

        // Apply environment variable overrides (highest precedence)
        self.apply_env_overrides(&mut settings)?;

        Ok(settings)
    }

    /// Load from specific file
    pub fn load_from(&self, path: &Path) -> Result<FusionSettings> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read settings from {}", path.display()))?;

        let expanded = self.expand_env_vars(&content)?;

        toml::from_str(&expanded)
            .with_context(|| format!("Failed to parse settings from {}", path.display()))
    }

    /// Expand environment variables in format ${VAR_NAME}
    fn expand_env_vars(&self, content: &str) -> Result<String> {
        let mut result = content.to_string();

        // Find all ${VAR} patterns
        let re = regex::Regex::new(r"\$\{([A-Z_][A-Z0-9_]*)\}").unwrap();

        for cap in re.captures_iter(content) {
            let full_match = &cap[0];
            let var_name = &cap[1];

            if let Ok(value) = env::var(var_name) {
                result = result.replace(full_match, &value);
            } else {
                // Keep the placeholder if env var not found
                tracing::warn!("Environment variable not found: {}", var_name);
            }
        }

        Ok(result)
    }

    /// Apply environment variable overrides
    fn apply_env_overrides(&self, settings: &mut FusionSettings) -> Result<()> {
        // AI settings
        if let Ok(model) = env::var("FUSION_AI_MODEL") {
            settings.ai.default_model = model;
        }
        if let Ok(max_tokens) = env::var("FUSION_AI_MAX_TOKENS") {
            if let Ok(tokens) = max_tokens.parse() {
                settings.ai.max_tokens = tokens;
            }
        }
        if let Ok(temp) = env::var("FUSION_AI_TEMPERATURE") {
            if let Ok(temperature) = temp.parse() {
                settings.ai.temperature = temperature;
            }
        }

        // Provider API keys
        if let Ok(key) = env::var("OPENAI_API_KEY") {
            settings.ai.providers.openai.api_key = Some(key);
        }
        if let Ok(key) = env::var("ANTHROPIC_API_KEY") {
            settings.ai.providers.anthropic.api_key = Some(key);
        }
        if let Ok(key) = env::var("GOOGLE_AI_API_KEY") {
            settings.ai.providers.google.api_key = Some(key);
        }

        // GitHub settings
        if let Ok(token) = env::var("GITHUB_TOKEN") {
            settings.github.token = Some(token);
        }
        if let Ok(owner) = env::var("FUSION_GITHUB_DEFAULT_OWNER") {
            settings.github.default_owner = Some(owner);
        }

        // Editor settings
        if let Ok(editor) = env::var("FUSION_EDITOR") {
            settings.editor.default = editor;
        }
        if let Ok(terminal) = env::var("FUSION_TERMINAL") {
            settings.editor.terminal = terminal;
        }

        // Project settings
        if let Ok(workspace) = env::var("FUSION_WORKSPACE_DIR") {
            settings.projects.workspace_dir = workspace;
        }

        Ok(())
    }

    /// Set custom global config path
    pub fn with_global_path(mut self, path: String) -> Self {
        self.global_path = Some(path);
        self
    }

    /// Set custom user config path
    pub fn with_user_path(mut self, path: String) -> Self {
        self.user_path = Some(path);
        self
    }

    /// Set custom project config path
    pub fn with_project_path(mut self, path: String) -> Self {
        self.project_path = Some(path);
        self
    }
}

impl Default for SettingsLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_env_var_expansion() {
        env::set_var("TEST_VAR", "test_value");
        let loader = SettingsLoader::new();
        let expanded = loader.expand_env_vars("key = \"${TEST_VAR}\"").unwrap();
        assert_eq!(expanded, "key = \"test_value\"");
        env::remove_var("TEST_VAR");
    }

    #[test]
    fn test_load_default() {
        let settings = SettingsLoader::new().load().unwrap();
        assert!(!settings.ai.default_model.is_empty());
    }
}
