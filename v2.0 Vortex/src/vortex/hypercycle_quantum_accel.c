#include "vortex/public/hypercycle_quantum_accel.h"
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define HYPERCYCLE_Q 3329

int virtual_quantum_init(virtual_quantum_accelerator_t *accelerator,
                         size_t dimension) {
  if (!accelerator || dimension == 0)
    return -1;

  accelerator->dimension = dimension;

  // Allocate Kronecker tables
  accelerator->kronecker_tables = malloc(dimension * sizeof(uint32_t *));
  if (!accelerator->kronecker_tables)
    return -1;

  for (size_t i = 0; i < dimension; i++) {
    accelerator->kronecker_tables[i] = malloc(dimension * sizeof(uint32_t));
    if (!accelerator->kronecker_tables[i]) {
      // Cleanup on failure
      for (size_t j = 0; j < i; j++) {
        free(accelerator->kronecker_tables[j]);
      }
      free(accelerator->kronecker_tables);
      return -1;
    }

    // Precompute Kronecker product decomposition tables
    for (size_t j = 0; j < dimension; j++) {
      accelerator->kronecker_tables[i][j] =
          (uint32_t)(((uint64_t)i * j) % HYPERCYCLE_Q);
    }
  }

  return 0;
}

void virtual_quantum_cleanup(virtual_quantum_accelerator_t *accelerator) {
  if (!accelerator || !accelerator->kronecker_tables)
    return;

  for (size_t i = 0; i < accelerator->dimension; i++) {
    free(accelerator->kronecker_tables[i]);
  }
  free(accelerator->kronecker_tables);
  accelerator->kronecker_tables = NULL;
}

void virtual_quantum_accelerate_keygen(
    const virtual_quantum_accelerator_t *accelerator, const uint8_t *seed,
    size_t seed_len, uint8_t *result, size_t *result_len) {
  if (!accelerator || !seed || !result || !result_len)
    return;

  size_t output_size = seed_len * 2;
  *result_len = output_size;

  for (size_t i = 0; i < seed_len; i += 32) {
    size_t chunk_size = (seed_len - i > 32) ? 32 : (seed_len - i);

    if (chunk_size >= 2) {
      size_t idx1 = seed[i] % accelerator->dimension;
      size_t idx2 = seed[i + 1] % accelerator->dimension;
      uint32_t accelerated = accelerator->kronecker_tables[idx1][idx2];

      // Store as little-endian
      size_t out_pos = (i / 32) * 8;
      if (out_pos + 4 <= output_size) {
        result[out_pos] = accelerated & 0xFF;
        result[out_pos + 1] = (accelerated >> 8) & 0xFF;
        result[out_pos + 2] = (accelerated >> 16) & 0xFF;
        result[out_pos + 3] = (accelerated >> 24) & 0xFF;
      }
    }
  }
}
