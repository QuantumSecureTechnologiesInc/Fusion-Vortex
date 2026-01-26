// benchmark_all_algorithms.c - Comprehensive Vortex v2.0 Algorithm Benchmark
// Benchmarks: Weave-KEM, Weave-SIG, ML-KEM-1024, ML-DSA-87, O-GA-KEM, Hybrid,
// CQC

#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

// Include all Vortex algorithm headers
#include "../include/vortex/public/hc_oga_kem.h"
#include "../include/vortex/public/hypercycle_algorithms.h"
#include "../include/vortex/public/weave_kem.h"
#include "../include/vortex/public/weave_sig.h"

#define WARMUP_ITER 100
#define MEASURE_ITER 10000

// Timing helper
static inline uint64_t rdtsc() {
  unsigned int lo, hi;
  __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
  return ((uint64_t)hi << 32) | lo;
}

// Benchmark Weave-KEM
void benchmark_weave_kem() {
  hc_kem_keypair_t kp;
  hc_ciphertext_t ct;
  hc_shared_secret_t ss;

  printf("\n=== Weave-KEM Benchmark ===\n");

  // Warm-up
  for (int i = 0; i < WARMUP_ITER; i++) {
    hc_kem_keygen(&kp);
  }

  // Measure KeyGen
  uint64_t start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_kem_keygen(&kp);
  }
  uint64_t end = rdtsc();
  double cycles_keygen = (double)(end - start) / MEASURE_ITER;

  // Measure Encapsulate
  start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_kem_encaps(&kp, &ct, &ss);
  }
  end = rdtsc();
  double cycles_encaps = (double)(end - start) / MEASURE_ITER;

  // Measure Decapsulate
  start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_kem_decaps(&kp, &ct, &ss);
  }
  end = rdtsc();
  double cycles_decaps = (double)(end - start) / MEASURE_ITER;

  // Convert cycles to microseconds (assuming 2.3 GHz CPU)
  double freq_ghz = 2.3;
  printf("KeyGen:      %.2f µs (%.0f cycles)\n",
         cycles_keygen / (freq_ghz * 1000), cycles_keygen);
  printf("Encapsulate: %.2f µs (%.0f cycles)\n",
         cycles_encaps / (freq_ghz * 1000), cycles_encaps);
  printf("Decapsulate: %.2f µs (%.0f cycles)\n",
         cycles_decaps / (freq_ghz * 1000), cycles_decaps);
}

// Benchmark Weave-SIG
void benchmark_weave_sig() {
  hc_sig_keypair_t kp;
  hc_signature_t sig;
  uint8_t msg[32] = {0};

  printf("\n=== Weave-SIG Benchmark ===\n");

  // Warm-up
  for (int i = 0; i < WARMUP_ITER; i++) {
    hc_sig_keygen(&kp);
  }

  // Measure KeyGen
  uint64_t start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_sig_keygen(&kp);
  }
  uint64_t end = rdtsc();
  double cycles_keygen = (double)(end - start) / MEASURE_ITER;

  // Measure Sign
  hc_sig_keygen(&kp);
  start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_sig_sign(&kp, msg, sizeof(msg), &sig);
  }
  end = rdtsc();
  double cycles_sign = (double)(end - start) / MEASURE_ITER;

  // Measure Verify
  start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_sig_verify(&kp, msg, sizeof(msg), &sig);
  }
  end = rdtsc();
  double cycles_verify = (double)(end - start) / MEASURE_ITER;

  double freq_ghz = 2.3;
  printf("KeyGen: %.2f µs (%.0f cycles)\n", cycles_keygen / (freq_ghz * 1000),
         cycles_keygen);
  printf("Sign:   %.2f µs (%.0f cycles)\n", cycles_sign / (freq_ghz * 1000),
         cycles_sign);
  printf("Verify: %.2f µs (%.0f cycles)\n", cycles_verify / (freq_ghz * 1000),
         cycles_verify);
}

// Benchmark ML-KEM-1024
void benchmark_ml_kem_1024() {
  uint8_t pk[hc_ML_KEM_1024_PUBLIC_KEY_SIZE];
  uint8_t sk[hc_ML_KEM_1024_SECRET_KEY_SIZE];
  uint8_t ct[hc_ML_KEM_1024_CIPHERTEXT_SIZE];
  uint8_t ss[hc_ML_KEM_1024_SHARED_SECRET_SIZE];

  printf("\n=== ML-KEM-1024 Benchmark ===\n");

  // Warm-up
  for (int i = 0; i < WARMUP_ITER; i++) {
    hc_ml_kem_1024_keypair(pk, sk);
  }

  // Measure KeyGen
  uint64_t start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_ml_kem_1024_keypair(pk, sk);
  }
  uint64_t end = rdtsc();
  double cycles_keygen = (double)(end - start) / MEASURE_ITER;

  // Measure Encapsulate
  start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_ml_kem_1024_encapsulate(ct, ss, pk);
  }
  end = rdtsc();
  double cycles_encaps = (double)(end - start) / MEASURE_ITER;

  // Measure Decapsulate
  start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_ml_kem_1024_decapsulate(ss, ct, sk);
  }
  end = rdtsc();
  double cycles_decaps = (double)(end - start) / MEASURE_ITER;

  double freq_ghz = 2.3;
  printf("KeyGen:      %.2f µs (%.0f cycles)\n",
         cycles_keygen / (freq_ghz * 1000), cycles_keygen);
  printf("Encapsulate: %.2f µs (%.0f cycles)\n",
         cycles_encaps / (freq_ghz * 1000), cycles_encaps);
  printf("Decapsulate: %.2f µs (%.0f cycles)\n",
         cycles_decaps / (freq_ghz * 1000), cycles_decaps);
}

