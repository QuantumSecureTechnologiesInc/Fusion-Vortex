/*
 * HyperCycle PQC – NVIDIA CUDA Backend
 *
 * This module implements the GPU backend for NVIDIA CUDA devices.  It
 * conforms to the `hc_gpu_backend_t` interface defined in
 * `hc_gpu_universal.h` and is loaded dynamically at runtime via the
 * universal loader.  If this file is compiled with a host compiler
 * lacking CUDA support the build system should skip it.
 */

#include "hc_gpu_universal.h"
#include "hc_math_core.h"

#include <cuda_runtime.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

/* Context implementation for CUDA.  We currently store a stream for
 * asynchronous operations; additional fields could be added for memory
 * pools or pinned buffers. */
typedef struct {
  cudaStream_t stream;
  hc_telemetry_t telemetry;
} cuda_context_impl_t;

/* Concrete definition of the opaque context structure */
struct hc_context_s {
  void *impl;
};

/* High‑resolution timer for telemetry. */
static inline double get_time_sec() {
  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);
  return (double)ts.tv_sec + (double)ts.tv_nsec * 1e-9;
}

/* Device kernel: generate entropy for each index.  Each thread processes
 * one element of the batch. */
__global__ static void hc_cuda_generate_kernel(const uint64_t *seeds,
                                               const uint64_t *blind,
                                               uint8_t *out, size_t count,
                                               bool use_blind) {
  size_t idx = blockIdx.x * blockDim.x + threadIdx.x;
  if (idx >= count)
    return;
  uint64_t seed = seeds[idx];
  uint64_t mask = 0;
  if (use_blind && blind)
    mask = blind[idx];
  uint8_t local_key[32];
  hc_generate_single_key(seed, mask, idx, local_key);
  uint8_t *dst = out + idx * 32;
#pragma unroll
  for (int i = 0; i < 32; i++) {
    dst[i] = local_key[i];
  }
}

