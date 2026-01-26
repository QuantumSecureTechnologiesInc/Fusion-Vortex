#ifndef HC_TELEMETRY_H
#define HC_TELEMETRY_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  uint64_t entropy_batches_generated;
  uint64_t entropy_bytes_generated;
  uint64_t entropy_health_failures;
  uint64_t entropy_pool_overruns;

  uint64_t backend_cpu_batches;
  uint64_t backend_gpu_batches;
  uint64_t backend_masked_batches;

  // Last known backend name (NUL-terminated)
  char active_backend[64];
} hc_telemetry_snapshot_t;

/** Populate a snapshot of process-level telemetry counters. */
void hc_telemetry_get_snapshot(hc_telemetry_snapshot_t *out);

/** Reset telemetry counters (useful for benchmarking). */
void hc_telemetry_reset(void);

/* Internal increment helpers used across modules (not part of stable ABI). */
void hc__telemetry_inc_entropy_batch(size_t bytes, int used_gpu, int used_mask);
void hc__telemetry_inc_health_failure(void);
void hc__telemetry_inc_pool_overrun(void);
void hc__telemetry_set_backend_name(const char *name);

#ifdef __cplusplus
}
#endif

#endif /* HC_TELEMETRY_H */
