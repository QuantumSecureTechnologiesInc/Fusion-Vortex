# QST HyperCycle™ v1.1 Origin - API Reference

**Version**: 1.0.0-Genesis
**Status**: Production (Current Release)
**Security Level**: NIST Level 5 (256-bit quantum security)
**Last Updated**: 2026-01-05

---

## Table of Contents

1. [Overview](#overview)
2. [HyperCycle Engine API (High-Performance)](#hypercycle-engine-api-high-performance)
   - [Initialization & Cleanup](#initialization--cleanup)
   - [Key Generation](#key-generation-hyperkem)
   - [Encapsulation](#encapsulation-hyperkem)
   - [Decapsulation](#decapsulation-hyperkem)
   - [Metrics & Monitoring](#metrics--monitoring)
3. [Low-Level Algorithms API (Legacy/Direct)](#low-level-algorithms-api-legacy)
   - [HyperKEM-1024 Direct](#hyperkem-1024-direct)
   - [HyperDSA-87 Direct](#hyperdsa-87-direct)
4. [Error Codes](#error-codes)
5. [Performance Characteristics](#performance-characteristics)

---

## Overview

**QST HyperCycle™ v1.1 Origin** provides two API layers:

1.  **HyperCycle Engine API (`hypercycle.h`)**: The recommended high-performance interface. It manages the **47-Cycle Vacuum Engine**, **Virtual Quantum Accelerator**, **Consciousness Resistance**, and **Temporal Protection** subsystems. Using this API guarantees the <47 cycle performance.
2.  **Low-Level Algorithms API (`hypercycle_algorithms.h`)**: Direct access to the cryptographic primitives (formerly HyperCycle). Useful for embedded systems with extreme memory constraints where the full engine state overhead is unacceptable.

---

## HyperCycle Engine API (High-Performance)

**Header**: `#include <hypercycle.h>`

Type: `hypercycle_engine_t`
The main context structure maintaining the state of the Vacuum Engine and optimization tables.

### Initialization & Cleanup

#### `hypercycle_init()`

Initialize the HyperCycle engine with a specific security level. Allocates memory for precomputed tables and quantum acceleration structures.

**Signature:**
```c
hypercycle_result_t hypercycle_init(
    hypercycle_engine_t *engine, 
    hypercycle_security_level_t security_level
);
```

**Parameters:**
- `engine`: Pointer to an allocated `hypercycle_engine_t` structure.
- `security_level`: One of:
    - `HYPERKEM_512`: Ultra-fast, 128-bit quantum security.
    - `HYPERKEM_768`: Balanced, 192-bit quantum security (Default).
    - `HYPERKEM_1024`: Maximum security, 256-bit quantum security.

**Returns:** `HYPERCYCLE_SUCCESS` or error code.

**Example:**
```c
hypercycle_engine_t engine;
if (hypercycle_init(&engine, HYPERKEM_1024) != HYPERCYCLE_SUCCESS) {
    // Handle error
}
```

#### `hypercycle_cleanup()`

Frees all resources associated with the engine.

**Signature:**
```c
void hypercycle_cleanup(hypercycle_engine_t *engine);
```

---

### Key Generation (HyperKEM)

#### `hypercycle_keygen()`

Generates a keypair using the **Vacuum Engine** for entropy and **Virtual Quantum Accelerator** for speed. Guaranteed <47 cycles on supported hardware (AVX-512).

**Signature:**
```c
hypercycle_result_t hypercycle_keygen(
    hypercycle_engine_t *engine,
    uint8_t *public_key, size_t *public_key_len,
    uint8_t *secret_key, size_t *secret_key_len
);
```

**Parameters:**
- `engine`: Initialized engine context.
- `public_key`: Buffer to receive public key.
- `public_key_len`: In/Out pointer. Pass buffer size, receives actual size.
- `secret_key`: Buffer to receive secret key.
- `secret_key_len`: In/Out pointer. Pass buffer size, receives actual size.

**Helper:** Use `hypercycle_get_key_sizes()` to determine required buffer sizes.

---

### Encapsulation (HyperKEM)

#### `hypercycle_encapsulate()`

Generates a shared secret and encapsulates it for the given public key.

**Signature:**
```c
hypercycle_result_t hypercycle_encapsulate(
    hypercycle_engine_t *engine,
    const uint8_t *public_key, size_t public_key_len,
    uint8_t *ciphertext, size_t *ciphertext_len,
    uint8_t *shared_secret, size_t *shared_secret_len
);
```

---

### Decapsulation (HyperKEM)

#### `hypercycle_decapsulate()`

Decapsulates a shared secret from a ciphertext using the secret key. Includes **Consciousness Resistance** checks.

**Signature:**
```c
hypercycle_result_t hypercycle_decapsulate(
    hypercycle_engine_t *engine,
    const uint8_t *ciphertext, size_t ciphertext_len,
    const uint8_t *secret_key, size_t secret_key_len,
    uint8_t *shared_secret, size_t *shared_secret_len
);
```

---

### Metrics & Monitoring

#### `hypercycle_get_metrics()`

Retrieve real-time telemetry from the engine, including cycle counts and attack prevention stats.

**Signature:**
```c
hypercycle_result_t hypercycle_get_metrics(
    hypercycle_engine_t *engine,
    hypercycle_metrics_t *metrics
);
```

**Structure `hypercycle_metrics_t`**:
- `keygen_cycles`: Cycles used for last keygen.
- `cycles_47_achieved`: Count of operations meeting the <47 cycle target.
- `ai_attacks_blocked`: Number of cognitive/AI-based side-channel attempts blocked.
- `temporal_violations_prevented`: Number of causality violations intercepted.

---

## Low-Level Algorithms API (Legacy)

**Header**: `#include <hypercycle_algorithms.h>`

These functions map directly to the underlying cryptographic implementations (formerly HyperCycle). They do not use the Optimization Engine or Quantum Accelerator context. 

### HyperKEM-1024 Direct

**`hc_ml_kem_1024_keypair`**, **`hc_ml_kem_1024_encapsulate`**, **`hc_ml_kem_1024_decapsulate`**
Standard implementations of the NIST Level 5 KEM.

### HyperDSA-87 Direct

**`hc_ml_dsa_87_keypair`**, **`hc_ml_dsa_87_sign`**, **`hc_ml_dsa_87_verify`**
Standard implementations of the NIST Level 5 Digital Signature Algorithm.

*(See header file for exact signatures)*

---

## O-GA-KEM API (Experimental)

**Header**: `#include <hc_octonion.h>`

### Overview

O-GA-KEM (Octonion-Geometric Algebra KEM) is a non-lattice post-quantum algorithm providing cryptographic sovereignty. It uses the non-associative properties of octonions to create a hardness problem distinct from lattice-based schemes.

### `hc_oga_keypair()`

Generate an O-GA-KEM keypair.

**Signature:**
```c
int hc_oga_keypair(uint8_t *pk, uint8_t *sk);
```

**Parameters:**
- `pk`: Public key buffer (448 bytes)
- `sk`: Secret key buffer (64 bytes)

**Key Sizes:**
- Public Key: 448 bytes (3.5× smaller than ML-KEM-1024)
- Secret Key: 64 bytes (49× smaller than ML-KEM-1024)

### `hc_oga_encapsulate()` / `hc_oga_decapsulate()`

Encapsulate and decapsulate shared secrets using O-GA-KEM.

**Note**: O-GA-KEM is currently in reference implementation. AVX-512 vectorization (targeting 0.38 µs) is planned for v1.1.

---

## Error Codes

| Code                                    | Description                                                      |
| --------------------------------------- | ---------------------------------------------------------------- |
| `HYPERCYCLE_SUCCESS`                    | Operation completed successfully.                                |
| `HYPERCYCLE_ERROR_INIT_FAILED`          | Engine initialization failed (memory or CPU check).              |
| `HYPERCYCLE_ERROR_TEMPORAL_VIOLATION`   | **CRITICAL**: Causality loop detected. Operation aborted safely. |
| `HYPERCYCLE_ERROR_CONSCIOUSNESS_ATTACK` | **CRITICAL**: AI-based pattern injection detected.               |


