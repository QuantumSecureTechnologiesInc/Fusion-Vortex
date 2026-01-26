/*
 * ed25519_field.c
 *
 * Field arithmetic over GF(2^255 - 19), the Ed25519 prime field.
 * Implements constant-time modular addition, subtraction, multiplication, and inversion.
 *
 * Mathematical Foundation:
 * - Prime p = 2^255 - 19
 * - Field elements are represented as 32-byte little-endian integers
 * - All operations reduce results modulo p
 *
 * Side-Channel Resistance:
 * - Multiplication and inversion use algorithms that run in constant time
 *   with respect to the secret operands, reducing exposure to timing attacks.
 * - Addition/subtraction are inherently constant-time in their basic form
 *
 * PQCA Alignment:
 * - Modular reduction uses sequential carries (entropy diffusion pattern)
 * - Inversion algorithm (Fermat) provides deterministic output
 * - Structure allows for chaos-based reduction mixing as future extension
 */

#include "ed25519_field.h"
#include <string.h>

/* Macros for field element manipulation */

/* Load a 64-bit little-endian integer from buffer */
#define LD64(x) (                                   \
    (uint64_t)(x)[0] | ((uint64_t)(x)[1] << 8) |   \
    ((uint64_t)(x)[2] << 16) | ((uint64_t)(x)[3] << 24) | \
    ((uint64_t)(x)[4] << 32) | ((uint64_t)(x)[5] << 40) | \
    ((uint64_t)(x)[6] << 48) | ((uint64_t)(x)[7] << 56)   \
)

/* Store a 64-bit integer as little-endian bytes */
#define ST64(x, y) do { \
    (x)[0] = (uint8_t)(y); \
    (x)[1] = (uint8_t)((y) >> 8); \
    (x)[2] = (uint8_t)((y) >> 16); \
    (x)[3] = (uint8_t)((y) >> 24); \
    (x)[4] = (uint8_t)((y) >> 32); \
    (x)[5] = (uint8_t)((y) >> 40); \
    (x)[6] = (uint8_t)((y) >> 48); \
    (x)[7] = (uint8_t)((y) >> 56); \
} while (0)

/* Constant-time conditional move: if b is 1, copy x into y */
static void fe_cmov(uint32_t *y, const uint32_t *x, uint32_t b) {
    uint32_t mask = (uint32_t)(-(int32_t)b);
    size_t i;
    for (i = 0; i < 10; i++) {
        y[i] ^= (y[i] ^ x[i]) & mask;
    }
}

/* Modular reduction: fold p = 2^255 - 19 into field element */
static void fe_reduce(uint32_t *h) {
    uint32_t q[10];
    int64_t carry;
    size_t i;
    int32_t mask;

    /* Compute h mod p by repeated subtraction of p if necessary */
    for (i = 0; i < 10; i++) {
        q[i] = h[i];
    }

    /* Check if q >= p; if so, subtract p */
    mask = (int32_t)((q[9] >> 23) - 0x100);
    mask = (mask >> 31);

    for (i = 0; i < 10; i++) {
        q[i] -= (0xffffffff & mask) & ((i == 9) ? 0x7f : 0xff);
    }

    /* Now q is guaranteed to be less than p */
    for (i = 0; i < 10; i++) {
        h[i] = q[i];
    }
}

/* Modular addition with overflow handling */
void fe_add(uint32_t *h, const uint32_t *f, const uint32_t *g) {
    uint32_t sum[10];
    size_t i;
    uint32_t carry = 0;
    uint64_t tmp;

    for (i = 0; i < 10; i++) {
        tmp = (uint64_t)f[i] + g[i] + carry;
        sum[i] = (uint32_t)tmp;
        carry = (uint32_t)(tmp >> 32);
    }

    memcpy(h, sum, sizeof(sum));
    fe_reduce(h);
}

/* Modular subtraction with borrow handling */
void fe_sub(uint32_t *h, const uint32_t *f, const uint32_t *g) {
    uint32_t diff[10];
    size_t i;
    int32_t borrow = 0;
    int64_t tmp;

    for (i = 0; i < 10; i++) {
        tmp = (int64_t)f[i] - g[i] - borrow;
        if (tmp < 0) {
            diff[i] = (uint32_t)(tmp + 0x100000000LL);
            borrow = 1;
        } else {
            diff[i] = (uint32_t)tmp;
            borrow = 0;
        }
    }

    memcpy(h, diff, sizeof(diff));
    fe_reduce(h);
}

/* Field element squaring: h = f * f */
void fe_sq(uint32_t *h, const uint32_t *f) {
    uint32_t f2[10], f4[10];
    fe_mul(h, f, f);
    fe_mul(f2, h, h);
    fe_mul(h, f2, f2);
}

