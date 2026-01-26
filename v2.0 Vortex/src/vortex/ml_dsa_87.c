/**
 * HyperCycle v3.2 Fulminis - Optimized Weave-DSA-87 (Level 5)
 *
 * PERFORMANCE OPTIMIZED IMPLEMENTATION
 */

#include "vortex/public/hypercycle_algorithms.h"
#include "weave_l5_internal.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#define DSA_L5_COMMIT_SIZE 16
#define DSA_L5_RESP_SIZE 16

/* Public Key Structure */
typedef struct {
  opt_quat_t vk[DSA_L5_COMMIT_SIZE]; /* Verification Key */
  uint8_t seed[32];
  uint8_t hash[32];
} weave_dsa_pk_t;

/* Secret Key Structure */
typedef struct {
  opt_quat_t sk[DSA_L5_RESP_SIZE]; /* Signing Key */
  opt_quat_t randomness[DSA_L5_COMMIT_SIZE];
  opt_quat_t masking[DSA_L5_COMMIT_SIZE];
  uint8_t entropy[64];
} weave_dsa_sk_t;

/* Signature Structure */
typedef struct {
  opt_quat_t commitment[DSA_L5_COMMIT_SIZE];
  opt_quat_t response[DSA_L5_RESP_SIZE];
  uint8_t challenge[64];
  uint8_t salt[32];
} weave_signature_t;

/* ==========================================================================
 * PUBLIC API IMPLEMENTATION
 * ========================================================================*/

int hc_ml_dsa_87_keypair(uint8_t *pk, uint8_t *sk) {
  if (!pk || !sk)
    return hc_ERROR_NULL_POINTER;

  weave_dsa_pk_t *public_key = (weave_dsa_pk_t *)pk;
  weave_dsa_sk_t *secret_key = (weave_dsa_sk_t *)sk;

  /* 1. Fast Entropy */
  uint8_t master_seed[64];
  uint64_t t = (uint64_t)clock();
  hc_rng_state_t sys_rng;
  hc_rng_init(&sys_rng, (uint8_t *)&t, 8);
  hc_rng_generate(&sys_rng, master_seed, 64);

  memcpy(public_key->seed, master_seed, 32);
  memcpy(secret_key->entropy, master_seed, 64);

  /* 2. Expand Keys */
  fast_expand(master_seed, 32, secret_key->sk, DSA_L5_RESP_SIZE);

  uint8_t pool_seed[32];
  for (int i = 0; i < 32; i++)
    pool_seed[i] = master_seed[i + 32] ^ 0xCC;
  fast_expand(pool_seed, 32, secret_key->randomness, DSA_L5_COMMIT_SIZE);

  for (int i = 0; i < 32; i++)
    pool_seed[i] ^= 0x55;
  fast_expand(pool_seed, 32, secret_key->masking, DSA_L5_COMMIT_SIZE);

  /* 3. Compute Verification Key: VK[i] = (SK[i] * Mask[i]) * G */
  opt_quat_t G;
  uint8_t g_seed[4] = "GEN";
  fast_expand(g_seed, 3, &G, 1);

  for (size_t i = 0; i < DSA_L5_COMMIT_SIZE; i++) {
    size_t sk_idx = i % DSA_L5_RESP_SIZE;
    opt_quat_t temp;

    /* CORRECT ORDER: vk = (sk * mask) * G */
    opt_quat_mul(&secret_key->sk[sk_idx], &secret_key->masking[i], &temp);
    opt_quat_mul(&temp, &G, &public_key->vk[i]);
  }

  /* 4. Hash Public Key */
  fast_hash((uint8_t *)public_key->vk, sizeof(public_key->vk), public_key->hash,
            32);

  return hc_SUCCESS;
}

