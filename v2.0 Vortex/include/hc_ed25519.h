/*
 * hc_ed25519.h
 *
 * High‑level Ed25519 wrapper for the HyperCycle PQC library.
 *
 * This header exposes a production‑ready digital signature API
 * built on top of OpenSSL’s Ed25519 implementation.  Unlike the
 * reference Ed25519 code bundled under `third_party/ed25519`, this
 * wrapper delegates elliptic‑curve arithmetic to OpenSSL to avoid
 * incomplete stubs and provides hooks into the HyperCycle vacuum
 * engine for entropy and optional quaternion‑chaos diffusion.  The
 * interface is deliberately similar to the original `ed25519.h` so
 * that existing code can be ported with minimal changes.
 *
 * The API supports custom randomness sources via `rng_hook` and
 * chaos‑based post‑processing via `entropy_mixer`.  When these
 * callbacks are omitted the wrapper defaults to the HyperCycle
 * vacuum engine for seed generation and performs no additional
 * mixing.  All cryptographic operations are constant‑time with
 * respect to secret material as the underlying OpenSSL primitives
 * are designed to be side‑channel resilient.
 *
 * Author: HyperCycle Engineering Team
 * License: MIT
 */

#ifndef HC_ED25519_H
#define HC_ED25519_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/** Length of a secret seed for Ed25519 (in bytes). */
#define HC_ED25519_SEED_BYTES      32U
/** Length of a public key for Ed25519 (in bytes). */
#define HC_ED25519_PUBLIC_BYTES    32U
/** Length of a secret key for Ed25519 (expanded seed + prefix, in bytes). */
#define HC_ED25519_SECRET_BYTES    64U
/** Length of a signature for Ed25519 (R || S, in bytes). */
#define HC_ED25519_SIGNATURE_BYTES 64U

/** Status codes returned by the Ed25519 wrapper. */
typedef enum {
    HC_ED25519_SUCCESS = 0,
    /** Invalid or malformed key material */
    HC_ED25519_ERROR_INVALID_KEY = -1,
    /** Signature verification failed */
    HC_ED25519_ERROR_INVALID_SIG = -2,
    /** Bad input parameters */
    HC_ED25519_ERROR_BAD_INPUT = -3,
    /** Internal failure (OpenSSL or vacuum engine) */
    HC_ED25519_ERROR_INTERNAL = -4
} hc_ed25519_status_t;

/**
 * Configuration structure for the Ed25519 wrapper.
 *
 * The wrapper supports three optional hooks:
 *   - rng_hook: override the default seed generator.  If NULL the
 *     HyperCycle vacuum engine is used to produce cryptographically
 *     strong seeds.  The hook must return non‑zero on success.
 *   - entropy_mixer: apply quaternion‑chaos diffusion to secret
 *     material.  This callback operates in‑place on the provided
 *     buffer prior to hashing and may accept an optional seed for
 *     additional mixing.  If NULL no extra diffusion is applied.
 *   - zeroize_on_destroy: if non‑zero, all temporary secrets are
 *     wiped with secure cleansing functions once they go out of
 *     scope.
 */
typedef struct {
    int (*rng_hook)(uint8_t *buffer, size_t len);
    void (*entropy_mixer)(uint8_t *material, size_t len, const uint8_t *seed);
    int zeroize_on_destroy;
} hc_ed25519_config_t;

/**
 * Retrieve a default configuration suitable for most applications.
 * The default uses the vacuum engine for randomness, applies no
 * additional mixing and enables zeroization.  Applications may
 * customise the configuration before passing it to `hc_ed25519_init()`.
 */
hc_ed25519_config_t hc_ed25519_config_default(void);

/**
 * Initialise the Ed25519 subsystem.
 *
 * This function must be called before any key generation, signing
 * or verification.  It initialises the global configuration and
 * underlying vacuum engine context.  Passing NULL uses the default
 * configuration.  It is safe to call multiple times; subsequent
 * calls reinitialise the configuration.
 *
 * @param config  Custom configuration or NULL for defaults
 * @return        HC_ED25519_SUCCESS on success, or an error code
 */