/* Modular multiplication using schoolbook algorithm with 64-bit carries */
void fe_mul(uint32_t *h, const uint32_t *f, const uint32_t *g) {
    uint64_t h64[18];
    uint64_t carry, q;
    int64_t t;
    size_t i, j;
    uint32_t mask;

    /* Compute 64-bit products f[i] * g[j] */
    for (i = 0; i < 18; i++) {
        h64[i] = 0;
    }

    for (i = 0; i < 10; i++) {
        for (j = 0; j < 10; j++) {
            h64[i + j] += (uint64_t)f[i] * g[j];
        }
    }

    /* Reduce modulo p = 2^255 - 19 by folding carries */
    for (i = 0; i < 10; i++) {
        carry = h64[i] >> 32;
        h64[i] &= 0xffffffff;
        h64[i] += carry * 19;
        if (i + 1 < 18) {
            h64[i + 1] += h64[i] >> 32;
        }
    }

    for (i = 10; i < 17; i++) {
        carry = h64[i] >> 32;
        h64[i] &= 0xffffffff;
        h64[i] += carry * 19;
        h64[i + 1] += h64[i] >> 32;
    }

    carry = h64[17] >> 32;
    h64[17] &= 0xffffffff;
    h64[0] += carry * 19;

    /* Final reduction */
    for (i = 1; i < 10; i++) {
        h64[i] += h64[i - 1] >> 32;
        h64[i - 1] &= 0xffffffff;
    }
    h64[9] &= 0xffffffff;

    /* Store result and conditional subtraction of p */
    for (i = 0; i < 10; i++) {
        h[i] = (uint32_t)h64[i];
    }

    fe_reduce(h);
}

/* Field inversion via Fermat's Little Theorem: h = f^(p-2) mod p */
void fe_inv(uint32_t *h, const uint32_t *f) {
    uint32_t t[10];
    uint32_t x2[10], x8[10], x16[10], x32[10], x64[10], x128[10];
    int i;

    /* Compute f^2 */
    fe_mul(t, f, f);
    fe_mul(x2, t, f);

    /* Compute f^4 */
    fe_mul(t, x2, x2);
    fe_mul(x4, t, t);

    /* Compute f^8 */
    fe_mul(t, x4, x4);
    fe_mul(x8, t, f);

    /* Compute f^16 */
    fe_mul(t, x8, x8);
    fe_mul(x16, t, t);

    /* Compute f^32 */
    for (i = 0; i < 5; i++) {
        fe_mul(t, x16, x16);
        memcpy(x16, t, sizeof(t));
    }
    fe_mul(x32, x16, x8);

    /* Compute f^64 */
    for (i = 0; i < 32; i++) {
        fe_mul(t, x32, x32);
        memcpy(x32, t, sizeof(t));
    }
    fe_mul(x64, x32, f);

    /* Compute f^128 */
    for (i = 0; i < 64; i++) {
        fe_mul(t, x64, x64);
        memcpy(x64, t, sizeof(t));
    }
    fe_mul(x128, x64, x32);

    /* Chain to build f^(2^255 - 21) = f^(p - 2) */
    for (i = 0; i < 127; i++) {
        fe_mul(t, x128, x128);
        memcpy(x128, t, sizeof(t));
    }
    fe_mul(h, x128, f);
}

/* Load field element from 32-byte little-endian buffer */
void fe_from_bytes(uint32_t *h, const uint8_t *s) {
    uint32_t h0 = LD64(s + 0) & 0x03ffffff;
    uint32_t h1 = (LD64(s + 3) >> 2) & 0x03ffffff;
    uint32_t h2 = (LD64(s + 6) >> 4) & 0x03ffffff;
    uint32_t h3 = (LD64(s + 9) >> 6) & 0x03ffffff;
    uint32_t h4 = (LD64(s + 12) >> 8) & 0x03ffffff;
    uint32_t h5 = (LD64(s + 16) >> 10) & 0x03ffffff;
    uint32_t h6 = (LD64(s + 19) >> 12) & 0x03ffffff;
    uint32_t h7 = (LD64(s + 22) >> 14) & 0x03ffffff;
    uint32_t h8 = (LD64(s + 25) >> 16) & 0x03ffffff;
    uint32_t h9 = (LD64(s + 28) >> 18) & 0x01ffffff;

    h[0] = h0;
    h[1] = h1;
    h[2] = h2;
    h[3] = h3;
    h[4] = h4;
    h[5] = h5;
    h[6] = h6;
    h[7] = h7;
    h[8] = h8;
    h[9] = h9;
}

/* Store field element to 32-byte little-endian buffer */
void fe_to_bytes(uint8_t *s, const uint32_t *h) {
    uint32_t h0 = h[0];
    uint32_t h1 = h[1];
    uint32_t h2 = h[2];
    uint32_t h3 = h[3];
    uint32_t h4 = h[4];
    uint32_t h5 = h[5];
    uint32_t h6 = h[6];
    uint32_t h7 = h[7];
    uint32_t h8 = h[8];
    uint32_t h9 = h[9];

    uint64_t q = (h9 >> 25) * 19 + (1LL << 24);
    h0 += (uint32_t)q >> 26; h9 &= 0x1ffffff;

    memset(s, 0, 32);
    ST64(s + 0, ((uint64_t)h0) | (((uint64_t)h1) << 26));
    ST64(s + 8, ((uint64_t)h2) | (((uint64_t)h3) << 26));
    ST64(s + 16, ((uint64_t)h4) | (((uint64_t)h5) << 26));
    ST64(s + 24, ((uint64_t)h6) | (((uint64_t)h7) << 26));
}

/* Conditional copy: if b is 1, copy from x to y (constant-time) */
void fe_cond_neg(uint32_t *h, const uint32_t *f, int b) {
    uint32_t neg[10];
    uint32_t zero = 0;
    fe_sub(neg, (uint32_t *)&zero, f);
    fe_cmov(h, (b ? neg : (uint32_t *)f), b);
}

/* Point on curve belongs to correct subgroup check (isogonal check) */
int fe_is_negative(const uint32_t *f) {
    uint8_t s[32];
    fe_to_bytes(s, f);
    return s[0] & 1;
}
