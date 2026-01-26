#include "../include/public/hypercycle_algorithms.h"
#include <stdint.h>
#include <stdio.h>
#ifndef _WIN32
#include <time.h>
#endif
#ifdef _WIN32
#include <windows.h>
#endif

// High‑resolution timer (Windows & POSIX)
static double now_seconds(void) {
#ifdef _WIN32
  LARGE_INTEGER freq, cnt;
  QueryPerformanceFrequency(&freq);
  QueryPerformanceCounter(&cnt);
  return (double)cnt.QuadPart / (double)freq.QuadPart;
#else
  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);
  return ts.tv_sec + ts.tv_nsec * 1e-9;
#endif
}

int main(void) {
  printf("=== HyperCycle v3.2 Legacy (Ref Baseline) ===\n");

  const int ITER = 5000;
  uint8_t pk[HC_ML_KEM_1024_PUBLIC_KEY_SIZE];
  uint8_t sk[HC_ML_KEM_1024_SECRET_KEY_SIZE];
  uint8_t ct[HC_ML_KEM_1024_CIPHERTEXT_SIZE];
  uint8_t ss[HC_ML_KEM_1024_SHARED_SECRET_SIZE];

  // Warm‑up
  hc_ml_kem_1024_keypair(pk, sk);
  hc_ml_kem_1024_encapsulate(ct, ss, pk);
  hc_ml_kem_1024_decapsulate(ss, ct, sk);

  // Benchmark keypair generation
  double t0 = now_seconds();
  for (int i = 0; i < ITER; ++i) {
    hc_ml_kem_1024_keypair(pk, sk);
  }
  double t1 = now_seconds();
  double kp_time = (t1 - t0) / ITER * 1e6; // µs per operation

  // Benchmark encapsulation
  t0 = now_seconds();
  for (int i = 0; i < ITER; ++i) {
    hc_ml_kem_1024_encapsulate(ct, ss, pk);
  }
  t1 = now_seconds();
  double enc_time = (t1 - t0) / ITER * 1e6;

  // Benchmark decapsulation
  t0 = now_seconds();
  for (int i = 0; i < ITER; ++i) {
    hc_ml_kem_1024_decapsulate(ss, ct, sk);
  }
  t1 = now_seconds();
  double dec_time = (t1 - t0) / ITER * 1e6;

  printf("[Legacy] Keypair      : %.2f µs\n", kp_time);
  printf("[Legacy] Encapsulate  : %.2f µs\n", enc_time);
  printf("[Legacy] Decapsulate  : %.2f µs\n", dec_time);

  return 0;
}
