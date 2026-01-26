// hc_vacuum_jitter.c – Stochastic Torsion Injection Implementation
// PATCH v3.3.2: Anisotropic Jitter Injection
// Target: Defeating Octonion Wirtinger Flow / Phase Retrieval Attacks

#include "vortex/internal/hc_vacuum_jitter.h"
#include "vortex/internal/hc_fast_validation.h"
#include "vortex/public/hc_octonion.h"
#include <math.h>
#include <stdint.h>
#include <string.h>

#ifdef __AVX512F__
#include <immintrin.h>
#endif

// Expansion constant: Jitter magnitude as fraction of total norm
// Set to < 0.0001% (0x7FF in Q32.32 is ~0.00019%)
// This is < 0.0001% of hc_Q32_32_SCALE (2^32)
#define hc_EXPANSION_CONSTANT 0x7FF

/**
 * Generate 512-bit stochastic torsion mask from entropy seed.
 *
 * This uses a simple deterministic expansion of the entropy seed
 * to generate independent jitter values for each imaginary component.
 * The mask is designed to be non-invertible and break phase gradient symmetry.
 */
static void generate_jitter_mask(const uint8_t entropy_seed[64],
                                 int64_t jitter[8]) {
  // Interpret entropy seed as int64 array (8 * 8 bytes = 64 bytes)
  const int64_t *seed = (const int64_t *)entropy_seed;

  // Generate jitter for scalar and 7 imaginary components
  // We use XOR and bit rotation to mix entropy across components
  for (int i = 0; i < 8; i++) {
    // Mix current seed value with neighbor (circular)
    int64_t mixed = seed[i] ^ seed[(i + 1) % 8];

    // Rotate bits to spread entropy
    int64_t rotated = (mixed << 7) | ((uint64_t)mixed >> 57);

    // Scale down to jitter magnitude (< 0.0001% of norm)
    // Mask to hc_EXPANSION_CONSTANT range, then apply sign from high bit
    int64_t magnitude = rotated & hc_EXPANSION_CONSTANT;
    int64_t sign = (rotated & 0x8000000000000000LL) ? -1 : 1;

    jitter[i] = magnitude * sign;
  }
}

void hc_apply_jitter_mask(hc_octonion_t *rotor,
                          const uint8_t entropy_seed[64]) {
  // 0. Fast SIMD Component Validation (2026 Security Mitigation)
  // Validate components before applying jitter for fused validation pipeline
#ifdef __AVX512F__
  __m512i components = _mm512_loadu_si512((__m512i *)rotor);
  __m512i abs_components = _mm512_abs_epi64(components);
  __m512i threshold = _mm512_set1_epi64(hc_MIN_COMPONENT_THRESHOLD);
  __mmask8 valid_mask = _mm512_cmpge_epi64_mask(abs_components, threshold);

  // If validation fails, mark rotor as degenerate (will be rejected by caller)
  if (valid_mask != 0xFF) {
    // Set scalar to zero as rejection signal
    rotor->s = 0;
    return;
  }
#endif

  // 1. Generate Stochastic Torsion Mask
  int64_t jitter[8]; // [s, e1, e2, e3, e4, e5, e6, e7]
  generate_jitter_mask(entropy_seed, jitter);

  // 2. Apply Jitter to Imaginary Components (e1...e7)
  // We perturb each dimension independently to break gradient symmetry.
  // The scalar part is left untouched to maintain rotor structure.
  //
  // Note: We apply jitter BEFORE renormalization to ensure the final
  // rotor remains unit norm after the entire operation completes.

  for (int i = 0; i < 7; i++) {
    // Shift < 0.0001% of the norm; sufficient to break math convergence
    // but negligible for decapsulation stability.
    rotor->v[i] += jitter[i + 1];
  }

  // 3. Renormalize to Unit Norm
  // After jitter injection, we must renormalize to maintain rotor properties
  int64_t n2 = hc_oga_norm_sq(rotor);

  // Check for zero (should never happen with proper entropy)
  if (n2 == 0) {
    // Fallback: identity rotor
    rotor->s = hc_Q32_32_SCALE;
    memset(rotor->v, 0, sizeof(rotor->v));
    return;
  }

  // Normalize using double precision sqrt (acceptable for setup phase)
  double n = sqrt(hc_FIXED_TO_DOUBLE(n2));
  double inv_d = 1.0 / n;
  int64_t inv = hc_DOUBLE_TO_FIXED(inv_d);

  // Apply inverse norm to all components
  rotor->s = (rotor->s * inv) >> 32;
  for (int i = 0; i < 7; i++) {
    rotor->v[i] = (rotor->v[i] * inv) >> 32;
  }
}
