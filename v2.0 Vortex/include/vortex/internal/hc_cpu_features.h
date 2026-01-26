#ifndef HC_CPU_FEATURES_H
#define HC_CPU_FEATURES_H

#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Detect CPU SIMD capabilities at runtime (x86/x64 only).
 *
 * Notes:
 * - Safe to call multiple times; results are cached.
 * - On non-x86 platforms all features are reported as false.
 */
void hc_cpu_features_init(void);

bool hc_cpu_has_avx512f(void);
bool hc_cpu_has_avx512dq(void);
bool hc_cpu_has_avx512ifma(void);

#ifdef __cplusplus
}
#endif

#endif /* HC_CPU_FEATURES_H */
