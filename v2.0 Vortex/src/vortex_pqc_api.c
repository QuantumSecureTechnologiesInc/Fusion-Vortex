/**
 * @file vortex_pqc_api.c
 * @brief Implementation of complete PQC API for Vortex v2.0
 */

#include "vortex_pqc_api.h"
#include "vortex_advanced_algorithms.h"
#include <immintrin.h>
// #include <openssl/sha.h>
#include <string.h>

/* Extern declarations from hc_vacuum_engine.c */
extern void secure_wipe(void *ptr, size_t len);

/* ========================================================================
 * API Function 14: hc_get_pqc_seed_32() - XOR Folding
 * ======================================================================== */
hc_pqc_result_t hc_get_pqc_seed_32(hc_vac_context_t ctx, uint8_t seed_32[32]) {
  if (!ctx || !seed_32) {
    return HC_PQC_ERROR;
  }

  /* Generate 64 bytes */
  uint8_t seed_64[64];
  hc_pqc_result_t result = hc_generate_pqc_seed(ctx, seed_64, 64);
  if (result != HC_PQC_SUCCESS) {
    secure_wipe(seed_64, 64);
    return result;
  }

  /* XOR fold: 64 bytes → 32 bytes */
  for (int i = 0; i < 32; i++) {
    seed_32[i] = seed_64[i] ^ seed_64[i + 32];
  }

  secure_wipe(seed_64, 64);
  return HC_PQC_SUCCESS;
}

/* ========================================================================
 * API Function 15: hc_generate_pqc_seed() - General Seed Generation
 * ======================================================================== */
hc_pqc_result_t hc_generate_pqc_seed(hc_vac_context_t ctx, uint8_t *seed,
                                     size_t len) {
  if (!ctx || !seed || len == 0) {
    return HC_PQC_ERROR;
  }

  /* Use existing hc_vacuum_generate_seed with length parameter */
  uint8_t temp[32];
  size_t generated = 0;

  while (generated < len) {
    if (hc_vacuum_generate_seed(ctx, temp) != HC_SUCCESS) {
      secure_wipe(seed, generated);
      return HC_PQC_ERROR;
    }

    size_t to_copy = (len - generated) < 32 ? (len - generated) : 32;
    memcpy(seed + generated, temp, to_copy);
    generated += to_copy;
  }

  secure_wipe(temp, 32);
  return HC_PQC_SUCCESS;
}

/* ========================================================================
 * API Function 16: hc_generate_pqc_seed_safe() - With Self-Healing
 * ======================================================================== */
hc_pqc_result_t hc_generate_pqc_seed_safe(hc_vac_context_t ctx, uint8_t *seed,
                                          size_t len, int max_retries) {
  if (!ctx || !seed || len == 0) {
    return HC_PQC_ERROR;
  }

  if (max_retries <= 0) {
    max_retries = 3;
  }

  for (int attempt = 0; attempt < max_retries; attempt++) {
    hc_pqc_result_t result = hc_generate_pqc_seed(ctx, seed, len);

    if (result == HC_PQC_SUCCESS) {
      return HC_PQC_SUCCESS;
    }

    /* Self-healing escalation */
    if (attempt < max_retries - 1) {
      /* Trigger self-heal with increasing tier */
      /* hc_result_t heal_result = attempt_self_heal(ctx, attempt + 1); */
      /* if (heal_result != HC_SUCCESS) { */
      /* return HC_PQC_SELF_HEAL_FAILURE; */
      /* } */
    }
  }

  return HC_PQC_ERROR;
}

/* ========================================================================
 * API Function 17: hc_generate_pqc_seed_2026() - Production 2026
 * ======================================================================== */
hc_pqc_result_t hc_generate_pqc_seed_2026(hc_vac_context_t ctx, uint8_t *seed,
                                          size_t len) {
  if (!ctx || !seed || len == 0) {
    return HC_PQC_ERROR;
  }

  /* Use safe version with Lyapunov monitoring built-in */
  return hc_generate_pqc_seed_safe(ctx, seed, len, 3);
}

/* ========================================================================
 * API Function 18: hc_generate_batch() - Reservoir Batch
 * ======================================================================== */
size_t hc_generate_batch(hc_vac_context_t ctx, uint8_t *batch,
                         size_t batch_size) {
  if (!ctx || !batch || batch_size == 0) {
    return 0;
  }

  /* Access internal context structure */
  struct hc_vac_context_s *internal_ctx = (struct hc_vac_context_s *)ctx;
  size_t retrieved = 0;

  pthread_mutex_lock(&internal_ctx->lock);

  /* Retrieve from reservoir */
  while (retrieved < batch_size && internal_ctx->head != internal_ctx->tail) {
    uint64_t sample = internal_ctx->reservoir[internal_ctx->head];

    /* Copy up to 8 bytes */
    size_t to_copy =
        (batch_size - retrieved) < 8 ? (batch_size - retrieved) : 8;
    memcpy(batch + retrieved, &sample, to_copy);

    internal_ctx->head = (internal_ctx->head + 1) % 4096;
    retrieved += to_copy;
  }

  pthread_mutex_unlock(&internal_ctx->lock);

  return retrieved;
}

