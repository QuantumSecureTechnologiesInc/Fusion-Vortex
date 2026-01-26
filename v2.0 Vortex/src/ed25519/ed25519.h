/*
 * ed25519.h
 * 
 * Public API for lightweight, MSVC-compatible Ed25519 signature library.
 * 
 * This library implements the Edwards-Curve Digital Signature Algorithm (EdDSA)
 * with Ed25519 parameters as specified in RFC 8032.
 * 
 * Design principles:
 * - Minimal external dependencies (C stdlib only)
 * - MSVC C99 compatibility (no VLAs, no GNU extensions)
 * - Fixed-size buffers for predictable memory footprint
 * - Extensibility hooks for PQCA (Pure Quaternion-Chaos Architecture)
 * - Constant-time scalar operations to mitigate timing side-channels
 * 
 * Author: QuantumSecure Technologies Ltd.
 * Status: Production-Ready
 */

#ifndef ED25519_H
#define ED25519_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Ed25519 Constants
 * ─────────────────
 * ED25519_SECRET_SEED_BYTES:  Length of secret key seed material (32 bytes)
 * ED25519_PUBLIC_KEY_BYTES:   Length of public key encoding (32 bytes)
 * ED25519_SECRET_KEY_BYTES:   Length of expanded secret key (64 bytes, internal use)
 * ED25519_SIGNATURE_BYTES:    Length of signature (64 bytes)
 * ED25519_MESSAGE_MAX_BYTES:  Maximum message length (practical limit: 2^32)
 */
#define ED25519_SECRET_SEED_BYTES   32
#define ED25519_PUBLIC_KEY_BYTES    32
#define ED25519_SECRET_KEY_BYTES    64
#define ED25519_SIGNATURE_BYTES     64

/**
 * Return codes for Ed25519 operations
 * ───────────────────────────────────
 * ED25519_SUCCESS:           Operation completed successfully
 * ED25519_ERROR_INVALID_KEY:  Invalid or malformed key material
 * ED25519_ERROR_INVALID_SIG:  Signature verification failed
 * ED25519_ERROR_BAD_INPUT:    Invalid input parameters
 */
typedef enum {
    ED25519_SUCCESS = 0,
    ED25519_ERROR_INVALID_KEY = -1,
    ED25519_ERROR_INVALID_SIG = -2,
    ED25519_ERROR_BAD_INPUT = -3
} ed25519_status_t;

/**
 * Configuration structure for Ed25519 library
 * ────────────────────────────────────────────
 * This structure allows fine-grained control over Ed25519 behavior,
 * including integration points for PQCA-based entropy sources and
 * custom random number generation.
 * 
 * Fields:
 * - rng_hook:  Optional custom RNG for key derivation (if NULL, use system CSPRNG)
 * - entropy_mixer: PQCA-style entropy diffusion callback (for future quantum-resistant modes)
 * - zeroize_on_destroy: If non-zero, aggressively zeroize sensitive material
 * 
 * All callbacks are optional and may be NULL for default behavior.
 */
typedef struct {
    /**
     * Custom RNG hook: generates random bytes for nonce/key material
     * Signature: int rng_hook(uint8_t *buffer, size_t len)
     *   buffer: output buffer for random bytes
     *   len:    number of bytes to generate
     *   return: non-zero on success, zero on failure
     * 
     * Default (NULL): Uses system CSPRNG (/dev/urandom on Unix, CryptGenRandom on Windows)
     */
    int (*rng_hook)(uint8_t *buffer, size_t len);

    /**
     * PQCA entropy mixer: optional quaternion-chaos-based entropy diffusion
     * Signature: void entropy_mixer(uint8_t *material, size_t len, const uint8_t *seed)
     *   material: secret material to be diffused (modified in-place)
     *   len:      length of material
     *   seed:     optional chaos-seed (may be NULL)
     * 
     * Default (NULL): No additional entropy diffusion
     * 
     * PQCA Extension Point:
     * This hook allows injection of quaternion-state entropy mixing before
     * scalar generation or key expansion, enabling hybrid PQCA/Ed25519 modes.
     */
    void (*entropy_mixer)(uint8_t *material, size_t len, const uint8_t *seed);

    /**
     * Security zeroization flag
     * If non-zero, all sensitive material is aggressively zeroized after use.
     * Default: 1 (enabled)
     */
    int zeroize_on_destroy;
} ed25519_config_t;

/**
 * Get the default Ed25519 configuration
 * ─────────────────────────────────────
 * Returns a configuration struct with sensible defaults:
 * - System CSPRNG for random number generation
 * - No PQCA entropy mixer (can be attached later)
 * - Zeroization enabled
 */
ed25519_config_t ed25519_config_default(void);

/**
 * Initialize the Ed25519 library with custom configuration
 * ──────────────────────────────────────────────────────────
 * This function should be called once at library initialization to configure
 * global behavior (e.g., RNG hooks, PQCA integration).
 * 
 * Parameters:
 *   config: Configuration struct (may be NULL for defaults)
 * 
 * Return:
 *   ED25519_SUCCESS on success
 *   ED25519_ERROR_BAD_INPUT if config is invalid
 */
ed25519_status_t ed25519_init(const ed25519_config_t *config);