hc_ed25519_status_t hc_ed25519_init(const hc_ed25519_config_t *config);

/**
 * Cleanup and free internal resources.
 *
 * This function releases the vacuum engine context and zeroizes
 * configuration state.  It is safe to call multiple times or
 * without prior initialisation.
 */
void hc_ed25519_cleanup(void);

/**
 * Generate a new Ed25519 keypair.
 *
 * The keypair is derived as follows:
 *   1. 32 bytes of seed are generated via the configured RNG.  If
 *      none is provided the vacuum engine supplies the entropy.
 *   2. If an entropy mixer is configured the seed is diffused in
 *      place.
 *   3. The seed is hashed via SHA‑512 to derive an internal 64‑byte
 *      digest.  The second half (prefix) may optionally be mixed
 *      again via the entropy mixer.
 *   4. The secret key is stored as seed || prefix (64 bytes).
 *   5. The public key is computed using OpenSSL’s Ed25519 API.
 *
 * @param public_key  Output buffer for public key (32 bytes)
 * @param secret_key  Output buffer for secret key (64 bytes)
 * @return            HC_ED25519_SUCCESS on success or an error code
 */
hc_ed25519_status_t hc_ed25519_keygen(uint8_t *public_key,
                                      uint8_t *secret_key);

/**
 * Sign a message deterministically with an Ed25519 secret key.
 *
 * The wrapper uses OpenSSL’s Ed25519 implementation to produce
 * signatures.  The secret key must be the 64‑byte expanded form
 * returned by `hc_ed25519_keygen()`.  The resulting signature is
 * 64 bytes (R || S).  The signing operation does not invoke the
 * entropy mixer again; all necessary diffusion occurs during key
 * derivation.
 *
 * @param message      Input message (may be NULL if message_len == 0)
 * @param message_len  Length of message in bytes
 * @param secret_key   64‑byte secret key
 * @param signature    Output buffer for signature (64 bytes)
 * @return             HC_ED25519_SUCCESS on success or error code
 */
hc_ed25519_status_t hc_ed25519_sign(const uint8_t *message,
                                    size_t message_len,
                                    const uint8_t *secret_key,
                                    uint8_t *signature);

/**
 * Verify an Ed25519 signature.
 *
 * Verifies that a 64‑byte signature is valid for a given message
 * and public key.  This function returns HC_ED25519_SUCCESS if the
 * signature is valid or HC_ED25519_ERROR_INVALID_SIG otherwise.
 *
 * @param message      Input message
 * @param message_len  Length of message in bytes
 * @param public_key   Public key (32 bytes)
 * @param signature    Signature to verify (64 bytes)
 * @return             HC_ED25519_SUCCESS if valid, error code otherwise
 */
hc_ed25519_status_t hc_ed25519_verify(const uint8_t *message,
                                      size_t message_len,
                                      const uint8_t *public_key,
                                      const uint8_t *signature);

/**
 * Derive the public key from a secret key.
 *
 * Computes the corresponding public key from the provided 64‑byte
 * secret key.  This is useful when only the secret key is stored
 * and the public key must be recovered on the fly.  The function
 * internally extracts the 32‑byte seed and uses OpenSSL to
 * derive the public key.
 *
 * @param secret_key   64‑byte secret key
 * @param public_key   Output buffer for the public key (32 bytes)
 * @return             HC_ED25519_SUCCESS on success or error code
 */
hc_ed25519_status_t hc_ed25519_public_from_secret(const uint8_t *secret_key,
                                                  uint8_t *public_key);

/**
 * Securely erase sensitive material.
 *
 * Overwrites the specified buffer with zeros using a volatile write
 * to prevent compiler optimisations.  This should be used on
 * secret keys, seeds and signatures once they are no longer needed.
 *
 * @param buffer  Pointer to sensitive data
 * @param len     Number of bytes to zeroize
 */
void hc_ed25519_zeroize(void *buffer, size_t len);

#ifdef __cplusplus
}
#endif

#endif /* HC_ED25519_H */