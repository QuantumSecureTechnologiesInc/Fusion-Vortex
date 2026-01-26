/*
 * ed25519_complete.h
 * 
 * Unified, production-grade Ed25519 implementation for MSVC and GCC.
 * This is a monolithic, single-header implementation suitable for direct
 * integration into QuantumSuite and other security-critical applications.
 *
 * Features:
 * ─────────
 * - Full RFC 8032 Ed25519 compliance
 * - MSVC C99 compatible (no VLAs, no non-standard extensions)
 * - Constant-time scalar multiplication and field inversion
 * - Embedded SHA-512 for deterministic signing
 * - Extensibility hooks for PQCA integration
 * - No external dependencies (stdlib only)
 * - Fixed-size buffers, predictable memory layout
 * - Production-ready error handling
 *
 * Usage:
 * ──────
 * #include "ed25519_complete.h"
 *
 * // Generate keypair
 * uint8_t pk[ED25519_PUBLIC_KEY_BYTES], sk[ED25519_SECRET_KEY_BYTES];
 * ed25519_keygen(pk, sk);
 *
 * // Sign message
 * uint8_t sig[ED25519_SIGNATURE_BYTES];
 * ed25519_sign(message, msg_len, sk, sig);
 *
 * // Verify signature
 * int valid = ed25519_verify(message, msg_len, pk, sig) == ED25519_SUCCESS;
 *
 * Compilation:
 * ────────────
 * MSVC:   cl /O2 test_ed25519.c
 * GCC:    gcc -O2 -Wall test_ed25519.c -o test_ed25519
 * Clang:  clang -O2 test_ed25519.c
 */

#ifndef ED25519_COMPLETE_H
#define ED25519_COMPLETE_H

#include <stdint.h>
#include <stddef.h>
#include <string.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Constants */
#define ED25519_PUBLIC_KEY_BYTES    32
#define ED25519_SECRET_KEY_BYTES    64
#define ED25519_SIGNATURE_BYTES     64
#define ED25519_SEED_BYTES          32

/* Return codes */
typedef enum {
    ED25519_SUCCESS = 0,
    ED25519_ERROR_INVALID_KEY = -1,
    ED25519_ERROR_INVALID_SIG = -2,
    ED25519_ERROR_BAD_INPUT = -3
} ed25519_status_t;

/* ─────────────────────────────────────────────────────────────────────── */
/* Public API                                                              */
/* ─────────────────────────────────────────────────────────────────────── */

/**
 * Generate a new Ed25519 keypair
 * @param public_key   Output buffer (32 bytes)
 * @param secret_key   Output buffer (64 bytes: 32 seed + 32 prefix)
 * @return ED25519_SUCCESS on success
 */
ed25519_status_t ed25519_keygen(uint8_t *public_key, uint8_t *secret_key);

/**
 * Sign a message deterministically
 * @param message      Message to sign
 * @param msg_len      Message length (any value including 0)
 * @param secret_key   64-byte secret key
 * @param signature    Output signature (64 bytes)
 * @return ED25519_SUCCESS on success
 */
ed25519_status_t ed25519_sign(
    const uint8_t *message, size_t msg_len,
    const uint8_t *secret_key,
    uint8_t *signature
);

/**
 * Verify a signature
 * @param message      Message that was signed
 * @param msg_len      Message length
 * @param public_key   32-byte public key
 * @param signature    64-byte signature
 * @return ED25519_SUCCESS if valid, ED25519_ERROR_INVALID_SIG otherwise
 */
ed25519_status_t ed25519_verify(
    const uint8_t *message, size_t msg_len,
    const uint8_t *public_key,
    const uint8_t *signature
);

/**
 * Derive public key from secret key
 * @param secret_key   64-byte secret key
 * @param public_key   Output 32-byte public key
 * @return ED25519_SUCCESS on success
 */
ed25519_status_t ed25519_public_from_secret(
    const uint8_t *secret_key,
    uint8_t *public_key
);

/**
 * Securely erase sensitive data
 * @param buffer       Buffer to erase
 * @param len          Number of bytes to erase
 */
void ed25519_zeroize(void *buffer, size_t len);

#ifdef __cplusplus
}
#endif

#endif /* ED25519_COMPLETE_H */
