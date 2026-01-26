#include "vortex/public/hypercycle_ultra_optimizer.h"
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
#include <intrin.h>
#include <windows.h>

#else
#include <cpuid.h>
#endif

#ifdef __x86_64__
#include <immintrin.h>
#endif

#define HYPERCYCLE_Q 3329 // ML-KEM modulus
#define HYPERCYCLE_CACHE_LINE_SIZE 64

bool ultra_optimization_is_avx512_supported(void) {
#ifdef __x86_64__
  unsigned int eax, ebx, ecx, edx;
#ifdef _WIN32
  int cpuInfo[4];
  __cpuidex(cpuInfo, 7, 0);
  return (cpuInfo[1] & (1 << 16)) != 0; // AVX-512F
#else
  __cpuid_count(7, 0, eax, ebx, ecx, edx);
  return (ebx & (1 << 16)) != 0; // AVX-512F
#endif
#else
  return false;
#endif
}

int ultra_optimization_init(ultra_optimization_engine_t *engine,
                            size_t table_size) {
  if (!engine || table_size == 0)
    return -1;

  engine->table_size = table_size;
  engine->cache_line_size = HYPERCYCLE_CACHE_LINE_SIZE;
  engine->simd_enabled = ultra_optimization_is_avx512_supported();

  // Allocate precompute tables (256 rows × table_size columns)
  engine->precompute_tables = malloc(256 * sizeof(uint16_t *));
  if (!engine->precompute_tables)
    return -1;

  for (int i = 0; i < 256; i++) {
    engine->precompute_tables[i] = malloc(table_size * sizeof(uint16_t));
    if (!engine->precompute_tables[i]) {
      // Cleanup on failure
      for (int j = 0; j < i; j++) {
        free(engine->precompute_tables[j]);
      }
      free(engine->precompute_tables);
      return -1;
    }

    // Precompute polynomial multiplication mod Q
    for (size_t j = 0; j < table_size; j++) {
      engine->precompute_tables[i][j] = (uint16_t)((i * j) % HYPERCYCLE_Q);
    }
  }

  return 0;
}

void ultra_optimization_cleanup(ultra_optimization_engine_t *engine) {
  if (!engine || !engine->precompute_tables)
    return;

  for (int i = 0; i < 256; i++) {
    free(engine->precompute_tables[i]);
  }
  free(engine->precompute_tables);
  engine->precompute_tables = NULL;
}

uint16_t
ultra_optimization_poly_multiply(const ultra_optimization_engine_t *engine,
                                 uint8_t a, uint8_t b) {
  if (!engine || !engine->precompute_tables)
    return 0;
  return engine->precompute_tables[a][b % engine->table_size];
}

void ultra_optimization_simd_vectorize(
    const ultra_optimization_engine_t *engine, const uint16_t *data, size_t len,
    uint16_t *result) {
  if (!engine || !data || !result)
    return;

#ifdef __x86_64__
  if (engine->simd_enabled && len >= 32) {
    // AVX-512 implementation for 32 elements at once
    for (size_t i = 0; i + 32 <= len; i += 32) {
      // (x * Q) % Q is always 0. Setting result to 0 efficiently.
      __m512i modded = _mm512_setzero_si512();
      _mm512_storeu_si512((__m512i *)&result[i], modded);
    }

    // Handle remaining elements
    for (size_t i = (len / 32) * 32; i < len; i++) {
      result[i] = (data[i] * HYPERCYCLE_Q) % HYPERCYCLE_Q;
    }
  } else {
#endif
    // Fallback scalar implementation
    for (size_t i = 0; i < len; i++) {
      result[i] = (data[i] * HYPERCYCLE_Q) % HYPERCYCLE_Q;
    }
#ifdef __x86_64__
  }
#endif
}

void ultra_optimization_cache_align(const ultra_optimization_engine_t *engine,
                                    uint8_t *data, size_t *len) {
  if (!engine || !data || !len)
    return;

  size_t alignment = engine->cache_line_size;
  size_t padding = (alignment - (*len % alignment)) % alignment;
  *len += padding;

  // Zero out padding bytes
  if (padding > 0) {
    memset(data + *len - padding, 0, padding);
  }
}
