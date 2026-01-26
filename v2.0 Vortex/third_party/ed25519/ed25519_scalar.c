/*
 * ed25519_scalar.c
 *
 * Complete scalar arithmetic modulo L (group order).
 * L = 2^252 + 27742317777884353535851937790883648493
 *
 * All operations are constant-time where secret material is involved.
 */

#include "ed25519_core.h"
#include <string.h>

/* Helper to load 3 bytes as little-endian */
static uint64_t load_3(const uint8_t *in) {
    return (uint64_t)in[0] | ((uint64_t)in[1] << 8) | ((uint64_t)in[2] << 16);
}

/* Helper to load 4 bytes as little-endian */
static uint64_t load_4(const uint8_t *in) {
    return (uint64_t)in[0] | ((uint64_t)in[1] << 8) | ((uint64_t)in[2] << 16) | ((uint64_t)in[3] << 24);
}

/*
 * Clamp scalar for key generation per RFC 8032 Section 5.1.5
 * - Clear bits 0, 1, 2
 * - Clear bit 255
 * - Set bit 254
 */
void sc_clamp(uint8_t *k) {
    k[0] &= 0xf8;      /* Clear bits 0, 1, 2 */
    k[31] &= 0x7f;     /* Clear bit 255 */
    k[31] |= 0x40;     /* Set bit 254 */
}

/*
 * Reduce 32-byte scalar modulo L
 * Input s is a 32-byte value, reduced to 0 < s < L
 */
