// Minimal O-GA KEM Benchmark
#include "../include/vortex/public/hc_oga_kem.h"
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
  printf("Benchmarking O-GA KEM...\n");

  __attribute__((aligned(64))) uint8_t pk[hc_OGA_PUBLIC_KEY_SIZE];
  __attribute__((aligned(64))) uint8_t sk[hc_OGA_SECRET_KEY_SIZE];
  __attribute__((aligned(64))) uint8_t ct[hc_OGA_CIPHERTEXT_SIZE];
  __attribute__((aligned(64))) uint8_t ss[hc_OGA_SHARED_SECRET_SIZE];
  __attribute__((aligned(64))) uint8_t ss2[hc_OGA_SHARED_SECRET_SIZE];

  // KeyGen
  const int ITER = 1000;

  double start = get_time_us();
  for (int i = 0; i < ITER; i++) {
    hc_oga_keypair(pk, sk);
  }
  double end = get_time_us();
  printf("KeyGen: %.3f us\n", (end - start) / ITER);

  // Encaps
  start = get_time_us();
  for (int i = 0; i < ITER; i++) {
    hc_oga_encapsulate(ct, ss, pk);
  }
  end = get_time_us();
  printf("Encaps: %.3f us\n", (end - start) / ITER);

  // Decaps
  start = get_time_us();
  for (int i = 0; i < ITER; i++) {
    hc_oga_decapsulate(ss2, ct, sk);
  }
  end = get_time_us();
  printf("Decaps: %.3f us\n", (end - start) / ITER);

  return 0;
}
