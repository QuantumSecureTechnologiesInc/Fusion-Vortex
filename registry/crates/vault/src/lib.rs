/// Production-Grade Mock HSM (Vault).
/// 
/// Features:
/// - Key Wrapping (Encryption at rest).
/// - Strict Access Control policies.
/// - Audit logging simulation.

use fusion_std::error::{StdResult, StdError};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// Simplified Key Object
struct KeyObject {
    material: Vec<u8>, // In real HSM: Encrypted with Master Key
    algorithm: String,
    allowed_ops: Vec<String>, // e.g. ["Sign", "Decrypt"]
}

pub struct Vault {
    keys: Arc<RwLock<HashMap<String, KeyObject>>>,
    master_key: Vec<u8>, // Simulation of hardware-backed key
}

impl Vault {
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
            master_key: vec![0xFE; 32], // Hardcoded mock master key
        }
    }

    pub fn generate_key(&self, id: &str, algo: &str, ops: Vec<String>) -> StdResult<()> {
        let mut store = self.keys.write().map_err(|_| StdError::PermissionDenied("Lock poisoned".into()))?;
        
        if store.contains_key(id) {
            return Err(StdError::PermissionDenied("Key ID already exists".into()));
        }

        // Generate random key material (Mock)
        let raw_key = vec![0xAA; 32];
        
        // Wrap Key (Encrypt with Master Key - XOR for demo simplicity)
        let wrapped_key: Vec<u8> = raw_key.iter().zip(self.master_key.iter().cycle())
            .map(|(k, m)| k ^ m)
            .collect();

        store.insert(id.to_string(), KeyObject {
            material: wrapped_key,
            algorithm: algo.to_string(),
            allowed_ops: ops,
        });
        
        println!("[Audit] Key '{}' generated. Algo: {}", id, algo);
        Ok(())
    }

    pub fn sign(&self, id: &str, _data: &[u8]) -> StdResult<Vec<u8>> {
        let store = self.keys.read().map_err(|_| StdError::PermissionDenied("Lock poisoned".into()))?;
        let key = store.get(id).ok_or(StdError::PermissionDenied("Key not found".into()))?;

        if !key.allowed_ops.contains(&"Sign".to_string()) {
            return Err(StdError::PermissionDenied(format!("Operation 'Sign' not allowed for key {}", id)));
        }

        // Unwrap Key
        let _unwrapped: Vec<u8> = key.material.iter().zip(self.master_key.iter().cycle())
            .map(|(k, m)| k ^ m)
            .collect();

        // Perform Sign (Simulated)
        println!("[Audit] Key '{}' used for Signing", id);
        Ok(vec![0xAA, 0xBB, 0xCC]) 
    }
}