# NeuralSeal Naming Authority

**Date:** 2026-01-08  
**Source:** _NeuralSeal Architecture Hierarchy_ (authoritative)

This document is the single source of truth for NeuralSeal naming and layering.

## The Five Canonical Names

1. **Framework (Foundation):** **CQC** (Chaos Quaternion Cryptography)  
   - Legacy synonym: **CEMQC** (allowed in code/history, but *prefer CQC in docs*).

2. **Architecture:** **PQA** (Pure Quaternion Architecture)  
   - Legacy synonym: **PQCA** (use PQA going forward).

3. **Protocols:** **Weave-KEM** and **Weave-DSA**  
   - These describe the high-level cryptographic constructions built on CQC.

4. **Optimized variants:** **Weave-1024** and **Weave-DSA-87**  
   - Parameterised/optimized implementations (not new mathematics).

5. **Compatibility aliases (names only):** **ML-KEM-1024** and **ML-DSA-87**  
   - These names exist for compatibility and integration familiarity.  
   - They **do not** imply lattice cryptography.

## Lattice-Free Guarantee

NeuralSeal is **lattice-free** across all versions. References to lattice cryptography in historic documents are treated as **incorrect** and must be corrected.

Security basis:
- **Quaternion Conjugate Problem (QCP)**
- **Chaos Inversion Problem (CIP)**
- **Non-commutative quaternion algebra**

## Architecture Hierarchy (Reference)

