/*
 * HyperCycle PQC – Universal GPU Loader Implementation
 *
 * This file provides a reference implementation of the universal GPU
 * interface described in `hc_gpu_universal.h`.  It includes a robust
 * CPU fallback backend and dynamic loader stubs for CUDA and ROCm
 * backends.  In environments where no GPU plugin is present, the CPU
 * implementation is selected automatically.
 *
 * The CPU backend leverages the chaos map defined in `hc_math_core.h`
 * to generate entropy in a single thread.  Telemetry counters record
 * the number of batches processed and the total number of keys
 * generated along with the execution time of the last batch.
 */

#include "hc_gpu_universal.h"
#include "hc_math_avx512.h"
#include "hc_math_core.h"

#ifdef _WIN32
#include <windows.h>
#define RTLD_LAZY 0
#define RTLD_LOCAL 0

static void *dlopen(const char *filename, int flags) {
  if (!filename)
    return NULL;
  return (void *)LoadLibraryA(filename);
}

static void *dlsym(void *handle, const char *symbol) {
  if (!handle || !symbol)
    return NULL;
  return (void *)GetProcAddress((HMODULE)handle, symbol);
}

static int dlclose(void *handle) {
  if (!handle)
    return -1;
  return FreeLibrary((HMODULE)handle) ? 0 : -1;
}

static const char *dlerror(void) { return "Windows Error"; }
#else
#include <dlfcn.h>
#endif

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

/* Forward declaration of the context structure.  Each backend is
 * responsible for defining its own layout; the universal layer treats
 * this pointer opaquely. */
struct hc_context_s {
  void *impl;
};

/* --------------------------------------------------------------------------
 * Helper functions
 * -------------------------------------------------------------------------- */

/* Return monotonic time in seconds.  Used for telemetry. */
/* Return monotonic time in seconds.  Used for telemetry. */
static double get_time_sec(void) {
#ifdef _WIN32
  LARGE_INTEGER frequency;
  LARGE_INTEGER count;
  QueryPerformanceFrequency(&frequency);
  QueryPerformanceCounter(&count);
  return (double)count.QuadPart / (double)frequency.QuadPart;
#else
  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);
  return (double)ts.tv_sec + (double)ts.tv_nsec * 1e-9;
#endif
}

/* Default error string mapping. */
static const char *default_error_string(hc_gpu_status_t code) {
  switch (code) {
  case HC_GPU_SUCCESS:
    return "success";
  case HC_GPU_ERR_NO_DEVICE:
    return "no device";
  case HC_GPU_ERR_MEMORY:
    return "memory allocation failed";
  case HC_GPU_ERR_KERNEL_FAILURE:
    return "kernel execution failed";
  case HC_GPU_ERR_NOT_INITIALIZED:
    return "not initialised";
  case HC_GPU_ERR_SYMBOL_MISSING:
    return "symbol missing";
  case HC_GPU_ERR_INVALID_ARGS:
    return "invalid arguments";
  case HC_GPU_ERR_UNSUPPORTED:
    return "operation unsupported";
  default:
    return "unknown error";
  }
}

/* --------------------------------------------------------------------------
 * CPU Backend
 *
 * The CPU backend is always available and acts as a safe fallback when
 * hardware acceleration is not present or fails to initialise.  It does
 * not allocate persistent GPU resources and therefore treats the context
 * pointer as unused.  The generate function iterates through the chaos
 * map to produce entropy on the host.
 * -------------------------------------------------------------------------- */

/* Telemetry counters for the CPU backend. */
static hc_telemetry_t g_cpu_telemetry;

/* CPU context structure.  We do not currently maintain per‑context state
 * beyond the reserved pointer, but this structure allows expansion in
 * the future (e.g., to cache allocated buffers). */
typedef struct {
  /* reserved for future use */
  size_t dummy;
} cpu_context_impl_t;

static hc_gpu_status_t cpu_init_context(hc_context_t *ctx,
                                        const hc_context_config_t *config) {
  (void)config;
  if (!ctx)
    return HC_GPU_ERR_INVALID_ARGS;
  cpu_context_impl_t *impl =
      (cpu_context_impl_t *)calloc(1, sizeof(cpu_context_impl_t));
  if (!impl)
    return HC_GPU_ERR_MEMORY;
  (*ctx) = (hc_context_t)malloc(sizeof(struct hc_context_s));
  if (!(*ctx)) {
    free(impl);
    return HC_GPU_ERR_MEMORY;
  }
  (*ctx)->impl = impl;
  return HC_GPU_SUCCESS;
}

