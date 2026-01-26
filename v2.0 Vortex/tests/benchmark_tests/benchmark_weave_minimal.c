// Minimal Weave KEM Benchmark
#include "vortex/public/weave_kem.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <windows.h>

// High-resolution Windows timer
static inline double get_time_us() {
  LARGE_INTEGER freq, counter;
  QueryPerformanceFrequency(&freq);
  QueryPerformanceCounter(&counter);
  return (double)counter.QuadPart / (double)freq.QuadPart * 1e6;
}

int main() {
  printf("Benchmarking Weave KEM...\n");

  // Warmup
  hc_kem_keypair_t kp;
  hc_ciphertext_t ct;
  hc_shared_secret_t ss, ss2;

  // Note: We ignore return codes for simplicity in minimal test, assuming lib
  // works if compiled
  hc_kem_keygen(&kp);

  const int ITER = 1000;
  double start = get_time_us();
  for (int i = 0; i < ITER; i++) {
    hc_kem_keygen(&kp);
  }
  double end = get_time_us();
  printf("KeyGen: %.3f us\n", (end - start) / ITER);

  start = get_time_us();
  for (int i = 0; i < ITER; i++) {
    hc_kem_encaps(&kp, &ct, &ss);
  }
  end = get_time_us();
  printf("Encaps: %.3f us\n", (end - start) / ITER);

  start = get_time_us();
  for (int i = 0; i < ITER; i++) {
    hc_kem_decaps(&kp, &ct, &ss2);
  }
  end = get_time_us();
  printf("Decaps: %.3f us\n", (end - start) / ITER);

  return 0;
}
