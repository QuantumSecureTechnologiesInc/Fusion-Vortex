//! Artifact Vault - State persistence and version management
//!
//! Manages crate metadata, version history, and state persistence to JSON.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

/// Crate metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Crate {
    pub name: String,
    pub version: String,
    pub code_hash: String,
    pub status: String,
    pub last_updated: u64,
}

impl Crate {
    /// Create a new Crate entry
    pub fn new(name: &str, version: &str) -> Self {
        let mut hasher = Sha512::new();
        hasher.update(format!("{}{}", name, version));
        let hash = hex::encode(hasher.finalize());

        Crate {
            name: name.to_string(),
            version: version.to_string(),
            code_hash: hash[0..64].to_string(),
            status: "Active".to_string(),
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Artifact Vault for managing active crates and archives
#[derive(Serialize, Deserialize)]
pub struct ArtifactVault {
    pub active_crates: HashMap<String, Crate>,
    pub archive: Vec<Crate>,
}

impl ArtifactVault {
    /// Create a new empty vault
    pub fn new() -> Self {
        ArtifactVault {
            active_crates: HashMap::new(),
            archive: Vec::new(),
        }
    }

    /// Load vault from disk
    pub fn load_from_disk(path: &str) -> Result<Self> {
        if Path::new(path).exists() {
            let file = File::open(path).context(format!("Failed to open vault at {}", path))?;
            let vault = serde_json::from_reader(file).context("Failed to deserialize vault")?;
            info!("Vault loaded from disk: {}", path);
            Ok(vault)
        } else {
            info!("No existing vault found. Creating new.");
            Ok(ArtifactVault::new())
        }
    }

    /// Save vault to disk
    pub fn save_to_disk(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Register a new crate
    pub fn register(&mut self, name: &str, version: &str) {
        if self.active_crates.contains_key(name) {
            info!("Crate {} is already registered.", name);
            return;
        }
        let krate = Crate::new(name, version);
        info!("Registered new crate: {} v{}", name, version);
        self.active_crates.insert(name.to_string(), krate);
    }

    /// Deploy a new version of a crate
    pub fn deploy_new(&mut self, krate: Crate) -> Result<()> {
        // Archive old version
        if let Some(old) = self.active_crates.get(&krate.name) {
            let mut archived = old.clone();
            archived.status = "Archived".to_string();
            self.archive.push(archived);
        }

        // Deploy new
        self.active_crates.insert(krate.name.clone(), krate.clone());
        self.terminal_notification(&krate);
        Ok(())
    }

    /// Display terminal notification for new deployment
    fn terminal_notification(&self, krate: &Crate) {
        println!("\n╔══════════════════════════════════════════════════╗");
        println!("║  🚀 SENTINAEL TRI-BRID NOTIFICATION              ║");
        println!("╠══════════════════════════════════════════════════╣");
        println!("║  Package: {:<38} ║", krate.name);
        println!("║  Version: {:<38} ║", krate.version);
        println!("║  Status:  READY TO USE                           ║");
        println!("║  Mesh:    OSCILLATING                            ║");
        println!("╚══════════════════════════════════════════════════╝\n");
    }
}

impl Default for ArtifactVault {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_creation() {
        let krate = Crate::new("test_crate", "1.0.0");
        assert_eq!(krate.name, "test_crate");
        assert_eq!(krate.version, "1.0.0");
        assert_eq!(krate.status, "Active");
        assert!(!krate.code_hash.is_empty());
    }

    #[test]
    fn test_vault_operations() {
        let mut vault = ArtifactVault::new();
        vault.register("core", "1.0.0");

        assert!(vault.active_crates.contains_key("core"));
        assert_eq!(vault.archive.len(), 0);
    }

    #[test]
    fn test_deployment_archival() {
        let mut vault = ArtifactVault::new();
        vault.register("app", "1.0.0");

        let new_version = Crate::new("app", "1.0.1");
        vault.deploy_new(new_version).unwrap();

        assert_eq!(vault.active_crates.get("app").unwrap().version, "1.0.1");
        assert_eq!(vault.archive.len(), 1);
        assert_eq!(vault.archive[0].version, "1.0.0");
    }
}
