/*
 * hc_ed25519.c
 *
 * Implementation of the Ed25519 wrapper for the HyperCycle PQC library.
 *
 * This module leverages OpenSSL’s Ed25519 primitives to perform key
 * generation, signing and verification while providing integration
 * points for custom entropy and quaternion‑chaos diffusion.  The
 * wrapper ensures all sensitive material is properly zeroised and
 * makes use of the HyperCycle vacuum engine as the default
 * randomness source.
 *
 * The implementation avoids any unfinished stubs or placeholder
 * functions found in the reference Ed25519 code.  Instead, all
 * cryptographic operations are delegated to OpenSSL which includes
 * complete and constant‑time implementations for EdDSA.  This
 * guarantees the resulting signatures are RFC 8032 compliant and
 * production‑ready.
 */

#include "hc_ed25519.h"
#include "hc_vacuum_engine.h"

#include <openssl/evp.h>
#include <openssl/sha.h>
#include <openssl/crypto.h>
#include <string.h>

/* -------------------------------------------------------------------------- */
/* Static state                                                               */
/* -------------------------------------------------------------------------- */

/* Global configuration.  This structure is initialised via
 * hc_ed25519_init().  It is safe to access from multiple threads
 * concurrently as all members are immutable after initialisation.
 */
static hc_ed25519_config_t g_ed25519_config;

/* Flag indicating whether the configuration has been initialised. */
static int g_ed25519_config_initialised = 0;

/* Vacuum engine context used when no custom RNG is supplied. */
static hc_vac_context_t g_vac_ctx = NULL;

/* Internal helper to wipe memory securely using OpenSSL’s cleanse. */
static void secure_wipe(void *buf, size_t len) {
    if (buf && len > 0) {
        OPENSSL_cleanse(buf, len);
    }
}

/* Default RNG that pulls entropy from the HyperCycle vacuum engine.  This
 * function may allocate a context on first use if none exists.  It fills
 * the provided buffer with `len` bytes by repeatedly calling
 * hc_vacuum_generate_seed_safe().  Returns non‑zero on success. */
static int default_rng(uint8_t *buffer, size_t len) {
    if (!buffer) {
        return 0;
    }
    /* Initialise vacuum context lazily if needed */
    if (g_vac_ctx == NULL) {
        hc_result_t rc = hc_vacuum_init_context(&g_vac_ctx, NULL);
        if (rc != HC_SUCCESS) {
            return 0;
        }
    }
    while (len > 0) {
        uint8_t seed[HC_PQC_SEED_SIZE];
        if (hc_vacuum_generate_seed_safe(g_vac_ctx, seed) != HC_SUCCESS) {
            return 0;
        }
        size_t to_copy = len < HC_PQC_SEED_SIZE ? len : HC_PQC_SEED_SIZE;
        memcpy(buffer, seed, to_copy);
        buffer += to_copy;
        len -= to_copy;
        /* Wipe the intermediate seed once copied */
        secure_wipe(seed, sizeof(seed));
    }
    return 1;
}

/* Retrieve a default configuration. */
hc_ed25519_config_t hc_ed25519_config_default(void) {
    hc_ed25519_config_t cfg;
    cfg.rng_hook = NULL;
    cfg.entropy_mixer = NULL;
    cfg.zeroize_on_destroy = 1;
    return cfg;
}

/* Initialise the Ed25519 subsystem. */
hc_ed25519_status_t hc_ed25519_init(const hc_ed25519_config_t *config) {
    /* Copy provided configuration or fall back to defaults */
    if (config) {
        g_ed25519_config = *config;
    } else {
        g_ed25519_config = hc_ed25519_config_default();
    }
    /* Mark as initialised */
    g_ed25519_config_initialised = 1;

    /* If no custom RNG is specified, ensure a vacuum context exists */
    if (g_ed25519_config.rng_hook == NULL && g_vac_ctx == NULL) {
        hc_result_t rc = hc_vacuum_init_context(&g_vac_ctx, NULL);
        if (rc != HC_SUCCESS) {
            return HC_ED25519_ERROR_INTERNAL;
        }
    }
    return HC_ED25519_SUCCESS;
}

/* Cleanup and free resources. */
void hc_ed25519_cleanup(void) {
    /* Free vacuum context if allocated */
    if (g_vac_ctx) {
        hc_vacuum_free_context(g_vac_ctx);
        g_vac_ctx = NULL;
    }
    /* Wipe configuration */
    secure_wipe(&g_ed25519_config, sizeof(g_ed25519_config));
    g_ed25519_config_initialised = 0;
}

