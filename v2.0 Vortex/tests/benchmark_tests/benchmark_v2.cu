/*
 * HyperCycle Vortex v2.0 Benchmark Suite
 * Covers AVX-512 CPU Engine and CUDA GPU Backend
 */

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Include Vortex headers
#include "../include/hc_gpu_universal.h"
#include "../include/hc_vacuum_engine.h"

// Define direct access to backend for CUDA (since we compile together)
extern "C" const hc_gpu_backend_t *hc_get_gpu_backend(void);

#define BATCH_SIZE 1000000
#define WARMUP_ITER 10
#define MEASURE_ITER 50

static double get_time_sec() {
  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);
  return (double)ts.tv_sec + (double)ts.tv_nsec * 1e-9;
}

void benchmark_cpu() {
  printf("--- CPU Benchmark (Vacuum Engine) ---\n");
  hc_vac_context_t ctx;
  hc_result_t res = hc_vacuum_init_context(&ctx, NULL);
  if (res != HC_SUCCESS) {
    printf("Failed to init CPU context: %d\n", res);
    return;
  }

  uint8_t seed[32];

  // Warmup
  for (int i = 0; i < 100; i++) {
    hc_vacuum_generate_seed(ctx, seed);
  }

  double start = get_time_sec();
  int iterations = 100000;
  for (int i = 0; i < iterations; i++) {
    hc_vacuum_generate_seed(ctx, seed);
  }
  double end = get_time_sec();

  double duration = end - start;
  double ops_sec = iterations / duration;
  double lat_us = (duration * 1e6) / iterations;

  printf("CPU AVX-512 Throughput: %.2f ops/sec\n", ops_sec);
  printf("CPU AVX-512 Latency:    %.3f us/key\n", lat_us);

  hc_vacuum_free_context(ctx);
}

void benchmark_gpu() {
  printf("\n--- GPU Benchmark (CUDA) ---\n");

  const hc_gpu_backend_t *backend = hc_get_gpu_backend();
  if (!backend) {
    printf("CUDA backend not found.\n");
    return;
  }

  printf("Backend: %s\n", backend->name);

  hc_context_t ctx;
  hc_context_config_t config = {0};
  config.device_id = 0;

  if (backend->init_context(&ctx, &config) != HC_GPU_SUCCESS) {
    printf("Failed to init GPU context (no device?)\n");
    return;
  }

  size_t count = BATCH_SIZE;
  uint64_t *seeds = (uint64_t *)malloc(count * sizeof(uint64_t));
  uint8_t *out = (uint8_t *)malloc(count * 32);

  // Init host buffers
  for (size_t i = 0; i < count; i++)
    seeds[i] = i;

  // Warmup
  backend->generate_batch(ctx, seeds, NULL, out, count, 0);
  backend->sync(ctx);

  double start = get_time_sec();
  for (int i = 0; i < MEASURE_ITER; i++) {
    backend->generate_batch(ctx, seeds, NULL, out, count, 0);
  }
  backend->sync(ctx);
  double end = get_time_sec();

  double total_keys = (double)count * MEASURE_ITER;
  double duration = end - start;
  double throughput =
      (total_keys * 32.0) / (1024.0 * 1024.0) / duration; // MB/s
  double ops_sec = total_keys / duration;

  printf("GPU Batch Size: %zu\n", count);
  printf("GPU Throughput: %.2f MB/s\n", throughput);
  printf("GPU Rate:       %.2f M keys/sec\n", ops_sec / 1e6);

  // Verify Output (Basic check)
  // First byte of first key should not be 00 all the time (entropy check)
  printf("First Key Sample: %02X %02X %02X %02X ...\n", out[0], out[1], out[2],
         out[3]);

  free(seeds);
  free(out);
  backend->free_context(ctx);
}

int main() {
  printf("HyperCycle Vortex v2.0 Benchmark\n");
  printf("================================\n");
  benchmark_cpu();
  benchmark_gpu();
  return 0;
}
