/*
 * HyperCycle PQC – Universal GPU Interface
 *
 * This header defines a universal abstraction layer for discovering and
 * interacting with hardware‑accelerated backends.  It provides a
 * function table (`hc_gpu_backend_t`) describing the capabilities of a
 * loaded backend and exposes API functions to initialise, query and
 * release that backend.  The design follows the principles outlined
 * in the integration documents:
 *
 *  - **Auto‑discovery**: backends are loaded on demand via dlopen().
 *  - **Zero‑copy**: backends may allocate pinned host memory to avoid
 *    unnecessary copies when transferring data to a device.
 *  - **Telemetry**: callers can request internal counters and timing
 *    information to aid optimisation and debugging.
 *  - **Introspection**: versioning and error strings are exposed for
 *    graceful error handling.
 *
 * Author: HyperCycle Engineering Team
 * License: MIT
 */

#ifndef HC_GPU_UNIVERSAL_H
#define HC_GPU_UNIVERSAL_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* --- Status Codes --- */
typedef enum {
    HC_GPU_SUCCESS = 0,
    HC_GPU_ERR_NO_DEVICE = -1,
    HC_GPU_ERR_MEMORY = -2,
    HC_GPU_ERR_KERNEL_FAILURE = -3,
    HC_GPU_ERR_NOT_INITIALIZED = -4,
    HC_GPU_ERR_SYMBOL_MISSING = -5,
    HC_GPU_ERR_INVALID_ARGS = -6,
    HC_GPU_ERR_UNSUPPORTED = -7
} hc_gpu_status_t;

/* --- Backend Types --- */
typedef enum {
    HC_BACKEND_CPU = 0,
    HC_BACKEND_CUDA = 1,
    HC_BACKEND_ROCM = 2
} hc_backend_type_t;

/* --- Generation Flags --- */
typedef enum {
    HC_FLAG_NONE                 = 0,
    HC_FLAG_ENABLE_BLINDING      = 1 << 0,
    HC_FLAG_NON_BLOCKING         = 1 << 1,
    HC_FLAG_OPT_COMBINED_ROTATIONS = 1 << 2,
    HC_FLAG_OPT_SANDWICH_ROTATION  = 1 << 3,
    HC_FLAG_OPT_SCALAR_SPONGE      = 1 << 4,
    HC_FLAG_OPT_BRANCHLESS_LOGIC   = 1 << 5,
    HC_FLAG_OPT_AVX512_IFMA        = 1 << 6,
    HC_FLAG_OPT_MAX_PERFORMANCE    = (1 << 2 | 1 << 3 | 1 << 4 | 1 << 5 | 1 << 6)
} hc_gen_flags_t;

/* --- Context & Capability Structures --- */

typedef struct hc_context_s* hc_context_t;

/* Configuration passed to `init_context`.  The field `numa_node` was
 * carved out of the reserved bytes to allow specifying NUMA affinity.
 * All other bytes remain reserved for future expansion and should be
 * zeroed by callers. */
typedef struct {
    int    device_id;           /* GPU ID or CPU socket; -1 for auto */
    int    stream_priority;     /* queue/stream priority */
    size_t memory_pool_size;    /* preallocated device memory */
    bool   enable_profiling;    /* enable internal telemetry */
    int    numa_node;           /* preferred NUMA node */
    uint8_t _reserved[60];      /* reserved padding */
} hc_context_config_t;

typedef struct {
    bool supports_dma;
    bool supports_async;
    bool supports_scalar_sponge;
    bool supports_avx512_ifma;
    uint32_t max_batch_size;
    uint32_t optimal_batch_size;
    uint8_t _reserved[64];
} hc_backend_caps_t;

/* Telemetry output structure.  Backends fill in whatever fields are
 * available.  Additional fields may be appended in the future; callers
 * should initialise the entire structure to zero before use. */
typedef struct {
    uint64_t total_batches;
    uint64_t total_keys_generated;
    double   last_batch_time_sec;
    uint64_t last_batch_count;
    uint8_t  _reserved[64];
} hc_telemetry_t;

/* --- Backend V‑Table --- */
typedef struct {
    const char* name;             /* human‑readable name */
    hc_backend_type_t type;       /* CPU, CUDA, ROCm */
    /* Lifecycle */
    hc_gpu_status_t (*init_context)(hc_context_t* ctx, const hc_context_config_t* config);
    void             (*free_context)(hc_context_t ctx);
    /* Capabilities */
    hc_gpu_status_t (*get_caps)(hc_context_t ctx, hc_backend_caps_t* caps);
    /* Memory management */
    void*            (*alloc_pinned)(hc_context_t ctx, size_t size);
    void             (*free_pinned)(hc_context_t ctx, void* ptr);
    /* Execution */
    hc_gpu_status_t (*generate_batch)(hc_context_t ctx,
                                      const uint64_t* seeds,
                                      const uint64_t* blinding_seeds,
                                      uint8_t* out_buffer,
                                      size_t count,
                                      uint32_t flags);
    hc_gpu_status_t (*sync)(hc_context_t ctx);
    /* Telemetry */
    hc_gpu_status_t (*get_telemetry)(hc_context_t ctx, hc_telemetry_t* out);
    /* Introspection */
    const char*      (*error_string)(hc_gpu_status_t code);
    uint32_t         (*get_version)(void);
} hc_gpu_backend_t;

/* Discover and initialise the most capable backend.  Returns a pointer to
 * the active backend or NULL on failure. */
const hc_gpu_backend_t* hc_gpu_auto_init(void);

/* Shut down the active backend and unload any dynamic libraries. */
void hc_gpu_shutdown(void);

/* Convenience wrappers around the active backend.  These functions
 * forward the request to the backend if it is initialised; otherwise
 * they return `HC_GPU_ERR_NOT_INITIALIZED`. */
hc_gpu_status_t hc_gpu_init_context(hc_context_t* ctx, const hc_context_config_t* config);
void            hc_gpu_free_context(hc_context_t ctx);
hc_gpu_status_t hc_gpu_get_caps(hc_context_t ctx, hc_backend_caps_t* caps);
void*           hc_gpu_alloc_pinned(hc_context_t ctx, size_t size);
void            hc_gpu_free_pinned(hc_context_t ctx, void* ptr);
hc_gpu_status_t hc_gpu_generate_batch(hc_context_t ctx,
                                      const uint64_t* seeds,
                                      const uint64_t* blinding_seeds,
                                      uint8_t* out_buffer,
                                      size_t count,
                                      uint32_t flags);
hc_gpu_status_t hc_gpu_sync(hc_context_t ctx);
hc_gpu_status_t hc_gpu_get_telemetry(hc_context_t ctx, hc_telemetry_t* out);

/* Return a human‑readable error string corresponding to a status code. */
const char* hc_gpu_error_string(hc_gpu_status_t status);

/* Return the semantic version of the active backend (0xAABBCCDD).  If no
 * backend is active this returns 0. */
uint32_t hc_gpu_get_version(void);

#ifdef __cplusplus
}
#endif

#endif /* HC_GPU_UNIVERSAL_H */