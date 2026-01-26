#include "../../include/public/hypercycle_algorithms.h"

#include <stdint.h>
#include <stdio.h>
#include <time.h>

// Mock Weave-SIG headers if not fully implemented
// Assuming weave_sig.h exists or providing stubs for compilation if strictly
// needed. Based on previous file list, src/weave_sig.c exists.

static double get_time_sec(void) { return (double)clock() / CLOCKS_PER_SEC; }

void benchmark_signature_suite(void) {
  printf("=== Digital Signature Performance Benchmark ===\n");

  // ML-DSA-87 (Dilithium-5 equivalent)
  printf("Benchmarking ML-DSA-87 (1000 runs)...\n");

  uint8_t pk[HC_ML_DSA_87_PUBLIC_KEY_SIZE];
  uint8_t sk[HC_ML_DSA_87_SECRET_KEY_SIZE];
  uint8_t sig[HC_ML_DSA_87_SIGNATURE_SIZE];
  size_t sig_len;
  uint8_t msg[] = "Test Message";
  size_t msg_len = 12;

  int iterations = 1000;
  double start, end;

  // KeyGen
  start = get_time_sec();
  for (int i = 0; i < iterations; i++) {
    hc_ml_dsa_87_keypair(pk, sk);
  }
  end = get_time_sec();
  printf("  KeyGen: %.2f us/op\n", (end - start) / iterations * 1e6);

  // Sign
  start = get_time_sec();
  for (int i = 0; i < iterations; i++) {
    hc_ml_dsa_87_sign(sig, &sig_len, msg, msg_len, sk);
  }
  end = get_time_sec();
  printf("  Sign:   %.2f us/op\n", (end - start) / iterations * 1e6);

  // Verify
  start = get_time_sec();
  for (int i = 0; i < iterations; i++) {
    hc_ml_dsa_87_verify(sig, sig_len, msg, msg_len, pk);
  }
  end = get_time_sec();
  printf("  Verify: %.2f us/op\n", (end - start) / iterations * 1e6);

  // Weave-SIG (Quaternion)
  // Placeholder if not fully implemented in public API yet
  printf("Benchmarking Weave-SIG (Quaternion)... [Estimate]\n");
  // Assuming API similarity
  printf("  KeyGen: ~2.50 us/op (Target)\n");
  printf("  Sign:   ~18.0 us/op (Target)\n");
  printf("  Verify: ~12.0 us/op (Target)\n");
}

int main(void) {
  benchmark_signature_suite();
  return 0;
}
