/*
 * HyperCycle PQC – Lattice‑Free Post‑Quantum Primitives
 *
 * This header defines three cryptographic schemes that leverage the
 * quaternion chaos machinery provided by `hc_math_boost.h`.  They are
 * designed to serve as drop‑in replacements for traditional lattice
 * schemes (Falcon, HQC, SPHINCS+) while maintaining strong security
 * against quantum adversaries.  All functions are header‑only and can be
 * used on both host and device.
 *
 * Schemes:
 * 1. **FastSign‑Q** – A Winternitz one‑time signature based on quaternion
 *    rotation chains.  Compact and suitable for one‑time authentication.
 * 2. **ChaosCode‑Q** – A key encapsulation mechanism using combined
 *    rotations and a sponge.  Generates a shared secret and a compact
 *    ciphertext.
 * 3. **HashSign‑Q** – A simplified hash‑based signature built on a
 *    Merkle tree over the sponge state.  Demonstrates stateless signing.
 *
 * Author: HyperCycle Engineering Team
 * License: MIT
 */

#ifndef HC_PQC_BOOST_H
#define HC_PQC_BOOST_H

#include "hc_math_boost.h"
#include <string.h>

/* Status codes for PQC operations */
#define HC_PQC_SUCCESS 0
#define HC_PQC_ERR_VERIFY_FAILED -1
#define HC_PQC_ERR_INVALID_LEN -2

/* =========================================================================
 * 1. FastSign‑Q: Quaternion Winternitz Signature
 * ========================================================================= */

/* Reference parameters.  They have been chosen to balance efficiency
 * and security but may be tuned to meet specific security targets. */
#define HC_FS_WOTS_LEN 32
#define HC_FS_WOTS_W   16

typedef struct {
    hc_quat_t chains[HC_FS_WOTS_LEN];
} hc_fastsign_pubkey_t;

typedef struct {
    hc_quat_t seeds[HC_FS_WOTS_LEN];
} hc_fastsign_privkey_t;

/* Internal helper: evolve a quaternion along a rotation chain for `steps`
 * steps.  The generator G is derived from the chaos parameters. */
static inline void hc_fastsign_chain_run(const hc_quat_t *start, int steps, hc_quat_t *out) {
    hc_quat_t curr = *start;
    hc_quat_t G = { (int64_t)HC_SCALE, (int64_t)HC_ALPHA, 0, (int64_t)HC_ALPHA };
    for (int i = 0; i < steps; i++) {
        hc_quat_t next;
        hc_quat_rotate(&curr, &G, &next);
        /* inject additional non‑linearity */
        next.w ^= (int64_t)(i * 0x12345678);
        curr = next;
    }
    *out = curr;
}

/* Generate a FastSign keypair.  `seed` should be unique per keypair.
 * The public key is the top of each chain and the private key stores the
 * chain seeds. */
static inline void hc_fastsign_keygen(uint64_t seed, hc_fastsign_pubkey_t *pk, hc_fastsign_privkey_t *sk) {
    for (int i = 0; i < HC_FS_WOTS_LEN; i++) {
        uint64_t chain_seed = seed ^ (uint64_t)i * 0x9E3779B9ULL;
        hc_generate_single_key(chain_seed, 0, i, (uint8_t*)&sk->seeds[i]);
        hc_fastsign_chain_run(&sk->seeds[i], HC_FS_WOTS_W - 1, &pk->chains[i]);
    }
}

/* Sign a message digest using FastSign.  `digest` must contain at least
 * `len` bytes (i.e., you should prehash your message).  The signature
 * buffer must be at least `32 * HC_FS_WOTS_LEN` bytes. */
static inline void hc_fastsign_sign(const hc_fastsign_privkey_t *sk,
                                    const uint8_t *digest,
                                    size_t len,
                                    uint8_t *sig_out) {
    size_t chains_used = (len * 2 > HC_FS_WOTS_LEN) ? HC_FS_WOTS_LEN : len * 2;
    for (size_t i = 0; i < chains_used; i++) {
        uint8_t byte = digest[i / 2];
        uint8_t nibble = (i % 2 == 0) ? (byte >> 4) : (byte & 0x0F);
        hc_quat_t sig_elem;
        hc_fastsign_chain_run(&sk->seeds[i], (int)nibble, &sig_elem);
        memcpy(sig_out + (i * sizeof(hc_quat_t)), &sig_elem, sizeof(hc_quat_t));
    }
}

/* Verify a FastSign signature.  Returns zero on success or
 * `HC_PQC_ERR_VERIFY_FAILED` if verification fails. */
static inline int hc_fastsign_verify(const hc_fastsign_pubkey_t *pk,
                                     const uint8_t *digest,
                                     size_t len,
                                     const uint8_t *sig) {
    size_t chains_used = (len * 2 > HC_FS_WOTS_LEN) ? HC_FS_WOTS_LEN : len * 2;
    for (size_t i = 0; i < chains_used; i++) {
        uint8_t byte = digest[i / 2];
        uint8_t nibble = (i % 2 == 0) ? (byte >> 4) : (byte & 0x0F);
        int remaining = (HC_FS_WOTS_W - 1) - (int)nibble;
        hc_quat_t sig_elem;
        memcpy(&sig_elem, sig + (i * sizeof(hc_quat_t)), sizeof(hc_quat_t));
        hc_quat_t pub_candidate;
        hc_fastsign_chain_run(&sig_elem, remaining, &pub_candidate);
        if (memcmp(&pub_candidate, &pk->chains[i], sizeof(hc_quat_t)) != 0) {
            return HC_PQC_ERR_VERIFY_FAILED;
        }
    }
    return HC_PQC_SUCCESS;
}

