/*
 * HyperCycle v1.0 "Genesis" - AMD ROCm Backend
 * Optimized for Instinct & Radeon Architectures
 *
 * Features:
 * - Wavefront-Aware Shuffling (LDS Bypass)
 * - Pinned Memory (hipHostMalloc)
 * - Persistent Context
 */

#include "hc_core.h"
#include <hip/hip_runtime.h>
#include "../include/internal/hc_sbox16.h"
#include <stdint.h>
#include <stdio.h>


#define HC_HIP_BLOCK_SIZE 256

// --- Kernel Optimization ---
// AMD GPUs (CDNA/RDNA) benefit massively from using scalar registers (SGPR)
// where possible, but for chaos maps, we need vector usage.

__global__ void __launch_bounds__(HC_HIP_BLOCK_SIZE)
    hc_vacuum_kernel_amd_opt(uint64_t *seeds, uint8_t *keys_out, int count) {
  int tid = hipBlockIdx_x * hipBlockDim_x + hipThreadIdx_x;
  if (tid >= count)
    return;

  uint64_t state = seeds[tid] ^ (uint64_t)tid;

// 47-Cycle Unrolled Loop
#pragma unroll 16
  for (int i = 0; i < 47; i++) {
    // Linear Congruential Generator step optimized for AMD ALU
    state = state * 6364136223846793005ULL + 1442695040888963407ULL;

    // Wavefront Shuffle (Simulating neighbor interaction)
    // __shfl_xor allows exchanging data between threads in a wavefront
    // This is much faster than global or shared memory.
    uint64_t neighbor = __shfl_xor(state, 1);
    state ^= neighbor;
  }

    // Write out: 32 bytes per seed (256-bit block)
  // Coalesced: each thread writes a contiguous 32-byte block.
  uint8_t *dst = keys_out + ((size_t)tid * 32);

  // Expand state into 4x64-bit words using a lightweight bijective mix.
  uint64_t s0 = state;
  uint64_t s1 = state ^ 0xA5A5A5A5A5A5A5A5ULL;
  uint64_t s2 = state + 0x9E3779B97F4A7C15ULL;
  uint64_t s3 = state ^ 0xC6BC279692B5C323ULL;

  // Extra diffusion per lane (cheap, deterministic, GPU-friendly)
#pragma unroll 4
  for (int k = 0; k < 4; k++) {
    s0 = s0 * 6364136223846793005ULL + 1442695040888963407ULL;
    s1 = s1 * 1442695040888963407ULL + 6364136223846793005ULL;
    s2 ^= (s2 >> 33); s2 *= 0xff51afd7ed558ccdULL; s2 ^= (s2 >> 33);
    s3 ^= (s3 >> 29); s3 *= 0xc4ceb9fe1a85ec53ULL; s3 ^= (s3 >> 32);
  }

  // Store little-endian words to bytes
  memcpy(dst + 0,  &s0, 8);
  memcpy(dst + 8,  &s1, 8);
  memcpy(dst + 16, &s2, 8);
  memcpy(dst + 24, &s3, 8);
}


// --- Context Management ---

typedef struct {
  uint64_t *d_seeds;
  uint8_t *d_output;
  uint64_t *h_pinned_seeds;
  uint8_t *h_pinned_out;
  hipStream_t stream;
  size_t buffer_size;
} hc_hip_context_t;

static hc_hip_context_t g_ctx = {0};

extern "C" {

static int hip_generate_impl(uint64_t *seeds, uint8_t *out_buffer,
                             size_t count) {
  if (count > g_ctx.buffer_size) {
    if (g_ctx.d_seeds) {
      hipFree(g_ctx.d_seeds);
      hipFree(g_ctx.d_output);
      hipHostFree(g_ctx.h_pinned_seeds);
      hipHostFree(g_ctx.h_pinned_out);
    }

    size_t new_size = count < (1024 * 1024) ? (1024 * 1024) : count;
    hipMalloc(&g_ctx.d_seeds, new_size * sizeof(uint64_t));
    hipMalloc(&g_ctx.d_output, new_size * 32 * sizeof(uint8_t));
    hipHostMalloc(&g_ctx.h_pinned_seeds, new_size * sizeof(uint64_t),
                  hipHostMallocDefault);
    hipHostMalloc(&g_ctx.h_pinned_out, new_size * sizeof(uint8_t),
                  hipHostMallocDefault);
    g_ctx.buffer_size = new_size;
  }

  memcpy(g_ctx.h_pinned_seeds, seeds, count * sizeof(uint64_t));

  hipMemcpyAsync(g_ctx.d_seeds, g_ctx.h_pinned_seeds, count * sizeof(uint64_t),
                 hipMemcpyHostToDevice, g_ctx.stream);

  int blocks = (count + HC_HIP_BLOCK_SIZE - 1) / HC_HIP_BLOCK_SIZE;
  hc_vacuum_kernel_amd_opt<<<blocks, HC_HIP_BLOCK_SIZE, 0, g_ctx.stream>>>(
      g_ctx.d_seeds, g_ctx.d_output, (int)count);

  hipMemcpyAsync(g_ctx.h_pinned_out, g_ctx.d_output, count * 32 * sizeof(uint8_t),
                 hipMemcpyDeviceToHost, g_ctx.stream);

  hipStreamSynchronize(g_ctx.stream);
  memcpy(out_buffer, g_ctx.h_pinned_out, count * 32 * sizeof(uint8_t));

  return 0;
}

static void hip_teardown_impl(void) {
  if (g_ctx.stream)
    hipStreamDestroy(g_ctx.stream);
  if (g_ctx.d_seeds)
    hipFree(g_ctx.d_seeds);
  if (g_ctx.d_output)
    hipFree(g_ctx.d_output);
  if (g_ctx.h_pinned_seeds)
    hipHostFree(g_ctx.h_pinned_seeds);
  if (g_ctx.h_pinned_out)
    hipHostFree(g_ctx.h_pinned_out);
  memset(&g_ctx, 0, sizeof(g_ctx));
}

static hc_backend_t hip_backend = {.backend_name =
                                       "AMD ROCm (HyperCycle Optimized)",
                                   .generate_entropy = hip_generate_impl,
                                   .teardown = hip_teardown_impl};

hc_backend_t *ns_rocm_backend_init(void) {
  int deviceCount = 0;
  if (hipGetDeviceCount(&deviceCount) != hipSuccess || deviceCount == 0)
    return NULL;

  if (hipStreamCreate(&g_ctx.stream) != hipSuccess)
    return NULL;

  size_t startup_size = 1024 * 16;
  hipMalloc(&g_ctx.d_seeds, startup_size * sizeof(uint64_t));
  hipMalloc(&g_ctx.d_output, startup_size * 32 * sizeof(uint8_t));
  hipHostMalloc(&g_ctx.h_pinned_seeds, startup_size * sizeof(uint64_t),
                hipHostMallocDefault);
  hipHostMalloc(&g_ctx.h_pinned_out, startup_size * sizeof(uint8_t),
                hipHostMallocDefault);
  g_ctx.buffer_size = startup_size;

  return &hip_backend;
}

} // extern "C"
