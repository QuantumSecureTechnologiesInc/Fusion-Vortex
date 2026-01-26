/*
 * HyperCycle v1.1 Origin - Universal GPU Interface (Entropy Backend Contract)
 * ==========================================================================
 *
 * This header defines the contract for dynamically-loaded entropy backends.
 *
 * Architectural Note (Vacuum Engine TRNG model):
 *   The entropy output is produced by evolving chaotic vacuum dynamics from
 *   Initial Vacuum Conditions (phase-space initializers). Parameter names
 *   retain "seed" for C familiarity; semantics are vacuum conditions.
 *
 * Masking / Blinding:
 *   If blinding_seed != 0, the backend must apply a vacuum mask:
 *     out = Chaos(seed_base) XOR Chaos(blinding_seed)
 *
 * Backends:
 *   - CPU (built-in fallback)
 *   - CUDA plugin (module)
 *   - ROCm/HIP plugin (module)
 */

#ifndef HC_GPU_UNIVERSAL_H
#define HC_GPU_UNIVERSAL_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
  HC_GPU_SUCCESS = 0,
  HC_GPU_ERR_NO_DEVICE = -1,
  HC_GPU_ERR_MEMORY = -2,
  HC_GPU_ERR_KERNEL_FAILURE = -3,
  HC_GPU_ERR_NOT_INITIALIZED = -4,
  HC_GPU_ERR_SYMBOL_MISSING = -5
} hc_gpu_status_t;

typedef enum {
  HC_BACKEND_CPU = 0,
  HC_BACKEND_CUDA = 1,
  HC_BACKEND_ROCM = 2
} hc_backend_type_t;

typedef struct {
  const char *name;
  hc_backend_type_t type;

  hc_gpu_status_t (*initialize)(void);
  void (*shutdown)(void);

  /**
   * @brief Vacuum TRNG Generation (Batch)
   *
   * @param seed_base Initial Vacuum Condition (phase-space initializer).
   * @param blinding_seed Optional mask Initial Vacuum Condition. If 0, masking is disabled.
   * @param out_buffer Output buffer for keys (each key is 32 bytes).
   * @param count Number of keys to generate.
   */
  hc_gpu_status_t (*generate_entropy_batch)(uint64_t seed_base,
                                           uint64_t blinding_seed,
                                           uint8_t *out_buffer, size_t count);
} hc_gpu_backend_t;

/**
 * @brief Auto-discover an available backend and initialize it.
 *
 * Tries CUDA then ROCm plugins (if present), otherwise returns CPU fallback.
 */
const hc_gpu_backend_t *hc_gpu_auto_init(void);

/**
 * @brief Shutdown active backend and unload any plugin libraries.
 */
void hc_gpu_shutdown(void);

#ifdef __cplusplus
}
#endif

#endif /* HC_GPU_UNIVERSAL_H */
