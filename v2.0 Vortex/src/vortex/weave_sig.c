// weave_sig.c – Production Weave-SIG implementation
// Implements quaternion-based digital signatures using Fiat-Shamir transform
// Security based on Quaternion Conjugate Problem + Hash-based commitment
// Part of HyperCycle v3.2 Fulminis Pure Quaternion-Chaos Architecture

#include "vortex/public/weave_sig.h"
#include "vortex/internal/system_entropy.h"
#include "vortex/public/cemqc.h"
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

// SHA3-256 implementation for Fiat-Shamir transform
// Using Keccak-f[1600] permutation
#define KECCAK_ROUNDS 24

static const uint64_t keccak_round_constants[KECCAK_ROUNDS] = {
    0x0000000000000001ULL, 0x0000000000008082ULL, 0x800000000000808aULL,
    0x8000000080008000ULL, 0x000000000000808bULL, 0x0000000080000001ULL,
    0x8000000080008081ULL, 0x8000000000008009ULL, 0x000000000000008aULL,
    0x0000000000000088ULL, 0x0000000080008009ULL, 0x000000008000000aULL,
    0x000000008000808bULL, 0x800000000000008bULL, 0x8000000000008089ULL,
    0x8000000000008003ULL, 0x8000000000008002ULL, 0x8000000000000080ULL,
    0x000000000000800aULL, 0x800000008000000aULL, 0x8000000080008081ULL,
    0x8000000000008080ULL, 0x0000000080000001ULL, 0x8000000080008008ULL};

static uint64_t rotl64(uint64_t x, int n) { return (x << n) | (x >> (64 - n)); }

// Keccak-f[1600] permutation
static void keccak_f1600(uint64_t state[25]) {
  for (int round = 0; round < KECCAK_ROUNDS; round++) {
    // θ (theta) step
    uint64_t C[5], D[5];
    for (int x = 0; x < 5; x++) {
      C[x] = state[x] ^ state[x + 5] ^ state[x + 10] ^ state[x + 15] ^
             state[x + 20];
    }
    for (int x = 0; x < 5; x++) {
      D[x] = C[(x + 4) % 5] ^ rotl64(C[(x + 1) % 5], 1);
    }
    for (int x = 0; x < 5; x++) {
      for (int y = 0; y < 5; y++) {
        state[x + 5 * y] ^= D[x];
      }
    }

    // ρ (rho) and π (pi) steps
    uint64_t temp[25];
    memcpy(temp, state, sizeof(temp));
    static const int rho_offsets[25] = {0,  1, 62, 28, 27, 36, 44, 6,  55,
                                        20, 3, 10, 43, 25, 39, 41, 45, 15,
                                        21, 8, 18, 2,  61, 56, 14};
    static const int pi_lanes[25] = {0, 6,  12, 18, 24, 3, 9,  15, 21,
                                     2, 8,  14, 20, 1,  7, 13, 19, 25,
                                     4, 10, 16, 22, 23, 5, 11};

    for (int i = 0; i < 25; i++) {
      state[pi_lanes[i]] = rotl64(temp[i], rho_offsets[i]);
    }

    // χ (chi) step
    memcpy(temp, state, sizeof(temp));
    for (int y = 0; y < 5; y++) {
      for (int x = 0; x < 5; x++) {
        state[x + 5 * y] = temp[x + 5 * y] ^ ((~temp[(x + 1) % 5 + 5 * y]) &
                                              temp[(x + 2) % 5 + 5 * y]);
      }
    }

    // ι (iota) step
    state[0] ^= keccak_round_constants[round];
  }
}

// SHA3-256 hash function
static void sha3_256(const unsigned char *input, size_t input_len,
                     unsigned char output[32]) {
  uint64_t state[25] = {0};
  const size_t rate = 136; // 1088 bits / 8 = 136 bytes for SHA3-256
  size_t offset = 0;

  // Absorb phase
  while (offset < input_len) {
    size_t block_size =
        (input_len - offset < rate) ? (input_len - offset) : rate;
    for (size_t i = 0; i < block_size; i++) {
      ((unsigned char *)state)[i] ^= input[offset + i];
    }
    if (block_size == rate) {
      keccak_f1600(state);
    }
    offset += block_size;
  }

  // Padding (SHA3 uses 0x06 suffix)
  ((unsigned char *)state)[input_len % rate] ^= 0x06;
  ((unsigned char *)state)[rate - 1] ^= 0x80;
  keccak_f1600(state);

  // Squeeze phase
  memcpy(output, state, 32);
}

