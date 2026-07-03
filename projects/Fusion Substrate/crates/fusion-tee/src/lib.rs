use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use thiserror::Error;

/// TEE errors
#[derive(Debug, Error)]
pub enum TeeError {
    #[error("Enclave not initialized")]
    NotInitialized,

    #[error("Attestation failed: {0}")]
    AttestationFailed(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Sealing failed: {0}")]
    SealingFailed(String),

    #[error("Unsealing failed: {0}")]
    UnsealingFailed(String),
}

/// Enclave measurement (code hash)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Measurement {
    pub hash: String,
    pub timestamp: u64,
}

/// Attestation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationReport {
    pub measurement: Measurement,
    pub platform_info: PlatformInfo,
    pub signature: String,
}

/// Platform information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub tee_type: String,
    pub version: String,
    pub security_version: u32,
}

/// Sealed data blob
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedData {
    pub ciphertext: Vec<u8>,
    pub measurement: Measurement,
    pub nonce: Vec<u8>,
}

/// Attestation provider trait for pluggable attestation backends
pub trait AttestationProvider {
    /// Get the runtime identity (e.g., TPM quote, SGX report)
    fn get_runtime_identity(&self) -> Result<String, TeeError>;

    /// Verify the runtime environment is trusted
    fn verify_environment(&self) -> Result<bool, TeeError>;

    /// Emit an attestation claim for external verification
    fn emit_claim(&self, data: &[u8]) -> Result<String, TeeError>;
}

/// Simple software-based attestation provider (for development)
pub struct SoftwareAttestationProvider {
    runtime_id: String,
}

impl SoftwareAttestationProvider {
    pub fn new() -> Self {
        // Generate a stable runtime ID based on the binary
        let runtime_id = format!("software-runtime-{}", std::process::id());
        Self { runtime_id }
    }
}

impl AttestationProvider for SoftwareAttestationProvider {
    fn get_runtime_identity(&self) -> Result<String, TeeError> {
        Ok(self.runtime_id.clone())
    }

    fn verify_environment(&self) -> Result<bool, TeeError> {
        // In software mode, we always trust the environment
        Ok(true)
    }

    fn emit_claim(&self, data: &[u8]) -> Result<String, TeeError> {
        let mut hasher = Sha3_256::new();
        hasher.update(&self.runtime_id);
        hasher.update(data);
        Ok(hex::encode(hasher.finalize()))
    }
}

/// Simulated hardware-bound attestation provider (e.g., TPM/SGX)
pub struct HardwareAttestationProvider {
    platform_key_id: String,
    security_version: u32,
}

impl HardwareAttestationProvider {
    pub fn new(platform_key: &str) -> Self {
        Self {
            platform_key_id: platform_key.to_string(),
            security_version: 2, // Production-grade SVN
        }
    }
}

impl AttestationProvider for HardwareAttestationProvider {
    fn get_runtime_identity(&self) -> Result<String, TeeError> {
        // In a real TEE, this would sign a quote with the AK (Attestation Key)
        let mut hasher = Sha3_256::new();
        hasher.update(self.platform_key_id.as_bytes());
        hasher.update(b":AK");
        Ok(format!(
            "hw-attested-{}",
            hex::encode(hasher.finalize())[..16].to_string()
        ))
    }

    fn verify_environment(&self) -> Result<bool, TeeError> {
        // Simulate checking for debuggers, side-channel mitigations, etc.
        // In this simulation, we check if we're in a "safe" mock environment
        let is_debug = std::env::var("FUSION_DEBUG_TEE").is_ok();

        if is_debug {
            return Err(TeeError::AttestationFailed(
                "Insecure environment detected: Debug mode active".to_string(),
            ));
        }

        Ok(true)
    }

