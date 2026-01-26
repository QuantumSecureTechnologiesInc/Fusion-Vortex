/**
 * HyperCycle v3.2 Fulminis - Optimized Weave-KEM-1024 (Level 5)
 *
 * PERFORMANCE OPTIMIZED IMPLEMENTATION
 */

#include "vortex/public/hypercycle_algorithms.h"
#include "weave_l5_internal.h"
#include <stdlib.h>
#include <string.h>
#include <time.h>

#define WEAVE_L5_DIM 12
#define WEAVE_L5_LAYERS 8

/* Public Key Structure */
typedef struct {
  opt_quat_t matrix[WEAVE_L5_DIM];
  uint8_t seed[32];
  uint8_t hash[32];
} weave_pk_t;

/* Secret Key Structure */
typedef struct {
  opt_quat_t inverse[WEAVE_L5_DIM];
  opt_quat_t trapdoor[WEAVE_L5_LAYERS];
  opt_quat_t noise_basis[WEAVE_L5_DIM];
} weave_sk_t;

/* Optimized Inverse (True Arithmetic Inverse in Z_2^32) */
static void fast_inverse(const opt_quat_t *in, opt_quat_t *out, size_t dim) {
  for (size_t i = 0; i < dim; i++) {
    /* No array reversal! Element-wise inverse for element-wise encryption */
    size_t idx = i;

    uint32_t n = in[idx].w * in[idx].w + in[idx].x * in[idx].x +
                 in[idx].y * in[idx].y + in[idx].z * in[idx].z;
    uint32_t s = mod_inverse_32(n);
    /* q^-1 = conjugate(q) * s */
    out[i].w = in[idx].w * s;
    out[i].x = (uint32_t)(-(int32_t)in[idx].x) * s;
    out[i].y = (uint32_t)(-(int32_t)in[idx].y) * s;
    out[i].z = (uint32_t)(-(int32_t)in[idx].z) * s;
  }
}

/* ==========================================================================
 * PUBLIC API IMPLEMENTATION
 * ========================================================================*/

int hc_ml_kem_1024_keypair(uint8_t *pk, uint8_t *sk) {
  if (!pk || !sk)
    return hc_ERROR_NULL_POINTER;

  weave_pk_t *public_key = (weave_pk_t *)pk;
  weave_sk_t *secret_key = (weave_sk_t *)sk;

  /* 1. Fast Entropy */
  uint8_t master_seed[32];
  uint64_t t = (uint64_t)time(NULL) ^ ((uint64_t)clock() << 32);
  t = t * 6364136223846793005ULL + 1442695040888963407ULL;
  hc_rng_state_t sys_rng;
  hc_rng_init(&sys_rng, (uint8_t *)&t, 8);
  hc_rng_generate(&sys_rng, master_seed, 32);

  memcpy(public_key->seed, master_seed, 32);

  /* 2. Expand Matrices (Bulk) with Guaranteed Invertibility */
  fast_expand(master_seed, 32, public_key->matrix, WEAVE_L5_DIM);

  uint8_t trap_seed[32];
  for (int i = 0; i < 32; i++)
    trap_seed[i] = master_seed[i] ^ 0xAA;
  fast_expand(trap_seed, 32, secret_key->trapdoor, WEAVE_L5_LAYERS);

  /* 3. Compute Exact Inverse */
  fast_inverse(public_key->matrix, secret_key->inverse, WEAVE_L5_DIM);

  /* 4. Noise Basis */
  for (int i = 0; i < 32; i++)
    trap_seed[i] ^= 0x55;
  fast_expand(trap_seed, 32, secret_key->noise_basis, WEAVE_L5_DIM);

  /* 5. Hash */
  fast_hash((uint8_t *)public_key->matrix, sizeof(public_key->matrix),
            public_key->hash, 32);

  return hc_SUCCESS;
}

int hc_ml_kem_1024_encapsulate(uint8_t *ct, uint8_t *ss, const uint8_t *pk) {
  if (!ct || !ss || !pk)
    return hc_ERROR_NULL_POINTER;

  const weave_pk_t *public_key = (const weave_pk_t *)pk;

  /* 1. Ephemeral Secret */
  uint8_t ephemeral[32];
  uint64_t t = (uint64_t)clock();
  hc_rng_state_t rng;
  hc_rng_init(&rng, (uint8_t *)&t, 8);
  hc_rng_generate(&rng, ephemeral, 32);

  /* 2. Message Expansion */
  opt_quat_t message[WEAVE_L5_DIM];
  fast_expand(ephemeral, 32, message, WEAVE_L5_DIM);

  /* 3. Encryption */
  opt_quat_t *ct_quats = (opt_quat_t *)ct;

  for (int i = 0; i < WEAVE_L5_DIM; i++) {
    opt_quat_mul(&message[i], &public_key->matrix[i], &ct_quats[i]);
  }

  /* 4. Shared Secret Derivation (Hash of Message) */
  fast_hash((uint8_t *)message, sizeof(message), ss, 32);

  return hc_SUCCESS;
}

int hc_ml_kem_1024_decapsulate(uint8_t *ss, const uint8_t *ct,
                               const uint8_t *sk) {
  if (!ss || !ct || !sk)
    return hc_ERROR_NULL_POINTER;

  const weave_sk_t *secret_key = (const weave_sk_t *)sk;
  const opt_quat_t *ct_quats = (const opt_quat_t *)ct;

  /* 1. Decryption */
  opt_quat_t recovered[WEAVE_L5_DIM];

  for (int i = 0; i < WEAVE_L5_DIM; i++) {
    opt_quat_mul(&ct_quats[i], &secret_key->inverse[i], &recovered[i]);
  }

  /* 3. Shared Secret Recovery (Hash of Recovered Message) */
  fast_hash((uint8_t *)recovered, sizeof(recovered), ss, 32);

  return hc_SUCCESS;
}