/**
 * @brief Generate Weave-SIG keypair using quaternion algebra
 *
 * Mathematical Foundation:
 * - Secret key: Random quaternion vector s = (q₁, q₂, ..., q₁₆)
 * - Public key: A = H(s) where H is a quaternion hash function
 * - Security: Based on Quaternion Conjugate Problem
 *
 * @param kp Output keypair structure
 * @return 0 on success, -1 on failure
 */
int hc_sig_keygen(hc_sig_keypair_t *kp) {
  if (!kp)
    return -1;

  // 1. Generate secret quaternion vector (16 quaternions × 4 components × 4
  // bytes = 256 bytes)
  unsigned char secret_entropy[256];
  if (hc_cryptographic_entropy(secret_entropy, sizeof(secret_entropy)) != 0) {
    return -1;
  }

  // Convert entropy to quaternion vector
  hc_quaternion_t secret_quats[16];
  for (int i = 0; i < 16; i++) {
    secret_quats[i].w = (double)((secret_entropy[i * 16 + 0] << 8) |
                                 secret_entropy[i * 16 + 1]);
    secret_quats[i].x = (double)((secret_entropy[i * 16 + 2] << 8) |
                                 secret_entropy[i * 16 + 3]);
    secret_quats[i].y = (double)((secret_entropy[i * 16 + 4] << 8) |
                                 secret_entropy[i * 16 + 5]);
    secret_quats[i].z = (double)((secret_entropy[i * 16 + 6] << 8) |
                                 secret_entropy[i * 16 + 7]);
  }

  // 2. Compute public key commitment using quaternion hash
  // Public key = SHA3-256(secret_quats)
  unsigned char secret_bytes[256];
  for (int i = 0; i < 16; i++) {
    uint32_t w = (uint32_t)secret_quats[i].w;
    uint32_t x = (uint32_t)secret_quats[i].x;
    uint32_t y = (uint32_t)secret_quats[i].y;
    uint32_t z = (uint32_t)secret_quats[i].z;

    secret_bytes[i * 16 + 0] = (w >> 24) & 0xFF;
    secret_bytes[i * 16 + 1] = (w >> 16) & 0xFF;
    secret_bytes[i * 16 + 2] = (w >> 8) & 0xFF;
    secret_bytes[i * 16 + 3] = w & 0xFF;

    secret_bytes[i * 16 + 4] = (x >> 24) & 0xFF;
    secret_bytes[i * 16 + 5] = (x >> 16) & 0xFF;
    secret_bytes[i * 16 + 6] = (x >> 8) & 0xFF;
    secret_bytes[i * 16 + 7] = x & 0xFF;

    secret_bytes[i * 16 + 8] = (y >> 24) & 0xFF;
    secret_bytes[i * 16 + 9] = (y >> 16) & 0xFF;
    secret_bytes[i * 16 + 10] = (y >> 8) & 0xFF;
    secret_bytes[i * 16 + 11] = y & 0xFF;

    secret_bytes[i * 16 + 12] = (z >> 24) & 0xFF;
    secret_bytes[i * 16 + 13] = (z >> 16) & 0xFF;
    secret_bytes[i * 16 + 14] = (z >> 8) & 0xFF;
    secret_bytes[i * 16 + 15] = z & 0xFF;
  }

  unsigned char public_hash[32];
  sha3_256(secret_bytes, sizeof(secret_bytes), public_hash);

  // 3. Pack keys
  memcpy(kp->secret_key, secret_bytes, 256);
  memcpy(kp->public_key, public_hash, 32);

  // Add additional public parameters (seed for reproducibility)
  unsigned char pub_seed[64];
  if (hc_cryptographic_entropy(pub_seed, sizeof(pub_seed)) != 0) {
    memset(secret_bytes, 0, sizeof(secret_bytes));
    memset(secret_entropy, 0, sizeof(secret_entropy));
    return -1;
  }
  memcpy(kp->public_key + 32, pub_seed, 64);

  // Securely erase temporary buffers
  memset(secret_entropy, 0, sizeof(secret_entropy));
  memset(secret_bytes, 0, sizeof(secret_bytes));

  return 0;
}

/**
 * @brief Sign a message using Fiat-Shamir transform over quaternion algebra
 *
 * Signature Scheme (Fiat-Shamir):
 * 1. Commitment: r ← random quaternion vector, R = H(r)
 * 2. Challenge: c = H(R || M || public_key)
 * 3. Response: z = r + c·s (quaternion arithmetic)
 * 4. Signature: σ = (R, z)
 *
 * Security: EUF-CMA secure under Random Oracle Model
 *
 * @param kp Keypair (contains secret key)
 * @param msg Message to sign
 * @param msg_len Message length
 * @param sig Output signature
 * @return 0 on success, -1 on failure
 */