    fn emit_claim(&self, data: &[u8]) -> Result<String, TeeError> {
        if !self.verify_environment()? {
            return Err(TeeError::AttestationFailed(
                "Environment check failed before emission".to_string(),
            ));
        }

        let mut hasher = Sha3_256::new();
        hasher.update(self.platform_key_id.as_bytes());
        hasher.update(data);
        hasher.update(&self.security_version.to_le_bytes());

        // Return a "hardware-signed" claim hash
        Ok(hex::encode(hasher.finalize()))
    }
}

/// Trusted Execution Environment enclave
///
/// Provides:
/// - Code isolation and execution
/// - Remote attestation
/// - Sealed storage for secrets
///
/// Note: This is an abstraction layer. In production, this would integrate with
/// Intel SGX, ARM TrustZone, or AMD SEV.
pub struct TeeEnclave {
    initialized: bool,
    measurement: Option<Measurement>,
    sealed_storage: HashMap<String, SealedData>,
    platform_info: PlatformInfo,
}

impl TeeEnclave {
    /// Create a new TEE enclave
    pub fn new() -> Self {
        Self {
            initialized: false,
            measurement: None,
            sealed_storage: HashMap::new(),
            platform_info: PlatformInfo {
                tee_type: "Fusion-TEE-Abstract".to_string(),
                version: "1.0.0".to_string(),
                security_version: 1,
            },
        }
    }

    /// Initialize the enclave with code measurement
    pub fn initialize(&mut self, code: &[u8]) -> Result<Measurement, TeeError> {
        let mut hasher = Sha3_256::new();
        hasher.update(code);
        let hash = hasher.finalize();

        let measurement = Measurement {
            hash: hex::encode(hash),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        self.measurement = Some(measurement.clone());
        self.initialized = true;

        Ok(measurement)
    }

    /// Generate an attestation report
    pub fn attest(&self) -> Result<AttestationReport, TeeError> {
        if !self.initialized {
            return Err(TeeError::NotInitialized);
        }

        let measurement = self.measurement.as_ref().unwrap().clone();

        // In production, this would use hardware-backed signing
        let signature_data = format!("{}{}", measurement.hash, measurement.timestamp);
        let mut hasher = Sha3_256::new();
        hasher.update(signature_data.as_bytes());
        let signature = hex::encode(hasher.finalize());

        Ok(AttestationReport {
            measurement,
            platform_info: self.platform_info.clone(),
            signature,
        })
    }

    /// Verify an attestation report
    pub fn verify_attestation(report: &AttestationReport) -> Result<(), TeeError> {
        // Recompute signature
        let signature_data = format!(
            "{}{}",
            report.measurement.hash, report.measurement.timestamp
        );
        let mut hasher = Sha3_256::new();
        hasher.update(signature_data.as_bytes());
        let expected_signature = hex::encode(hasher.finalize());

        if report.signature != expected_signature {
            return Err(TeeError::AttestationFailed("Invalid signature".to_string()));
        }

        Ok(())
    }

    /// Execute code in the enclave
    pub fn execute<F, R>(&self, func: F) -> Result<R, TeeError>
    where
        F: FnOnce() -> Result<R, String>,
    {
        if !self.initialized {
            return Err(TeeError::NotInitialized);
        }

        func().map_err(|e| TeeError::ExecutionFailed(e))
    }

    /// Seal data (encrypt and bind to enclave measurement)
    pub fn seal(&mut self, key: String, data: &[u8]) -> Result<(), TeeError> {
        if !self.initialized {
            return Err(TeeError::NotInitialized);
        }

        let measurement = self.measurement.as_ref().unwrap().clone();

        // In production, this would use hardware-backed encryption
        // For now, we simulate sealing with XOR (DO NOT use in production!)
        let nonce: Vec<u8> = (0..16).map(|i| (i * 7 + 13) as u8).collect();
        let mut ciphertext = data.to_vec();
        for (i, byte) in ciphertext.iter_mut().enumerate() {
            *byte ^= nonce[i % nonce.len()];
        }

        let sealed = SealedData {
            ciphertext,
            measurement,
            nonce,
        };

        self.sealed_storage.insert(key, sealed);
        Ok(())
    }

    /// Unseal data (decrypt and verify measurement)
    pub fn unseal(&self, key: &str) -> Result<Vec<u8>, TeeError> {
        if !self.initialized {
            return Err(TeeError::NotInitialized);
        }

        let sealed = self
            .sealed_storage
            .get(key)
            .ok_or_else(|| TeeError::UnsealingFailed(format!("Key {} not found", key)))?;

        // Verify measurement matches
        if let Some(current_measurement) = &self.measurement {
            if sealed.measurement.hash != current_measurement.hash {
                return Err(TeeError::UnsealingFailed(
                    "Measurement mismatch - data sealed by different enclave".to_string(),
                ));
            }
        }

        // Decrypt (reverse XOR)
        let mut plaintext = sealed.ciphertext.clone();
        for (i, byte) in plaintext.iter_mut().enumerate() {
            *byte ^= sealed.nonce[i % sealed.nonce.len()];
        }

        Ok(plaintext)
    }

    /// Check if enclave is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get current measurement
    pub fn get_measurement(&self) -> Option<&Measurement> {
        self.measurement.as_ref()
    }
}

impl Default for TeeEnclave {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enclave_initialization() {
        let mut enclave = TeeEnclave::new();
        assert!(!enclave.is_initialized());