/* Helper to obtain random bytes via configured RNG or default. */
static int rng_dispatch(uint8_t *out, size_t len) {
    if (!out) {
        return 0;
    }
    if (g_ed25519_config.rng_hook) {
        return g_ed25519_config.rng_hook(out, len);
    }
    return default_rng(out, len);
}

/* Invoke the configured entropy mixer if present. */
static void mix_entropy(uint8_t *material, size_t len, const uint8_t *seed) {
    if (g_ed25519_config.entropy_mixer) {
        g_ed25519_config.entropy_mixer(material, len, seed);
    }
}

/* Generate a new Ed25519 keypair. */
hc_ed25519_status_t hc_ed25519_keygen(uint8_t *public_key,
                                      uint8_t *secret_key) {
    if (!public_key || !secret_key) {
        return HC_ED25519_ERROR_BAD_INPUT;
    }
    if (!g_ed25519_config_initialised) {
        /* Not initialised */
        return HC_ED25519_ERROR_INTERNAL;
    }
    uint8_t seed[HC_ED25519_SEED_BYTES];
    uint8_t hash[64];

    /* Generate random seed via configured RNG */
    if (!rng_dispatch(seed, sizeof(seed))) {
        return HC_ED25519_ERROR_INVALID_KEY;
    }
    /* Apply chaos diffusion to the seed if requested */
    mix_entropy(seed, sizeof(seed), NULL);
    /* Derive SHA‑512 digest of the seed */
    SHA512(seed, sizeof(seed), hash);
    /* Mix the prefix half of the digest with the seed if requested */
    mix_entropy(hash + 32, 32, seed);
    /* Store secret key as seed || prefix */
    memcpy(secret_key, seed, 32);
    memcpy(secret_key + 32, hash + 32, 32);
    /* Create an OpenSSL Ed25519 private key using the seed */
    EVP_PKEY *pkey = EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519, NULL,
                                                  seed, sizeof(seed));
    if (!pkey) {
        /* Clean up sensitive material */
        if (g_ed25519_config.zeroize_on_destroy) {
            secure_wipe(seed, sizeof(seed));
            secure_wipe(hash, sizeof(hash));
        }
        return HC_ED25519_ERROR_INTERNAL;
    }
    /* Extract the raw public key */
    size_t pk_len = HC_ED25519_PUBLIC_BYTES;
    if (EVP_PKEY_get_raw_public_key(pkey, public_key, &pk_len) != 1 ||
        pk_len != HC_ED25519_PUBLIC_BYTES) {
        EVP_PKEY_free(pkey);
        if (g_ed25519_config.zeroize_on_destroy) {
            secure_wipe(seed, sizeof(seed));
            secure_wipe(hash, sizeof(hash));
        }
        return HC_ED25519_ERROR_INTERNAL;
    }
    /* Free the temporary key */
    EVP_PKEY_free(pkey);
    /* Zeroise sensitive material if requested */
    if (g_ed25519_config.zeroize_on_destroy) {
        secure_wipe(seed, sizeof(seed));
        secure_wipe(hash, sizeof(hash));
    }
    return HC_ED25519_SUCCESS;
}

/* Sign a message deterministically. */
hc_ed25519_status_t hc_ed25519_sign(const uint8_t *message,
                                    size_t message_len,
                                    const uint8_t *secret_key,
                                    uint8_t *signature) {
    if (!secret_key || !signature || (!message && message_len > 0)) {
        return HC_ED25519_ERROR_BAD_INPUT;
    }
    if (!g_ed25519_config_initialised) {
        return HC_ED25519_ERROR_INTERNAL;
    }
    /* Extract the 32‑byte seed from the secret key */
    uint8_t seed[HC_ED25519_SEED_BYTES];
    memcpy(seed, secret_key, HC_ED25519_SEED_BYTES);
    /* Create an OpenSSL PKEY from raw seed */
    EVP_PKEY *pkey = EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519, NULL,
                                                  seed, sizeof(seed));
    if (!pkey) {
        if (g_ed25519_config.zeroize_on_destroy) {
            secure_wipe(seed, sizeof(seed));
        }
        return HC_ED25519_ERROR_INTERNAL;
    }
    /* Create digest/sign context */
    EVP_MD_CTX *mdctx = EVP_MD_CTX_new();
    if (!mdctx) {
        EVP_PKEY_free(pkey);
        if (g_ed25519_config.zeroize_on_destroy) {
            secure_wipe(seed, sizeof(seed));
        }
        return HC_ED25519_ERROR_INTERNAL;
    }
    /* Initialise signing (Ed25519 uses no digest) */
    if (EVP_DigestSignInit(mdctx, NULL, NULL, NULL, pkey) != 1) {
        EVP_MD_CTX_free(mdctx);
        EVP_PKEY_free(pkey);
        if (g_ed25519_config.zeroize_on_destroy) {
            secure_wipe(seed, sizeof(seed));
        }
        return HC_ED25519_ERROR_INTERNAL;
    }
    /* Sign the message */
    size_t siglen = HC_ED25519_SIGNATURE_BYTES;
    if (EVP_DigestSign(mdctx, signature, &siglen,
                       message ? message : (const uint8_t *)"",
                       message_len) != 1 ||
        siglen != HC_ED25519_SIGNATURE_BYTES) {
        EVP_MD_CTX_free(mdctx);
        EVP_PKEY_free(pkey);
        if (g_ed25519_config.zeroize_on_destroy) {
            secure_wipe(seed, sizeof(seed));
        }
        return HC_ED25519_ERROR_INTERNAL;
    }
    /* Clean up */
    EVP_MD_CTX_free(mdctx);
    EVP_PKEY_free(pkey);
    if (g_ed25519_config.zeroize_on_destroy) {
        secure_wipe(seed, sizeof(seed));
    }
    return HC_ED25519_SUCCESS;
}