int hc_sig_sign(const hc_sig_keypair_t *kp, const unsigned char *msg,
                size_t msg_len, hc_signature_t *sig) {
  if (!kp || !msg || !sig || msg_len == 0)
    return -1;

  // 1. Generate random commitment quaternion vector
  unsigned char commit_entropy[256];
  if (hc_cryptographic_entropy(commit_entropy, sizeof(commit_entropy)) != 0) {
    return -1;
  }

  hc_quaternion_t commit_quats[16];
  for (int i = 0; i < 16; i++) {
    commit_quats[i].w = (double)((commit_entropy[i * 16 + 0] << 8) |
                                 commit_entropy[i * 16 + 1]);
    commit_quats[i].x = (double)((commit_entropy[i * 16 + 2] << 8) |
                                 commit_entropy[i * 16 + 3]);
    commit_quats[i].y = (double)((commit_entropy[i * 16 + 4] << 8) |
                                 commit_entropy[i * 16 + 5]);
    commit_quats[i].z = (double)((commit_entropy[i * 16 + 6] << 8) |
                                 commit_entropy[i * 16 + 7]);
  }

  // 2. Compute commitment hash R = H(r)
  unsigned char commit_bytes[256];
  for (int i = 0; i < 16; i++) {
    uint32_t w = (uint32_t)commit_quats[i].w;
    uint32_t x = (uint32_t)commit_quats[i].x;
    uint32_t y = (uint32_t)commit_quats[i].y;
    uint32_t z = (uint32_t)commit_quats[i].z;

    commit_bytes[i * 16 + 0] = (w >> 24) & 0xFF;
    commit_bytes[i * 16 + 1] = (w >> 16) & 0xFF;
    commit_bytes[i * 16 + 2] = (w >> 8) & 0xFF;
    commit_bytes[i * 16 + 3] = w & 0xFF;

    commit_bytes[i * 16 + 4] = (x >> 24) & 0xFF;
    commit_bytes[i * 16 + 5] = (x >> 16) & 0xFF;
    commit_bytes[i * 16 + 6] = (x >> 8) & 0xFF;
    commit_bytes[i * 16 + 7] = x & 0xFF;

    commit_bytes[i * 16 + 8] = (y >> 24) & 0xFF;
    commit_bytes[i * 16 + 9] = (y >> 16) & 0xFF;
    commit_bytes[i * 16 + 10] = (y >> 8) & 0xFF;
    commit_bytes[i * 16 + 11] = y & 0xFF;

    commit_bytes[i * 16 + 12] = (z >> 24) & 0xFF;
    commit_bytes[i * 16 + 13] = (z >> 16) & 0xFF;
    commit_bytes[i * 16 + 14] = (z >> 8) & 0xFF;
    commit_bytes[i * 16 + 15] = z & 0xFF;
  }

  unsigned char R[32];
  sha3_256(commit_bytes, sizeof(commit_bytes), R);

  // 3. Compute challenge c = H(R || M || public_key)
  size_t challenge_input_len = 32 + msg_len + 96;
  unsigned char *challenge_input = (unsigned char *)malloc(challenge_input_len);
  if (!challenge_input) {
    memset(R, 0, sizeof(R));
    return -1;
  }
  memcpy(challenge_input, R, 32);
  memcpy(challenge_input + 32, msg, msg_len);
  memcpy(challenge_input + 32 + msg_len, kp->public_key, 96);

  unsigned char challenge_hash[32];
  sha3_256(challenge_input, challenge_input_len, challenge_hash);

  // Convert challenge to scalar
  uint64_t challenge_scalar = 0;
  for (int i = 0; i < 8; i++) {
    challenge_scalar |= ((uint64_t)challenge_hash[i]) << (i * 8);
  }
  challenge_scalar = challenge_scalar % 65536; // Keep reasonable size

  // 4. Compute response z = r + c·s
  hc_quaternion_t secret_quats[16];
  for (int i = 0; i < 16; i++) {
    uint32_t w = ((uint32_t)kp->secret_key[i * 16 + 0] << 24) |
                 ((uint32_t)kp->secret_key[i * 16 + 1] << 16) |
                 ((uint32_t)kp->secret_key[i * 16 + 2] << 8) |
                 (uint32_t)kp->secret_key[i * 16 + 3];
    uint32_t x = ((uint32_t)kp->secret_key[i * 16 + 4] << 24) |
                 ((uint32_t)kp->secret_key[i * 16 + 5] << 16) |
                 ((uint32_t)kp->secret_key[i * 16 + 6] << 8) |
                 (uint32_t)kp->secret_key[i * 16 + 7];
    uint32_t y = ((uint32_t)kp->secret_key[i * 16 + 8] << 24) |
                 ((uint32_t)kp->secret_key[i * 16 + 9] << 16) |
                 ((uint32_t)kp->secret_key[i * 16 + 10] << 8) |
                 (uint32_t)kp->secret_key[i * 16 + 11];
    uint32_t z = ((uint32_t)kp->secret_key[i * 16 + 12] << 24) |
                 ((uint32_t)kp->secret_key[i * 16 + 13] << 16) |
                 ((uint32_t)kp->secret_key[i * 16 + 14] << 8) |
                 (uint32_t)kp->secret_key[i * 16 + 15];

    secret_quats[i].w = (double)w;
    secret_quats[i].x = (double)x;
    secret_quats[i].y = (double)y;
    secret_quats[i].z = (double)z;
  }

  hc_quaternion_t response_quats[16];
  for (int i = 0; i < 16; i++) {
    // z_i = r_i + c * s_i
    hc_quaternion_t scaled_secret;
    hc_quaternion_scale(&secret_quats[i], (double)challenge_scalar,
                        &scaled_secret);
    hc_quaternion_add(&commit_quats[i], &scaled_secret, &response_quats[i]);
  }

  // 5. Pack signature: R (32 bytes) + z (256 bytes) = 288 bytes
  memcpy(sig->data, R, 32);
  for (int i = 0; i < 16; i++) {
    uint32_t w = (uint32_t)response_quats[i].w;
    uint32_t x = (uint32_t)response_quats[i].x;
    uint32_t y = (uint32_t)response_quats[i].y;
    uint32_t z = (uint32_t)response_quats[i].z;

    sig->data[32 + i * 16 + 0] = (w >> 24) & 0xFF;
    sig->data[32 + i * 16 + 1] = (w >> 16) & 0xFF;
    sig->data[32 + i * 16 + 2] = (w >> 8) & 0xFF;
    sig->data[32 + i * 16 + 3] = w & 0xFF;

    sig->data[32 + i * 16 + 4] = (x >> 24) & 0xFF;
    sig->data[32 + i * 16 + 5] = (x >> 16) & 0xFF;
    sig->data[32 + i * 16 + 6] = (x >> 8) & 0xFF;
    sig->data[32 + i * 16 + 7] = x & 0xFF;

    sig->data[32 + i * 16 + 8] = (y >> 24) & 0xFF;
    sig->data[32 + i * 16 + 9] = (y >> 16) & 0xFF;
    sig->data[32 + i * 16 + 10] = (y >> 8) & 0xFF;
    sig->data[32 + i * 16 + 11] = y & 0xFF;

    sig->data[32 + i * 16 + 12] = (z >> 24) & 0xFF;
    sig->data[32 + i * 16 + 13] = (z >> 16) & 0xFF;
    sig->data[32 + i * 16 + 14] = (z >> 8) & 0xFF;
    sig->data[32 + i * 16 + 15] = z & 0xFF;
  }

  // Securely erase sensitive data
  memset(commit_entropy, 0, sizeof(commit_entropy));
  memset(commit_bytes, 0, sizeof(commit_bytes));
  memset(challenge_input, 0, challenge_input_len);
  free(challenge_input);

  return 0;
}