/* Initialise CUDA context. */
static hc_gpu_status_t cuda_init_context(hc_context_t *ctx,
                                         const hc_context_config_t *config) {
  if (!ctx)
    return HC_GPU_ERR_INVALID_ARGS;
  int deviceCount = 0;
  if (cudaGetDeviceCount(&deviceCount) != cudaSuccess || deviceCount == 0) {
    return HC_GPU_ERR_NO_DEVICE;
  }
  int device_id = 0;
  if (config && config->device_id >= 0 && config->device_id < deviceCount) {
    device_id = config->device_id;
  }
  cudaSetDevice(device_id);
  cuda_context_impl_t *impl =
      (cuda_context_impl_t *)calloc(1, sizeof(cuda_context_impl_t));
  if (!impl)
    return HC_GPU_ERR_MEMORY;
  /* Use default stream by initialising to zero.  Advanced profiles could
   * create high‑priority streams. */
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

/* Free CUDA context. */
static void cuda_free_context(hc_context_t ctx) {
  if (!ctx)
    return;
  cuda_context_impl_t *impl = (cuda_context_impl_t *)ctx->impl;
  if (impl) {
    /* No persistent allocations to release at present. */
    free(impl);
  }
  free(ctx);
}

/* Query capabilities of CUDA backend. */
static hc_gpu_status_t cuda_get_caps(hc_context_t ctx,
                                     hc_backend_caps_t *caps) {
  (void)ctx;
  if (!caps)
    return HC_GPU_ERR_INVALID_ARGS;
  caps->supports_dma = true;
  /* We support asynchronous execution by utilising CUDA streams and
   * pinned host memory. */
  caps->supports_async = true;
  caps->supports_scalar_sponge = true;
  caps->supports_avx512_ifma = false;
  caps->max_batch_size = 0;
  caps->optimal_batch_size = 1024;
  memset(caps->_reserved, 0, sizeof(caps->_reserved));
  return HC_GPU_SUCCESS;
}

/* Allocate pinned host memory using CUDA. */
static void *cuda_alloc_pinned(hc_context_t ctx, size_t size) {
  (void)ctx;
  void *ptr = NULL;
  if (cudaHostAlloc(&ptr, size, cudaHostAllocDefault) != cudaSuccess) {
    return NULL;
  }
  return ptr;
}

/* Free pinned host memory. */
static void cuda_free_pinned(hc_context_t ctx, void *ptr) {
  (void)ctx;
  if (ptr)
    cudaFreeHost(ptr);
}

/* Generate a batch of entropy on CUDA. */
static hc_gpu_status_t cuda_generate_batch(hc_context_t ctx,
                                           const uint64_t *seeds,
                                           const uint64_t *blinding,
                                           uint8_t *out_buffer, size_t count,
                                           uint32_t flags) {
  if (!seeds || !out_buffer)
    return HC_GPU_ERR_INVALID_ARGS;
  cuda_context_impl_t *impl = (cuda_context_impl_t *)ctx->impl;
  bool use_blind = (flags & HC_FLAG_ENABLE_BLINDING) != 0;
  bool non_blocking = (flags & HC_FLAG_NON_BLOCKING) != 0;
  /* Start timer for telemetry */
  double start = get_time_sec();
  /* Allocate device buffers */
  uint64_t *d_seeds = NULL;
  uint64_t *d_blind = NULL;
  uint8_t *d_out = NULL;
  size_t seed_bytes = count * sizeof(uint64_t);
  size_t out_bytes = count * 32;
  if (cudaMalloc(&d_seeds, seed_bytes) != cudaSuccess)
    return HC_GPU_ERR_MEMORY;
  if (use_blind) {
    if (cudaMalloc(&d_blind, seed_bytes) != cudaSuccess) {
      cudaFree(d_seeds);
      return HC_GPU_ERR_MEMORY;
    }
  }
  if (cudaMalloc(&d_out, out_bytes) != cudaSuccess) {
    cudaFree(d_seeds);
    if (d_blind)
      cudaFree(d_blind);
    return HC_GPU_ERR_MEMORY;
  }
  /* Detect whether host pointers are pinned. */
  cudaPointerAttributes attr;
  bool seeds_pinned = false;
  bool blind_pinned = false;
  bool out_pinned = false;
  if (cudaPointerGetAttributes(&attr, (void *)seeds) == cudaSuccess) {
    seeds_pinned = (attr.type == cudaMemoryTypeHost);
  }
  if (use_blind && blinding) {
    if (cudaPointerGetAttributes(&attr, (void *)blinding) == cudaSuccess) {
      blind_pinned = (attr.type == cudaMemoryTypeHost);
    }
  }
  if (cudaPointerGetAttributes(&attr, (void *)out_buffer) == cudaSuccess) {
    out_pinned = (attr.type == cudaMemoryTypeHost);
  }
  /* Copy seeds and blinding seeds to device.  Use asynchronous copy
   * if the host memory is pinned. */
  cudaError_t copy_err;
  if (seeds_pinned) {
    copy_err = cudaMemcpyAsync(d_seeds, seeds, seed_bytes,
                               cudaMemcpyHostToDevice, impl ? impl->stream : 0);
  } else {
    copy_err = cudaMemcpy(d_seeds, seeds, seed_bytes, cudaMemcpyHostToDevice);
  }
  if (copy_err != cudaSuccess) {
    cudaFree(d_seeds);
    if (d_blind)
      cudaFree(d_blind);
    cudaFree(d_out);
    return HC_GPU_ERR_MEMORY;
  }
  if (use_blind && blinding) {
    if (blind_pinned) {
      copy_err =
          cudaMemcpyAsync(d_blind, blinding, seed_bytes, cudaMemcpyHostToDevice,
                          impl ? impl->stream : 0);
    } else {
      copy_err =
          cudaMemcpy(d_blind, blinding, seed_bytes, cudaMemcpyHostToDevice);
    }
    if (copy_err != cudaSuccess) {
      cudaFree(d_seeds);
      cudaFree(d_blind);
      cudaFree(d_out);
      return HC_GPU_ERR_MEMORY;
    }
  }
  /* Launch kernel */
  int blockSize = 256;
  int gridSize = (int)((count + blockSize - 1) / blockSize);
  hc_cuda_generate_kernel<<<gridSize, blockSize, 0, impl ? impl->stream : 0>>>(
      d_seeds, d_blind, d_out, count, use_blind);
  if (cudaGetLastError() != cudaSuccess) {
    cudaFree(d_seeds);
    if (d_blind)
      cudaFree(d_blind);
    cudaFree(d_out);
    return HC_GPU_ERR_KERNEL_FAILURE;
  }
  /* Copy result back.  Use asynchronous copy if out_buffer is pinned. */
  cudaError_t err;
  if (out_pinned) {
    err = cudaMemcpyAsync(out_buffer, d_out, out_bytes, cudaMemcpyDeviceToHost,
                          impl ? impl->stream : 0);
  } else {
    err = cudaMemcpy(out_buffer, d_out, out_bytes, cudaMemcpyDeviceToHost);
  }
  /* In non‑blocking mode we do not synchronise the stream here; the
   * caller is expected to invoke hc_gpu_sync() to complete the
   * transfer.  In blocking mode we wait for all operations to finish. */
  if (!non_blocking) {
    /* Wait for all asynchronous copies and kernel execution to complete */
    cudaStreamSynchronize(impl ? impl->stream : 0);
  }
  cudaFree(d_seeds);
  if (d_blind)
    cudaFree(d_blind);
  cudaFree(d_out);
  if (err != cudaSuccess)
    return HC_GPU_ERR_MEMORY;
  /* Update telemetry */
  impl->telemetry.total_batches += 1;
  impl->telemetry.total_keys_generated += count;
  if (!non_blocking) {
    double end = get_time_sec();
    impl->telemetry.last_batch_time_sec = end - start;
  }
  impl->telemetry.last_batch_count = count;
  return HC_GPU_SUCCESS;
}

/* Synchronise: wait for all queued operations to complete. */
static hc_gpu_status_t cuda_sync(hc_context_t ctx) {
  cuda_context_impl_t *impl = (cuda_context_impl_t *)ctx->impl;
  cudaError_t err = cudaStreamSynchronize(impl ? impl->stream : 0);
  return (err == cudaSuccess) ? HC_GPU_SUCCESS : HC_GPU_ERR_KERNEL_FAILURE;
}

/* Telemetry: populate the output structure with counters collected by
 * the CUDA backend.  Fields are updated after each batch. */
static hc_gpu_status_t cuda_get_telemetry(hc_context_t ctx,
                                          hc_telemetry_t *out) {
  if (!ctx || !out)
    return HC_GPU_ERR_INVALID_ARGS;
  cuda_context_impl_t *impl = (cuda_context_impl_t *)ctx->impl;
  *out = impl->telemetry;
  return HC_GPU_SUCCESS;
}

/* Error string mapping. */
static const char *cuda_error_string(hc_gpu_status_t code) {
  switch (code) {
  case HC_GPU_SUCCESS:
    return "success";
  case HC_GPU_ERR_NO_DEVICE:
    return "no CUDA device found";
  case HC_GPU_ERR_MEMORY:
    return "CUDA memory error";
  case HC_GPU_ERR_KERNEL_FAILURE:
    return "CUDA kernel failure";
  case HC_GPU_ERR_NOT_INITIALIZED:
    return "backend not initialised";
  case HC_GPU_ERR_SYMBOL_MISSING:
    return "symbol missing";
  case HC_GPU_ERR_INVALID_ARGS:
    return "invalid arguments";
  case HC_GPU_ERR_UNSUPPORTED:
    return "unsupported operation";
  default:
    return "unknown error";
  }
}

/* Return backend version. */
static uint32_t cuda_get_version(void) { return 0x01070100; }

/* Compose the CUDA backend. */
static hc_gpu_backend_t g_cuda_backend = {.name = "HyperCycle CUDA Backend",
                                          .type = HC_BACKEND_CUDA,
                                          .init_context = cuda_init_context,
                                          .free_context = cuda_free_context,
                                          .get_caps = cuda_get_caps,
                                          .alloc_pinned = cuda_alloc_pinned,
                                          .free_pinned = cuda_free_pinned,
                                          .generate_batch = cuda_generate_batch,
                                          .sync = cuda_sync,
                                          .get_telemetry = cuda_get_telemetry,
                                          .error_string = cuda_error_string,
                                          .get_version = cuda_get_version};

/* Exported symbol: returns a pointer to the CUDA backend. */
extern "C" const hc_gpu_backend_t *hc_get_gpu_backend(void) {
  return &g_cuda_backend;
}