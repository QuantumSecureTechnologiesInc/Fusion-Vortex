/**
 * HyperCycle v3.2 Fulminis - AVX-512 Optimized Kernel
 *
 * Implements:
 * 1. Structure-of-Arrays (SoA) Layout for vectorization efficiency
 * 2. AVX-512 Intrinsics for parallel Q32.32 arithmetic
 * 3. Lazy Normalization (update non-linearity every N cycles)
 */

#ifndef hc_VACUUM_AVX512_H
#define hc_VACUUM_AVX512_H

#include "hc_vacuum_fixed.h"
#include <immintrin.h>
#include <stdint.h>

// Alignment macros (applied to first member)
#if defined(_MSC_VER)
#define ALIGN_MEMBER __declspec(align(64))
#else
#define ALIGN_MEMBER _Alignas(64)
#endif

// Use 256 for optimal cache line / ZMM alignment (holds 32 x 8 int64s)
// If hc_VACUUM_DIM is different, we iterate appropriately
#define hc_AVX_DIM 256

/**
 * Structure-of-Arrays (SoA) State
 * Optimized for AVX-512 vertical processing
 */
typedef struct {
  ALIGN_MEMBER int64_t w[hc_AVX_DIM]; // ZMM0, ZMM4...
  int64_t x[hc_AVX_DIM];              // ZMM1, ZMM5...
  int64_t y[hc_AVX_DIM];
  int64_t z[hc_AVX_DIM];
} hc_vacuum_state_soa_t;

// Check if AVX-512 is supported by compiler
#if defined(__AVX512F__) && defined(__AVX512DQ__)

/**
 * Optimized Evolution Cycle (AVX-512)
 *
 * @param state SoA state pointer
 * @param cycles Number of cycles to evolve
 */
void hc_vacuum_evolve_avx512(hc_vacuum_state_soa_t *state, int cycles);

/**
 * Converter: AoS -> SoA
 */
// Forward declare struct to avoid include order issues
struct hc_vacuum_state_t;

void hc_vacuum_aos_to_soa(const struct hc_vacuum_state_t *aos,
                          hc_vacuum_state_soa_t *soa);

/**
 * Converter: SoA -> AoS
 */
/**
 * Converter: SoA -> AoS
 */
void hc_vacuum_soa_to_aos(const hc_vacuum_state_soa_t *soa,
                          struct hc_vacuum_state_t *aos);

// ----------------------------------------------------------------------------
// Optimization #4: Batch Key Generation (Throughput)
// ----------------------------------------------------------------------------

/**
 * Generate 8 keys in parallel using AVX-512 vertical batching.
 *
 * @param states_in Array of 8 initialized AoS states (seeds)
 * @param keys_out  Buffer for 8 generated keys (flat, 8 * out_len)
 * @param out_len   Length of each key
 * @return 0 on success
 */
int hc_generate_vacuum_key_x8(struct hc_vacuum_state_t *states_in[8],
                              uint8_t *keys_out, size_t out_len);

/**
 * Converter: Batch Pack (8x AoS -> 1x Interleaved SoA)
 * Maps Lane K -> State K
 */
void hc_vacuum_aos_to_soa_x8(struct hc_vacuum_state_t *aos[8],
                             hc_vacuum_state_soa_t *soa);

/**
 * Converter: Batch Unpack (1x Interleaved SoA -> 8x AoS)
 */
void hc_vacuum_soa_to_aos_x8(const hc_vacuum_state_soa_t *soa,
                             struct hc_vacuum_state_t *aos[8]);

#endif // __AVX512F__

#endif // hc_VACUUM_AVX512_H