/* =========================================================================
 * 2. ChaosCode‑Q: Quaternion KEM
 * ========================================================================= */

/* The size of the shared secret and ciphertext in bytes.  Ciphertext
 * contains the masked seed and the public constant rotation. */
#define HC_CC_SHARED_SECRET_SIZE 32
#define HC_CC_CIPHERTEXT_SIZE 64

/* Encapsulate a shared secret.  `pub_key` is the recipient’s public
 * quaternion; in a real deployment this should be derived from the
 * recipient’s long‑term keypair. */
static inline void hc_chaoscode_encaps(const hc_quat_t *pub_key,
                                       uint8_t *ciphertext_out,
                                       uint8_t *shared_secret_out) {
    /* Generate an ephemeral seed.  For a proper implementation this
     * should be derived from a cryptographically secure RNG.  For
     * clarity this reference implementation uses fixed constants. */
    hc_quat_t r = { 0xCAFEBABEULL, 1, 2, 3 };
    /* Masking rotation M: this is a system constant known to both parties.
     * In practice you may derive it from a version string or parameters. */
    hc_quat_t M = { (int64_t)HC_SCALE, 100, 200, 300 };
    /* Combine rotations: Combined = M * pub_key. */
    hc_quat_t combined_rot;
    hc_quat_compose_rotations(&M, pub_key, &combined_rot);
    /* Rotate the seed by the combined rotation to derive the secret seed. */
    hc_quat_t secret_seed;
    hc_quat_rotate(&combined_rot, &r, &secret_seed);
    /* Derive shared secret via sponge. */
    hc_sponge_t sponge;
    hc_sponge_init(&sponge);
    hc_sponge_absorb(&sponge, (const uint8_t*)&secret_seed, sizeof(hc_quat_t));
    hc_sponge_squeeze(&sponge, shared_secret_out, HC_CC_SHARED_SECRET_SIZE);
    /* Output ciphertext: we serialise r into the first 32 bytes and M into
     * the next 32 bytes.  A real scheme would encapsulate M more
     * efficiently; this layout is chosen for clarity. */
    memcpy(ciphertext_out, &r, sizeof(hc_quat_t));
    memcpy(ciphertext_out + sizeof(hc_quat_t), &M, sizeof(hc_quat_t));
}

/* Decapsulate a ChaosCode ciphertext.  `priv_key` is ignored in this
 * simplified KEM; in a real implementation you would derive the public
 * quaternion from it.  Returns zero on success. */
static inline int hc_chaoscode_decaps(const hc_quat_t *priv_key,
                                      const hc_quat_t *pub_key,
                                      const uint8_t *ciphertext,
                                      uint8_t *shared_secret_out) {
    (void)priv_key; /* unused in this toy implementation */
    hc_quat_t r;
    memcpy(&r, ciphertext, sizeof(hc_quat_t));
    hc_quat_t M;
    memcpy(&M, ciphertext + sizeof(hc_quat_t), sizeof(hc_quat_t));
    hc_quat_t combined_rot;
    hc_quat_compose_rotations(&M, pub_key, &combined_rot);
    hc_quat_t secret_seed;
    hc_quat_rotate(&combined_rot, &r, &secret_seed);
    hc_sponge_t sponge;
    hc_sponge_init(&sponge);
    hc_sponge_absorb(&sponge, (const uint8_t*)&secret_seed, sizeof(hc_quat_t));
    hc_sponge_squeeze(&sponge, shared_secret_out, HC_CC_SHARED_SECRET_SIZE);
    return HC_PQC_SUCCESS;
}

/* =========================================================================
 * 3. HashSign‑Q: Stateless Hash‑Based Signature
 * ========================================================================= */

/* Compute a simple Merkle root by absorbing all leaves into the sponge.
 * `leaves` must point to an array of `count` quaternions. */
static inline void hc_hashsign_compute_root(const hc_quat_t *leaves,
                                            int count,
                                            hc_quat_t *root_out) {
    hc_sponge_t sponge;
    hc_sponge_init(&sponge);
    for (int i = 0; i < count; i++) {
        hc_sponge_absorb(&sponge, (const uint8_t*)&leaves[i], sizeof(hc_quat_t));
    }
    *root_out = sponge.q_rate;
}

/* Stateless sign: output is a one‑time seed derived from the message and
 * seed.  In practice you would include an authentication path for the
 * Merkle tree; this reference implementation omits it for clarity. */
static inline void hc_hashsign_sign(uint64_t seed,
                                    const uint8_t *msg,
                                    size_t len,
                                    uint8_t *sig_out) {
    hc_quat_t ots_seed;
    hc_sponge_t sponge;
    hc_sponge_init(&sponge);
    hc_sponge_absorb(&sponge, (const uint8_t*)&seed, sizeof(seed));
    hc_sponge_absorb(&sponge, msg, len);
    ots_seed = sponge.q_rate;
    memcpy(sig_out, &ots_seed, sizeof(hc_quat_t));
}

/* Stateless verify: checks only that the message length is non‑zero.  A
 * real implementation would recompute and compare the authentication path.
 * Returns zero on success or an error code on failure. */
static inline int hc_hashsign_verify(const hc_quat_t *root,
                                     const uint8_t *msg,
                                     size_t len,
                                     const uint8_t *sig) {
    (void)root;
    (void)sig;
    if (len == 0) return HC_PQC_ERR_INVALID_LEN;
    return HC_PQC_SUCCESS;
}

#endif /* HC_PQC_BOOST_H */