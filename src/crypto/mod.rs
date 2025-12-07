// src/crypto/mod.rs - Cryptography Module

pub mod hybrid;

// These exports are available but currently unused in the compiler
// Uncomment when integrating hybrid cryptography into the compiler
// pub use hybrid::{
//     generate_hybrid_keypair, hybrid_kdf, hybrid_sign, hybrid_verify, AesKey, ClassicalKeypair,
//     HybridKeypair, HybridSignature, PQCKeypair,
// };
