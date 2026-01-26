#ifndef HYPERCYCLE_ULTRA_OPTIMIZER_H
#define HYPERCYCLE_ULTRA_OPTIMIZER_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef struct {
  uint16_t **precompute_tables; // 256 × table_size lookup table
  bool simd_enabled;            // AVX-512 availability
  size_t cache_line_size;       // Cache alignment (64 bytes)
  size_t table_size;            // Security level dependent
} ultra_optimization_engine_t;

int ultra_optimization_init(ultra_optimization_engine_t *engine,
                            size_t table_size);
void ultra_optimization_cleanup(ultra_optimization_engine_t *engine);
uint16_t
ultra_optimization_poly_multiply(const ultra_optimization_engine_t *engine,
                                 uint8_t a, uint8_t b);
void ultra_optimization_simd_vectorize(
    const ultra_optimization_engine_t *engine, const uint16_t *data, size_t len,
    uint16_t *result);
void ultra_optimization_cache_align(const ultra_optimization_engine_t *engine,
                                    uint8_t *data, size_t *len);
bool ultra_optimization_is_avx512_supported(void);

#endif // HYPERCYCLE_ULTRA_OPTIMIZER_H
