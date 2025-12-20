use chrono::{DateTime, Utc};
/// Production-Grade HSM (Vault).
///
/// Features:
/// - Key Versioning (Rotation support).
/// - Strict Access Control Lists (ACLs).
/// - Immutable Audit Logging.
/// - HMAC-SHA256 Signing.
use fusion_std::error::{StdError, StdResult};
use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// Type alias for HMAC-SHA256
#[allow(dead_code)]
type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub key_id: String,
    pub user_context: String,
    pub success: bool,
}

#[derive(Clone)]
#[allow(dead_code)]
struct KeyVersion {
    version: u32,
    material: Vec<u8>,
    created_at: DateTime<Utc>,
    active: bool,
}

#[allow(dead_code)]
struct KeyObject {
    id: String,
    algorithm: String,
    versions: Vec<KeyVersion>,
    allowed_ops: Vec<String>, // e.g. ["Sign", "Verify", "Rotate"]
}

pub struct Vault {
    keys: Arc<RwLock<HashMap<String, KeyObject>>>,
    audit_log: Arc<RwLock<Vec<AuditEntry>>>,
}

impl Vault {
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
            audit_log: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Logs an action to the internal audit trail (Immutable append-only).
    fn log(&self, key_id: &str, action: &str, success: bool) {
        let entry = AuditEntry {
            timestamp: Utc::now(),
            action: action.to_string(),
            key_id: key_id.to_string(),
            user_context: "system".to_string(), // In real implementation, pass UserContext
            success,
        };
        if let Ok(mut log) = self.audit_log.write() {
            log.push(entry);
        }
    }

    /// Generates a new key with the specified algorithm and allowed operations.
    pub fn generate_key(&self, id: &str, algo: &str, ops: Vec<String>) -> StdResult<()> {
        let mut store = self
            .keys
            .write()
            .map_err(|_| StdError::PermissionDenied("Lock poisoned".into()))?;

        if store.contains_key(id) {
            self.log(id, "generate_key", false);
            return Err(StdError::PermissionDenied("Key ID already exists".into()));
        }

        let mut rng = rand::thread_rng();
        let mut material = vec![0u8; 32];
        rng.fill_bytes(&mut material);

        let initial_version = KeyVersion {
            version: 1,
            material,
            created_at: Utc::now(),
            active: true,
        };

        let key_obj = KeyObject {
            id: id.to_string(),
            algorithm: algo.to_string(),
            versions: vec![initial_version],
            allowed_ops: ops,
        };

        store.insert(id.to_string(), key_obj);

        // Release lock before logging to prevent potential deadlocks (though unlikely here)
        drop(store);
        self.log(id, "generate_key", true);
        Ok(())
    }

    /// Rotates the key by generating a new version.
    pub fn rotate_key(&self, id: &str) -> StdResult<u32> {
        let mut store = self
            .keys
            .write()
            .map_err(|_| StdError::PermissionDenied("Lock poisoned".into()))?;
        let key = store
            .get_mut(id)
            .ok_or(StdError::PermissionDenied("Key not found".into()))?;

        if !key.allowed_ops.contains(&"Rotate".to_string()) {
            drop(store);
            self.log(id, "rotate_key", false);
            return Err(StdError::PermissionDenied(format!(
                "Operation 'Rotate' not allowed for key {}",
                id
            )));
        }

        let mut rng = rand::thread_rng();
        let mut material = vec![0u8; 32];
        rng.fill_bytes(&mut material);

        let new_version_id = key.versions.len() as u32 + 1;
        let new_version = KeyVersion {
            version: new_version_id,
            material,
            created_at: Utc::now(),
            active: true,
        };

        key.versions.push(new_version);

        drop(store);
        self.log(id, "rotate_key", true);
        Ok(new_version_id)
    }

    /// Signs data using the latest active key version (HMAC-SHA256).
    pub fn sign(&self, id: &str, data: &[u8]) -> StdResult<Vec<u8>> {
        let store = self
            .keys
            .read()
            .map_err(|_| StdError::PermissionDenied("Lock poisoned".into()))?;
        let key = store
            .get(id)
            .ok_or(StdError::PermissionDenied("Key not found".into()))?;

        if !key.allowed_ops.contains(&"Sign".to_string()) {
            drop(store);
            self.log(id, "sign", false);
            return Err(StdError::PermissionDenied(format!(
                "Operation 'Sign' not allowed for key {}",
                id
            )));
        }

        // Use the latest version
        let latest = key
            .versions
            .last()
            .ok_or(StdError::PermissionDenied("No key versions found".into()))?;

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(&latest.material).map_err(|_| {
            StdError::Core(fusion_core::FusionError::CompilationError(
                "Key invalid length".into(),
            ))
        })?;

        mac.update(data);
        let result = mac.finalize().into_bytes();

        drop(store);
        self.log(id, "sign", true);
        Ok(result.to_vec())
    }

    pub fn get_audit_log(&self) -> Vec<AuditEntry> {
        self.audit_log.read().unwrap().clone()
    }
}
