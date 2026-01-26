/*
 * HyperCycle Vacuum Engine – Hamiltonian Entropy Generator
 *
 * This header defines a stand‑alone entropy generator built upon
 * Hamiltonian dynamics and advanced NIST SP 800‑90B health monitoring.
 * It provides a context object and functions for initialisation,
 * PQC‑ready seed extraction and safe teardown.  The design is
 * inspired by the integration documents provided with the
 * HyperCycle v1.1 Origin sources and has been extended to include
 * symplectic integration, jitter injection, SHA3 conditioning and
 * multi‑tiered self healing.  All sensitive state is wiped on
 * destruction.
 *
 * The exported API is intentionally orthogonal to the GPU loader
 * interface.  Applications may use the vacuum engine directly
 * alongside the existing `hc_gpu_universal` backend to obtain
 * cryptographically strong seeds for PQC algorithms such as ML‑KEM
 * (Kyber) or ML‑DSA (Dilithium).
 *
 * Author: HyperCycle Engineering Team
 * License: MIT
 */

#ifndef HC_VACUUM_ENGINE_H
#define HC_VACUUM_ENGINE_H

#include "hc_gpu_universal.h" /* for hc_context_config_t and hc_telemetry_t */
#include <immintrin.h>
// #include <pthread.h> // Handled below
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#if defined(_WIN32) || defined(_WIN64) || defined(_MSC_VER) ||                 \
    defined(__MINGW32__)
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
typedef CRITICAL_SECTION pthread_mutex_t;
typedef HANDLE pthread_t;

static inline int pthread_mutex_init(pthread_mutex_t *m, void *attr) {
  (void)attr;
  InitializeCriticalSection(m);
  return 0;
}

static inline int pthread_mutex_lock(pthread_mutex_t *m) {
  EnterCriticalSection(m);
  return 0;
}

static inline int pthread_mutex_unlock(pthread_mutex_t *m) {
  LeaveCriticalSection(m);
  return 0;
}

static inline int pthread_mutex_destroy(pthread_mutex_t *m) {
  DeleteCriticalSection(m);
  return 0;
}

static inline int pthread_create(pthread_t *thread, const void *attr,
                                 void *(*start_routine)(void *), void *arg) {
  (void)thread;
  (void)attr;
  (void)start_routine;
  (void)arg;
  return 0; // Success (no-op)
}

static inline int pthread_join(pthread_t thread, void **retval) {
  (void)thread;
  (void)retval;
  return 0;
}
#else
#include <pthread.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

/* Return codes for the vacuum engine.  These mirror the generic
 * `hc_gpu_status_t` values where appropriate but are scoped to
 * entropy generation.  Zero indicates success; negative values
 * signal an error condition. */
typedef enum {
  HC_SUCCESS = 0,
  HC_ERR_OUT_OF_MEMORY = -1,
  HC_ERR_INVALID_ARGS = -2,
  HC_ERR_KERNEL_FAILURE = -3,
  HC_ERR_DMA_FAILURE = -4
} hc_result_t;

/* NIST health monitor state */
typedef struct {
  uint64_t reservoir[512]; /* HC_VAC_APT_WINDOW */
  uint64_t last_value;
  int rct_counter;
  int apt_idx;
} hc_health_monitor_t;

/* Internal vacuum context structure */
struct hc_vac_context_s {
  __m512i state_q;            /* Position vectors (8 lanes) */
  __m512i state_p;            /* Momentum vectors (8 lanes) */
  hc_health_monitor_t health; /* NIST health monitor */
  uint64_t entropy_failures;  /* Persistent failure count */
  pthread_mutex_t lock;       /* Protects state and health */
  /* Entropy reservoir for background generation */
  uint64_t reservoir[4096]; /* Ring buffer of pre-generated entropy */
  uint32_t head;            /* Read position in ring buffer */
  uint32_t tail;            /* Write position in ring buffer */
  pthread_t worker;         /* Background entropy generation thread */
  bool running;             /* Worker thread control flag */
  /* Telemetry counters */
  uint64_t total_bytes_generated;
  uint64_t total_requests;
  double last_request_time_sec;
};

/* Opaque handle to a vacuum entropy context. */
typedef struct hc_vac_context_s *hc_vac_context_t;

/* PQC seed size (in bytes). */
#define HC_PQC_SEED_SIZE 32U

/* Initialise a new vacuum entropy context.  The configuration is
 * shared with the GPU loader; unused fields should be zeroed.
 *
 *  - `ctx`    will be set to a valid context on success.
 *  - `config` supplies a device identifier or other entropy to
 *    initialise the state.  Passing `NULL` uses a default seed.
 *
 * Returns `HC_SUCCESS` on success or a negative error code on
 * failure.  If initialisation fails no context is allocated. */
hc_result_t hc_vacuum_init_context(hc_vac_context_t *ctx,
                                   const hc_context_config_t *config);

/* Generate a post‑quantum cryptographic seed.  On success the
 * function fills `out_seed` with `HC_PQC_SEED_SIZE` bytes of full
 * entropy.  Internally this performs a Hamiltonian step, runs
 * continuous NIST SP 800‑90B health tests, conditions the raw
 * entropy via SHA3‑256 and applies forward secrecy by mutating
 * the internal state.  If any health test fails the function
 * returns an error and the caller must free and reinitialise the
 * context.
 *
 * Returns `HC_SUCCESS` on success or `HC_ERR_KERNEL_FAILURE` if
 * the entropy source has collapsed.
 */
hc_result_t hc_vacuum_generate_seed(hc_vac_context_t ctx,
                                    uint8_t out_seed[HC_PQC_SEED_SIZE]);

/* Generate a seed with automated self healing.  If a health test
 * failure occurs the generator will attempt to recover the
 * Hamiltonian system via a multi‑tiered strategy (perturbation,
 * hardware reseed or hard reset).  If recovery fails after
 * exhausting all tiers the function returns an error.  On
 * success `out_seed` is filled with `HC_PQC_SEED_SIZE` bytes.
 */
hc_result_t hc_vacuum_generate_seed_safe(hc_vac_context_t ctx,
                                         uint8_t out_seed[HC_PQC_SEED_SIZE]);

/* Query telemetry counters from a vacuum context.  The output
 * structure should be zeroed by the caller before use.  On
 * success the fields `total_batches`, `total_keys_generated`
 * (interpreted here as total bytes generated) and `last_batch_time_sec`
 * are populated. */
hc_result_t hc_vacuum_get_telemetry(hc_vac_context_t ctx, hc_telemetry_t *out);

/* Free a vacuum context and securely wipe all internal state.  It is
 * safe to call this on a NULL handle. */
void hc_vacuum_free_context(hc_vac_context_t ctx);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* HC_VACUUM_ENGINE_H */