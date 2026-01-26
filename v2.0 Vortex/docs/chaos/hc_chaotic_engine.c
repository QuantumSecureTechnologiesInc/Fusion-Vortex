#include "hc_chaotic_engine.h"

#define HC_CHAOS_MU_MAX 0xFFFE
#define HC_CHAOS_THRESHOLD 0x80000000

int hc_chaos_init(hc_chaos_ctx_t *ctx, uint32_t seed) {
  if (!ctx)
    return -1;
  ctx->state = (seed == 0) ? 0xACE24681 : seed;
  ctx->iteration = 0;
  ctx->control = HC_CHAOS_MU_MAX;
  return 0;
}

uint32_t hc_chaos_next(hc_chaos_ctx_t *ctx) {
  uint64_t temp;
  if (ctx->state < HC_CHAOS_THRESHOLD) {
    temp = (uint64_t)ctx->state * 2;
  } else {
    temp = (uint64_t)(0xFFFFFFFF - ctx->state) * 2;
  }
  ctx->state = (uint32_t)temp;
  ctx->iteration++;

  /* Post-processing XOR-shift to ensure uniform distribution */
  uint32_t x = ctx->state;
  x ^= x << 13;
  x ^= x >> 17;
  x ^= x << 5;

  if ((ctx->iteration & 0x3FF) == 0) {
    ctx->state ^= 0x12345678;
  }
  return x;
}

int hc_chaos_univ_random(void *context, uint8_t *out, size_t out_len) {
  hc_chaos_ctx_t *ctx = (hc_chaos_ctx_t *)context;
  if (!ctx || !out)
    return -1;

  for (size_t i = 0; i < out_len; i++) {
    if (i % 4 == 0) {
      uint32_t val = hc_chaos_next(ctx);
      out[i] = (uint8_t)(val >> 24);
      for (int j = 1; j < 4 && (i + j) < out_len; j++) {
        out[i + j] = (uint8_t)(val >> (24 - (j * 8)));
      }
    }
  }
  return 0;
}