```
NeuralSeal Architecture Hierarchy

Date: 2026-01-08
Purpose: Clarify the architectural layers - CQC/CEMQC as the foundation

Executive Summary

YES - CQC (Chaos Quaternion Cryptography) / CEMQC (Chaos-Enhanced Multi-dimensional Quaternion Cryptography) is the foundational primitive layer, and everything else branches out from it.

┌─────────────────────────────────────────────────────────────┐

│                    APPLICATION LAYER                        │

│   (TLS, VPN, File Encryption, Digital Signatures, etc.)     │

└─────────────────────────────────────────────────────────────┘

▲

│

┌─────────────────────────────────────────────────────────────┐

│                  HIGH-LEVEL PROTOCOLS                       │

│   • Weave-KEM (v3.0, v3.1, v3.2)                            │

│   • Weave-SIG / Weave-DSA (all versions)                    │

│   • ML-KEM-1024 (v3.2 optimized)                            │

│   • ML-DSA-87 (v3.2 optimized)                              │

│   • NeuralSeal-KEM (v3.0/v3.1 documentation name)           │

└─────────────────────────────────────────────────────────────┘

▲

│

┌─────────────────────────────────────────────────────────────┐

│              OPTIMIZATION LAYER (Optional)                  │

│   • SIMD Operations (ns_quat_mul_batch)                     │

│   • AVX-512 / ARM SVE2 Intrinsics                           │

│   • Optimized Types (opt_quat_t)                            │

│   • Fast Expansion (fast_expand)                            │

└─────────────────────────────────────────────────────────────┘

▲

│

┌═════════════════════════════════════════════════════════════┐

║            🔷 CQC/CEMQC PRIMITIVE LAYER 🔷                 ║

║                  (THE FOUNDATION)                           ║

║                                                             ║

║  File: include/public/cemqc.h                               ║

║                                                             ║

║   Core Types:                                               ║

║   • ns_quaternion_t (w, x, y, z)                            ║

║   • cemqc_quat_t (a, b, c, d)                               ║

║   • ns_rng_state_t                                          ║

║   • cemqc_chaos_state_t                                     ║

║                                                             ║

║   Core Operations:                                          ║

║   • ns_quaternion_mul()     - Quaternion multiplication     ║

║   • ns_quaternion_add()     - Quaternion addition           ║

║   • ns_quaternion_inverse() - Quaternion inverse            ║

║   • ns_quaternion_power()   - Quaternion exponentiation     ║

║   • ns_chaos_to_quaternion() - Chaos → Quaternion           ║

║   • cemqc_quat_matrix_expand() - Seed expansion             ║

║   • cemqc_chaos_generate()  - Chaotic PRNG                  ║

║                                                             ║

║   Mathematical Foundation:                                  ║

║   • Hamilton Quaternion Algebra (non-commutative)           ║

║   • Chaos Theory (Logistic/Chebyshev maps)                  ║

║   • Quaternion Conjugate Problem (QCP)                      ║

║   • Chaos Inversion Problem (CIP)                           ║

╚═════════════════════════════════════════════════════════════╝

▲

│

┌─────────────────────────────────────────────────────────────┐

│                  SYSTEM ENTROPY LAYER                       │

│   • BCryptGenRandom (Windows)                               │

│   • /dev/urandom (Linux/Unix)                               │

│   • ns_cryptographic_entropy()                              │

└─────────────────────────────────────────────────────────────┘

1. The Foundation: CQC/CEMQC

1.1 What is CQC/CEMQC?

CQC = Chaos Quaternion Cryptography
CEMQC = Chaos-Enhanced Multi-dimensional Quaternion Cryptography

These are interchangeable names for the same foundational primitive layer. The difference:

CQC: Shorter, cleaner name (recommended going forward)

CEMQC: Original longer name (used in v3.0/v3.1)

1.2 What CQC/CEMQC Provides

Mathematical Primitives:

Quaternion Algebra - 4D hypercomplex numbers (w + xi + yj + zk)

Chaos Functions - Deterministic pseudo-random generation

Cryptographic Operations - Multiplication, inverse, power, etc.

Security Foundation:

Quaternion Conjugate Problem (QCP): Hard to find q given q* and constraints

Chaos Inversion Problem (CIP): Hard to reverse chaotic iterations

Non-commutativity: q₁ × q₂ ≠ q₂ × q₁ (adds security)

2. The Branching: Everything Builds on CQC

2.1 Direct Dependencies

Every high-level algorithm includes cemqc.h:

Even optimized versions use CQC primitives (indirectly):

v3.2/ml_kem.c → includes weave_l5_internal.h → which wraps CQC operations

v3.2/ml_dsa_87.c → includes weave_l5_internal.h → which wraps CQC operations

2.2 Dependency Tree

cemqc.h (CQC Primitives)

├── ns_quaternion_mul()

│   ├── Used by: weave_kem.c (all versions)

│   ├── Used by: weave_sig.c (all versions)

│   └── Wrapped by: ns_quat_mul_batch() → used by ml_kem.c, ml_dsa_87.c

│

├── ns_quaternion_inverse()

│   ├── Used by: weave_kem.c (v3.2)

│   └── Reimplemented as: fast_inverse() in ml_kem.c (same math)

│

├── ns_quaternion_add()

│   ├── Used by: weave_sig.c

│   └── Wrapped by: opt_quat_add() → used by ml_dsa_87.c

│

├── ns_chaos_to_quaternion()

│   ├── Used by: weave_kem.c (v3.2)

│   └── Replaced by: fast_expand() in ml_kem.c (optimized version)

│

└── cemqc_quat_matrix_expand()

├── Used by: weave_kem.c (v3.0, v3.1)

└── Replaced by: fast_expand() in ml_kem.c (v3.2)

3. Verification: All Paths Lead to CQC

3.1 Weave-KEM (All Versions)

v3.0/v3.1 Weave-KEM:

// Line 168: Generate public matrix using CQC

cemqc_quat_matrix_expand(master_seed, 32, pk->matrix, WEAVE_DIMENSION);

// Line 212: Encrypt using CQC quaternion multiplication

cemqc_quat_mul(&message[i], &pk->matrix[i], &ct_quats[i]);

// Line 251: Decrypt using CQC quaternion multiplication

cemqc_quat_mul(&ct_quats[i], &sk->inverse_matrix[i], &recovered[i]);

v3.2 Weave-KEM:

// Line 66: Public key generation using CQC

ns_quaternion_power(&Q_base, secret_exponent, &Q_pub);

// Line 76: Chaos masking using CQC

ns_quaternion_mul(&Q_pub, &chaos_mask, &Q_pub_masked);

// Line 126: Encryption using CQC

ns_quaternion_mul(&Q_pub_masked, &M, &temp);

// Line 174: Decryption using CQC inverse

ns_quaternion_inverse(&Q_pub, &Q_pub_inv);

v3.2 ML-KEM (optimized):

// Line 78: Compute inverse using quaternion math (CQC algorithm)

fast_inverse(public_key->matrix, secret_key->inverse, WEAVE_L5_DIM);

// Line 113: Encrypt using batched quaternion multiplication (CQC operation)

ns_quat_mul_batch(message, public_key->matrix, ct_quats, WEAVE_L5_DIM);

// Line 133: Decrypt using batched quaternion multiplication (CQC operation)

ns_quat_mul_batch(ct_quats, secret_key->inverse, recovered, WEAVE_L5_DIM);

Conclusion: All three use CQC quaternion operations, just with different optimization levels.

3.2 Weave-SIG/DSA (All Versions)

v3.2 Weave-SIG:

// Line 321: Signature computation using CQC

ns_quaternion_scale(&secret_quats[i], (double)challenge_scalar, &scaled_secret);

ns_quaternion_add(&commit_quats[i], &scaled_secret, &response_quats[i]);

v3.2 ML-DSA (optimized):

// Line 86-88: Key generation using batched CQC operations

ns_quat_mul_batch(secret_key->sk, secret_key->masking, temp, DSA_L5_COMMIT_SIZE);

ns_quat_mul_batch(temp, G_batch, public_key->vk, DSA_L5_COMMIT_SIZE);

// Line 150-152: Signature response using CQC addition

opt_quat_add(&secret_key->randomness[i], &temp2[i], &signature->response[i]);

Conclusion: Both use CQC quaternion operations (scale, add, multiply).

4. The Hierarchy in Practice

4.1 Code Organization

NeuralSeal Codebase

│

├── include/public/cemqc.h ← 🔷 THE FOUNDATION 🔷

│   └── Defines: Quaternion types, chaos types, core operations

│

├── src/cemqc.c ← Implementation of CQC primitives

│   └── Implements: All quaternion operations, chaos functions

│

├── include/internal/weave_l5_internal.h ← Optimization wrapper

│   └── Wraps CQC primitives with SIMD-friendly interfaces

│

├── High-Level Protocols (Built on CQC):

│   ├── src/weave_kem.c ← Uses CQC directly

│   ├── src/weave_sig.c ← Uses CQC directly

│   ├── src/ml_kem.c ← Uses CQC via optimized wrappers

│   └── src/ml_dsa_87.c ← Uses CQC via optimized wrappers

│

└── Applications (Built on protocols):

├── TLS integration

├── VPN encryption

├── File encryption

└── Digital signatures

4.2 Naming Consistency

The Foundation Layer (pick one name):

✅ CQC (Chaos Quaternion Cryptography) - Recommended

✅ CEMQC (Chaos-Enhanced Multi-dimensional Quaternion Cryptography) - Legacy

The Protocol Layer (built on CQC):

Weave-KEM - Key Encapsulation Mechanism

Weave-SIG / Weave-DSA - Digital Signature Algorithm

ML-KEM-1024 - Optimized KEM (API name for v3.2)

ML-DSA-87 - Optimized DSA (API name for v3.2)

NeuralSeal-KEM - Documentation name (v3.0/v3.1)

5. Why This Matters

5.1 Security Implications

Single Point of Trust:

If CQC primitives are secure → All protocols are secure

If CQC primitives are broken → All protocols are broken

This is by design - one well-audited foundation

Cryptanalysis Focus:

Attack the foundation (CQC) = Attack everything

Attack a protocol (Weave-KEM) = Only affects that protocol

Priority: Audit CQC primitives thoroughly

5.2 Development Implications

Code Reuse:

Write CQC primitives once, use everywhere

Optimize CQC → All protocols benefit

Fix bug in CQC → All protocols fixed

Maintenance:

Single source of truth for quaternion math

Easier to verify correctness

Simpler to add new protocols

5.3 Naming Implications

Current Confusion:

"CEMQC" vs "CQC" - Same thing, different names

"Weave-KEM" vs "ML-KEM-1024" vs "NeuralSeal-KEM" - Same algorithm, different names

Recommendation:

Foundation:  CQC (or CEMQC) ← Keep both, they're interchangeable

Protocols:   Weave-KEM, Weave-DSA ← Consistent naming

Optimized:   Weave-1024, Weave-DSA-87 ← Drop "ML-" prefix

Legacy:      NeuralSeal-KEM ← Add note: "Now called Weave-KEM"

6. Summary Diagram

┌─────────────────────────────────────────────────────────────┐

│                     APPLICATIONS                            │

│          (TLS, VPN, SSH, File Encryption, etc.)             │

└─────────────────────────────────────────────────────────────┘

▲

│ uses

│

┌─────────────────────────────────────────────────────────────┐

│                  PROTOCOL LAYER                             │

│    ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │

│    │  Weave-KEM   │  │  Weave-DSA   │  │   Hybrid     │     │

│    │  (all names) │  │  (all names) │  │   Modes      │     │

│    └──────────────┘  └──────────────┘  └──────────────┘     │

└─────────────────────────────────────────────────────────────┘

▲

│ built on

│

┌═════════════════════════════════════════════════════════════┐

║              🔷 CQC/CEMQC FOUNDATION 🔷                    ║

║                                                             ║

║  • Quaternion Algebra (Hamilton)                            ║

║  • Chaos Theory (Logistic/Chebyshev)                        ║

║  • Cryptographic Operations                                 ║

║  • Security: QCP + CIP                                      ║

║                                                             ║

║  Everything branches from here!                             ║

╚═════════════════════════════════════════════════════════════╝

7. Final Answer

Q: Is CQC/CEMQC the base and everything else branches out from it?

A: YES, absolutely!

✅ CQC/CEMQC (cemqc.h) is the foundational primitive layer
✅ All protocols (Weave-KEM, Weave-DSA, ML-KEM, ML-DSA) are built on it
✅ All versions (v3.0, v3.1, v3.2) use these same primitives
✅ All optimizations (SIMD, AVX-512) wrap these primitives
✅ All security derives from the quaternion + chaos foundation

The hierarchy:

Foundation: CQC/CEMQC primitives (quaternion algebra + chaos)

Protocols: Weave-KEM, Weave-DSA (built on CQC)

Optimizations: ML-KEM, ML-DSA (optimized versions of protocols)

Applications: TLS, VPN, etc. (use the protocols)

Everything branches from CQC/CEMQC! 🌳

Report Prepared By: Matthew Stennett
Verification Method: Dependency analysis, include tracing, primitive usage verification
Confidence Level: 100% - Confirmed via source code analysis
```


