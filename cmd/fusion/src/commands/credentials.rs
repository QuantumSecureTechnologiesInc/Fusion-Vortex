use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Secure credential storage for extension API keys
#[derive(Serialize, Deserialize, Default)]
pub struct CredentialStore {
    #[serde(skip)]
    store_path: PathBuf,

    /// Extension ID -> API Key mapping
    api_keys: HashMap<String, String>,

    /// Extension ID -> OAuth token mapping
    oauth_tokens: HashMap<String, String>,
}

impl CredentialStore {
    /// Load or create credential store
    pub fn load() -> Result<Self> {
        let store_path = Self::get_store_path()?;

        if store_path.exists() {
            let contents = fs::read_to_string(&store_path)?;
            let mut store: Self = serde_json::from_str(&contents)?;
            store.store_path = store_path;
            Ok(store)
        } else {
            Ok(Self {
                store_path,
                api_keys: HashMap::new(),
                oauth_tokens: HashMap::new(),
            })
        }
    }

    /// Get the credential store file path
    fn get_store_path() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

        let config_dir = home.join(".fusion").join("credentials");
        fs::create_dir_all(&config_dir)?;

        Ok(config_dir.join("store.json"))
    }

    /// Save credential store to disk
    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&self.store_path, json)?;
        Ok(())
    }

    /// Set API key for an extension
    pub fn set_api_key(&mut self, extension_id: &str, api_key: String) -> Result<()> {
        self.api_keys.insert(extension_id.to_string(), api_key);
        self.save()?;
        println!("✅ API key stored for extension: {}", extension_id);
        Ok(())
    }

    /// Get API key for an extension
    pub fn get_api_key(&self, extension_id: &str) -> Option<&String> {
        self.api_keys.get(extension_id)
    }

    /// Set OAuth token for an extension
    pub fn set_oauth_token(&mut self, extension_id: &str, token: String) -> Result<()> {
        self.oauth_tokens.insert(extension_id.to_string(), token);
        self.save()?;
        println!("✅ OAuth token stored for extension: {}", extension_id);
        Ok(())
    }

    /// Get OAuth token for an extension
    pub fn get_oauth_token(&self, extension_id: &str) -> Option<&String> {
        self.oauth_tokens.get(extension_id)
    }

    /// Check if extension has credentials
    pub fn has_credentials(&self, extension_id: &str) -> bool {
        self.api_keys.contains_key(extension_id) || self.oauth_tokens.contains_key(extension_id)
    }

    /// Remove all credentials for an extension
    pub fn remove_credentials(&mut self, extension_id: &str) -> Result<()> {
        self.api_keys.remove(extension_id);
        self.oauth_tokens.remove(extension_id);
        self.save()?;
        println!("🗑️  Credentials removed for extension: {}", extension_id);
        Ok(())
    }

    /// List all extensions with stored credentials
    pub fn list_credentials(&self) -> Vec<String> {
        let mut extensions: Vec<String> = self
            .api_keys
            .keys()
            .chain(self.oauth_tokens.keys())
            .cloned()
            .collect();
        extensions.sort();
        extensions.dedup();
        extensions
    }
}
