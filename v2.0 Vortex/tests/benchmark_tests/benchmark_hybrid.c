// Benchmark Hybrid X25519
#include "../include/vortex/public/hc_hybrid_x25519.h"
#include "../include/vortex/public/weave_kem.h"
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
  printf("Benchmarking Hybrid X25519...\n");

  uint8_t x_pk[32];
  uint8_t x_sk[32];
  uint8_t q_pk[WEAVE_KEM_PUBLIC_KEY_SIZE];
  uint8_t q_sk[WEAVE_KEM_SECRET_KEY_SIZE];

  const int ITER = 1000;

  // KeyGen
  double start = get_time_us();
  for (int i = 0; i < ITER; i++) {
    hc_hybrid_keygen_x25519(x_pk, x_sk, q_pk, q_sk);
  }
  double end = get_time_us();
  printf("KeyGen: %.3f us\n", (end - start) / ITER);

  return 0;
}