static void cpu_free_context(hc_context_t ctx) {
  if (!ctx)
    return;
  cpu_context_impl_t *impl = (cpu_context_impl_t *)ctx->impl;
  free(impl);
  free(ctx);
}

static hc_gpu_status_t cpu_get_caps(hc_context_t ctx, hc_backend_caps_t *caps) {
  (void)ctx;
  if (!caps)
    return HC_GPU_ERR_INVALID_ARGS;
  caps->supports_dma = false;
  caps->supports_async = false;
  caps->supports_scalar_sponge = true;
  /* Detect at compile time whether AVX‑512 IFMA is available. */
#if defined(__AVX512F__) && defined(__AVX512IFMA__)
  caps->supports_avx512_ifma = true;
#else
  caps->supports_avx512_ifma = false;
#endif
  caps->max_batch_size = 0; /* unlimited */
  caps->optimal_batch_size = 1;
  memset(caps->_reserved, 0, sizeof(caps->_reserved));
  return HC_GPU_SUCCESS;
}

static void *cpu_alloc_pinned(hc_context_t ctx, size_t size) {
  (void)ctx;
  /* Align allocation to 64 bytes. */
#ifdef _WIN32
  return _aligned_malloc(size, 64);
#else
  void *ptr = NULL;
  int res = posix_memalign(&ptr, 64, size);
  if (res != 0)
    return NULL;
  return ptr;
#endif
}

static void cpu_free_pinned(hc_context_t ctx, void *ptr) {
  (void)ctx;
#ifdef _WIN32
  if (ptr)
    _aligned_free(ptr);
#else
  if (ptr)
    free(ptr);
#endif
}

static hc_gpu_status_t cpu_generate_batch(hc_context_t ctx,
                                          const uint64_t *seeds,
                                          const uint64_t *blinding_seeds,
                                          uint8_t *out_buffer, size_t count,
                                          uint32_t flags) {
  (void)ctx;
  if (!out_buffer || !seeds)
    return HC_GPU_ERR_INVALID_ARGS;
  double start = get_time_sec();
  bool use_blinding = (flags & HC_FLAG_ENABLE_BLINDING) != 0;
  bool use_avx512 = (flags & HC_FLAG_OPT_AVX512_IFMA) != 0;

  for (size_t i = 0; i < count; i++) {
    uint64_t seed = seeds[i];
    uint64_t blind = 0;
    if (use_blinding && blinding_seeds)
      blind = blinding_seeds[i];

    /* Inline the generation logic from hc_generate_single_key so we
     * can substitute the chaos map step with an AVX‑512 variant
     * when requested.  Using a local index (i) ensures deterministic
     * variation of the mask derivation. */
    hc_quat_t q_main;
    size_t idx = i;
    bool mask = (blind != 0);
    if (!mask) {
      hc_init_state(&q_main, seed);
      for (int j = 0; j < HC_CYCLES; ++j) {
#if defined(__AVX512F__) && defined(__AVX512IFMA__)
        if (use_avx512) {
          hc_chaos_step_avx512_ifma(&q_main);
        } else {
          hc_chaos_map_step(&q_main);
        }
#else
        hc_chaos_map_step(&q_main);
#endif
      }
      uint64_t *out_ptr = (uint64_t *)(out_buffer + i * 32);
      out_ptr[0] = (uint64_t)q_main.w ^ (uint64_t)q_main.x;
      out_ptr[1] = (uint64_t)q_main.y ^ (uint64_t)q_main.z;
      out_ptr[2] = (uint64_t)q_main.w + (uint64_t)q_main.z;
      out_ptr[3] = (uint64_t)q_main.x - (uint64_t)q_main.y;
    } else {
      hc_quat_t q_mask;
      uint64_t mask_idx = idx ^ 0xAAAAAAAA55555555ULL;
      hc_init_state(&q_main, seed);
      hc_init_state(&q_mask, blind ^ mask_idx);
      for (int j = 0; j < HC_CYCLES; ++j) {
#if defined(__AVX512F__) && defined(__AVX512IFMA__)
        if (use_avx512) {
          hc_chaos_step_avx512_ifma(&q_main);
          hc_chaos_step_avx512_ifma(&q_mask);
        } else {
          hc_chaos_map_step(&q_main);
          hc_chaos_map_step(&q_mask);
        }
#else
        hc_chaos_map_step(&q_main);
        hc_chaos_map_step(&q_mask);
#endif
      }
      uint64_t k0 = (uint64_t)q_main.w ^ (uint64_t)q_main.x;
      uint64_t k1 = (uint64_t)q_main.y ^ (uint64_t)q_main.z;
      uint64_t k2 = (uint64_t)q_main.w + (uint64_t)q_main.z;
      uint64_t k3 = (uint64_t)q_main.x - (uint64_t)q_main.y;
      uint64_t m0 = (uint64_t)q_mask.w ^ (uint64_t)q_mask.x;
      uint64_t m1 = (uint64_t)q_mask.y ^ (uint64_t)q_mask.z;
      uint64_t m2 = (uint64_t)q_mask.w + (uint64_t)q_mask.z;
      uint64_t m3 = (uint64_t)q_mask.x - (uint64_t)q_mask.y;
      uint64_t *out_ptr = (uint64_t *)(out_buffer + i * 32);
      out_ptr[0] = k0 ^ m0;
      out_ptr[1] = k1 ^ m1;
      out_ptr[2] = k2 ^ m2;
      out_ptr[3] = k3 ^ m3;
    }
  }
  double end = get_time_sec();
  g_cpu_telemetry.total_batches += 1;
  g_cpu_telemetry.total_keys_generated += count;
  g_cpu_telemetry.last_batch_time_sec = end - start;
  g_cpu_telemetry.last_batch_count = count;
  return HC_GPU_SUCCESS;
}

