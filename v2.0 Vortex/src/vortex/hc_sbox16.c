#include "internal/hc_sbox16.h"
#include <stdatomic.h>
#include <string.h>

static uint16_t g_sbox[65536];
static _Atomic int g_inited = 0;
static _Atomic int g_mode = 0;

/* Deterministic 32-bit mix (splitmix-like) */
static inline uint32_t mix32(uint32_t x) {
  x += 0x9e3779b9u;
  x ^= x >> 16;
  x *= 0x85ebca6bu;
  x ^= x >> 13;
  x *= 0xc2b2ae35u;
  x ^= x >> 16;
  return x;
}

/* Piecewise linear "tent" map on 16-bit domain */
static inline uint16_t tent16(uint16_t v, uint16_t p) {
  // domain [0,65535], p in (0,65535)
  // f(x)= x/p if x<p else (1-x)/(1-p); scaled back to 16-bit
  if (v < p) {
    // (v * 65535) / p
    return (uint16_t)((uint32_t)v * 65535u / (uint32_t)p);
  } else {
    uint32_t num = (uint32_t)(65535u - v) * 65535u;
    uint32_t den = (uint32_t)(65535u - p);
    return (uint16_t)(den ? (num / den) : 0);
  }
}

void hc_sbox16_init(hc_map_mode_t mode) {
  int expected = 0;
  int prev_mode = atomic_load(&g_mode);

  if (atomic_load(&g_inited) && prev_mode == (int)mode) return;

  if (atomic_compare_exchange_strong(&g_inited, &expected, 1)) {
    // First initializer builds table
    atomic_store(&g_mode, (int)mode);

    const uint16_t p = 32749u; // fixed tent pivot (odd, near 0.5*65535)
    for (uint32_t i = 0; i < 65536u; i++) {
      uint32_t m = mix32(i);
      uint16_t v = (uint16_t)m;

      if (mode == HC_MAP_TENT) {
        // Apply tent map + a small nonlinear perturbation from mix
        uint16_t t = tent16(v, p);
        g_sbox[i] = (uint16_t)(t ^ (uint16_t)(m >> 16));
      } else {
        // "Polynomial" chaotic-ish map on 16-bit:
        // v' = v + a*v*(65535-v) + b (mod 2^16)
        // Implemented with 32-bit arithmetic; constants chosen odd.
        uint32_t a = 40503u;   // odd
        uint32_t b = 11467u;   // odd
        uint32_t vv = (uint32_t)v;
        uint32_t term = (a * vv * (65535u - vv)) >> 16;
        uint32_t out = (vv + term + b + (m >> 17)) & 0xFFFFu;
        g_sbox[i] = (uint16_t)out;
      }
    }
    return;
  }

  // If another thread initialized with different mode, rebuild (rare).
  // Keep it simple: if mode differs, rebuild in-place non-atomically and set mode.
  if (atomic_load(&g_mode) != (int)mode) {
    atomic_store(&g_mode, (int)mode);
    const uint16_t p = 32749u;
    for (uint32_t i = 0; i < 65536u; i++) {
      uint32_t m = mix32(i);
      uint16_t v = (uint16_t)m;
      if (mode == HC_MAP_TENT) {
        uint16_t t = tent16(v, p);
        g_sbox[i] = (uint16_t)(t ^ (uint16_t)(m >> 16));
      } else {
        uint32_t a = 40503u, b = 11467u;
        uint32_t vv = (uint32_t)v;
        uint32_t term = (a * vv * (65535u - vv)) >> 16;
        uint32_t out = (vv + term + b + (m >> 17)) & 0xFFFFu;
        g_sbox[i] = (uint16_t)out;
      }
    }
  }
}

const uint16_t *hc_sbox16_table(void) {
  // default init if never called
  if (!atomic_load(&g_inited)) hc_sbox16_init(HC_MAP_POLY);
  return g_sbox;
}

uint64_t hc_sbox16_step_u64(uint64_t x) {
  if (!atomic_load(&g_inited)) hc_sbox16_init((hc_map_mode_t)atomic_load(&g_mode));
  const uint16_t *T = g_sbox;

  uint16_t a = (uint16_t)(x & 0xFFFFu);
  uint16_t b = (uint16_t)((x >> 16) & 0xFFFFu);
  uint16_t c = (uint16_t)((x >> 32) & 0xFFFFu);
  uint16_t d = (uint16_t)((x >> 48) & 0xFFFFu);

  a = T[a];
  b = T[b];
  c = T[c];
  d = T[d];

  uint64_t y = ((uint64_t)d << 48) | ((uint64_t)c << 32) | ((uint64_t)b << 16) | (uint64_t)a;

  // Cross-mix (cheap) to emulate coupling and avoid separability
  y ^= (y << 13) | (y >> 51);
  y ^= (y << 37) | (y >> 27);
  y *= 0x9E3779B97F4A7C15ULL;
  return y;
}
