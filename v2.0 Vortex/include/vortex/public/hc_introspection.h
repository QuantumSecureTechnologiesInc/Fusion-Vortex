#ifndef HC_INTROSPECTION_H
#define HC_INTROSPECTION_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  int has_avx512f;
  int has_avx512dq;
  int has_avx512ifma;
} hc_cpu_features_t;

/** Returns runtime CPU SIMD feature flags. */
hc_cpu_features_t hc_introspection_cpu_features(void);

/** Returns a stable string describing the active entropy backend (CPU/CUDA/ROCm). */
const char *hc_introspection_active_backend(void);

/** Returns the current entropy option bitmask (see hc_core.h). */
uint32_t hc_introspection_entropy_options(void);

#ifdef __cplusplus
}
#endif

#endif /* HC_INTROSPECTION_H */