static hc_gpu_status_t cpu_sync(hc_context_t ctx) {
  (void)ctx;
  /* CPU operations are synchronous */
  return HC_GPU_SUCCESS;
}

static hc_gpu_status_t cpu_get_telemetry(hc_context_t ctx,
                                         hc_telemetry_t *out) {
  (void)ctx;
  if (!out)
    return HC_GPU_ERR_INVALID_ARGS;
  *out = g_cpu_telemetry;
  return HC_GPU_SUCCESS;
}

static const char *cpu_error_string(hc_gpu_status_t code) {
  return default_error_string(code);
}

static uint32_t cpu_get_version(void) {
  /* Version 1.7.1.0 encoded as 0x01070100.  Major=1, Minor=7, Patch=1, Build=0.
   */
  return 0x01070100;
}

/* Compose the CPU backend v‑table. */
static hc_gpu_backend_t g_cpu_backend = {.name = "HyperCycle CPU Core",
                                         .type = HC_BACKEND_CPU,
                                         .init_context = cpu_init_context,
                                         .free_context = cpu_free_context,
                                         .get_caps = cpu_get_caps,
                                         .alloc_pinned = cpu_alloc_pinned,
                                         .free_pinned = cpu_free_pinned,
                                         .generate_batch = cpu_generate_batch,
                                         .sync = cpu_sync,
                                         .get_telemetry = cpu_get_telemetry,
                                         .error_string = cpu_error_string,
                                         .get_version = cpu_get_version};

/* --------------------------------------------------------------------------
 * Dynamic Loader
 *
 * The universal loader attempts to load GPU backends from dynamic
 * libraries at runtime.  Backends must export a symbol
 * `hc_get_gpu_backend` which returns a pointer to an instance of
 * `hc_gpu_backend_t`.  If loading fails or the symbol is missing the
 * loader falls back to the CPU backend.
 * -------------------------------------------------------------------------- */

static void *g_backend_handle = NULL;
static hc_gpu_backend_t g_active_backend;
static bool g_is_initialised = false;

/* Try to load a backend from a shared object.  `lib_name` is the path to
 * the library and `symbol` is the exported symbol providing the v‑table. */
static bool try_load_backend(const char *lib_name, const char *symbol) {
  dlerror();
  void *handle = dlopen(lib_name, RTLD_LAZY | RTLD_LOCAL);
  if (!handle)
    return false;
  /* backends must export a function returning a pointer to their v‑table */
  typedef const hc_gpu_backend_t *(*pf_get_backend)(void);
  pf_get_backend getter = (pf_get_backend)dlsym(handle, symbol);
  const char *err = dlerror();
  if (err || !getter) {
    dlclose(handle);
    return false;
  }
  const hc_gpu_backend_t *backend = getter();
  if (!backend) {
    dlclose(handle);
    return false;
  }
  /* Copy the backend into our static storage */
  g_active_backend = *backend;
  g_backend_handle = handle;
  return true;
}

