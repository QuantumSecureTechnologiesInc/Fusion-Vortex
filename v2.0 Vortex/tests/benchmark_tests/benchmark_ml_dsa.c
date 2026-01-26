// Benchmark ML-DSA-87
#include "vortex/public/hypercycle_algorithms.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <windows.h>

static inline double get_time_us() {
  LARGE_INTEGER freq, counter;
  QueryPerformanceFrequency(&freq);
  QueryPerformanceCounter(&counter);
  return (double)counter.QuadPart / (double)freq.QuadPart * 1e6;
}

int main() {
  printf("Benchmarking ML-DSA-87...\n");

  uint8_t pk[hc_ML_DSA_87_PUBLIC_KEY_SIZE];
  uint8_t sk[hc_ML_DSA_87_SECRET_KEY_SIZE];
  uint8_t sig[hc_ML_DSA_87_SIGNATURE_SIZE];
  size_t sig_len;
  uint8_t msg[] = "Test Message for Signing";

  const int ITER = 1000;

  // KeyGen
  double start = get_time_us();
  for (int i = 0; i < ITER; i++) {
    hc_ml_dsa_87_keypair(pk, sk);
  }
  double end = get_time_us();
  printf("KeyGen: %.3f us\n", (end - start) / ITER);

  // Sign
  start = get_time_us();
  for (int i = 0; i < ITER; i++) {
    hc_ml_dsa_87_sign(sig, &sig_len, msg, sizeof(msg), sk);
  }
  end = get_time_us();
  printf("Sign: %.3f us\n", (end - start) / ITER);

  // Verify
  start =
      get_time_us(); // Verify is usually slower or faster depending on scheme
  for (int i = 0; i < ITER; i++) {
    hc_ml_dsa_87_verify(sig, sig_len, msg, sizeof(msg), pk);
  }
  end = get_time_us();
  printf("Verify: %.3f us\n", (end - start) / ITER);

  return 0;
}
