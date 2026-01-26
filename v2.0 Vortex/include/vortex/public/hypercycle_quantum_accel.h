#ifndef HYPERCYCLE_QUANTUM_ACCEL_H
#define HYPERCYCLE_QUANTUM_ACCEL_H

#include <stddef.h>
#include <stdint.h>

typedef struct {
  uint32_t **kronecker_tables; // Precomputed Kronecker products
  size_t dimension;            // 256, 512, or 768
} virtual_quantum_accelerator_t;

int virtual_quantum_init(virtual_quantum_accelerator_t *accelerator,
                         size_t dimension);
void virtual_quantum_cleanup(virtual_quantum_accelerator_t *accelerator);
void virtual_quantum_accelerate_keygen(
    const

    virtual_quantum_accelerator_t *accelerator,
    const uint8_t *seed, size_t seed_len, uint8_t *result, size_t *result_len);

#endif // HYPERCYCLE_QUANTUM_ACCEL_H