/* Public API: auto initialise.  Discovers GPU backends and loads the
 * first one that succeeds.  If none are found the CPU backend is
 * selected. */
const hc_gpu_backend_t *hc_gpu_auto_init(void) {
  if (g_is_initialised)
    return &g_active_backend;
  /* Attempt to load CUDA backend */
  if (try_load_backend("./libhc_cuda.so", "hc_get_gpu_backend")) {
    g_is_initialised = true;
    return &g_active_backend;
  }
  /* Attempt to load ROCm backend */
  if (try_load_backend("./libhc_rocm.so", "hc_get_gpu_backend")) {
    g_is_initialised = true;
    return &g_active_backend;
  }
  /* Fallback to CPU */
  g_active_backend = g_cpu_backend;
  g_is_initialised = true;
  return &g_active_backend;
}

/* Release resources and unload the backend library. */
void hc_gpu_shutdown(void) {
  if (!g_is_initialised)
    return;
  /* Free any global resources. */
  if (g_active_backend.free_context) {
    /* We cannot free contexts here because the user must have freed
     * them manually.  We simply call shutdown on dynamic library. */
  }
  if (g_backend_handle) {
    dlclose(g_backend_handle);
    g_backend_handle = NULL;
  }
  g_is_initialised = false;
}

/* Convenience wrappers */
static hc_gpu_status_t ensure_active(void) {
  return g_is_initialised ? HC_GPU_SUCCESS : HC_GPU_ERR_NOT_INITIALIZED;
}

hc_gpu_status_t hc_gpu_init_context(hc_context_t *ctx,
                                    const hc_context_config_t *config) {
  if (!g_is_initialised)
    return HC_GPU_ERR_NOT_INITIALIZED;
  return g_active_backend.init_context(ctx, config);
}

void hc_gpu_free_context(hc_context_t ctx) {
  if (!g_is_initialised)
    return;
  if (g_active_backend.free_context)
    g_active_backend.free_context(ctx);
}

hc_gpu_status_t hc_gpu_get_caps(hc_context_t ctx, hc_backend_caps_t *caps) {
  if (!g_is_initialised)
    return HC_GPU_ERR_NOT_INITIALIZED;
  if (!g_active_backend.get_caps)
    return HC_GPU_ERR_UNSUPPORTED;
  return g_active_backend.get_caps(ctx, caps);
}

void *hc_gpu_alloc_pinned(hc_context_t ctx, size_t size) {
  if (!g_is_initialised || !g_active_backend.alloc_pinned)
    return NULL;
  return g_active_backend.alloc_pinned(ctx, size);
}

void hc_gpu_free_pinned(hc_context_t ctx, void *ptr) {
  if (!g_is_initialised || !g_active_backend.free_pinned)
    return;
  g_active_backend.free_pinned(ctx, ptr);
}

hc_gpu_status_t hc_gpu_generate_batch(hc_context_t ctx, const uint64_t *seeds,
                                      const uint64_t *blinding_seeds,
                                      uint8_t *out_buffer, size_t count,
                                      uint32_t flags) {
  if (!g_is_initialised)
    return HC_GPU_ERR_NOT_INITIALIZED;
  return g_active_backend.generate_batch(ctx, seeds, blinding_seeds, out_buffer,
                                         count, flags);
}

hc_gpu_status_t hc_gpu_sync(hc_context_t ctx) {
  if (!g_is_initialised)
    return HC_GPU_ERR_NOT_INITIALIZED;
  if (!g_active_backend.sync)
    return HC_GPU_SUCCESS;
  return g_active_backend.sync(ctx);
}

hc_gpu_status_t hc_gpu_get_telemetry(hc_context_t ctx, hc_telemetry_t *out) {
  if (!g_is_initialised)
    return HC_GPU_ERR_NOT_INITIALIZED;
  if (!g_active_backend.get_telemetry)
    return HC_GPU_ERR_UNSUPPORTED;
  return g_active_backend.get_telemetry(ctx, out);
}

const char *hc_gpu_error_string(hc_gpu_status_t status) {
  if (!g_is_initialised || !g_active_backend.error_string) {
    return default_error_string(status);
  }
  return g_active_backend.error_string(status);
}

uint32_t hc_gpu_get_version(void) {
  if (!g_is_initialised || !g_active_backend.get_version)
    return 0;
  return g_active_backend.get_version();
}