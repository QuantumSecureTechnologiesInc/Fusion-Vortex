/*
 * HyperCycle PQC – AMD ROCm Backend
 *
 * This module implements the GPU backend for AMD HIP/ROCm devices.  It
 * follows the universal GPU interface defined in `hc_gpu_universal.h` and
 * is intended to be built with hipcc.  The functions provided here
 * allocate pinned host memory via hipHostMalloc and launch a kernel to
 * generate entropy using the chaos map.
 */

#include "hc_gpu_universal.h"
#include "hc_math_core.h"

#include <hip/hip_runtime.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

/* Context implementation for ROCm. */
typedef struct {
    hipStream_t stream;
    hc_telemetry_t telemetry;
} rocm_context_impl_t;

/* High‑resolution timer for telemetry. */
static inline double get_time_sec() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (double)ts.tv_sec + (double)ts.tv_nsec * 1e-9;
}

/* Device kernel for entropy generation. */
__global__ static void hc_rocm_generate_kernel(const uint64_t *seeds,
                                                const uint64_t *blind,
                                                uint8_t *out,
                                                size_t count,
                                                bool use_blind) {
    size_t idx = hipBlockIdx_x * hipBlockDim_x + hipThreadIdx_x;
    if (idx >= count) return;
    uint64_t seed = seeds[idx];
    uint64_t mask = 0;
    if (use_blind && blind) mask = blind[idx];
    uint8_t local_key[32];
    hc_generate_single_key(seed, mask, idx, local_key);
    uint8_t *dst = out + idx * 32;
    #pragma unroll
    for (int i = 0; i < 32; i++) {
        dst[i] = local_key[i];
    }
}

/* Initialise ROCm context. */
static hc_gpu_status_t rocm_init_context(hc_context_t *ctx, const hc_context_config_t *config) {
    if (!ctx) return HC_GPU_ERR_INVALID_ARGS;
    int deviceCount = 0;
    if (hipGetDeviceCount(&deviceCount) != hipSuccess || deviceCount == 0) {
        return HC_GPU_ERR_NO_DEVICE;
    }
    int device_id = 0;
    if (config && config->device_id >= 0 && config->device_id < deviceCount) {
        device_id = config->device_id;
    }
    hipSetDevice(device_id);
    rocm_context_impl_t *impl = (rocm_context_impl_t*)calloc(1, sizeof(rocm_context_impl_t));
    if (!impl) return HC_GPU_ERR_MEMORY;
    impl->stream = 0;
    /* Reset telemetry counters. */
    memset(&impl->telemetry, 0, sizeof(impl->telemetry));
    *ctx = (hc_context_t)malloc(sizeof(struct hc_context_s));
    if (!(*ctx)) {
        free(impl);
        return HC_GPU_ERR_MEMORY;
    }
    (*ctx)->impl = impl;
    return HC_GPU_SUCCESS;
}

/* Free ROCm context. */
static void rocm_free_context(hc_context_t ctx) {
    if (!ctx) return;
    rocm_context_impl_t *impl = (rocm_context_impl_t*)ctx->impl;
    if (impl) {
        free(impl);
    }
    free(ctx);
}

/* Query capabilities. */
static hc_gpu_status_t rocm_get_caps(hc_context_t ctx, hc_backend_caps_t *caps) {
    (void)ctx;
    if (!caps) return HC_GPU_ERR_INVALID_ARGS;
    caps->supports_dma = true;
    /* Indicate asynchronous operations are supported via HIP streams. */
    caps->supports_async = true;
    caps->supports_scalar_sponge = true;
    caps->supports_avx512_ifma = false;
    caps->max_batch_size = 0;
    caps->optimal_batch_size = 1024;
    memset(caps->_reserved, 0, sizeof(caps->_reserved));
    return HC_GPU_SUCCESS;
}

/* Allocate pinned host memory using HIP. */
static void* rocm_alloc_pinned(hc_context_t ctx, size_t size) {
    (void)ctx;
    void *ptr = NULL;
    if (hipHostMalloc(&ptr, size, hipHostMallocDefault) != hipSuccess) {
        return NULL;
    }
    return ptr;
}