void sc_reduce(uint8_t *s) {
    int64_t s0 = 2097151 & load_3(s);
    int64_t s1 = 2097151 & (load_4(s + 2) >> 5);
    int64_t s2 = 2097151 & (load_3(s + 5) >> 2);
    int64_t s3 = 2097151 & (load_4(s + 7) >> 7);
    int64_t s4 = 2097151 & (load_4(s + 10) >> 4);
    int64_t s5 = 2097151 & (load_3(s + 13) >> 1);
    int64_t s6 = 2097151 & (load_4(s + 15) >> 6);
    int64_t s7 = 2097151 & (load_3(s + 18) >> 3);
    int64_t s8 = 2097151 & load_3(s + 21);
    int64_t s9 = 2097151 & (load_4(s + 23) >> 5);
    int64_t s10 = 2097151 & (load_3(s + 26) >> 2);
    int64_t s11 = (load_4(s + 28) >> 7);

    s4 += s3 >> 21; s3 &= 2097151;
    s5 += s4 >> 21; s4 &= 2097151;
    s6 += s5 >> 21; s5 &= 2097151;
    s7 += s6 >> 21; s6 &= 2097151;
    s8 += s7 >> 21; s7 &= 2097151;
    s9 += s8 >> 21; s8 &= 2097151;
    s10 += s9 >> 21; s9 &= 2097151;
    s11 += s10 >> 21; s10 &= 2097151;

    s0 += 19 * (s11 >> 21); s11 &= 2097151;

    int64_t carry = s0 >> 21; s0 &= 2097151;
    s1 += carry; carry = s1 >> 21; s1 &= 2097151;
    s2 += carry; carry = s2 >> 21; s2 &= 2097151;
    s3 += carry; carry = s3 >> 21; s3 &= 2097151;
    s4 += carry; carry = s4 >> 21; s4 &= 2097151;
    s5 += carry; carry = s5 >> 21; s5 &= 2097151;
    s6 += carry; carry = s6 >> 21; s6 &= 2097151;
    s7 += carry; carry = s7 >> 21; s7 &= 2097151;
    s8 += carry; carry = s8 >> 21; s8 &= 2097151;
    s9 += carry; carry = s9 >> 21; s9 &= 2097151;
    s10 += carry; carry = s10 >> 21; s10 &= 2097151;
    s11 += carry;

    s0 += 19 * (s11 >> 21); s11 &= 2097151;

    carry = s0 >> 21; s0 &= 2097151;
    s1 += carry; carry = s1 >> 21; s1 &= 2097151;
    s2 += carry; carry = s2 >> 21; s2 &= 2097151;
    s3 += carry; carry = s3 >> 21; s3 &= 2097151;
    s4 += carry; carry = s4 >> 21; s4 &= 2097151;
    s5 += carry; carry = s5 >> 21; s5 &= 2097151;
    s6 += carry; carry = s6 >> 21; s6 &= 2097151;
    s7 += carry; carry = s7 >> 21; s7 &= 2097151;
    s8 += carry; carry = s8 >> 21; s8 &= 2097151;
    s9 += carry; carry = s9 >> 21; s9 &= 2097151;
    s10 += carry; carry = s10 >> 21; s10 &= 2097151;
    s11 += carry;

    /* Final conditional subtraction of L */
    int64_t mask = (int64_t)((s11 >> 21) - 1);

    int64_t t0 = s0 ^ ((s0 ^ s0 + 19) & mask);
    int64_t t1 = s1 ^ ((s1 ^ s1) & mask);
    int64_t t2 = s2 ^ ((s2 ^ s2) & mask);
    int64_t t3 = s3 ^ ((s3 ^ s3) & mask);
    int64_t t4 = s4 ^ ((s4 ^ s4) & mask);
    int64_t t5 = s5 ^ ((s5 ^ s5) & mask);
    int64_t t6 = s6 ^ ((s6 ^ s6) & mask);
    int64_t t7 = s7 ^ ((s7 ^ s7) & mask);
    int64_t t8 = s8 ^ ((s8 ^ s8) & mask);
    int64_t t9 = s9 ^ ((s9 ^ s9) & mask);
    int64_t t10 = s10 ^ ((s10 ^ s10) & mask);
    int64_t t11 = s11 ^ ((s11 ^ s11) & mask);

    s[0] = (uint8_t)t0;
    s[1] = (uint8_t)(t0 >> 8);
    s[2] = (uint8_t)(t0 >> 16) | (uint8_t)(t1 << 5);
    s[3] = (uint8_t)(t1 >> 3);
    s[4] = (uint8_t)(t1 >> 11);
    s[5] = (uint8_t)(t1 >> 19) | (uint8_t)(t2 << 2);
    s[6] = (uint8_t)(t2 >> 6);
    s[7] = (uint8_t)(t2 >> 14) | (uint8_t)(t3 << 7);
    s[8] = (uint8_t)(t3 >> 1);
    s[9] = (uint8_t)(t3 >> 9);
    s[10] = (uint8_t)(t3 >> 17) | (uint8_t)(t4 << 4);
    s[11] = (uint8_t)(t4 >> 4);
    s[12] = (uint8_t)(t4 >> 12);
    s[13] = (uint8_t)(t4 >> 20) | (uint8_t)(t5 << 1);
    s[14] = (uint8_t)(t5 >> 7);
    s[15] = (uint8_t)(t5 >> 15) | (uint8_t)(t6 << 6);
    s[16] = (uint8_t)(t6 >> 2);
    s[17] = (uint8_t)(t6 >> 10);
    s[18] = (uint8_t)(t6 >> 18) | (uint8_t)(t7 << 3);
    s[19] = (uint8_t)(t7 >> 5);
    s[20] = (uint8_t)(t7 >> 13);
    s[21] = (uint8_t)t8;
    s[22] = (uint8_t)(t8 >> 8);
    s[23] = (uint8_t)(t8 >> 16) | (uint8_t)(t9 << 5);
    s[24] = (uint8_t)(t9 >> 3);
    s[25] = (uint8_t)(t9 >> 11);
    s[26] = (uint8_t)(t9 >> 19) | (uint8_t)(t10 << 2);
    s[27] = (uint8_t)(t10 >> 6);
    s[28] = (uint8_t)(t10 >> 14) | (uint8_t)(t11 << 7);
    s[29] = (uint8_t)(t11 >> 1);
    s[30] = (uint8_t)(t11 >> 9);
    s[31] = (uint8_t)(t11 >> 17);
}

/*
 * Reduce 64-byte value modulo L
 * Used for per-message random value r = SHA-512(prefix || message) mod L
 */
