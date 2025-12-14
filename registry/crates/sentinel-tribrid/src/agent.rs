//! TriBrid Agent - Main autonomous agent with three-tier fallback system
//!
//! Implements Apex (fast) → Audit (safe) → Golden (resilient) redundancy.

use crate::crypto::CryptoVerifier;
use crate::mesh::OscillatingMesh;
use crate::vault::{ArtifactVault, Crate};
use anyhow::{anyhow, Result};
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

/// Main Sentinael Agent with TriBrid redundancy
pub struct SentinaelAgent {
    mesh: Arc<Mutex<OscillatingMesh>>,
    crypto: Arc<Mutex<CryptoVerifier>>,
    vault: Arc<Mutex<ArtifactVault>>,
    db_path: String,
}

impl SentinaelAgent {
    /// Create a new Sentinael Agent
    ///
    /// # Arguments
    ///
    /// * `db_path` - Path to the vault JSON file
    pub fn new(db_path: &str) -> Result<Self> {
        let vault = ArtifactVault::load_from_disk(db_path)?;

        Ok(SentinaelAgent {
            mesh: Arc::new(Mutex::new(OscillatingMesh::new())),
            crypto: Arc::new(Mutex::new(CryptoVerifier::new())),
            vault: Arc::new(Mutex::new(vault)),
            db_path: db_path.to_string(),
        })
    }

    /// Save agent state to disk
    pub fn save_state(&self) -> Result<()> {
        let vault = self.vault.lock();
        vault.save_to_disk(&self.db_path)
    }

    /// Initialize the vault with core components
    pub fn initialize(&self) -> Result<()> {
        info!("Initializing Sentinael Vault...");
        let mut vault = self.vault.lock();
        vault.register("neural_core", "1.0.4");
        vault.register("net_driver", "2.1.0");
        drop(vault);
        self.save_state()?;
        info!("Initialization Complete.");
        Ok(())
    }

    /// Monitor and optimize a crate (Apex Algorithm)
    ///
    /// # Arguments
    ///
    /// * `crate_name` - Name of the crate to monitor
    /// * `simulate_failure` - If true, simulates a failure to test fallback
    pub fn monitor_and_optimize(&self, crate_name: &str, simulate_failure: bool) -> Result<()> {
        let mut mesh = self.mesh.lock();
        let vector = mesh.get_valid_vector();

        // 1. Mesh Access Check (MTD)
        if !mesh.validate_access(vector) {
            error!("Mesh Access Denied. Attack Surface Shifted. Ignoring Request.");
            return Ok(());
        }
        drop(mesh);

        let vault_guard = self.vault.lock();
        let current = match vault_guard.active_crates.get(crate_name) {
            Some(c) => c.clone(),
            None => {
                error!("Unknown crate: {}", crate_name);
                return Ok(());
            }
        };
        drop(vault_guard);

        info!(
            "Scanning {} v{} (Apex Algorithm)...",
            current.name, current.version
        );

        // Simulate build/check time
        std::thread::sleep(Duration::from_millis(500));

        let needs_update = true; // In production, this would check for actual updates

        if needs_update {
            info!("Optimization identified. Initiating Build Cycle...");

            let candidate = self.build_candidate(&current)?;

            let mut crypto = self.crypto.lock();
            let sig = crypto.sign(&candidate.code_hash)?;

            // Verify
            let is_valid = if simulate_failure {
                false
            } else {
                crypto.verify(&candidate.code_hash, &sig)?
            };

            if is_valid {
                info!("Apex Verification Passed (Chaos-HMAC). Deploying.");
                let mut vault = self.vault.lock();
                vault.deploy_new(candidate)?;
                drop(vault);
                self.save_state()?;
            } else {
                warn!("Apex Algorithm Integrity Check Failed. Engaging Fallback 1.");
                drop(crypto); // Release lock before fallback
                self.fallback_audit_protocol(&current)?;
            }
        }
        Ok(())
    }

    /// Build a candidate version (increments patch version)
    fn build_candidate(&self, old: &Crate) -> Result<Crate> {
        let parts: Vec<&str> = old.version.split('.').collect();
        if parts.len() < 3 {
            return Err(anyhow!("Invalid version format"));
        }

        let major = parts[0];
        let minor = parts[1];
        let patch: u32 = parts[2].parse().unwrap_or(0);
        let new_ver = format!("{}.{}.{}", major, minor, patch + 1);

        Ok(Crate::new(&old.name, &new_ver))
    }

    /// Fallback 1: Deep Audit Protocol
    fn fallback_audit_protocol(&self, current: &Crate) -> Result<()> {
        info!("--- ENGAGING FALLBACK 1: DEEP AUDIT ---");

        let candidate = self.build_candidate(current)?;

        // Deep Audit: Double Sign Verification
        let mut crypto = self.crypto.lock();
        let sig1 = crypto.sign(&candidate.code_hash)?;
        // Force chaos evolution
        crypto.chaos.next_val();
        let sig2 = crypto.sign(&candidate.code_hash)?;

        // In a chaos system, signatures MUST differ over time/state
        if sig1 != sig2 {
            info!("Audit Passed: Chaos Entropy confirmed active.");
            let mut vault = self.vault.lock();
            vault.deploy_new(candidate)?;
            drop(vault);
            self.save_state()?;
        } else {
            error!("Audit Failed: Entropy Stagnation. Engaging Fallback 2.");
            drop(crypto); // Release lock
            self.fallback_golden_rollback(current)?;
        }
        Ok(())
    }

    /// Fallback 2: Golden Rollback
    fn fallback_golden_rollback(&self, current: &Crate) -> Result<()> {
        info!("--- ENGAGING FALLBACK 2: GOLDEN ROLLBACK ---");

        let mut vault = self.vault.lock();

        // Find last version in archive
        let backup = vault
            .archive
            .iter()
            .filter(|c| c.name == current.name)
            .last()
            .cloned();

        match backup {
            Some(golden) => {
                warn!("Restoring Golden Image: v{}", golden.version);
                vault
                    .active_crates
                    .insert(golden.name.clone(), golden.clone());
                println!(
                    "\x1b[31m!!! ALERT: ROLLED BACK {} TO v{} !!!\x1b[0m",
                    golden.name, golden.version
                );
            }
            None => error!("CRITICAL: No Golden State found. Locking component."),
        }
        drop(vault);
        self.save_state()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_agent_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let agent = SentinaelAgent::new(path).unwrap();
        assert!(agent.db_path == path);
    }

    #[test]
    fn test_build_candidate() {
        let temp_file = NamedTempFile::new().unwrap();
        let agent = SentinaelAgent::new(temp_file.path().to_str().unwrap()).unwrap();

        let old = Crate::new("test", "1.0.5");
        let candidate = agent.build_candidate(&old).unwrap();

        assert_eq!(candidate.version, "1.0.6");
    }
}