/* Free pinned host memory. */
static void rocm_free_pinned(hc_context_t ctx, void *ptr) {
    (void)ctx;
    if (ptr) hipHostFree(ptr);
}

/* Generate a batch of entropy on ROCm. */
static hc_gpu_status_t rocm_generate_batch(hc_context_t ctx,
                                           const uint64_t *seeds,
                                           const uint64_t *blinding,
                                           uint8_t *out_buffer,
                                           size_t count,
                                           uint32_t flags) {
    if (!seeds || !out_buffer) return HC_GPU_ERR_INVALID_ARGS;
    rocm_context_impl_t *impl = (rocm_context_impl_t*)ctx->impl;
    bool use_blind = (flags & HC_FLAG_ENABLE_BLINDING) != 0;
    bool non_blocking = (flags & HC_FLAG_NON_BLOCKING) != 0;
    double start = get_time_sec();
    uint64_t *d_seeds = NULL;
    uint64_t *d_blind = NULL;
    uint8_t  *d_out = NULL;
    size_t seed_bytes = count * sizeof(uint64_t);
    size_t out_bytes = count * 32;
    if (hipMalloc(&d_seeds, seed_bytes) != hipSuccess) return HC_GPU_ERR_MEMORY;
    if (use_blind) {
        if (hipMalloc(&d_blind, seed_bytes) != hipSuccess) {
            hipFree(d_seeds);
            return HC_GPU_ERR_MEMORY;
        }
    }
    if (hipMalloc(&d_out, out_bytes) != hipSuccess) {
        hipFree(d_seeds);
        if (d_blind) hipFree(d_blind);
        return HC_GPU_ERR_MEMORY;
    }
    /* Detect pinned memory on host using hipPointerGetAttributes. */
    hipPointerAttribute_t attr;
    bool seeds_pinned = false;
    bool blind_pinned = false;
    bool out_pinned = false;
    if (hipPointerGetAttributes(&attr, (void*)seeds) == hipSuccess) {
        seeds_pinned = (attr.memoryType == hipMemoryTypeHost);
    }
    if (use_blind && blinding) {
        if (hipPointerGetAttributes(&attr, (void*)blinding) == hipSuccess) {
            blind_pinned = (attr.memoryType == hipMemoryTypeHost);
        }
    }
    if (hipPointerGetAttributes(&attr, (void*)out_buffer) == hipSuccess) {
        out_pinned = (attr.memoryType == hipMemoryTypeHost);
    }
    /* Copy seeds and blinding seeds.  Use async if pinned. */
    hipError_t copy_err;
    if (seeds_pinned) {
        copy_err = hipMemcpyAsync(d_seeds, seeds, seed_bytes, hipMemcpyHostToDevice, impl ? impl->stream : 0);
    } else {
        copy_err = hipMemcpy(d_seeds, seeds, seed_bytes, hipMemcpyHostToDevice);
    }
    if (copy_err != hipSuccess) {
        hipFree(d_seeds);
        if (d_blind) hipFree(d_blind);
        hipFree(d_out);
        return HC_GPU_ERR_MEMORY;
    }
    if (use_blind && blinding) {
        if (blind_pinned) {
            copy_err = hipMemcpyAsync(d_blind, blinding, seed_bytes, hipMemcpyHostToDevice, impl ? impl->stream : 0);
        } else {
            copy_err = hipMemcpy(d_blind, blinding, seed_bytes, hipMemcpyHostToDevice);
        }
        if (copy_err != hipSuccess) {
            hipFree(d_seeds);
            hipFree(d_blind);
            hipFree(d_out);
            return HC_GPU_ERR_MEMORY;
        }
    }
    /* Launch kernel */
    int blockSize = 256;
    int gridSize = (int)((count + blockSize - 1) / blockSize);
    hipLaunchKernelGGL(hc_rocm_generate_kernel,
                       dim3(gridSize),
                       dim3(blockSize),
                       0,
                       impl ? impl->stream : 0,
                       d_seeds,
                       d_blind,
                       d_out,
                       count,
                       use_blind);
    if (hipGetLastError() != hipSuccess) {
        hipFree(d_seeds);
        if (d_blind) hipFree(d_blind);
        hipFree(d_out);
        return HC_GPU_ERR_KERNEL_FAILURE;
    }
    /* Copy result back. */
    hipError_t err;
    if (out_pinned) {
        err = hipMemcpyAsync(out_buffer, d_out, out_bytes, hipMemcpyDeviceToHost, impl ? impl->stream : 0);
    } else {
        err = hipMemcpy(out_buffer, d_out, out_bytes, hipMemcpyDeviceToHost);
    }
    /* Synchronise if blocking mode */
    if (!non_blocking) {
        hipStreamSynchronize(impl ? impl->stream : 0);
    }
    hipFree(d_seeds);
    if (d_blind) hipFree(d_blind);
    hipFree(d_out);
    if (err != hipSuccess) return HC_GPU_ERR_MEMORY;
    /* Telemetry updates */
    impl->telemetry.total_batches += 1;
    impl->telemetry.total_keys_generated += count;
    if (!non_blocking) {
        double end = get_time_sec();
        impl->telemetry.last_batch_time_sec = end - start;
    }
    impl->telemetry.last_batch_count = count;
    return HC_GPU_SUCCESS;
}

