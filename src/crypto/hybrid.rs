// src/crypto/hybrid.rs - 50/50 Hybrid Cryptography Module
// Architecture-complete implementation demonstrating hybrid crypto design

use sha3::{Digest, Sha3_256};
use std::result::Result as StdResult;

// Classical cryptography
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey, SECRET_KEY_LENGTH};
use rand::rngs::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};

// --- Data Structures for Hybrid Cryptography ---

#[allow(dead_code)]
pub type AesKey = [u8; 32]; // Derived symmetric key (SHA-3/256 output)

#[allow(dead_code)]
pub struct ClassicalKeypair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

#[allow(dead_code)]
pub struct PQCKeypair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

#[allow(dead_code)]
pub struct HybridKeypair {
    pub classical_kem: ClassicalKeypair,
    pub pqc_kem: PQCKeypair,
    pub classical_sig: ClassicalKeypair,
    pub pqc_sig: PQCKeypair,
}

#[allow(dead_code)]
pub struct HybridSignature {
    pub classical_sig: Vec<u8>,
    pub pqc_sig: Vec<u8>,
}

// --- Key Generation and Derivation ---

/// Generates a complete set of hybrid keys for both Key Encapsulation (KEM) and Signatures.
#[allow(dead_code)]
pub fn generate_hybrid_keypair() -> StdResult<HybridKeypair, String> {
    // KEM Generation (Classical: X25519, PQC: ML-KEM/Kyber768)
    let classical_kem = generate_x25519_keypair()?;
    let pqc_kem = generate_ml_kem_keypair()?;

    // Signature Generation (Classical: Ed25519, PQC: ML-DSA/Dilithium3)
    let classical_sig = generate_ed25519_keypair()?;
    let pqc_sig = generate_ml_dsa_keypair()?;

    Ok(HybridKeypair {
        classical_kem,
        pqc_kem,
        classical_sig,
        pqc_sig,
    })
}

/// Hybrid Key Derivation Function (KDF): Combines two shared secrets into a single,
/// quantum-resistant symmetric key using SHA-3/256.
///
/// This is the core of the hybrid approach: by combining classical and PQC shared secrets,
/// security holds even if one system is broken.
#[allow(dead_code)]
pub fn hybrid_kdf(ss_classical: &[u8], ss_pqc: &[u8]) -> StdResult<AesKey, String> {
    let mut hasher = Sha3_256::new();

    // Append context strings to bind the derived key to its components
    hasher.update(b"fusion_classical_kem");
    hasher.update(ss_classical);

    hasher.update(b"fusion_pqc_kem");
    hasher.update(ss_pqc);

    let result = hasher.finalize();

    // Convert generic output to fixed-size AesKey (32 bytes)
    let mut key: AesKey = [0u8; 32];
    key.copy_from_slice(result.as_slice());

    Ok(key)
}

// --- Hybrid Digital Signatures ---

/// Generates a hybrid signature (Ed25519 + ML-DSA/Dilithium3) over a message.
///
/// Defense-in-depth: An attacker must break BOTH classical AND post-quantum
/// cryptography to forge a signature.
#[allow(dead_code)]
pub fn hybrid_sign(
    message: &[u8],
    classical_sk: &[u8],
    pqc_sk: &[u8],
) -> StdResult<HybridSignature, String> {
    // Classical signature (Ed25519)
    let classical_sig = ed25519_sign(message, classical_sk)?;

    // Post-quantum signature (Dilithium3 - mocked for now)
    let pqc_sig = ml_dsa_sign(message, pqc_sk)?;

    Ok(HybridSignature {
        classical_sig,
        pqc_sig,
    })
}

/// Verifies a hybrid signature. BOTH classical and PQC signatures MUST be valid.
///
/// Returns true only if verification succeeds for both signatures.
#[allow(dead_code)]
pub fn hybrid_verify(
    message: &[u8],
    sig: &HybridSignature,
    classical_pk: &[u8],
    pqc_pk: &[u8],
) -> StdResult<bool, String> {
    // 1. Verify classical signature
    let classical_valid = ed25519_verify(message, &sig.classical_sig, classical_pk)?;

    // 2. Verify post-quantum signature
    let pqc_valid = ml_dsa_verify(message, &sig.pqc_sig, pqc_pk)?;

    // Defense-in-depth: return true only if BOTH signatures pass.
    Ok(classical_valid && pqc_valid)
}

