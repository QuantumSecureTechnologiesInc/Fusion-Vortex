#include "../../hypercycle.h"
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define ITERATIONS 1000

int main() {
  printf("=== HyperCycle v1.0 Genesis: Benchmark Suite ===\n");
  printf("Engine: Octonion-Geometric Algebra (O-GA-KEM)\n");
  printf("Target: 0.38us / 47 Cycles (ASIC)\n\n");

  hypercycle_engine_t engine;
  if (hypercycle_init(&engine, HYPERKEM_1024) != HYPERCYCLE_SUCCESS) {
    fprintf(stderr, "Failed to init engine\n");
    return 1;
  }

  // Get Sizes
  size_t pk_len, sk_len, ct_len, ss_len;
  hypercycle_get_key_sizes(HYPERKEM_1024, &pk_len, &sk_len, &ct_len, &ss_len);
  printf("[INFO] Key Sizes: PK=%zu, SK=%zu, CT=%zu, SS=%zu\n", pk_len, sk_len,
         ct_len, ss_len);

  uint8_t pk[pk_len], sk[sk_len];
  uint8_t ct[ct_len], ss_enc[ss_len], ss_dec[ss_len];

  // Benchmark KeyGen
  clock_t start = clock();
  for (int i = 0; i < ITERATIONS; i++) {
    hypercycle_keygen(&engine, pk, &pk_len, sk, &sk_len);
  }
  clock_t end = clock();
  double cpu_time_used = ((double)(end - start)) / CLOCKS_PER_SEC;
  printf("[BENCH] KeyGen Avg Time: %.2f us\n",
         (cpu_time_used * 1000000.0) / ITERATIONS);

  // Benchmark Encap
  start = clock();
  for (int i = 0; i < ITERATIONS; i++) {
    hypercycle_encapsulate(&engine, pk, pk_len, ct, &ct_len, ss_enc, &ss_len);
  }
  end = clock();
  cpu_time_used = ((double)(end - start)) / CLOCKS_PER_SEC;
  printf("[BENCH] Encap Avg Time: %.2f us\n",
         (cpu_time_used * 1000000.0) / ITERATIONS);

  // Benchmark Decap
  start = clock();
  for (int i = 0; i < ITERATIONS; i++) {
    hypercycle_decapsulate(&engine, ct, ct_len, sk, sk_len, ss_dec, &ss_len);
  }
  end = clock();
  cpu_time_used = ((double)(end - start)) / CLOCKS_PER_SEC;
  printf("[BENCH] Decap Avg Time: %.2f us\n",
         (cpu_time_used * 1000000.0) / ITERATIONS);

  // Verify
  hypercycle_metrics_t metrics;
  hypercycle_get_metrics(&engine, &metrics);
  printf("\n[METRICS] Total Ops: %llu\n", metrics.total_operations);
  printf("[METRICS] 47 Cycles Achieved: %s\n",
         metrics.cycles_47_achieved ? "YES" : "NO");

  hypercycle_cleanup(&engine);
  return 0;
}
