//! NeuralMESH Vortex PQC Library
//!
//! Rust bindings for the HyperCycle Vortex v2.0 Post-Quantum Cryptography library.
//!
//! # Features
//!
//! - **Weave-SIG**: Quaternion-based digital signatures
//!   - Public Key: 96 bytes
//!   - Secret Key: 192 bytes
//!   - Signature: 96 bytes
//!
//! - **Weave-KEM**: Quaternion-based Key Encapsulation Mechanism
//!   - Public Key: 96 bytes
//!   - Secret Key: 192 bytes
//!   - Ciphertext: 128 bytes
//!   - Shared Secret: 32 bytes
//!
//! # Example
//!
//! ```rust
//! use neuralmesh_vortex_pqc::{WeaveSigKeypair, WeaveKemKeypair};
//!
//! // Generate signature keypair
//! let sig_keypair = WeaveSigKeypair::generate().unwrap();
//!
//! // Sign a message
//! let message = b"Hello, Post-Quantum World!";
//! let signature = sig_keypair.sign(message).unwrap();
//!
//! // Verify the signature
//! assert!(sig_keypair.verify(message, &signature).unwrap());
//!
//! // Generate KEM keypair
//! let kem_keypair = WeaveKemKeypair::generate().unwrap();
//!
//! // Encapsulate a shared secret
//! let (ciphertext, shared_secret) = kem_keypair.encapsulate().unwrap();
//!
//! // Decapsulate to recover the shared secret
//! let recovered_secret = kem_keypair.decapsulate(&ciphertext).unwrap();
//! assert_eq!(shared_secret, recovered_secret);
//! ```

#![warn(missing_docs)]

use std::fmt;
use thiserror::Error;

/// Key size constants for Weave-SIG
pub mod weave_sig_sizes {
    /// Weave-SIG public key size in bytes
    pub const PUBLIC_KEY: usize = 96;
    /// Weave-SIG secret key size in bytes
    pub const SECRET_KEY: usize = 192;
    /// Weave-SIG signature size in bytes (32 bytes R + 256 bytes response)
    pub const SIGNATURE: usize = 288;
}

/// Key size constants for Weave-KEM
pub mod weave_kem_sizes {
    /// Weave-KEM public key size in bytes
    pub const PUBLIC_KEY: usize = 96;
    /// Weave-KEM secret key size in bytes
    pub const SECRET_KEY: usize = 192;
    /// Weave-KEM ciphertext size in bytes
    pub const CIPHERTEXT: usize = 128;
    /// Weave-KEM shared secret size in bytes
    pub const SHARED_SECRET: usize = 32;
}

/// Error types for Vortex PQC operations
#[derive(Debug, Error)]
pub enum VortexError {
    /// Key generation failed
    #[error("Key generation failed")]
    KeyGenFailed,
    /// Signing failed
    #[error("Signing failed")]
    SignFailed,
    /// Signature verification failed
    #[error("Signature verification failed")]
    VerifyFailed,
    /// Encapsulation failed
    #[error("Encapsulation failed")]
    EncapsulateFailed,
    /// Decapsulation failed
    #[error("Decapsulation failed")]
    DecapsulateFailed,
    /// Invalid key size
    #[error("Invalid key size: expected {expected}, got {actual}")]
    InvalidKeySize {
        /// Expected size
        expected: usize,
        /// Actual size
        actual: usize,
    },
    /// Invalid signature size
    #[error("Invalid signature size: expected {expected}, got {actual}")]
    InvalidSignatureSize {
        /// Expected size
        expected: usize,
        /// Actual size
        actual: usize,
    },
    /// Invalid ciphertext size
    #[error("Invalid ciphertext size: expected {expected}, got {actual}")]
    InvalidCiphertextSize {
        /// Expected size
        expected: usize,
        /// Actual size
        actual: usize,
    },
    /// Entropy generation failed
    #[error("Entropy generation failed")]
    EntropyFailed,
}

/// Result type alias for Vortex operations
pub type Result<T> = std::result::Result<T, VortexError>;

// FFI declarations for Weave-SIG
#[repr(C)]
struct HcSigKeypair {
    public_key: [u8; weave_sig_sizes::PUBLIC_KEY],
    secret_key: [u8; weave_sig_sizes::SECRET_KEY],
}

#[repr(C)]
struct HcSignature {
    data: [u8; weave_sig_sizes::SIGNATURE],
}

