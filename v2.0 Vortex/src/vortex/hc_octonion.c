#include "vortex/public/hc_octonion.h"
#include <immintrin.h>
#include <stdint.h>
#include <string.h>

// ============================================================================
// Fixed-Point Math Helpers
// ============================================================================

#if defined(_MSC_VER)
#include <intrin.h>
#endif

// Multiply two Q32.32 numbers
// Result = (A * B) >> 32
static inline int64_t fp_mul(int64_t a, int64_t b) {
#if defined(_MSC_VER) && defined(_M_X64)
  int64_t high;
  int64_t low = _mul128(a, b, &high);
  return (high << 32) | ((uint64_t)low >> 32);
#elif defined(__SIZEOF_INT128__)
  __int128 r = (__int128)a * (__int128)b;
  return (int64_t)(r >> 32);
#else
  // 32-bit fallback or no 128-bit int
  int64_t a_hi = a >> 32;
  int64_t a_lo = a & 0xFFFFFFFFLL;
  int64_t b_hi = b >> 32;
  int64_t b_lo = b & 0xFFFFFFFFLL;

  int64_t mid = a_hi * b_lo + a_lo * b_hi;
  int64_t lo = (a_lo * b_lo) >> 32;
  int64_t hi = a_hi * b_hi;
  return (hi << 32) + mid + lo;
#endif
}

// ============================================================================
// Fano Plane Logic (7D Cross Product)
// ============================================================================

static const int FANO_TRIPLES[7][3] = {{0, 1, 3}, {1, 2, 4}, {2, 3, 5},
                                       {3, 4, 6}, {4, 5, 0}, {5, 6, 1},
                                       {6, 0, 2}};

void hc_cross_product_7d(const int64_t *a, const int64_t *b, int64_t *out) {
  memset(out, 0, sizeof(int64_t) * 7);

  for (int t = 0; t < 7; t++) {
    int i = FANO_TRIPLES[t][0];
    int j = FANO_TRIPLES[t][1];
    int k = FANO_TRIPLES[t][2];

    // k = i * j
    out[k] += fp_mul(a[i], b[j]) - fp_mul(a[j], b[i]);
    // i = j * k
    out[i] += fp_mul(a[j], b[k]) - fp_mul(a[k], b[j]);
    // j = k * i
    out[j] += fp_mul(a[k], b[i]) - fp_mul(a[i], b[k]);
  }
}

int64_t hc_dot_product_7d(const int64_t *a, const int64_t *b) {
  int64_t sum = 0;
  for (int i = 0; i < 7; i++)
    sum += fp_mul(a[i], b[i]);
  return sum;
}

// ============================================================================
// Core Algebra Operations
// ============================================================================

void hc_oga_mul(const hc_octonion_t *a, const hc_octonion_t *b,
                hc_octonion_t *out) {
#if defined(__AVX512F__) && defined(__AVX512DQ__)
  // Placeholder for full AVX-512 optimization in future pass
  // For now, use correct scalar implementation for stability
  out->s = fp_mul(a->s, b->s) - hc_dot_product_7d(a->v, b->v);

  int64_t cross[7];
  hc_cross_product_7d(a->v, b->v, cross);

  for (int i = 0; i < 7; i++) {
    out->v[i] = fp_mul(a->s, b->v[i]) + fp_mul(a->v[i], b->s) + cross[i];
  }
#else
  out->s = fp_mul(a->s, b->s) - hc_dot_product_7d(a->v, b->v);

  int64_t cross[7];
  hc_cross_product_7d(a->v, b->v, cross);

  for (int i = 0; i < 7; i++) {
    out->v[i] = fp_mul(a->s, b->v[i]) + fp_mul(a->v[i], b->s) + cross[i];
  }
#endif
}

void hc_oga_conjugate(const hc_octonion_t *a, hc_octonion_t *out) {
  out->s = a->s;
  for (int i = 0; i < 7; i++)
    out->v[i] = -(a->v[i]);
}

int64_t hc_oga_norm_sq(const hc_octonion_t *a) {
  int64_t sum = fp_mul(a->s, a->s);
  sum += hc_dot_product_7d(a->v, a->v);
  return sum;
}

void hc_oga_inverse(const hc_octonion_t *a, hc_octonion_t *out) {
  int64_t n2 = hc_oga_norm_sq(a);
  if (n2 == 0) {
    memset(out, 0, sizeof(hc_octonion_t));
    return;
  }

  double n2_d = hc_FIXED_TO_DOUBLE(n2);
  double inv_d = 1.0 / n2_d;
  int64_t inv_n2 = hc_DOUBLE_TO_FIXED(inv_d);

  hc_octonion_t conj;
  hc_oga_conjugate(a, &conj);

  out->s = fp_mul(conj.s, inv_n2);
  for (int i = 0; i < 7; i++)
    out->v[i] = fp_mul(conj.v[i], inv_n2);
}

void hc_oga_associator(const hc_octonion_t *a, const hc_octonion_t *b,
                       const hc_octonion_t *c, hc_octonion_t *out) {
  hc_octonion_t ab, bc, ab_c, a_bc;
  hc_oga_mul(a, b, &ab);
  hc_oga_mul(&ab, c, &ab_c);
  hc_oga_mul(b, c, &bc);
  hc_oga_mul(a, &bc, &a_bc);

  out->s = ab_c.s - a_bc.s;
  for (int i = 0; i < 7; i++)
    out->v[i] = ab_c.v[i] - a_bc.v[i];
}