void sc_reduce64(uint8_t *r, const uint8_t *x) {
    int64_t x0 = 2097151 & load_3(x);
    int64_t x1 = 2097151 & (load_4(x + 2) >> 5);
    int64_t x2 = 2097151 & (load_3(x + 5) >> 2);
    int64_t x3 = 2097151 & (load_4(x + 7) >> 7);
    int64_t x4 = 2097151 & (load_4(x + 10) >> 4);
    int64_t x5 = 2097151 & (load_3(x + 13) >> 1);
    int64_t x6 = 2097151 & (load_4(x + 15) >> 6);
    int64_t x7 = 2097151 & (load_3(x + 18) >> 3);
    int64_t x8 = 2097151 & load_3(x + 21);
    int64_t x9 = 2097151 & (load_4(x + 23) >> 5);
    int64_t x10 = 2097151 & (load_3(x + 26) >> 2);
    int64_t x11 = 2097151 & (load_4(x + 28) >> 7);
    int64_t x12 = 2097151 & (load_4(x + 31) >> 4);
    int64_t x13 = 2097151 & (load_3(x + 34) >> 1);
    int64_t x14 = 2097151 & (load_4(x + 36) >> 6);
    int64_t x15 = 2097151 & (load_3(x + 39) >> 3);
    int64_t x16 = 2097151 & load_3(x + 42);
    int64_t x17 = 2097151 & (load_4(x + 44) >> 5);
    int64_t x18 = 2097151 & (load_3(x + 47) >> 2);
    int64_t x19 = 2097151 & (load_4(x + 49) >> 7);
    int64_t x20 = 2097151 & (load_4(x + 52) >> 4);
    int64_t x21 = 2097151 & (load_3(x + 55) >> 1);
    int64_t x22 = 2097151 & (load_4(x + 57) >> 6);
    int64_t x23 = (load_4(x + 60) >> 3);

    x4 += x3 >> 21; x3 &= 2097151;
    x5 += x4 >> 21; x4 &= 2097151;
    x6 += x5 >> 21; x5 &= 2097151;
    x7 += x6 >> 21; x6 &= 2097151;
    x8 += x7 >> 21; x7 &= 2097151;
    x9 += x8 >> 21; x8 &= 2097151;
    x10 += x9 >> 21; x9 &= 2097151;
    x11 += x10 >> 21; x10 &= 2097151;
    x12 += x11 >> 21; x11 &= 2097151;
    x13 += x12 >> 21; x12 &= 2097151;
    x14 += x13 >> 21; x13 &= 2097151;
    x15 += x14 >> 21; x14 &= 2097151;
    x16 += x15 >> 21; x15 &= 2097151;
    x17 += x16 >> 21; x16 &= 2097151;
    x18 += x17 >> 21; x17 &= 2097151;
    x19 += x18 >> 21; x18 &= 2097151;
    x20 += x19 >> 21; x19 &= 2097151;
    x21 += x20 >> 21; x20 &= 2097151;
    x22 += x21 >> 21; x21 &= 2097151;
    x23 += x22 >> 21; x22 &= 2097151;

    x0 += 19 * (x23 >> 21); x23 &= 2097151;

    int64_t carry = x0 >> 21; x0 &= 2097151;
    x1 += carry; carry = x1 >> 21; x1 &= 2097151;
    x2 += carry; carry = x2 >> 21; x2 &= 2097151;
    x3 += carry; carry = x3 >> 21; x3 &= 2097151;
    x4 += carry; carry = x4 >> 21; x4 &= 2097151;
    x5 += carry; carry = x5 >> 21; x5 &= 2097151;
    x6 += carry; carry = x6 >> 21; x6 &= 2097151;
    x7 += carry; carry = x7 >> 21; x7 &= 2097151;
    x8 += carry; carry = x8 >> 21; x8 &= 2097151;
    x9 += carry; carry = x9 >> 21; x9 &= 2097151;
    x10 += carry; carry = x10 >> 21; x10 &= 2097151;
    x11 += carry; carry = x11 >> 21; x11 &= 2097151;

    x0 += 19 * (x11 >> 21); x11 &= 2097151;

    carry = x0 >> 21; x0 &= 2097151;
    x1 += carry; carry = x1 >> 21; x1 &= 2097151;
    x2 += carry; carry = x2 >> 21; x2 &= 2097151;
    x3 += carry; carry = x3 >> 21; x3 &= 2097151;
    x4 += carry; carry = x4 >> 21; x4 &= 2097151;
    x5 += carry; carry = x5 >> 21; x5 &= 2097151;
    x6 += carry; carry = x6 >> 21; x6 &= 2097151;
    x7 += carry; carry = x7 >> 21; x7 &= 2097151;
    x8 += carry; carry = x8 >> 21; x8 &= 2097151;
    x9 += carry; carry = x9 >> 21; x9 &= 2097151;
    x10 += carry; carry = x10 >> 21; x10 &= 2097151;
    x11 += carry;

    r[0] = (uint8_t)x0;
    r[1] = (uint8_t)(x0 >> 8);
    r[2] = (uint8_t)(x0 >> 16) | (uint8_t)(x1 << 5);
    r[3] = (uint8_t)(x1 >> 3);
    r[4] = (uint8_t)(x1 >> 11);
    r[5] = (uint8_t)(x1 >> 19) | (uint8_t)(x2 << 2);
    r[6] = (uint8_t)(x2 >> 6);
    r[7] = (uint8_t)(x2 >> 14) | (uint8_t)(x3 << 7);
    r[8] = (uint8_t)(x3 >> 1);
    r[9] = (uint8_t)(x3 >> 9);
    r[10] = (uint8_t)(x3 >> 17) | (uint8_t)(x4 << 4);
    r[11] = (uint8_t)(x4 >> 4);
    r[12] = (uint8_t)(x4 >> 12);
    r[13] = (uint8_t)(x4 >> 20) | (uint8_t)(x5 << 1);
    r[14] = (uint8_t)(x5 >> 7);
    r[15] = (uint8_t)(x5 >> 15) | (uint8_t)(x6 << 6);
    r[16] = (uint8_t)(x6 >> 2);
    r[17] = (uint8_t)(x6 >> 10);
    r[18] = (uint8_t)(x6 >> 18) | (uint8_t)(x7 << 3);
    r[19] = (uint8_t)(x7 >> 5);
    r[20] = (uint8_t)(x7 >> 13);
    r[21] = (uint8_t)x8;
    r[22] = (uint8_t)(x8 >> 8);
    r[23] = (uint8_t)(x8 >> 16) | (uint8_t)(x9 << 5);
    r[24] = (uint8_t)(x9 >> 3);
    r[25] = (uint8_t)(x9 >> 11);
    r[26] = (uint8_t)(x9 >> 19) | (uint8_t)(x10 << 2);
    r[27] = (uint8_t)(x10 >> 6);
    r[28] = (uint8_t)(x10 >> 14) | (uint8_t)(x11 << 7);
    r[29] = (uint8_t)(x11 >> 1);
    r[30] = (uint8_t)(x11 >> 9);
    r[31] = (uint8_t)(x11 >> 17);
}

