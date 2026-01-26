/**
 * @file hc_chaotic_engine.h
 * @brief Universal Fixed-point chaotic engine for PQC.
 */

#ifndef HC_CHAOTIC_ENGINE_H
#define HC_CHAOTIC_ENGINE_H

#include <stddef.h>
#include <stdint.h>

/**
 * @brief Chaotic engine state container.
 */
typedef struct {
  uint32_t state;
  uint32_t iteration;
  uint32_t control;
} hc_chaos_ctx_t;

/**
 * @brief UNIVERSAL ADAPTER: Generic randomness callback.
 * Fits standard PQC library signatures (f_rng, random_bytes callbacks).
 *
 * @param context Pointer to an hc_chaos_ctx_t.
 * @param out Output buffer.
 * @param out_len Length to generate.
 * @return 0 on success.
 */
int hc_chaos_univ_random(void *context, uint8_t *out, size_t out_len);

int hc_chaos_init(hc_chaos_ctx_t *ctx, uint32_t seed);
uint32_t hc_chaos_next(hc_chaos_ctx_t *ctx);

#endif
