#include "../include/public/hypercycle_algorithms.h"
#include "public/weave_kem.h"
#include <stdint.h>
#include <stdio.h>
#include <time.h>

// Helper for timing
static double get_time_sec(void) { return (double)clock() / CLOCKS_PER_SEC; }

void benchmark_kem_suite(void) {
  printf("=== KEM Performance Benchmark ===\n");

  uint8_t pk[3168], sk[3168], ct[1568], ss[32];
  int iterations = 1000;

  double start, end;

  printf("Benchmarking ML-KEM-1024 (%d runs)...\n", iterations);
  start = get_time_sec();
  for (int i = 0; i < iterations; i++) {
    hc_ml_kem_1024_keypair(pk, sk);
  }
  end = get_time_sec();
  printf("  KeyGen: %.2f us/op\n", (end - start) / iterations * 1e6);

  // Weave-KEM
  printf("Benchmarking Weave-KEM (Quaternion-Chaos)...\n");
  hc_kem_keypair_t weave_kp;
  hc_ciphertext_t weave_ct;
  hc_shared_secret_t weave_ss;

  start = get_time_sec();
  for (int i = 0; i < iterations; i++) {
    hc_kem_keygen(&weave_kp);
  }
  end = get_time_sec();
  printf("  KeyGen: %.2f us/op\n", (end - start) / iterations * 1e6);

  start = get_time_sec();
  for (int i = 0; i < iterations; i++) {
    hc_kem_encaps(&weave_kp, &weave_ct, &weave_ss);
  }
  end = get_time_sec();
  printf("  Encaps: %.2f us/op\n", (end - start) / iterations * 1e6);
}

int main(void) {
  benchmark_kem_suite();
  return 0;
}
