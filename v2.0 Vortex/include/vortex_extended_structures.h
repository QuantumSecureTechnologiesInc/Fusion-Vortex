/**
 * @file vortex_extended_structures.h
 * @brief Extended context structures for comprehensive Vortex monitoring
 */

#ifndef VORTEX_EXTENDED_STRUCTURES_H
#define VORTEX_EXTENDED_STRUCTURES_H

#include <pthread.h>
#include <stdbool.h>
#include <stdint.h>


/* Enhanced Health Monitor with 512-entry reservoir */
typedef struct {
  /* Existing fields */
  uint64_t reservoir[512]; /* APT reservoir (increased from basic) */
  uint64_t last_value;
  int rct_counter;
  int apt_idx;

  /* Extended tracking fields */
  uint64_t total_samples;      /* Total samples processed */
  uint64_t rct_failures;       /* RCT failure count */
  uint64_t apt_failures;       /* APT failure count */
  double entropy_estimate;     /* Shannon entropy estimate */
  uint64_t last_failure_time;  /* Timestamp of last failure */
  uint32_t consecutive_passes; /* Streak of successful tests */
} hc_health_monitor_extended_t;

/* Perpetual Chaos Context */
typedef struct {
  __m512i chaos_state;         /* Current chaos state vector */
  uint64_t jitter_accumulator; /* Accumulated hardware jitter */
  uint64_t chaos_iterations;   /* Total chaos injection cycles */
  double chaos_intensity;      /* Current chaos intensity (0.0-1.0) */
  bool chaos_active;           /* Chaos injection enabled flag */
  pthread_mutex_t chaos_lock;  /* Thread safety for chaos state */
} hc_perpetual_ctx_t;

/* Origin-Specific Context (for compatibility between versions) */
typedef struct {
  uint32_t origin_version;      /* Origin version identifier */
  uint64_t origin_features;     /* Feature flags bitmap */
  char origin_build_id[32];     /* Build identifier */
  uint64_t compatibility_flags; /* Cross-version compatibility */
  void *origin_private;         /* Version-specific private data */
} hc_origin_ctx_t;

/* Master Monitoring Context (combines all monitoring data) */
typedef struct {
  hc_health_monitor_extended_t health;
  hc_lyapunov_monitor_t lyapunov;
  hc_perpetual_ctx_t perpetual;
  hc_origin_ctx_t origin;

  /* Security & Access Control */
  pthread_mutex_t monitoring_lock;
  uint64_t access_count;
  uint64_t last_access_time;
  char client_id[64];         /* Authenticated client identifier */
  bool monitoring_authorized; /* Authorization flag */
} hc_monitoring_master_ctx_t;

#endif /* VORTEX_EXTENDED_STRUCTURES_H */