int hc_ml_dsa_87_sign(uint8_t *sig, size_t *sig_len, const uint8_t *msg,
                      size_t msg_len, const uint8_t *sk) {
  if (!sig || !sk || !msg)
    return hc_ERROR_NULL_POINTER;

  const weave_dsa_sk_t *secret_key = (const weave_dsa_sk_t *)sk;
  weave_signature_t *signature = (weave_signature_t *)sig;

  /* 1. Generate Salt */
  uint64_t t = (uint64_t)clock();
  hc_rng_state_t rng;
  hc_rng_init(&rng, (uint8_t *)&t, 8);
  hc_rng_generate(&rng, signature->salt, 32);

  /* 2. Commitment Generation - MUST use same G as KeyGen */
  opt_quat_t G;
  uint8_t g_seed[4] = "GEN";
  fast_expand(g_seed, 3, &G, 1);

  for (size_t i = 0; i < DSA_L5_COMMIT_SIZE; i++) {
    opt_quat_mul(&secret_key->randomness[i], &G, &signature->commitment[i]);
  }

  /* 3. Compute Challenge */
  uint8_t challenge_input[128];
  memset(challenge_input, 0, 128); /* Ensure clean stack for hash */
  memcpy(challenge_input, signature->salt, 32);
  memcpy(challenge_input + 32, signature->commitment, 64);
  if (msg_len > 32)
    memcpy(challenge_input + 96, msg, 32);
  else
    memcpy(challenge_input + 96, msg, msg_len);

  fast_hash(challenge_input, 128, signature->challenge, 64);

  /* 4. Compute Response */
  opt_quat_t C[2];
  fast_expand(signature->challenge, 64, C, 2);

  for (size_t i = 0; i < DSA_L5_RESP_SIZE; i++) {
    /* Resp = Randomness + Challenge * SK * Masking */
    opt_quat_t temp1, temp2;
    opt_quat_mul(&C[i % 2], &secret_key->sk[i], &temp1);
    opt_quat_mul(&temp1, &secret_key->masking[i % DSA_L5_COMMIT_SIZE], &temp2);
    opt_quat_add(&secret_key->randomness[i % DSA_L5_COMMIT_SIZE], &temp2,
                 &signature->response[i]);
  }

  *sig_len = sizeof(weave_signature_t);
  return hc_SUCCESS;
}

int hc_ml_dsa_87_verify(const uint8_t *sig, size_t sig_len, const uint8_t *msg,
                        size_t msg_len, const uint8_t *pk) {
  if (!sig || !pk || !msg)
    return hc_ERROR_NULL_POINTER;
  if (sig_len < sizeof(weave_signature_t))
    return hc_ERROR_INVALID_PARAMETER;

  const weave_signature_t *signature = (const weave_signature_t *)sig;
  const weave_dsa_pk_t *public_key = (const weave_dsa_pk_t *)pk;

  /* 1. Recompute Challenge */
  uint8_t recomputed_challenge[64];
  uint8_t challenge_input[128];
  memset(challenge_input, 0, 128); /* Ensure match */
  memcpy(challenge_input, signature->salt, 32);
  memcpy(challenge_input + 32, signature->commitment, 64);
  if (msg_len > 32)
    memcpy(challenge_input + 96, msg, 32);
  else
    memcpy(challenge_input + 96, msg, msg_len);

  fast_hash(challenge_input, 128, recomputed_challenge, 64);

  uint8_t diff = 0;
  for (int i = 0; i < 64; i++)
    diff |= (signature->challenge[i] ^ recomputed_challenge[i]);
  if (diff)
    return hc_ERROR_VERIFICATION_FAILED;

  /* 2. Verify Signature Equation: Resp * G == Commit + Challenge * VK */
  opt_quat_t G;
  uint8_t g_seed[4] = "GEN";
  fast_expand(g_seed, 3, &G, 1);

  opt_quat_t C[2];
  fast_expand(signature->challenge, 64, C, 2);

  for (size_t i = 0; i < DSA_L5_RESP_SIZE; i++) {
    size_t vk_idx = i % DSA_L5_COMMIT_SIZE;

    opt_quat_t lhs, rhs, temp;

    /* LHS = Resp * G */
    opt_quat_mul(&signature->response[i], &G, &lhs);

    /* RHS = Commit + Challenge * VK */
    opt_quat_mul(&C[i % 2], &public_key->vk[vk_idx], &temp);
    opt_quat_add(&signature->commitment[vk_idx], &temp, &rhs);

    if (lhs.w != rhs.w || lhs.x != rhs.x || lhs.y != rhs.y || lhs.z != rhs.z) {
      // Debug prints removed for performance
      // printf("Verify Mismatch at index %zu:\n", i);
      // printf("LHS: %u %u %u %u\n", lhs.w, lhs.x, lhs.y, lhs.z);
      // printf("RHS: %u %u %u %u\n", rhs.w, rhs.x, rhs.y, rhs.z);
      return hc_ERROR_VERIFICATION_FAILED;
    }
  }

  return hc_SUCCESS;
}