/* Verify an Ed25519 signature. */
hc_ed25519_status_t hc_ed25519_verify(const uint8_t *message,
                                      size_t message_len,
                                      const uint8_t *public_key,
                                      const uint8_t *signature) {
    if (!public_key || !signature || (!message && message_len > 0)) {
        return HC_ED25519_ERROR_BAD_INPUT;
    }
    if (!g_ed25519_config_initialised) {
        return HC_ED25519_ERROR_INTERNAL;
    }
    /* Construct a PKEY from the raw public key */
    EVP_PKEY *pkey = EVP_PKEY_new_raw_public_key(EVP_PKEY_ED25519, NULL,
                                                 public_key, HC_ED25519_PUBLIC_BYTES);
    if (!pkey) {
        return HC_ED25519_ERROR_INTERNAL;
    }
    /* Create a verification context */
    EVP_MD_CTX *mdctx = EVP_MD_CTX_new();
    if (!mdctx) {
        EVP_PKEY_free(pkey);
        return HC_ED25519_ERROR_INTERNAL;
    }
    if (EVP_DigestVerifyInit(mdctx, NULL, NULL, NULL, pkey) != 1) {
        EVP_MD_CTX_free(mdctx);
        EVP_PKEY_free(pkey);
        return HC_ED25519_ERROR_INTERNAL;
    }
    /* Perform verification.  On success EVP_DigestVerify returns 1. */
    int ok = EVP_DigestVerify(mdctx,
                              signature, HC_ED25519_SIGNATURE_BYTES,
                              message ? message : (const uint8_t *)"",
                              message_len);
    EVP_MD_CTX_free(mdctx);
    EVP_PKEY_free(pkey);
    if (ok == 1) {
        return HC_ED25519_SUCCESS;
    }
    /* Signature invalid */
    return HC_ED25519_ERROR_INVALID_SIG;
}

/* Derive the public key from a secret key. */
hc_ed25519_status_t hc_ed25519_public_from_secret(const uint8_t *secret_key,
                                                  uint8_t *public_key) {
    if (!secret_key || !public_key) {
        return HC_ED25519_ERROR_BAD_INPUT;
    }
    if (!g_ed25519_config_initialised) {
        return HC_ED25519_ERROR_INTERNAL;
    }
    /* Extract seed from secret key */
    uint8_t seed[HC_ED25519_SEED_BYTES];
    memcpy(seed, secret_key, HC_ED25519_SEED_BYTES);
    /* Build private key and derive public key */
    EVP_PKEY *pkey = EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519, NULL,
                                                  seed, sizeof(seed));
    if (!pkey) {
        if (g_ed25519_config.zeroize_on_destroy) {
            secure_wipe(seed, sizeof(seed));
        }
        return HC_ED25519_ERROR_INTERNAL;
    }
    size_t pk_len = HC_ED25519_PUBLIC_BYTES;
    if (EVP_PKEY_get_raw_public_key(pkey, public_key, &pk_len) != 1 ||
        pk_len != HC_ED25519_PUBLIC_BYTES) {
        EVP_PKEY_free(pkey);
        if (g_ed25519_config.zeroize_on_destroy) {
            secure_wipe(seed, sizeof(seed));
        }
        return HC_ED25519_ERROR_INTERNAL;
    }
    EVP_PKEY_free(pkey);
    if (g_ed25519_config.zeroize_on_destroy) {
        secure_wipe(seed, sizeof(seed));
    }
    return HC_ED25519_SUCCESS;
}

/* Securely erase sensitive material. */
void hc_ed25519_zeroize(void *buffer, size_t len) {
    secure_wipe(buffer, len);
}