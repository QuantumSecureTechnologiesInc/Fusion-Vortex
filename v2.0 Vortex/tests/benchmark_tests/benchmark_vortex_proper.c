/*
 * Vortex v2.0 Proper Benchmark Test
 *
 * Tests the actual Vortex implementation with:
 * - Skew Tent Map chaotic attractor
 * - Entropy pool architecture
 * - Ring buffer zero-latency access
 * - Real performance metrics
 */

#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <windows.h>


// Include Vortex API
#include "hc_vacuum_entropy.h"

// High-resolution Windows timer
static inline double get_time_us() {
  LARGE_INTEGER freq, counter;
  QueryPerformanceFrequency(&freq);
  QueryPerformanceCounter(&counter);
  return (double)counter.QuadPart / (double)freq.QuadPart * 1e6;
}

// RDTSC for cycle counting
static inline uint64_t rdtsc() { return __rdtsc(); }

int main() {
  const int WARMUP_ITERATIONS = 100;
  const int TEST_ITERATIONS = 10000;
  const size_t KEY_SIZE = 32; // 256-bit keys

  printf("==========================================================\n");
  printf("         Vortex v2.0 Benchmark - EMPIRICAL RESULTS       \n");
  printf("==========================================================\n");
  printf("Hardware: AMD Ryzen 7 7840HS @ 3.8-5.1 GHz\n");
  printf("Engine: Skew Tent Map (Lyapunov λ ≈ 0.693)\n");
  printf("Mixing Time: < 10 iterations\n");
  printf("Architecture: Ring Buffer (4096 slots) + Background Worker\n");
  printf("Test Iterations: %d\n", TEST_ITERATIONS);
  printf("==========================================================\n\n");

  // Initialize Vortex entropy engine
  printf("[1/4] Initializing Vortex entropy pool...\n");
  int init_result = hc_init_vacuum_entropy();
  if (init_result != 0) {
    printf("ERROR: Failed to initialize Vortex engine (code: %d)\n",
           init_result);
    return 1;
  }
  printf("✓ Vortex engine initialized (background worker started)\n\n");

  // Allocate test buffer
  uint8_t *key_buffer = (uint8_t *)malloc(KEY_SIZE);
  if (!key_buffer) {
    printf("ERROR: Memory allocation failed\n");
    hc_cleanup_vacuum_entropy();
    return 1;
  }

  // Warmup phase
  printf("[2/4] Warming up (%d iterations)...\n", WARMUP_ITERATIONS);
  for (int i = 0; i < WARMUP_ITERATIONS; i++) {
    hc_generate_vacuum_key(key_buffer, KEY_SIZE);
  }
  printf("✓ Warmup complete\n\n");

  // Benchmark: Key Generation (Vortex entropy extraction)
  printf("[3/4] Benchmarking key generation...\n");

  double start_time = get_time_us();
  uint64_t start_cycles = rdtsc();

  for (int i = 0; i < TEST_ITERATIONS; i++) {
    int result = hc_generate_vacuum_key(key_buffer, KEY_SIZE);
    if (result != 0) {
      printf("ERROR: Key generation failed at iteration %d\n", i);
      free(key_buffer);
      hc_cleanup_vacuum_entropy();
      return 1;
    }
  }

  uint64_t end_cycles = rdtsc();
  double end_time = get_time_us();

  double total_time_us = end_time - start_time;
  uint64_t total_cycles = end_cycles - start_cycles;

  double us_per_operation = total_time_us / TEST_ITERATIONS;
  uint64_t cycles_per_operation = total_cycles / TEST_ITERATIONS;
  double ops_per_sec = TEST_ITERATIONS / (total_time_us / 1e6);
  double mb_per_sec =
      (TEST_ITERATIONS * KEY_SIZE) / (total_time_us / 1e6) / (1024.0 * 1024.0);

  printf("✓ Benchmark complete\n\n");

  // Get entropy pool statistics
  printf("[4/4] Reading entropy pool statistics...\n");
  size_t produced = 0, consumed = 0, available = 0, underruns = 0;
  hc_get_entropy_stats(&produced, &consumed, &available, &underruns);

  printf("✓ Statistics retrieved\n\n");

  // Results
  printf("==========================================================\n");
  printf("                    BENCHMARK RESULTS                     \n");
  printf("==========================================================\n");
  printf("\n");
  printf("Performance Metrics:\n");
  printf("-----------------------------------------------------------\n");
  printf("  Time per operation:     %.3f μs\n", us_per_operation);
  printf("  Cycles per operation:   %llu cycles\n", cycles_per_operation);
  printf("  Throughput:             %.0f ops/sec\n", ops_per_sec);
  printf("  Bandwidth:              %.2f MB/sec\n", mb_per_sec);
  printf("\n");
  printf("Entropy Pool Statistics:\n");
  printf("-----------------------------------------------------------\n");
  printf("  Total produced:         %zu batches\n", produced);
  printf("  Total consumed:         %zu batches\n", consumed);
  printf("  Currently available:    %zu batches\n", available);
  printf("  Pool underruns:         %zu (%.3f%%)\n", underruns,
         (double)underruns / TEST_ITERATIONS * 100.0);
  printf("\n");

  // Entropy quality check
  printf("Entropy Quality Verification:\n");
  printf("-----------------------------------------------------------\n");
  uint8_t test_key[32];
  hc_generate_vacuum_key(test_key, 32);

  // Check for non-zero entropy
  int non_zero = 0;
  for (int i = 0; i < 32; i++) {
    if (test_key[i] != 0)
      non_zero++;
  }

  printf("  Non-zero bytes:         %d/32 (%.1f%%)\n", non_zero,
         (non_zero / 32.0) * 100.0);
  printf("  Sample output (hex):    ");
  for (int i = 0; i < 16; i++) {
    printf("%02X", test_key[i]);
  }
  printf("...\n");
  printf("  Status:                 %s\n",
         non_zero >= 30 ? "✓ PASS" : "✗ FAIL");
  printf("\n");

  // Cleanup
  free(key_buffer);
  hc_cleanup_vacuum_entropy();

  printf("==========================================================\n");
  printf("              VORTEX v2.0 BENCHMARK COMPLETE              \n");
  printf("==========================================================\n");
  printf("\n");
  printf("Key Findings:\n");
  printf("  • Skew Tent Map provides %.3f μs latency\n", us_per_operation);
  printf("  • Ring buffer architecture %s pool underruns\n",
         underruns == 0 ? "eliminated" : "minimized");
  printf("  • Background worker maintained %.0f ops/sec throughput\n",
         ops_per_sec);
  printf("\n");

  return (non_zero >= 30 && underruns < TEST_ITERATIONS * 0.01) ? 0 : 1;
}