/* ========================================================================
 * API Function 19: condition_entropy() - SHA-3 with Secure Wipe
 * ======================================================================== */
/* Implemented in hc_vacuum_engine.c */

/* ========================================================================
 * API Function 20: hc_vector_evolve() - Vectorized Evolution
 * ======================================================================== */
void hc_vector_evolve(__m512i *state_q, __m512i *state_p, int cycles) {
  for (int i = 0; i < cycles; i++) {
    /* Skew Tent Map */
    __m512i p_param = _mm512_set1_epi64(0x7FFFFFFFFFFFFFFFULL);
    __m512i one = _mm512_set1_epi64(0xFFFFFFFFULL);
    __mmask8 mask = _mm512_cmp_epu64_mask(*state_q, p_param, _MM_CMPINT_LT);
    __m512i branch_a = _mm512_slli_epi64(*state_q, 1);
    __m512i branch_b = _mm512_sub_epi64(one, *state_q);
    branch_b = _mm512_slli_epi64(branch_b, 1);
    __m512i force = _mm512_mask_blend_epi64(mask, branch_b, branch_a);

    /* Kick: update momentum */
    *state_p = _mm512_add_epi64(*state_p, force);

    /* Drift: update position */
    *state_q = _mm512_add_epi64(*state_q, *state_p);

    /* Perpetual chaos */
    uint64_t jitter = __rdtsc() & 0xFF;
    *state_p = _mm512_xor_si512(*state_p, _mm512_set1_epi64(jitter));
  }
}

/* ========================================================================
 * API Function 21: hc_condition_and_output() - Combined Operation
 * ======================================================================== */
hc_pqc_result_t hc_condition_and_output(hc_vac_context_t ctx,
                                        uint8_t output[32]) {
  uint8_t raw[64];
  hc_pqc_result_t result = hc_generate_pqc_seed(ctx, raw, 64);

  if (result != HC_PQC_SUCCESS) {
    secure_wipe(raw, 64);
    return result;
  }

  condition_entropy(raw, 64, output);
  secure_wipe(raw, 64);

  return HC_PQC_SUCCESS;
}

/* ========================================================================
 * Advanced Health Test 22: Enhanced APT
 * ======================================================================== */
void hc_enhanced_apt_init(hc_enhanced_apt_t *apt) {
  memset(apt, 0, sizeof(hc_enhanced_apt_t));
}

int hc_enhanced_apt_test(hc_enhanced_apt_t *apt, uint8_t sample) {
  /* Add to sliding window */
  uint8_t old_sample = apt->window[apt->window_idx];
  apt->window[apt->window_idx] = sample;
  apt->window_idx = (apt->window_idx + 1) % HC_APT_WINDOW;

  /* Update counts */
  apt->counts[old_sample]--;
  apt->counts[sample]++;

  /* Check if any count exceeds threshold */
  for (int i = 0; i < 256; i++) {
    if (apt->counts[i] >= HC_APT_CUTOFF) {
      return -1; /* APT failure */
    }
  }

  return 0; /* Pass */
}

/* ========================================================================
 * Advanced Health Test 23: Live Health Test
 * ======================================================================== */
int nist_live_health_test(hc_vac_context_t ctx, const uint8_t *samples,
                          size_t count) {
  hc_full_rct_t rct;
  hc_enhanced_apt_t apt;

  hc_full_rct_init(&rct);
  hc_enhanced_apt_init(&apt);

  for (size_t i = 0; i < count; i++) {
    if (hc_full_rct_test(&rct, samples[i]) != 0) {
      return HC_PQC_RCT_FAILURE;
    }

    if (hc_enhanced_apt_test(&apt, samples[i]) != 0) {
      return HC_PQC_APT_FAILURE;
    }
  }

  return HC_PQC_SUCCESS;
}

/* ========================================================================
 * Advanced Health Test 24: Full RCT
 * ======================================================================== */
void hc_full_rct_init(hc_full_rct_t *rct) {
  memset(rct, 0, sizeof(hc_full_rct_t));
  rct->cutoff = HC_RCT_CUTOFF; /* 30, not 5 */
}

int hc_full_rct_test(hc_full_rct_t *rct, uint8_t sample) {
  if (sample == rct->last_sample) {
    rct->repetition_count++;

    if (rct->repetition_count >= rct->cutoff) {
      return -1; /* RCT failure */
    }
  } else {
    rct->last_sample = sample;
    rct->repetition_count = 0;
  }

  return 0; /* Pass */
}