extern "C" {
    fn hc_sig_keygen(kp: *mut HcSigKeypair) -> i32;
    fn hc_sig_sign(
        kp: *const HcSigKeypair,
        msg: *const u8,
        msg_len: usize,
        sig: *mut HcSignature,
    ) -> i32;
    fn hc_sig_verify(
        kp: *const HcSigKeypair,
        msg: *const u8,
        msg_len: usize,
        sig: *const HcSignature,
    ) -> i32;
}

// FFI declarations for Weave-KEM
#[repr(C)]
struct HcKemKeypair {
    public_key: [u8; weave_kem_sizes::PUBLIC_KEY],
    secret_key: [u8; weave_kem_sizes::SECRET_KEY],
}

#[repr(C)]
struct HcCiphertext {
    data: [u8; weave_kem_sizes::CIPHERTEXT],
}

#[repr(C)]
struct HcSharedSecret {
    data: [u8; weave_kem_sizes::SHARED_SECRET],
}

extern "C" {
    fn hc_kem_keygen(kp: *mut HcKemKeypair) -> i32;
    fn hc_kem_encaps(
        kp: *const HcKemKeypair,
        ct: *mut HcCiphertext,
        ss: *mut HcSharedSecret,
    ) -> i32;
    fn hc_kem_decaps(
        kp: *const HcKemKeypair,
        ct: *const HcCiphertext,
        ss: *mut HcSharedSecret,
    ) -> i32;
}

/// Weave-SIG keypair for digital signatures
///
/// # Key Sizes
/// - Public key: 96 bytes
/// - Secret key: 192 bytes
#[derive(Clone)]
pub struct WeaveSigKeypair {
    public_key: [u8; weave_sig_sizes::PUBLIC_KEY],
    secret_key: [u8; weave_sig_sizes::SECRET_KEY],
}

impl fmt::Debug for WeaveSigKeypair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WeaveSigKeypair")
            .field("public_key", &hex::encode(&self.public_key))
            .field("secret_key", &"[REDACTED]")
            .finish()
    }
}

impl WeaveSigKeypair {
    /// Generate a new Weave-SIG keypair
    ///
    /// # Returns
    /// * `Ok(WeaveSigKeypair)` - New keypair
    /// * `Err(VortexError)` - If key generation fails
    pub fn generate() -> Result<Self> {
        let mut kp = HcSigKeypair {
            public_key: [0u8; weave_sig_sizes::PUBLIC_KEY],
            secret_key: [0u8; weave_sig_sizes::SECRET_KEY],
        };

        let result = unsafe { hc_sig_keygen(&mut kp) };
        if result != 0 {
            return Err(VortexError::KeyGenFailed);
        }

        Ok(WeaveSigKeypair {
            public_key: kp.public_key,
            secret_key: kp.secret_key,
        })
    }

    /// Create a keypair from existing keys
    ///
    /// # Arguments
    /// * `public_key` - Public key (must be exactly 96 bytes)
    /// * `secret_key` - Secret key (must be exactly 192 bytes)
    ///
    /// # Returns
    /// * `Ok(WeaveSigKeypair)` - Keypair with provided keys
    /// * `Err(VortexError)` - If key sizes are invalid
    pub fn from_keys(public_key: &[u8], secret_key: &[u8]) -> Result<Self> {
        if public_key.len() != weave_sig_sizes::PUBLIC_KEY {
            return Err(VortexError::InvalidKeySize {
                expected: weave_sig_sizes::PUBLIC_KEY,
                actual: public_key.len(),
            });
        }
        if secret_key.len() != weave_sig_sizes::SECRET_KEY {
            return Err(VortexError::InvalidKeySize {
                expected: weave_sig_sizes::SECRET_KEY,
                actual: secret_key.len(),
            });
        }

        let mut pk = [0u8; weave_sig_sizes::PUBLIC_KEY];
        let mut sk = [0u8; weave_sig_sizes::SECRET_KEY];
        pk.copy_from_slice(public_key);
        sk.copy_from_slice(secret_key);

        Ok(WeaveSigKeypair {
            public_key: pk,
            secret_key: sk,
        })
    }

    /// Get the public key
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    /// Get the secret key (use with caution)
    pub fn secret_key(&self) -> &[u8] {
        &self.secret_key
    }

    /// Sign a message using Weave-SIG
    ///
    /// # Arguments
    /// * `message` - The message to sign
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Signature (96 bytes)
    /// * `Err(VortexError)` - If signing fails
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
        let kp = HcSigKeypair {
            public_key: self.public_key,
            secret_key: self.secret_key,
        };

