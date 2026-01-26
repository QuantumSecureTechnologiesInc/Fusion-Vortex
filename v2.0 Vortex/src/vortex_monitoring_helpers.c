/* Helper functions for Lyapunov monitoring in Vortex */
#include "hc_vacuum_engine.h"
#include <immintrin.h>

static inline void
apply_ergodic_phase_shift_inline(struct hc_vac_context_s *ctx) {
  /* Symplectic perturbation maintaining phase space volume */
  __m512i phi = _mm512_set1_epi64(0x9E3779B97F4A7C15ULL); /* Golden ratio */
  ctx->state_p = _mm512_add_epi64(ctx->state_p, phi);

  /* Involutive constraint: ensure reversibility via XOR swap */
  __m512i q_backup = ctx->state_q;
  ctx->state_q = _mm512_xor_si512(ctx->state_q, ctx->state_p);
  ctx->state_p = _mm512_xor_si512(ctx->state_p, q_backup);
  ctx->state_q = _mm512_xor_si512(ctx->state_q, ctx->state_p);
}