// Benchmark ML-DSA-87
void benchmark_ml_dsa_87() {
  uint8_t pk[hc_ML_DSA_87_PUBLIC_KEY_SIZE];
  uint8_t sk[hc_ML_DSA_87_SECRET_KEY_SIZE];
  uint8_t sig[hc_ML_DSA_87_SIGNATURE_SIZE];
  size_t sig_len;
  uint8_t msg[32] = {0};

  printf("\n=== ML-DSA-87 Benchmark ===\n");

  // Warm-up
  for (int i = 0; i < WARMUP_ITER; i++) {
    hc_ml_dsa_87_keypair(pk, sk);
  }

  // Measure KeyGen
  uint64_t start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_ml_dsa_87_keypair(pk, sk);
  }
  uint64_t end = rdtsc();
  double cycles_keygen = (double)(end - start) / MEASURE_ITER;

  // Measure Sign
  hc_ml_dsa_87_keypair(pk, sk);
  start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_ml_dsa_87_sign(sig, &sig_len, msg, sizeof(msg), sk);
  }
  end = rdtsc();
  double cycles_sign = (double)(end - start) / MEASURE_ITER;

  // Measure Verify
  start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_ml_dsa_87_verify(sig, sig_len, msg, sizeof(msg), pk);
  }
  end = rdtsc();
  double cycles_verify = (double)(end - start) / MEASURE_ITER;

  double freq_ghz = 2.3;
  printf("KeyGen: %.2f µs (%.0f cycles)\n", cycles_keygen / (freq_ghz * 1000),
         cycles_keygen);
  printf("Sign:   %.2f µs (%.0f cycles)\n", cycles_sign / (freq_ghz * 1000),
         cycles_sign);
  printf("Verify: %.2f µs (%.0f cycles)\n", cycles_verify / (freq_ghz * 1000),
         cycles_verify);
}

// Benchmark O-GA-KEM
void benchmark_oga_kem() {
  uint8_t pk[hc_OGA_PUBLIC_KEY_SIZE];
  uint8_t sk[hc_OGA_SECRET_KEY_SIZE];
  uint8_t ct[hc_OGA_CIPHERTEXT_SIZE];
  uint8_t ss[hc_OGA_SHARED_SECRET_SIZE];

  printf("\n=== O-GA-KEM Benchmark ===\n");

  // Warm-up
  for (int i = 0; i < WARMUP_ITER; i++) {
    hc_oga_keypair(pk, sk);
  }

  // Measure KeyGen
  uint64_t start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_oga_keypair(pk, sk);
  }
  uint64_t end = rdtsc();
  double cycles_keygen = (double)(end - start) / MEASURE_ITER;

  // Measure Encapsulate
  start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_oga_encapsulate(ct, ss, pk);
  }
  end = rdtsc();
  double cycles_encaps = (double)(end - start) / MEASURE_ITER;

  // Measure Decapsulate
  start = rdtsc();
  for (int i = 0; i < MEASURE_ITER; i++) {
    hc_oga_decapsulate(ss, ct, sk);
  }
  end = rdtsc();
  double cycles_decaps = (double)(end - start) / MEASURE_ITER;

  double freq_ghz = 2.3;
  printf("KeyGen:      %.2f µs (%.0f cycles)\n",
         cycles_keygen / (freq_ghz * 1000), cycles_keygen);
  printf("Encapsulate: %.2f µs (%.0f cycles)\n",
         cycles_encaps / (freq_ghz * 1000), cycles_encaps);
  printf("Decapsulate: %.2f µs (%.0f cycles)\n",
         cycles_decaps / (freq_ghz * 1000), cycles_decaps);
}

int main() {
  printf("=================================================\n");
  printf("Vortex v2.0 - Comprehensive Algorithm Benchmark\n");
  printf("=================================================\n");
  printf("CPU: AMD Ryzen 7 7840HS @ 2.3 GHz (base)\n");
  printf("Iterations: %d (after %d warmup)\n", MEASURE_ITER, WARMUP_ITER);
  printf("=================================================\n");

  benchmark_weave_kem();
  benchmark_weave_sig();
  benchmark_ml_kem_1024();
  benchmark_ml_dsa_87();
  benchmark_oga_kem();

  printf("\n=================================================\n");
  printf("Benchmark Complete\n");
  printf("=================================================\n");

  return 0;
}
