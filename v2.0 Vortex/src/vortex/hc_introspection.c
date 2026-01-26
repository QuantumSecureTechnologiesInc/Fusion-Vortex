#include "vortex/public/hc_introspection.h"
#include "vortex/internal/hc_cpu_features.h"
#include "vortex/public/hc_core.h"
#include "vortex/public/hc_gpu_universal.h"

hc_cpu_features_t hc_introspection_cpu_features(void) {
  hc_cpu_features_t out = {0, 0, 0};
  hc_cpu_features_init();
  out.has_avx512f = hc_cpu_has_avx512f();
  out.has_avx512dq = hc_cpu_has_avx512dq();
  out.has_avx512ifma = hc_cpu_has_avx512ifma();
  return out;
}

const char *hc_introspection_active_backend(void) {
  const hc_gpu_backend_t *b = hc_gpu_auto_init();
  if (!b || !b->name)
    return "CPU";
  return b->name;
}

uint32_t hc_introspection_entropy_options(void) {
  return hc_entropy_get_options();
}
