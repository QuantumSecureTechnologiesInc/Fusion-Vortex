// Benchmark ML-KEM-1024
#include "../include/vortex/public/hypercycle_algorithms.h"
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
  printf("Benchmarking ML-KEM-1024...\n");

  uint8_t pk[hc_ML_KEM_1024_PUBLIC_KEY_SIZE];
  uint8_t sk[hc_ML_KEM_1024_SECRET_KEY_SIZE];
  uint8_t ct[hc_ML_KEM_1024_CIPHERTEXT_SIZE];
  uint8_t ss[hc_ML_KEM_1024_SHARED_SECRET_SIZE];
  uint8_t ss2[hc_ML_KEM_1024_SHARED_SECRET_SIZE];

  const int ITER = 1000;

  // KeyGen
  double start = get_time_us();
  for (int i = 0; i < ITER; i++) {
    hc_ml_kem_1024_keypair(pk, sk);
  }
  double end = get_time_us();
  printf("KeyGen: %.3f us\n", (end - start) / ITER);

  // Encaps
  start = get_time_us();
  for (int i = 0; i < ITER; i++) {
    hc_ml_kem_1024_encapsulate(ct, ss, pk);
  }
  end = get_time_us();
  printf("Encaps: %.3f us\n", (end - start) / ITER);

  // Decaps
  start = get_time_us();
  for (int i = 0; i < ITER; i++) {
    hc_ml_kem_1024_decapsulate(ss2, ct, sk);
  }
  end = get_time_us();
  printf("Decaps: %.3f us\n", (end - start) / ITER);

  return 0;
}
