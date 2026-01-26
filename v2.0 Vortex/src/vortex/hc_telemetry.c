#include "vortex/public/hc_telemetry.h"
#include <stdatomic.h>
#include <string.h>

static _Atomic uint64_t g_entropy_batches = 0;
static _Atomic uint64_t g_entropy_bytes = 0;
static _Atomic uint64_t g_health_failures = 0;
static _Atomic uint64_t g_pool_overruns = 0;

static _Atomic uint64_t g_cpu_batches = 0;
static _Atomic uint64_t g_gpu_batches = 0;
static _Atomic uint64_t g_masked_batches = 0;

static char g_backend_name[64] = "CPU";

void hc_telemetry_get_snapshot(hc_telemetry_snapshot_t *out) {
  if (!out)
    return;
  memset(out, 0, sizeof(*out));

  out->entropy_batches_generated = atomic_load(&g_entropy_batches);
  out->entropy_bytes_generated = atomic_load(&g_entropy_bytes);
  out->entropy_health_failures = atomic_load(&g_health_failures);
  out->entropy_pool_overruns = atomic_load(&g_pool_overruns);

  out->backend_cpu_batches = atomic_load(&g_cpu_batches);
  out->backend_gpu_batches = atomic_load(&g_gpu_batches);
  out->backend_masked_batches = atomic_load(&g_masked_batches);

  // copy backend name non-atomically (best-effort)
  strncpy(out->active_backend, g_backend_name, sizeof(out->active_backend) - 1);
}

void hc_telemetry_reset(void) {
  atomic_store(&g_entropy_batches, 0);
  atomic_store(&g_entropy_bytes, 0);
  atomic_store(&g_health_failures, 0);
  atomic_store(&g_pool_overruns, 0);
  atomic_store(&g_cpu_batches, 0);
  atomic_store(&g_gpu_batches, 0);
  atomic_store(&g_masked_batches, 0);
  hc__telemetry_set_backend_name("CPU");
}

void hc__telemetry_inc_entropy_batch(size_t bytes, int used_gpu,
                                     int used_mask) {
  atomic_fetch_add(&g_entropy_batches, 1);
  atomic_fetch_add(&g_entropy_bytes, (uint64_t)bytes);
  if (used_gpu)
    atomic_fetch_add(&g_gpu_batches, 1);
  else
    atomic_fetch_add(&g_cpu_batches, 1);
  if (used_mask)
    atomic_fetch_add(&g_masked_batches, 1);
}

void hc__telemetry_inc_health_failure(void) {
  atomic_fetch_add(&g_health_failures, 1);
}

void hc__telemetry_inc_pool_overrun(void) {
  atomic_fetch_add(&g_pool_overruns, 1);
}

void hc__telemetry_set_backend_name(const char *name) {
  if (!name)
    name = "CPU";
  // best-effort copy
  memset(g_backend_name, 0, sizeof(g_backend_name));
  strncpy(g_backend_name, name, sizeof(g_backend_name) - 1);
}