        let mut sig = HcSignature {
            data: [0u8; weave_sig_sizes::SIGNATURE],
        };

        let result =
            unsafe { hc_sig_sign(&kp, message.as_ptr(), message.len(), &mut sig) };

        if result != 0 {
            return Err(VortexError::SignFailed);
        }

        Ok(sig.data.to_vec())
    }

    /// Verify a signature using Weave-SIG
    ///
    /// # Arguments
    /// * `message` - The original message
    /// * `signature` - The signature to verify
    ///
    /// # Returns
    /// * `Ok(true)` - Signature is valid
    /// * `Ok(false)` - Signature is invalid
    /// * `Err(VortexError)` - If verification fails (e.g., wrong signature size)
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool> {
        if signature.len() != weave_sig_sizes::SIGNATURE {
            return Err(VortexError::InvalidSignatureSize {
                expected: weave_sig_sizes::SIGNATURE,
                actual: signature.len(),
            });
        }

        let kp = HcSigKeypair {
            public_key: self.public_key,
            secret_key: self.secret_key,
        };

        let mut sig_data = [0u8; weave_sig_sizes::SIGNATURE];
        sig_data.copy_from_slice(signature);
        let sig = HcSignature { data: sig_data };

        let result =
            unsafe { hc_sig_verify(&kp, message.as_ptr(), message.len(), &sig) };

        Ok(result == 0)
    }
}

/// Weave-KEM keypair for key encapsulation
///
/// # Key Sizes
/// - Public key: 96 bytes
/// - Secret key: 192 bytes
#[derive(Clone)]
pub struct WeaveKemKeypair {
    public_key: [u8; weave_kem_sizes::PUBLIC_KEY],
    secret_key: [u8; weave_kem_sizes::SECRET_KEY],
}

impl fmt::Debug for WeaveKemKeypair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WeaveKemKeypair")
            .field("public_key", &hex::encode(&self.public_key))
            .field("secret_key", &"[REDACTED]")
            .finish()
    }
}

impl WeaveKemKeypair {
    /// Generate a new Weave-KEM keypair
    ///
    /// # Returns
    /// * `Ok(WeaveKemKeypair)` - New keypair
    /// * `Err(VortexError)` - If key generation fails
    pub fn generate() -> Result<Self> {
        let mut kp = HcKemKeypair {
            public_key: [0u8; weave_kem_sizes::PUBLIC_KEY],
            secret_key: [0u8; weave_kem_sizes::SECRET_KEY],
        };

        let result = unsafe { hc_kem_keygen(&mut kp) };
        if result != 0 {
            return Err(VortexError::KeyGenFailed);
        }

        Ok(WeaveKemKeypair {
            public_key: kp.public_key,
            secret_key: kp.secret_key,
        })
    }

    /// Create a keypair from existing keys
    ///
    /// # Arguments
    /// * `public_key` - Public key (must be exactly 96 bytes)
    /// * `secret_key` - Secret key (must be exactly 192 bytes)
    ///
    /// # Returns
    /// * `Ok(WeaveKemKeypair)` - Keypair with provided keys
    /// * `Err(VortexError)` - If key sizes are invalid
    pub fn from_keys(public_key: &[u8], secret_key: &[u8]) -> Result<Self> {
        if public_key.len() != weave_kem_sizes::PUBLIC_KEY {
            return Err(VortexError::InvalidKeySize {
                expected: weave_kem_sizes::PUBLIC_KEY,
                actual: public_key.len(),
            });
        }
        if secret_key.len() != weave_kem_sizes::SECRET_KEY {
            return Err(VortexError::InvalidKeySize {
                expected: weave_kem_sizes::SECRET_KEY,
                actual: secret_key.len(),
            });
        }

        let mut pk = [0u8; weave_kem_sizes::PUBLIC_KEY];
        let mut sk = [0u8; weave_kem_sizes::SECRET_KEY];
        pk.copy_from_slice(public_key);
        sk.copy_from_slice(secret_key);

        Ok(WeaveKemKeypair {
            public_key: pk,
            secret_key: sk,
        })
    }

    /// Get the public key
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    /// Get the secret key (use with caution)
    pub fn secret_key(&self) -> &[u8] {
        &self.secret_key
    }