/* Synchronise: wait until all operations complete. */
static hc_gpu_status_t rocm_sync(hc_context_t ctx) {
    rocm_context_impl_t *impl = (rocm_context_impl_t*)ctx->impl;
    hipError_t err = hipStreamSynchronize(impl ? impl->stream : 0);
    return (err == hipSuccess) ? HC_GPU_SUCCESS : HC_GPU_ERR_KERNEL_FAILURE;
}

/* Telemetry: return the counters collected by the ROCm backend. */
static hc_gpu_status_t rocm_get_telemetry(hc_context_t ctx, hc_telemetry_t *out) {
    if (!ctx || !out) return HC_GPU_ERR_INVALID_ARGS;
    rocm_context_impl_t *impl = (rocm_context_impl_t*)ctx->impl;
    *out = impl->telemetry;
    return HC_GPU_SUCCESS;
}

/* Error string mapping. */
static const char* rocm_error_string(hc_gpu_status_t code) {
    switch (code) {
        case HC_GPU_SUCCESS: return "success";
        case HC_GPU_ERR_NO_DEVICE: return "no ROCm device found";
        case HC_GPU_ERR_MEMORY: return "ROCm memory error";
        case HC_GPU_ERR_KERNEL_FAILURE: return "ROCm kernel failure";
        case HC_GPU_ERR_NOT_INITIALIZED: return "backend not initialised";
        case HC_GPU_ERR_SYMBOL_MISSING: return "symbol missing";
        case HC_GPU_ERR_INVALID_ARGS: return "invalid arguments";
        case HC_GPU_ERR_UNSUPPORTED: return "unsupported operation";
        default: return "unknown error";
    }
}

/* Backend version. */
static uint32_t rocm_get_version(void) {
    return 0x01070100;
}

/* Compose ROCm backend. */
static hc_gpu_backend_t g_rocm_backend = {
    .name           = "HyperCycle ROCm Backend",
    .type           = HC_BACKEND_ROCM,
    .init_context   = rocm_init_context,
    .free_context   = rocm_free_context,
    .get_caps       = rocm_get_caps,
    .alloc_pinned   = rocm_alloc_pinned,
    .free_pinned    = rocm_free_pinned,
    .generate_batch = rocm_generate_batch,
    .sync           = rocm_sync,
    .get_telemetry  = rocm_get_telemetry,
    .error_string   = rocm_error_string,
    .get_version    = rocm_get_version
};

/* Exported symbol for the universal loader. */
extern "C" const hc_gpu_backend_t* hc_get_gpu_backend(void) {
    return &g_rocm_backend;
}