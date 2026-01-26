// hc_octonion_simd.h – AVX-512 Accelerated Octonion Operations
// Keypair Generation Optimization (2.5x speedup target)

#ifndef hc_OCTONION_SIMD_H
#define hc_OCTONION_SIMD_H

#include "../public/hc_octonion.h"
#include <stdint.h>

#ifdef __AVX512F__
#include <immintrin.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * SIMD-accelerated octonion multiplication.
 *
 * Packs all 8 components (s, e1...e7) into AVX-512 registers and performs
 * vectorized dot product, cross product, and scalar multiplication.
 *
 * @param a First octonion
 * @param b Second octonion
 * @param out Result (a * b)
 *
 * Performance: ~70 cycles (vs ~200 scalar)
 */
void hc_oga_mul_simd(const hc_octonion_t *a, const hc_octonion_t *b,
                     hc_octonion_t *out);

/**
 * SIMD-accelerated twist basis computation.
 *
 * Computes P[i] = S * G[i] * S^-1 for all 7 basis vectors using
 * vectorized operations and batch processing.
 *
 * @param S Secret rotor
 * @param P Output twisted basis (array of 7 octonions)
 *
 * Performance: ~900 cycles (vs ~2700 scalar)
 */
void hc_twist_basis_simd(const hc_octonion_t *S, hc_octonion_t *P);

#ifdef __cplusplus
}
#endif

#endif // __AVX512F__

#endif // hc_OCTONION_SIMD_H
