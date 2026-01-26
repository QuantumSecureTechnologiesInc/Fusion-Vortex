/**
 * @file vortex_monitoring_api.h
 * @brief User-facing API to access Lyapunov monitoring data
 */

#ifndef VORTEX_MONITORING_API_H
#define VORTEX_MONITORING_API_H

#include <stdint.h>

/**
 * Extended Telemetry Structure with Lyapunov Monitoring
 *
 * This extends the standard hc_telemetry_t to include real-time
 * chaos monitoring metrics.
 */
typedef struct {
  /* Standard telemetry */
  uint64_t total_batches;
  uint64_t total_keys_generated;
  double last_batch_time_sec;
  uint64_t last_batch_count;

  /* Lyapunov Horizon Monitoring (Vortex v2.0 exclusive) */
  double lyapunov_exponent;   /* Current Largest Lyapunov Exponent (LLE) */
  uint64_t phase_shifts;      /* Count of ergodic phase shifts applied */
  uint64_t collapse_warnings; /* Number of near-collapse detections */
  uint64_t self_heal_count;   /* Number of self-heal operations */
} hc_telemetry_extended_t;

/**
 * Get Extended Telemetry with Monitoring Data
 *
 * Usage:
 *   hc_telemetry_extended_t stats;
 *   hc_vacuum_get_telemetry(ctx, (hc_telemetry_t*)&stats);
 *   printf("Lyapunov Exponent: %f\n", stats.lyapunov_exponent);
 *   printf("Phase Shifts: %llu\n", stats.phase_shifts);
 */

/**
 * Monitoring Health Status
 */
typedef enum {
  HC_CHAOS_HEALTHY = 0,  /* LLE > 0.05, system is chaotic */
  HC_CHAOS_MARGINAL = 1, /* 0.01 < LLE < 0.05, approaching collapse */
  HC_CHAOS_CRITICAL = 2  /* LLE < 0.01, needs immediate phase shift */
} hc_chaos_health_t;

/**
 * Get Chaos Health Status
 *
 * Interprets the Lyapunov exponent to provide a simple health indicator.
 */
static inline hc_chaos_health_t get_chaos_health(double lyapunov_exponent) {
  if (lyapunov_exponent > 0.05) {
    return HC_CHAOS_HEALTHY;
  } else if (lyapunov_exponent > 0.01) {
    return HC_CHAOS_MARGINAL;
  } else {
    return HC_CHAOS_CRITICAL;
  }
}

#endif /* VORTEX_MONITORING_API_H */