        let code = b"fn main() { println!(\"Hello\"); }";
        let measurement = enclave.initialize(code).unwrap();

        assert!(enclave.is_initialized());
        assert_eq!(enclave.get_measurement(), Some(&measurement));
    }

    #[test]
    fn test_attestation() {
        let mut enclave = TeeEnclave::new();
        let code = b"test code";
        enclave.initialize(code).unwrap();

        let report = enclave.attest().unwrap();
        assert!(!report.signature.is_empty());
        assert_eq!(report.platform_info.tee_type, "Fusion-TEE-Abstract");
    }

    #[test]
    fn test_attestation_verification() {
        let mut enclave = TeeEnclave::new();
        enclave.initialize(b"test code").unwrap();

        let report = enclave.attest().unwrap();
        assert!(TeeEnclave::verify_attestation(&report).is_ok());
    }

    #[test]
    fn test_seal_and_unseal() {
        let mut enclave = TeeEnclave::new();
        enclave.initialize(b"enclave code").unwrap();

        let secret_data = b"my secret password";
        enclave.seal("password".to_string(), secret_data).unwrap();

        let unsealed = enclave.unseal("password").unwrap();
        assert_eq!(unsealed, secret_data);
    }

    #[test]
    fn test_unseal_nonexistent_key() {
        let mut enclave = TeeEnclave::new();
        enclave.initialize(b"code").unwrap();

        let result = enclave.unseal("nonexistent");
        assert!(matches!(result, Err(TeeError::UnsealingFailed(_))));
    }

    #[test]
    fn test_execute_in_enclave() {
        let mut enclave = TeeEnclave::new();
        enclave.initialize(b"code").unwrap();

        let result = enclave
            .execute(|| {
                let x = 5 + 3;
                Ok(x)
            })
            .unwrap();

        assert_eq!(result, 8);
    }

    #[test]
    fn test_execute_failure() {
        let mut enclave = TeeEnclave::new();
        enclave.initialize(b"code").unwrap();

        let result: Result<i32, _> = enclave.execute(|| Err("computation failed".to_string()));

        assert!(matches!(result, Err(TeeError::ExecutionFailed(_))));
    }

    #[test]
    fn test_measurement_consistency() {
        let mut enclave1 = TeeEnclave::new();
        let mut enclave2 = TeeEnclave::new();

        let code = b"same code";
        let measurement1 = enclave1.initialize(code).unwrap();
        let measurement2 = enclave2.initialize(code).unwrap();

        assert_eq!(measurement1.hash, measurement2.hash);
    }

    #[test]
    fn test_seal_before_init() {
        let mut enclave = TeeEnclave::new();
        let result = enclave.seal("key".to_string(), b"data");
        assert!(matches!(result, Err(TeeError::NotInitialized)));
    }
}
