//! # Sentinael Tri-Brid
//!
//! Autonomous update agent with chaos-enhanced security mesh.
//!
//! ## Features
//!
//! - **Chaos Math Engine**: Deterministic entropy source using logistic maps
//! - **Oscillating Security Mesh**: Moving Target Defence (MTD)
//! - **Chaos-Enhanced Cryptography**: HMAC-SHA512 with chaos-seeded keys
//! - **Tri-Brid Redundancy**: Three-tier fallback system
//! - **Autonomous Operation**: Self-healing update orchestrator
//!
//! ## Architecture
//!
//! The system employs three distinct algorithms to guarantee stability:
//!
//! 1. **Apex (Speed)**: Heuristic, fast-path update engine
//! 2. **Audit (Safety)**: Strict, double-blind verification protocol (Fallback 1)
//! 3. **Golden (Resilience)**: Immutable rollback mechanism (Fallback 2)

pub mod agent;
pub mod chaos;
pub mod crypto;
pub mod mesh;
pub mod vault;

pub use agent::SentinaelAgent;
pub use chaos::ChaosEngine;
pub use crypto::CryptoVerifier;
pub use mesh::{OscillatingMesh, SecurityState};
pub use vault::{ArtifactVault, Crate};

/// Re-export commonly used types
pub use anyhow::{anyhow, Context, Result};
