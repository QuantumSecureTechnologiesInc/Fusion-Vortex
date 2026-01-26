/**
 * @file vortex_pqc_api.h
 * @brief Complete PQC seed generation API for Vortex v2.0
 *
 * Implements all production-ready seed generation functions with
 * proper NIST health tests, self-healing, and batch operations.
 */

#ifndef VORTEX_PQC_API_H
#define VORTEX_PQC_API_H

#include "hc_vacuum_engine.h"
#include <immintrin.h>
#include <stddef.h>
#include <stdint.h>

/* NIST Health Test Thresholds (Corrected) */
#define HC_RCT_CUTOFF 30  /* Repetition Count Test (was 5) */
#define HC_APT_CUTOFF 13  /* Adaptive Proportion Test (512-window) */
#define HC_APT_WINDOW 512 /* APT sliding window size */

/* API Result Codes */
typedef enum {
  HC_PQC_SUCCESS = 0,
  HC_PQC_ERROR = -1,
  HC_PQC_RCT_FAILURE = -101,
  HC_PQC_APT_FAILURE = -102,
  HC_PQC_SELF_HEAL_FAILURE = -103
} hc_pqc_result_t;

/**
 * ========================================================================
 * API Function 14: hc_get_pqc_seed_32()
 * ========================================================================
 * Generate 32-byte seed for ML-KEM/ML-DSA via XOR folding
 *
 * Generates 64 bytes internally, then XOR-folds to 32 bytes for
 * compatibility with NIST PQC algorithms.
 *
 * @param ctx Vortex context handle
 * @param seed_32 Output buffer (32 bytes)
 * @return HC_PQC_SUCCESS or error code
 */
hc_pqc_result_t hc_get_pqc_seed_32(hc_vac_context_t ctx, uint8_t seed_32[32]);

/**
 * ========================================================================
 * API Function 15: hc_generate_pqc_seed()
 * ========================================================================
 * General PQC seed generation (variable length)
 *
 * @param ctx Vortex context handle
 * @param seed Output buffer
 * @param len Desired seed length (bytes)
 * @return HC_PQC_SUCCESS or error code
 */
hc_pqc_result_t hc_generate_pqc_seed(hc_vac_context_t ctx, uint8_t *seed,
                                     size_t len);

/**
 * ========================================================================
 * API Function 16: hc_generate_pqc_seed_safe()
 * ========================================================================
 * Safe PQC seed generation with self-healing loop-retry
 *
 * Automatically retries with progressive self-healing tiers if
 * NIST health tests fail.
 *
 * @param ctx Vortex context handle
 * @param seed Output buffer
 * @param len Desired seed length (bytes)
 * @param max_retries Maximum retry attempts (default: 3)
 * @return HC_PQC_SUCCESS or error code
 */
hc_pqc_result_t hc_generate_pqc_seed_safe(hc_vac_context_t ctx, uint8_t *seed,
                                          size_t len, int max_retries);

/**
 * ========================================================================
 * API Function 17: hc_generate_pqc_seed_2026()
 * ========================================================================
 * 2026 Production version with Symplectic Storm Integrator
 *
 * Latest production-ready implementation with:
 * - Full Kick-Drift-Kick symplectic integrator
 * - Lyapunov horizon monitoring
 * - Predictive phase shifting
 * - Enhanced health tests
 *
 * @param ctx Vortex context handle
 * @param seed Output buffer
 * @param len Desired seed length (bytes)
 * @return HC_PQC_SUCCESS or error code
 */
hc_pqc_result_t hc_generate_pqc_seed_2026(hc_vac_context_t ctx, uint8_t *seed,
                                          size_t len);

/**
 * ========================================================================
 * API Function 18: hc_generate_batch()
 * ========================================================================
 * Batch entropy generation from pre-filled reservoir
 *
 * Zero-latency retrieval from background-generated entropy.
 *
 * @param ctx Vortex context handle
 * @param batch Output buffer
 * @param batch_size Number of bytes to retrieve
 * @return Number of bytes actually retrieved
 */
size_t hc_generate_batch(hc_vac_context_t ctx, uint8_t *batch,
                         size_t batch_size);

/**
 * ========================================================================
 * API Function 19: condition_entropy()
 * ========================================================================
 * SHA-3 conditioning wrapper with secure memory wipe
 *
 * @param raw Raw entropy input
 * @param raw_len Input length
 * @param conditioned Output buffer (32 bytes for SHA3-256)
 */
void condition_entropy(const uint8_t *raw, size_t raw_len,
                       uint8_t conditioned[32]);

/**
 * ========================================================================
 * API Function 20: hc_vector_evolve()
 * ========================================================================
 * Vectorized Hamiltonian evolution (8-way AVX-512)
 *
 * @param state_q Position vector (8 lanes)
 * @param state_p Momentum vector (8 lanes)
 * @param cycles Number of evolution cycles
 */
void hc_vector_evolve(__m512i *state_q, __m512i *state_p, int cycles);

/**
 * ========================================================================
 * API Function 21: hc_condition_and_output()
 * ========================================================================
 * Combined conditioning and output in single call
 *
 * @param ctx Vortex context handle
 * @param output Conditioned output buffer (32 bytes)
 * @return HC_PQC_SUCCESS or error code
 */
hc_pqc_result_t hc_condition_and_output(hc_vac_context_t ctx,
                                        uint8_t output[32]);

/**
 * ========================================================================
 * Advanced Health Test 22: Enhanced APT
 * ========================================================================
 * 512-sample sliding window APT with proper NIST threshold
 */
typedef struct {
  uint64_t window[HC_APT_WINDOW];
  int window_idx;
  uint32_t counts[256];
} hc_enhanced_apt_t;

void hc_enhanced_apt_init(hc_enhanced_apt_t *apt);
int hc_enhanced_apt_test(hc_enhanced_apt_t *apt, uint8_t sample);

/**
 * ========================================================================
 * Advanced Health Test 23: Live Health Test
 * ========================================================================
 * Real-time NIST health monitoring during generation
 */
int nist_live_health_test(hc_vac_context_t ctx, const uint8_t *samples,
                          size_t count);

/**
 * ========================================================================
 * Advanced Health Test 24: Full RCT
 * ========================================================================
 * Proper RCT with corrected cutoff (30 instead of 5)
 */
typedef struct {
  uint8_t last_sample;
  int repetition_count;
  int cutoff;
} hc_full_rct_t;

void hc_full_rct_init(hc_full_rct_t *rct);
int hc_full_rct_test(hc_full_rct_t *rct, uint8_t sample);

#endif /* VORTEX_PQC_API_H */
