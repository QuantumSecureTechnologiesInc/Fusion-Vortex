#include "../../include/public/hypercycle_algorithms.h"
#include <stdint.h>
#include <stdio.h>
#include <time.h>
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
  const int ITER = 5000; // iterations for stable average
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

  // Signature benchmark (ML‑DSA‑87)
  uint8_t pk_sig[HC_ML_DSA_87_PUBLIC_KEY_SIZE];
  uint8_t sk_sig[HC_ML_DSA_87_SECRET_KEY_SIZE];
  uint8_t sig[HC_ML_DSA_87_SIGNATURE_SIZE];
  size_t sig_len = 0;
  const uint8_t msg[] = "Benchmark message for HyperCycle";
  const size_t msg_len = sizeof(msg) - 1;

  hc_ml_dsa_87_keypair(pk_sig, sk_sig);
  hc_ml_dsa_87_sign(sig, &sig_len, msg, msg_len, sk_sig);

  // Benchmark sign
  t0 = now_seconds();
  for (int i = 0; i < ITER; ++i) {
    hc_ml_dsa_87_sign(sig, &sig_len, msg, msg_len, sk_sig);
  }
  t1 = now_seconds();
  double sign_time = (t1 - t0) / ITER * 1e6;

  // Benchmark verify
  t0 = now_seconds();
  for (int i = 0; i < ITER; ++i) {
    hc_ml_dsa_87_verify(sig, sig_len, msg, msg_len, pk_sig);
  }
  t1 = now_seconds();
  double verify_time = (t1 - t0) / ITER * 1e6;

  printf("=== HyperCycle v1.0 Genesis Benchmark (average µs) ===\n");
  printf("Keypair      : %.2f µs\n", kp_time);
  printf("Encapsulate : %.2f µs\n", enc_time);
  printf("Decapsulate : %.2f µs\n", dec_time);
  printf("Sign         : %.2f µs\n", sign_time);
  printf("Verify       : %.2f µs\n", verify_time);
  return 0;
}
