#ifndef WEAVE_L5_INTERNAL_H
#define WEAVE_L5_INTERNAL_H

#include "vortex/public/cemqc.h"
#include "vortex/public/hypercycle_algorithms.h"
#include <stdint.h>
#include <string.h>

/* ==========================================================================
 * INTERNAL OPTIMIZED TYPES
 * ========================================================================*/

/* Packed 128-bit quaternion structure */
typedef struct {
  uint32_t w, x, y, z;
} opt_quat_t;

/* ==========================================================================
 * INLINE PRIMITIVES
 * ========================================================================*/

/* Fast Quaternion Multiplication */
static inline void opt_quat_mul(const opt_quat_t *a, const opt_quat_t *b,
                                opt_quat_t *out) {
  uint32_t aw = a->w, ax = a->x, ay = a->y, az = a->z;
  uint32_t bw = b->w, bx = b->x, by = b->y, bz = b->z;

  out->w = aw * bw - ax * bx - ay * by - az * bz;
  out->x = aw * bx + ax * bw + ay * bz - az * by;
  out->y = aw * by - ax * bz + ay * bw + az * bx;
  out->z = aw * bz + ax * by - ay * bx + az * bw;
}

/* Fast Quaternion Addition */
static inline void opt_quat_add(const opt_quat_t *a, const opt_quat_t *b,
                                opt_quat_t *out) {
  out->w = a->w + b->w;
  out->x = a->x + b->x;
  out->y = a->y + b->y;
  out->z = a->z + b->z;
}

/* Fast Quaternion Subtraction */
static inline void opt_quat_sub(const opt_quat_t *a, const opt_quat_t *b,
                                opt_quat_t *out) {
  out->w = a->w - b->w;
  out->x = a->x - b->x;
  out->y = a->y - b->y;
  out->z = a->z - b->z;
}

/* Modular Inverse (Newton-Raphson for 2^32) */
static inline uint32_t mod_inverse_32(uint32_t a) {
  uint32_t x = 3 * a ^ 2;
  x *= 2 - a * x;
  x *= 2 - a * x;
  x *= 2 - a * x;
  x *= 2 - a * x;
  return x;
}

/* Fast block-based expansion */
static inline void fast_expand(const uint8_t *seed, size_t seed_len,
                               opt_quat_t *out, size_t count) {
  hc_rng_state_t rng;
  hc_rng_init(&rng, seed, seed_len);
  hc_rng_generate(&rng, (unsigned char *)out, count * sizeof(opt_quat_t));

  /* Ensure odd w component for invertibility (odd norm) */
  for (size_t i = 0; i < count; i++) {
    out[i].w |= 1;
    /* Check parity of x^2 + y^2 + z^2 */
    int parity = (out[i].x & 1) ^ (out[i].y & 1) ^ (out[i].z & 1);
    if (parity) {
      out[i].x ^= 1; /* Fix parity so x^2+y^2+z^2 is Even */
    }
    /* Result: w^2 is Odd, Sum is Odd + Even = Odd Norm. Invertible. */
  }
}

/* Fast hash function (Secure Folding) */
static inline void fast_hash(const uint8_t *in, size_t in_len, uint8_t *out,
                             size_t out_len) {
  /* Fold input into a 64-bit seed to ensure dependence on all bytes */
  uint64_t seed64 = 0xCBF29CE484222325ULL;
  for (size_t i = 0; i < in_len; i++) {
    seed64 ^= in[i];
    seed64 = (seed64 << 5) | (seed64 >> 59);
    seed64 *= 0x100000001B3ULL;
  }

  hc_rng_state_t rng;
  hc_rng_init(&rng, (uint8_t *)&seed64, 8);

  /* Additional Mixing */
  uint64_t h = rng.state;
  for (int i = 0; i < 16; i++) {
    h ^= (h << 13);
    h ^= (h >> 7);
    h ^= (h << 17);
    h += 0x9E3779B97F4A7C15ULL;
  }
  rng.state = h;

  hc_rng_generate(&rng, out, out_len);
}

#endif /* WEAVE_L5_INTERNAL_H */