    /// Encapsulate a shared secret using the public key
    ///
    /// # Returns
    /// * `Ok((Vec<u8>, Vec<u8>))` - Tuple of (ciphertext, shared_secret)
    ///   - Ciphertext: 128 bytes
    ///   - Shared secret: 32 bytes
    /// * `Err(VortexError)` - If encapsulation fails
    pub fn encapsulate(&self) -> Result<(Vec<u8>, Vec<u8>)> {
        let kp = HcKemKeypair {
            public_key: self.public_key,
            secret_key: self.secret_key,
        };

        let mut ct = HcCiphertext {
            data: [0u8; weave_kem_sizes::CIPHERTEXT],
        };
        let mut ss = HcSharedSecret {
            data: [0u8; weave_kem_sizes::SHARED_SECRET],
        };

        let result = unsafe { hc_kem_encaps(&kp, &mut ct, &mut ss) };
        if result != 0 {
            return Err(VortexError::EncapsulateFailed);
        }

        Ok((ct.data.to_vec(), ss.data.to_vec()))
    }

    /// Decapsulate to recover the shared secret
    ///
    /// # Arguments
    /// * `ciphertext` - The ciphertext to decapsulate (must be 128 bytes)
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Shared secret (32 bytes)
    /// * `Err(VortexError)` - If decapsulation fails
    pub fn decapsulate(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() != weave_kem_sizes::CIPHERTEXT {
            return Err(VortexError::InvalidCiphertextSize {
                expected: weave_kem_sizes::CIPHERTEXT,
                actual: ciphertext.len(),
            });
        }

        let kp = HcKemKeypair {
            public_key: self.public_key,
            secret_key: self.secret_key,
        };

        let mut ct_data = [0u8; weave_kem_sizes::CIPHERTEXT];
        ct_data.copy_from_slice(ciphertext);
        let ct = HcCiphertext { data: ct_data };

        let mut ss = HcSharedSecret {
            data: [0u8; weave_kem_sizes::SHARED_SECRET],
        };

        let result = unsafe { hc_kem_decaps(&kp, &ct, &mut ss) };
        if result != 0 {
            return Err(VortexError::DecapsulateFailed);
        }

        Ok(ss.data.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weave_sig_keypair_generation() {
        let keypair = WeaveSigKeypair::generate().unwrap();
        assert_eq!(keypair.public_key().len(), weave_sig_sizes::PUBLIC_KEY);
        assert_eq!(keypair.secret_key().len(), weave_sig_sizes::SECRET_KEY);
    }

    #[test]
    fn test_weave_sig_sign_and_verify() {
        let keypair = WeaveSigKeypair::generate().unwrap();
        let message = b"Test message for signing";

        let signature = keypair.sign(message).unwrap();
        assert_eq!(signature.len(), weave_sig_sizes::SIGNATURE);

        let is_valid = keypair.verify(message, &signature).unwrap();
        assert!(is_valid, "Valid signature should verify");

        // Test with wrong message
        let wrong_message = b"Different message";
        let is_valid_wrong = keypair.verify(wrong_message, &signature).unwrap();
        assert!(!is_valid_wrong, "Wrong message should not verify");
    }

    #[test]
    fn test_weave_kem_keypair_generation() {
        let keypair = WeaveKemKeypair::generate().unwrap();
        assert_eq!(keypair.public_key().len(), weave_kem_sizes::PUBLIC_KEY);
        assert_eq!(keypair.secret_key().len(), weave_kem_sizes::SECRET_KEY);
    }

    #[test]
    fn test_weave_kem_encapsulate_decapsulate() {
        let keypair = WeaveKemKeypair::generate().unwrap();

        let (ciphertext, shared_secret) = keypair.encapsulate().unwrap();
        assert_eq!(ciphertext.len(), weave_kem_sizes::CIPHERTEXT);
        assert_eq!(shared_secret.len(), weave_kem_sizes::SHARED_SECRET);

        let recovered_secret = keypair.decapsulate(&ciphertext).unwrap();
        assert_eq!(recovered_secret.len(), weave_kem_sizes::SHARED_SECRET);
        assert_eq!(shared_secret, recovered_secret, "Shared secrets should match");
    }

    #[test]
    fn test_key_sizes_constants() {
        // Weave-SIG
        assert_eq!(weave_sig_sizes::PUBLIC_KEY, 96);
        assert_eq!(weave_sig_sizes::SECRET_KEY, 192);
        assert_eq!(weave_sig_sizes::SIGNATURE, 96);

        // Weave-KEM
        assert_eq!(weave_kem_sizes::PUBLIC_KEY, 96);
        assert_eq!(weave_kem_sizes::SECRET_KEY, 192);
        assert_eq!(weave_kem_sizes::CIPHERTEXT, 128);
        assert_eq!(weave_kem_sizes::SHARED_SECRET, 32);
    }
}