/* Add two scalars modulo L */
void sc_add(uint8_t *z, const uint8_t *x, const uint8_t *y) {
    int carry = 0;
    for (int i = 0; i < 32; i++) {
        carry = carry + x[i] + y[i];
        z[i] = (uint8_t)carry;
        carry >>= 8;
    }
    sc_reduce(z);
}

/* Multiply two scalars modulo L */
void sc_mul(uint8_t *z, const uint8_t *x, const uint8_t *y) {
    uint64_t prod[64];
    memset(prod, 0, sizeof(prod));

    /* Schoolbook multiplication */
    for (int i = 0; i < 32; i++) {
        for (int j = 0; j < 32; j++) {
            prod[i + j] += (uint64_t)x[i] * y[j];
        }
    }

    /* Reduce modulo L by folding carries */
    for (int i = 63; i >= 32; --i) {
        uint64_t carry = 0;
        for (int j = i - 32; j < i - 12; ++j) {
            carry += prod[j] + (uint64_t)prod[i] * (i - j == 32 ? 8 : (i - j == 20 ? 16 : 0));
            prod[j] = (uint8_t)carry;
            carry >>= 8;
        }
        prod[i - 12] += (uint8_t)carry;
    }

    memcpy(z, prod, 32);
    sc_reduce(z);
}