/**
 * Generate a new Ed25519 keypair
 * ───────────────────────────────
 * Generates a fresh public/private keypair by:
 * 1. Sampling 32 random bytes (seed) from the configured RNG
 * 2. Hashing the seed via SHA-512 to derive secret scalar and prefix
 * 3. Clamping the scalar per RFC 8032 (clear bits 0-2, set bit 254, clear bit 255)
 * 4. Computing the public key as [scalar]B (scalar mult of base point)
 * 
 * Parameters:
 *   public_key:  Output buffer for public key (must be ED25519_PUBLIC_KEY_BYTES)
 *   secret_key:  Output buffer for secret key (must be ED25519_SECRET_KEY_BYTES)
 * 
 * Return:
 *   ED25519_SUCCESS on success
 *   ED25519_ERROR_BAD_INPUT if any pointer is NULL
 *   ED25519_ERROR_INVALID_KEY if entropy source fails
 * 
 * Note:
 *   The secret_key buffer contains: seed (32 bytes) || prefix (32 bytes)
 *   This is the full expanded form and must be kept confidential.
 * 
 * PQCA Integration:
 *   If entropy_mixer is configured, the initial 32-byte seed is diffused
 *   through quaternion-chaos before SHA-512 to strengthen key derivation.
 */
ed25519_status_t ed25519_keygen(uint8_t *public_key, uint8_t *secret_key);

/**
 * Sign a message with an Ed25519 secret key
 * ──────────────────────────────────────────
 * Generates a deterministic signature on a message using the secret key.
 * 
 * Algorithm:
 * 1. Extract scalar and prefix from secret key
 * 2. Hash(prefix || message) to derive per-message random value
 * 3. Compute commitment R = [r]B
 * 4. Compute challenge k = Hash(R || A || message)
 * 5. Compute response S = r + k*scalar (mod L)
 * 6. Return encoded (R || S)
 * 
 * Parameters:
 *   message:      Input message (may be any length, including 0)
 *   message_len:  Length of message in bytes
 *   secret_key:   Secret key (ED25519_SECRET_KEY_BYTES)
 *   signature:    Output signature buffer (ED25519_SIGNATURE_BYTES)
 * 
 * Return:
 *   ED25519_SUCCESS on success
 *   ED25519_ERROR_BAD_INPUT if any pointer is NULL or secret_key malformed
 * 
 * Note:
 *   This is deterministic signing (as per RFC 8032), not randomized.
 *   The same message/key pair always produces the same signature.
 * 
 * Side-Channel Resistance:
 *   Scalar multiplication uses constant-time algorithms to resist
 *   timing analysis of the secret scalar. Field operations employ
 *   constant-time modular arithmetic where feasible in portable C.
 * 
 * PQCA Extension Point:
 *   If entropy_mixer is configured, the prefix material can be diffused
 *   through quaternion-chaos to add additional entropy/mixing to the
 *   per-message random value derivation.
 */
ed25519_status_t ed25519_sign(
    const uint8_t *message, size_t message_len,
    const uint8_t *secret_key,
    uint8_t *signature
);

/**
 * Verify an Ed25519 signature
 * ───────────────────────────
 * Verifies that a signature is valid for a given message and public key.
 * 
 * Algorithm:
 * 1. Decode signature as (R_bytes || S_bytes)
 * 2. Decode public key as point A
 * 3. Decode R as curve point
 * 4. Compute challenge k = Hash(R || A || message)
 * 5. Check if [8*S]B == [8*k]A + [8]R (accounting for cofactor)
 * 
 * Parameters:
 *   message:      Input message
 *   message_len:  Length of message in bytes
 *   public_key:   Public key (ED25519_PUBLIC_KEY_BYTES)
 *   signature:    Signature to verify (ED25519_SIGNATURE_BYTES)
 * 
 * Return:
 *   ED25519_SUCCESS if signature is valid
 *   ED25519_ERROR_INVALID_SIG if signature verification fails
 *   ED25519_ERROR_BAD_INPUT if any pointer is NULL or malformed
 * 
 * Timing Resistance:
 *   Verification uses non-constant-time scalar multiplication (w-NAF method)
 *   since the public key and message are public. However, the point
 *   comparison is done in constant time to resist cache attacks.
 */
ed25519_status_t ed25519_verify(
    const uint8_t *message, size_t message_len,
    const uint8_t *public_key,
    const uint8_t *signature
);

/**
 * Derive the public key from a secret key
 * ────────────────────────────────────────
 * Extracts and recomputes the public key from an existing secret key.
 * Useful for key recovery or verification.
 * 
 * Parameters:
 *   secret_key:  Secret key (ED25519_SECRET_KEY_BYTES)
 *   public_key:  Output buffer for public key (ED25519_PUBLIC_KEY_BYTES)
 * 
 * Return:
 *   ED25519_SUCCESS on success
 *   ED25519_ERROR_BAD_INPUT if any pointer is NULL
 */
ed25519_status_t ed25519_public_from_secret(
    const uint8_t *secret_key,
    uint8_t *public_key
);

/**
 * Securely erase sensitive material
 * ──────────────────────────────────
 * Overwrites a buffer with zeros to prevent information leakage.
 * Uses volatile writes to prevent compiler optimizations.
 * 
 * Parameters:
 *   buffer: Pointer to sensitive data
 *   len:    Number of bytes to zeroize
 * 
 * Note:
 *   This is always available and recommended before freeing any
 *   buffers containing secret keys, signatures, or intermediate values.
 */
void ed25519_zeroize(void *buffer, size_t len);

/**
 * Library cleanup and resource deallocation
 * ──────────────────────────────────────────
 * Called at library shutdown to clean up any allocated resources
 * or cached precomputed tables.
 * 
 * It is safe to call this multiple times or without calling ed25519_init().
 */
void ed25519_cleanup(void);

#ifdef __cplusplus
}
#endif

#endif /* ED25519_H */
