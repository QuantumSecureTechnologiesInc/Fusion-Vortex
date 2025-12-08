// test_crypto_hybrid.fu - Integration tests for the Hybrid Cryptography Module

use fusion::crypto::{generate_hybrid_keypair, hybrid_kdf, hybrid_sign, hybrid_verify};
use fusion::runtime::assert_eq;

// Test 1: Key Derivation Function Determinism and Length
#[test]
fn test_hybrid_kdf_determinism():
    let ss_a = [0xAA; 32]; // Mock shared secret 1
    let ss_b = [0xBB; 32]; // Mock shared secret 2
    
    // Check that combining secrets produces a 32-byte key (SHA-3/256 output)
    let key1 = hybrid_kdf(&ss_a, &ss_b).expect("KDF failed");
    let key2 = hybrid_kdf(&ss_a, &ss_b).expect("KDF failed");
    
    assert_eq!(key1.len(), 32);
    assert_eq!(key1, key2);

// Test 2: Hybrid Sign/Verify Lifecycle (Success Case)
#[test]
fn test_hybrid_sign_verify_success():
    let message = b"Data integrity is quantum-secure.";
    let keys = generate_hybrid_keypair().expect("Key generation failed");
    
    // Sign the message using both key components
    let signature = hybrid_sign(
        message, 
        &keys.classical_sig.private_key, 
        &keys.pqc_sig.private_key
    ).expect("Hybrid signing failed");
    
    // Verify using both public keys (both must pass)
    let is_valid = hybrid_verify(
        message, 
        &signature, 
        &keys.classical_sig.public_key, 
        &keys.pqc_sig.public_key
    ).expect("Verification check failed");
    
    assert_eq!(is_valid, true, "Verification must pass when signatures are valid.");

// Test 3: Hybrid Verify (Failure Case - Tampered Message)
#[test]
fn test_hybrid_verify_tampered_message():
    let message = b"Data integrity is quantum-secure.";
    let tampered_message = b"Data integrity is compromised!";
    let keys = generate_hybrid_keypair().expect("Key generation failed");
    
    // Generate valid signature for the original message
    let signature = hybrid_sign(
        message, 
        &keys.classical_sig.private_key, 
        &keys.pqc_sig.private_key
    ).expect("Hybrid signing failed");
    
    // Verify using the tampered message (should fail verification)
    let is_valid = hybrid_verify(
        tampered_message, 
        &signature, 
        &keys.classical_sig.public_key, 
        &keys.pqc_sig.public_key
    ).expect("Verification check failed");
    
    assert_eq!(is_valid, false, "Verification must fail when the message is tampered.");