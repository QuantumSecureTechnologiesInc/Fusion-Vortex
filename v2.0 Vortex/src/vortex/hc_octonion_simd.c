// hc_octonion_simd.c – AVX-512 Accelerated Octonion Operations
// Keypair Generation Optimization (2.5x speedup target)

#include "vortex/internal/hc_octonion_simd.h"

#ifdef __AVX512F__
#include <immintrin.h>
#include <stdint.h>
#include <string.h>

// ============================================================================
// SIMD Fixed-Point Multiplication
// ============================================================================

// Multiply two Q32.32 vectors (8 parallel multiplications)
static inline __m512i simd_fp_mul(__m512i a, __m512i b) {
  // Split into high/low 32-bit parts for 64x64→128 multiplication
  __m512i a_lo = _mm512_and_si512(a, _mm512_set1_epi64(0xFFFFFFFF));
  __m512i a_hi = _mm512_srli_epi64(a, 32);
  __m512i b_lo = _mm512_and_si512(b, _mm512_set1_epi64(0xFFFFFFFF));
  __m512i b_hi = _mm512_srli_epi64(b, 32);

  // Compute partial products
  __m512i p_ll = _mm512_mul_epu32(a_lo, b_lo); // Low × Low
  __m512i p_lh = _mm512_mul_epu32(a_lo, b_hi); // Low × High
  __m512i p_hl = _mm512_mul_epu32(a_hi, b_lo); // High × Low
  __m512i p_hh = _mm512_mul_epu32(a_hi, b_hi); // High × High

  // Combine: result = (p_hh << 64) + (p_lh << 32) + (p_hl << 32) + (p_ll)
  // After >> 32: result = (p_hh << 32) + p_lh + p_hl + (p_ll >> 32)
  __m512i mid = _mm512_add_epi64(p_lh, p_hl);
  __m512i mid_lo = _mm512_slli_epi64(mid, 32);
  __m512i result = _mm512_add_epi64(p_hh, _mm512_srli_epi64(mid, 32));
  result = _mm512_add_epi64(result, _mm512_srli_epi64(p_ll, 32));
  result = _mm512_add_epi64(result, _mm512_srli_epi64(mid_lo, 32));

  return result;
}

// ============================================================================
// SIMD Octonion Multiplication
// ============================================================================

void hc_oga_mul_simd(const hc_octonion_t *a, const hc_octonion_t *b,
                     hc_octonion_t *out) {
  // Load all 8 components into AVX-512 registers
  __m512i a_vec = _mm512_loadu_si512((__m512i *)a);
  __m512i b_vec = _mm512_loadu_si512((__m512i *)b);

  // Extract scalar parts (first element)
  int64_t a_s = a->s;
  int64_t b_s = b->s;

  // Compute dot product of vector parts: v_a · v_b
  // Load only vector parts (skip scalar at index 0)
  __m512i a_v = _mm512_loadu_si512((__m512i *)a->v);
  __m512i b_v = _mm512_loadu_si512((__m512i *)b->v);

  __m512i dot_prod = simd_fp_mul(a_v, b_v);
  int64_t dot = _mm512_reduce_add_epi64(dot_prod);

  // Scalar part: s_out = a_s * b_s - dot(a_v, b_v)
#if defined(_MSC_VER) && defined(_M_X64)
  int64_t high;
  int64_t low = _mul128(a_s, b_s, &high);
  int64_t s_prod_res = (high << 32) | ((uint64_t)low >> 32);
  out->s = s_prod_res - dot;
#elif defined(__SIZEOF_INT128__)
  __int128 s_prod = (__int128)a_s * (__int128)b_s;
  out->s = (int64_t)(s_prod >> 32) - dot;
#else
  // Fallback
  int64_t a_hi = a_s >> 32;
  int64_t a_lo = a_s & 0xFFFFFFFFLL;
  int64_t b_hi = b_s >> 32;
  int64_t b_lo = b_s & 0xFFFFFFFFLL;
  int64_t mid = a_hi * b_lo + a_lo * b_hi;
  int64_t lo = (a_lo * b_lo) >> 32;
  int64_t hi = a_hi * b_hi;
  out->s = ((hi << 32) + mid + lo) - dot;
#endif

  // Vector part: v_out = a_s * b_v + b_s * a_v + cross(a_v, b_v)
  // Note: Cross product still uses scalar Fano plane logic for correctness
  // (SIMD cross product is complex due to irregular Fano plane structure)

  int64_t cross[7];
  hc_cross_product_7d(a->v, b->v, cross);

  // Vectorize the scalar multiplications: a_s * b_v and b_s * a_v
  __m512i a_s_vec = _mm512_set1_epi64(a_s);
  __m512i b_s_vec = _mm512_set1_epi64(b_s);

  __m512i a_s_b_v = simd_fp_mul(a_s_vec, b_v);
  __m512i b_s_a_v = simd_fp_mul(b_s_vec, a_v);

  // Load cross product into SIMD
  __m512i cross_vec = _mm512_loadu_si512((__m512i *)cross);

  // Final vector: a_s * b_v + b_s * a_v + cross
  __m512i v_out = _mm512_add_epi64(a_s_b_v, b_s_a_v);
  v_out = _mm512_add_epi64(v_out, cross_vec);

  // Store result
  _mm512_storeu_si512((__m512i *)out->v, v_out);
}

// ============================================================================
// SIMD Twist Basis Computation
// ============================================================================

void hc_twist_basis_simd(const hc_octonion_t *S, hc_octonion_t *P) {
  // Compute S^-1 once
  hc_octonion_t invS;
  hc_oga_inverse(S, &invS);

  // Standard basis vectors (from hc_oga_kem.c)
  static const hc_octonion_t BASIS_G[7] = {
      {0, {hc_Q32_32_SCALE, 0, 0, 0, 0, 0, 0}},
      {0, {0, hc_Q32_32_SCALE, 0, 0, 0, 0, 0}},
      {0, {0, 0, hc_Q32_32_SCALE, 0, 0, 0, 0}},
      {0, {0, 0, 0, hc_Q32_32_SCALE, 0, 0, 0}},
      {0, {0, 0, 0, 0, hc_Q32_32_SCALE, 0, 0}},
      {0, {0, 0, 0, 0, 0, hc_Q32_32_SCALE, 0}},
      {0, {0, 0, 0, 0, 0, 0, hc_Q32_32_SCALE}}};

  // Compute P[i] = S * G[i] * S^-1 for each basis vector
  // Using SIMD multiplication for each step
  for (int i = 0; i < 7; i++) {
    hc_octonion_t tmp;
    hc_oga_mul_simd(S, &BASIS_G[i], &tmp); // S * ei (~70 cycles)
    hc_oga_mul_simd(&tmp, &invS, &P[i]);   // (S * ei) * S^-1 (~70 cycles)
  }

  // Total: 7 × (70 + 70) = ~980 cycles (vs ~2800 scalar)
}

#endif // __AVX512F__
