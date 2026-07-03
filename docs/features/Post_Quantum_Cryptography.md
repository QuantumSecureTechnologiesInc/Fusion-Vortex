> **Phase 0 audit (2026-06-24) found this doc overclaims reality.**
> Treat feature lists here as roadmap, not current state.
> See `docs-truth-audit/TRUTH_REPORT.md` for details.

# Post-Quantum Cryptography

## Overview
Fusion assumes a post-quantum world. Standard cryptographic primitives in the language (TLS, SSH, Signatures) utilize **NIST-standardized post-quantum algorithms** by default, ensuring data remains secure even against future quantum computer attacks.

## Supported Primitives

### 🔑 Key Encapsulation (KEM)
- **ML-KEM (Kyber)**: The primary algorithm for secure key exchange.
- **McEliece**: Classic, conservative backup for long-term storage.

### ✍️ Digital Signatures
- **ML-DSA (Dilithium)**: Fast signatures for general use.
- **SPHINCS+**: Stateless hash-based signatures for high-security applications.

### 🛡️ Novel Architectures
- **Vortex Entropy Engine**: A native chaotic entropy generator (`src/stdlib/vortex.fu`) providing high-throughput, self-healing randomness.
- **Chaos Quaternion Cryptography (CQC)**: A Fusion-exclusive approach using chaos theory for lightweight encryption.
- **Hybrid Mode**: Combines Classical (ECC/RSA) + Post-Quantum algorithms to satisfy compliance requirements (FIPS, etc.) while guaranteeing future security.

## Zero-Trust by Default
The standard library implements "Secure by Design" principles:
- **No Unsafe Defaults**: TLS 1.3+ only.
- **Identity Management**: Built-in JWT and OIDC support.
- **Secret Hygiene**: Memory-protected types for keys (preventing swap-to-disk or core dumps).

## Example

```fusion
use std::vortex;

// Initialize the Vortex Entropy Engine
let ctx = vortex::VortexContext::new()?;

// Generate a NIST-compliant PQC seed with self-healing
let seed = ctx.generate_seed_safe()?;

// Use seed for ML-KEM key generation
let (pk, sk) = ml_kem_1024::keypair_from_seed(&seed);
```