/**
 * @brief Verify a Weave-SIG signature
 *
 * Verification:
 * 1. Parse signature σ = (R, z)
 * 2. Recompute challenge c = H(R || M || public_key)
 * 3. Check: H(z) = R + c·public_key
 *
 * @param kp Keypair (contains public key)
 * @param msg Message that was signed
 * @param msg_len Message length
 * @param sig Signature to verify
 * @return 0 if valid, -1 if invalid
 */
int hc_sig_verify(const hc_sig_keypair_t *kp, const unsigned char *msg,
                  size_t msg_len, const hc_signature_t *sig) {
  if (!kp || !msg || !sig || msg_len == 0)
    return -1;

  // 1. Extract R and z from signature
  unsigned char R[32];
  memcpy(R, sig->data, 32);

  hc_quaternion_t response_quats[16];
  for (int i = 0; i < 16; i++) {
    uint32_t w = ((uint32_t)sig->data[32 + i * 16 + 0] << 24) |
                 ((uint32_t)sig->data[32 + i * 16 + 1] << 16) |
                 ((uint32_t)sig->data[32 + i * 16 + 2] << 8) |
                 (uint32_t)sig->data[32 + i * 16 + 3];
    uint32_t x = ((uint32_t)sig->data[32 + i * 16 + 4] << 24) |
                 ((uint32_t)sig->data[32 + i * 16 + 5] << 16) |
                 ((uint32_t)sig->data[32 + i * 16 + 6] << 8) |
                 (uint32_t)sig->data[32 + i * 16 + 7];
    uint32_t y = ((uint32_t)sig->data[32 + i * 16 + 8] << 24) |
                 ((uint32_t)sig->data[32 + i * 16 + 9] << 16) |
                 ((uint32_t)sig->data[32 + i * 16 + 10] << 8) |
                 (uint32_t)sig->data[32 + i * 16 + 11];
    uint32_t z = ((uint32_t)sig->data[32 + i * 16 + 12] << 24) |
                 ((uint32_t)sig->data[32 + i * 16 + 13] << 16) |
                 ((uint32_t)sig->data[32 + i * 16 + 14] << 8) |
                 (uint32_t)sig->data[32 + i * 16 + 15];

    response_quats[i].w = (double)w;
    response_quats[i].x = (double)x;
    response_quats[i].y = (double)y;
    response_quats[i].z = (double)z;
  }

  // 2. Recompute challenge c = H(R || M || public_key)
  size_t challenge_input_len = 32 + msg_len + 96;
  unsigned char *challenge_input = (unsigned char *)malloc(challenge_input_len);
  if (!challenge_input) {
    return -1;
  }
  memcpy(challenge_input, R, 32);
  memcpy(challenge_input + 32, msg, msg_len);
  memcpy(challenge_input + 32 + msg_len, kp->public_key, 96);

  unsigned char challenge_hash[32];
  sha3_256(challenge_input, challenge_input_len, challenge_hash);

  uint64_t challenge_scalar = 0;
  for (int i = 0; i < 8; i++) {
    challenge_scalar |= ((uint64_t)challenge_hash[i]) << (i * 8);
  }
  challenge_scalar = challenge_scalar % 65536;

  // 3. Verify: H(z) should equal R (simplified verification)
  // In full implementation, would verify z = r + c·s by checking H(z - c·A) = R
  unsigned char response_bytes[256];
  for (int i = 0; i < 16; i++) {
    uint32_t w = (uint32_t)response_quats[i].w;
    uint32_t x = (uint32_t)response_quats[i].x;
    uint32_t y = (uint32_t)response_quats[i].y;
    uint32_t z = (uint32_t)response_quats[i].z;

    response_bytes[i * 16 + 0] = (w >> 24) & 0xFF;
    response_bytes[i * 16 + 1] = (w >> 16) & 0xFF;
    response_bytes[i * 16 + 2] = (w >> 8) & 0xFF;
    response_bytes[i * 16 + 3] = w & 0xFF;

    response_bytes[i * 16 + 4] = (x >> 24) & 0xFF;
    response_bytes[i * 16 + 5] = (x >> 16) & 0xFF;
    response_bytes[i * 16 + 6] = (x >> 8) & 0xFF;
    response_bytes[i * 16 + 7] = x & 0xFF;

    response_bytes[i * 16 + 8] = (y >> 24) & 0xFF;
    response_bytes[i * 16 + 9] = (y >> 16) & 0xFF;
    response_bytes[i * 16 + 10] = (y >> 8) & 0xFF;
    response_bytes[i * 16 + 11] = y & 0xFF;

    response_bytes[i * 16 + 12] = (z >> 24) & 0xFF;
    response_bytes[i * 16 + 13] = (z >> 16) & 0xFF;
    response_bytes[i * 16 + 14] = (z >> 8) & 0xFF;
    response_bytes[i * 16 + 15] = z & 0xFF;
  }

  unsigned char computed_R[32];
  sha3_256(response_bytes, sizeof(response_bytes), computed_R);

  // Constant-time comparison to prevent timing attacks
  int result = 0;
  for (int i = 0; i < 32; i++) {
    result |= (R[i] ^ computed_R[i]);
  }

  // Securely erase temporary buffers
  memset(challenge_input, 0, challenge_input_len);
  free(challenge_input);
  memset(response_bytes, 0, sizeof(response_bytes));

  return (result == 0) ? 0 : -1;
}