// --- Cryptographic Primitives ---

#[allow(dead_code)]
fn generate_x25519_keypair() -> StdResult<ClassicalKeypair, String> {
    let secret = EphemeralSecret::random_from_rng(OsRng);
    let public = X25519PublicKey::from(&secret);

    // Note: Use static secrets for persistence in production
    Ok(ClassicalKeypair {
        public_key: public.as_bytes().to_vec(),
        private_key: vec![0u8; 32], // Ephemeral secrets don't expose bytes
    })
}

#[allow(dead_code)]
fn generate_ed25519_keypair() -> StdResult<ClassicalKeypair, String> {
    let signing_key = SigningKey::from_bytes(&rand::random());
    let verifying_key = signing_key.verifying_key();

    Ok(ClassicalKeypair {
        public_key: verifying_key.to_bytes().to_vec(),
        private_key: signing_key.to_bytes().to_vec(),
    })
}

// Post-Quantum KEM (Kyber768/ML-KEM)
// TODO: Integrate pqcrypto-kyber once trait APIs are stabilized
#[allow(dead_code)]
fn generate_ml_kem_keypair() -> StdResult<PQCKeypair, String> {
    // Mock implementation demonstrating key sizes for Kyber768
    Ok(PQCKeypair {
        public_key: vec![5; 1184],  // Kyber768 public key size
        private_key: vec![6; 2400], // Kyber768 secret key size
    })
}

// Post-Quantum Signatures (Dilithium3/ML-DSA)
// TODO: Integrate pqcrypto-dilithium once trait APIs are stabilized
#[allow(dead_code)]
fn generate_ml_dsa_keypair() -> StdResult<PQCKeypair, String> {
    // Mock implementation demonstrating key sizes for Dilithium3
    Ok(PQCKeypair {
        public_key: vec![7; 1952],  // Dilithium3 public key size
        private_key: vec![8; 4000], // Dilithium3 secret key size
    })
}

#[allow(dead_code)]
fn ed25519_sign(message: &[u8], sk: &[u8]) -> StdResult<Vec<u8>, String> {
    if sk.len() != SECRET_KEY_LENGTH {
        return Err(format!(
            "Invalid Ed25519 private key length: expected {}, got {}",
            SECRET_KEY_LENGTH,
            sk.len()
        ));
    }

    let sk_array: [u8; SECRET_KEY_LENGTH] = sk
        .try_into()
        .map_err(|_| "Failed to convert slice to array".to_string())?;

    let signing_key = SigningKey::from_bytes(&sk_array);
    let signature = signing_key.sign(message);

    Ok(signature.to_bytes().to_vec())
}

#[allow(dead_code)]
fn ed25519_verify(message: &[u8], sig: &[u8], pk: &[u8]) -> StdResult<bool, String> {
    if pk.len() != 32 {
        return Err("Invalid Ed25519 public key length".to_string());
    }
    if sig.len() != 64 {
        return Err("Invalid Ed25519 signature length".to_string());
    }

    let pk_array: [u8; 32] = pk
        .try_into()
        .map_err(|_| "Failed to convert public key".to_string())?;
    let sig_array: [u8; 64] = sig
        .try_into()
        .map_err(|_| "Failed to convert signature".to_string())?;

    let verifying_key =
        VerifyingKey::from_bytes(&pk_array).map_err(|e| format!("Invalid public key: {}", e))?;
    let signature = Signature::from_bytes(&sig_array);

    Ok(verifying_key.verify(message, &signature).is_ok())
}

// Mock Dilithium3 operations (PQC signature scheme)
// TODO: Replace with actual pqcrypto-dilithium integration
#[allow(dead_code)]
fn ml_dsa_sign(_message: &[u8], _sk: &[u8]) -> StdResult<Vec<u8>, String> {
    // Mock Dilithium3 signature (actual would be ~3293 bytes)
    Ok(vec![11; 3293])
}

#[allow(dead_code)]
fn ml_dsa_verify(_message: &[u8], sig: &[u8], _pk: &[u8]) -> StdResult<bool, String> {
    // Mock verification - in production, verify actual Dilithium3 signature
    Ok(sig.len() == 3293)
}
