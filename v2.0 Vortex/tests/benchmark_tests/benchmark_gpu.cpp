/**
 * @file benchmark_gpu.c
 * @brief Standalone runner for GPU-accelerated entropy generation
 */

#include "../../include/public/hc_core.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// Forward declaration of backend init (linked statically for this benchmark)
// In plugin architecture, this would be loaded via dlsym
#ifdef __cplusplus
extern "C" {
#endif
extern hc_backend_t *ns_cuda_backend_init(void);
#ifdef __cplusplus
}
#endif

static double get_time_sec() { return (double)clock() / CLOCKS_PER_SEC; }

int main() {
  printf("╔════════════════════════════════════════════════════════╗\n");
  printf("║   HyperCycle v1.0 Genesis - GPU Benchmark             ║\n");
  printf("╚════════════════════════════════════════════════════════╝\n");

  // Initialize NVIDIA Backend
  printf("[Init] Loading NVIDIA CUDA Backend...\n");
  hc_backend_t *backend = ns_cuda_backend_init();

  if (!backend) {
    fprintf(stderr,
            "ERROR: Failed to initialize backend. Check GPU/Drivers.\n");
    return 1;
  }

  printf("       Loaded: %s\n", backend->backend_name);

  // Benchmark Parameters
  const size_t BATCH_SIZE = 1024 * 1024; // 1 Million Keys per batch
  const int ITERATIONS = 100;

  // Allocate Host Memory
  uint64_t *seeds = (uint64_t *)malloc(BATCH_SIZE * sizeof(uint64_t));
  uint8_t *keys = (uint8_t *)malloc(BATCH_SIZE * sizeof(uint8_t));

  // Initialize Seeds
  // Avoid 0^0 degeneracy and ensure high hamming distance
  for (size_t i = 0; i < BATCH_SIZE; i++)
    seeds[i] = (uint64_t)i * 6364136223846793005ULL + 1442695040888963407ULL;

  // Warmup (Context Creation cost)
  printf("[Warmup] Generating 1M keys to prime GPU...\n");
  backend->generate_entropy(seeds, keys, BATCH_SIZE);

  // Benchmark Loop
  printf("[Bench] Running %d iterations of %zu keys...\n", ITERATIONS,
         BATCH_SIZE);

  double start_time = get_time_sec();

  for (int i = 0; i < ITERATIONS; i++) {
    backend->generate_entropy(seeds, keys, BATCH_SIZE);
  }

  double end_time = get_time_sec();
  double total_time = end_time - start_time;

  // Metrics
  double total_keys = (double)BATCH_SIZE * ITERATIONS;
  double keys_per_sec = total_keys / total_time;
  double throughput_gbps =
      keys_per_sec * 32.0 / 1e9; // 32 bytes/key (assumption? No, vacuum kernel
                                 // outputs 1 byte per seed per cycle? Wait.)
  // hc_vacuum_kernel_opt outputs 1 byte per thread.
  // The struct said "keys_out[tid] = (uint8_t)(state >> 56);"
  // So 1 byte per seed.
  // Throughput in MB/s of entropy.

  printf("\n=== Results ===\n");
  printf("Total Time:      %.4f seconds\n", total_time);
  printf("Throughput:      %.2f Million Bytes/sec\n", keys_per_sec / 1e6);
  printf("Latency (Batch): %.4f ms\n", (total_time / ITERATIONS) * 1000.0);

  // Check output
  if (keys[0] == 0 && keys[1] == 0) {
    printf("WARNING: Output seems empty (check implementation)\n");
  } else {
    printf("Verification: First bytes = %02X %02X %02X %02X\n", keys[0],
           keys[1], keys[2], keys[3]);
  }

  // Teardown
  backend->teardown();
  free(seeds);
  free(keys);

  return 0;
}
