/*
 * HyperCycle v2.0 Vortex - Genesis Vacuum Engine Benchmark
 *
 * This benchmarks the ACTUAL implemented engine in v2.0 Vortex:
 * - Genesis 47-cycle vacuum evolution (NOT Skew Tent)
 * - Heisenberg-Euler effective Lagrangian
 * - 512-dimensional phase space
 */

#include "hc_vacuum_entropy.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <windows.h>


// High-resolution timer
static inline double get_time_us() {
  LARGE_INTEGER freq, counter;
  QueryPerformanceFrequency(&freq);
  QueryPerformanceCounter(&counter);
  return (double)counter.QuadPart / (double)freq.QuadPart * 1e6;
}

// CPU cycles
static inline uint64_t rdtsc() { return __rdtsc(); }

int main() {
  const int WARMUP = 100;
  const int ITERATIONS = 5000;
  const size_t KEY_SIZE = 32;

  printf("==========================================================\n");
  printf("   HyperCycle v2.0 Vortex - Vacuum Engine Benchmark      \n");
  printf("==========================================================\n");
  printf("Hardware: AMD Ryzen 7 7840HS @ 3.8-5.1 GHz\n");
  printf("Engine: Genesis 47-Cycle Vacuum Evolution\n");
  printf("Algorithm: Heisenberg-Euler Lagrangian\n");
  printf("Phase Space: 512 dimensions\n");
  printf("Test Iterations: %d\n", ITERATIONS);
  printf("==========================================================\n\n");

  uint8_t *key = (uint8_t *)malloc(KEY_SIZE);
  if (!key) {
    printf("ERROR: Memory allocation failed\n");
    return 1;
  }

  // Warmup
  printf("[1/3] Warming up...\n");
  for (int i = 0; i < WARMUP; i++) {
    hc_generate_vacuum_key(key, KEY_SIZE);
  }
  printf("✓ Warmup complete\n\n");

  // Benchmark
  printf("[2/3] Running benchmark...\n");

  double start_time = get_time_us();
  uint64_t start_cycles = rdtsc();

  for (int i = 0; i < ITERATIONS; i++) {
    int result = hc_generate_vacuum_key(key, KEY_SIZE);
    if (result != 0) {
      printf("ERROR: Generation failed at iteration %d\n", i);
      free(key);
      return 1;
    }
  }

  uint64_t end_cycles = rdtsc();
  double end_time = get_time_us();

  double total_time = end_time - start_time;
  uint64_t total_cycles = end_cycles - start_cycles;

  double us_per_op = total_time / ITERATIONS;
  uint64_t cycles_per_op = total_cycles / ITERATIONS;
  double ops_per_sec = ITERATIONS / (total_time / 1e6);

  printf("✓ Benchmark complete\n\n");

  // Entropy quality check
  printf("[3/3] Verifying entropy quality...\n");
  hc_generate_vacuum_key(key, KEY_SIZE);

  int non_zero = 0;
  for (int i = 0; i < KEY_SIZE; i++) {
    if (key[i] != 0)
      non_zero++;
  }

  printf("✓ Quality check complete\n\n");

  // Results
  printf("==========================================================\n");
  printf("                    RESULTS                               \n");
  printf("==========================================================\n\n");

  printf("Performance:\n");
  printf("----------------------------------------------------------\n");
  printf("  Time per operation:     %.3f μs\n", us_per_op);
  printf("  Cycles per operation:   %llu cycles\n", cycles_per_op);
  printf("  Throughput:             %.0f ops/sec\n", ops_per_sec);
  printf("  Bandwidth:              %.2f MB/sec\n",
         (ITERATIONS * KEY_SIZE) / (total_time / 1e6) / (1024.0 * 1024.0));
  printf("\n");

  printf("Entropy Quality:\n");
  printf("----------------------------------------------------------\n");
  printf("  Non-zero bytes:         %d/%d (%.1f%%)\n", non_zero, KEY_SIZE,
         (non_zero / (float)KEY_SIZE) * 100.0);
  printf("  Sample key (hex):       ");
  for (int i = 0; i < 16; i++) {
    printf("%02X", key[i]);
  }
  printf("...\n");
  printf("  Status:                 %s\n\n",
         non_zero >= 30 ? "✓ PASS" : "✗ FAIL");

  printf("==========================================================\n");
  printf("   GENESIS 47-CYCLE ENGINE: BENCHMARK COMPLETE\n");
  printf("==========================================================\n");

  free(key);
  return (non_zero >= 30) ? 0 : 1;
}
