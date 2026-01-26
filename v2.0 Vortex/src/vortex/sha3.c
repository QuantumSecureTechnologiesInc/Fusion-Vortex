#include "internal/sha3.h"
#include <stdint.h>
#include <string.h>


#define KECCAK_ROUNDS 24

static const uint64_t keccakf_rndc[24] = {
    0x0000000000000001, 0x0000000000008082, 0x800000000000808a,
    0x8000000080008000, 0x000000000000808b, 0x0000000080000001,
    0x8000000080008081, 0x8000000000008009, 0x000000000000008a,
    0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
    0x000000008000808b, 0x800000000000008b, 0x8000000000008089,
    0x8000000000008003, 0x8000000000008002, 0x8000000000000080,
    0x000000000000800a, 0x800000008000000a, 0x8000000080008081,
    0x8000000000008080, 0x0000000080000001, 0x8000000080008008};

static const int keccakf_rotc[24] = {1,  3,  6,  10, 15, 21, 28, 36,
                                     45, 55, 2,  14, 27, 41, 56, 8,
                                     25, 43, 62, 18, 39, 61, 20, 44};

static const int keccakf_piln[24] = {10, 7,  11, 17, 18, 3,  5,  16,
                                     8,  21, 24, 4,  15, 23, 19, 13,
                                     12, 2,  20, 14, 22, 9,  6,  1};

static inline uint64_t rotl64(uint64_t x, int i) {
  return (x << i) | (x >> (64 - i));
}

static void keccakf(uint64_t st[25]) {
  int i, j, round;
  uint64_t t, bc[5];

  for (round = 0; round < KECCAK_ROUNDS; round++) {
    // Theta
    for (i = 0; i < 5; i++)
      bc[i] = st[i] ^ st[i + 5] ^ st[i + 10] ^ st[i + 15] ^ st[i + 20];

    for (i = 0; i < 5; i++) {
      t = bc[(i + 4) % 5] ^ rotl64(bc[(i + 1) % 5], 1);
      for (j = 0; j < 25; j += 5)
        st[j + i] ^= t;
    }

    // Rho Pi
    t = st[1];
    for (i = 0; i < 24; i++) {
      j = keccakf_piln[i];
      bc[0] = st[j];
      st[j] = rotl64(t, keccakf_rotc[i]);
      t = bc[0];
    }

    // Chi
    for (j = 0; j < 25; j += 5) {
      for (i = 0; i < 5; i++)
        bc[i] = st[j + i];
      for (i = 0; i < 5; i++)
        st[j + i] ^= (~bc[(i + 1) % 5]) & bc[(i + 2) % 5];
    }

    // Iota
    st[0] ^= keccakf_rndc[round];
  }
}

void hc_sha3_256(const uint8_t *in, size_t inlen, uint8_t *out) {
  uint64_t st[25];
  uint8_t *st8 = (uint8_t *)st;
  size_t i;
  int rsiz = 136; // Rate for SHA3-256

  memset(st, 0, sizeof(st));

  for (; inlen >= (size_t)rsiz; inlen -= rsiz, in += rsiz) {
    for (i = 0; i < (size_t)rsiz; i++)
      st8[i] ^= in[i];
    keccakf(st);
  }

  // Padding
  for (i = 0; i < inlen; i++)
    st8[i] ^= in[i];

  st8[inlen] ^= 0x06;
  st8[rsiz - 1] ^= 0x80;

  keccakf(st);

  for (i = 0; i < 32; i++)
    out[i] = st8[i];
}

void hc_shake256(uint8_t *out, size_t outlen, const uint8_t *in, size_t inlen) {
  uint64_t st[25];
  uint8_t *st8 = (uint8_t *)st;
  size_t i;
  int rsiz = 136;

  memset(st, 0, sizeof(st));

  for (; inlen >= (size_t)rsiz; inlen -= rsiz, in += rsiz) {
    for (i = 0; i < (size_t)rsiz; i++)
      st8[i] ^= in[i];
    keccakf(st);
  }

  // Squeeze padding
  for (i = 0; i < inlen; i++)
    st8[i] ^= in[i];

  st8[inlen] ^= 0x1F;
  st8[rsiz - 1] ^= 0x80;

  keccakf(st);

  while (outlen > 0) {
    size_t len = outlen < (size_t)rsiz ? outlen : (size_t)rsiz;
    for (i = 0; i < len; i++)
      out[i] = st8[i];

    out += len;
    outlen -= len;

    if (outlen > 0)
      keccakf(st);
  }
}
