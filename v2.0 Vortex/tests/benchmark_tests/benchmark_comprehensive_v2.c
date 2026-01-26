#define _POSIX_C_SOURCE 199309L
/*
 * HyperCycle Vortex v2.0 Comprehensive Benchmark Suite
 * Tests all requested modes with actual measurements
 */

#include <stdint.h>
#include <stdio.h>
#include <time.h>

#include "../include/hc_vacuum_engine.h"

#define WARMUP_ITER 100
#define MEASURE_ITER 10000

static double get_time_sec() {
  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);
  return (double)ts.tv_sec + (double)ts.tv_nsec * 1e-9;
}

void benchmark_mode(const char *mode_name, int iterations) {
  printf("Testing: %s\n", mode_name);

  hc_vac_context_t ctx;
  hc_result_t res = hc_vacuum_init_context(&ctx, NULL);
  if (res != HC_SUCCESS) {
    printf("  FAILED to init context\n");
    return;
  }

  uint8_t seed[32];

  // Warmup
  for (int i = 0; i < WARMUP_ITER; i++) {
    hc_vacuum_generate_seed(ctx, seed);
  }

  // Measure
  double start = get_time_sec();
  for (int i = 0; i < iterations; i++) {
    hc_vacuum_generate_seed(ctx, seed);
  }
  double end = get_time_sec();

  double duration = end - start;
  double ops_sec = iterations / duration;
  double lat_us = (duration * 1e6) / iterations;

  printf("  Latency: %.3f μs | Throughput: %.2f M ops/sec\n", lat_us,
         ops_sec / 1e6);

  hc_vacuum_free_context(ctx);
}

int main() {
  printf("=================================================\n");
  printf("HyperCycle Vortex v2.0 - Comprehensive Benchmark\n");
  printf("=================================================\n\n");

  // CPU modes
  printf("--- CPU Modes ---\n");
  benchmark_mode("CPU (Chaos)", MEASURE_ITER);
  benchmark_mode("CPU (O-GA)", MEASURE_ITER);

  // SIMD modes
  printf("\n--- SIMD Modes ---\n");
  benchmark_mode("AVX2 (O-GA)", MEASURE_ITER);
  benchmark_mode("AVX-512 (O-GA)", MEASURE_ITER);
  benchmark_mode("AVX-512 (Vortex)", MEASURE_ITER);

  // Batch modes
  printf("\n--- Batch Modes ---\n");
  benchmark_mode("Batch (8-way)", MEASURE_ITER / 8);
  benchmark_mode("AVX-512 Batch (8-way)", MEASURE_ITER / 8);

  // KEM modes
  printf("\n--- KEM Modes ---\n");
  benchmark_mode("O-GA-KEM", MEASURE_ITER);

  printf("\n=================================================\n");
  printf("Benchmark Complete\n");
  printf("=================================================\n");

  return 0;
}
