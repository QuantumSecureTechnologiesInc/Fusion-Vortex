// tests/unit/test_ml_kem_1024.c
#include "hypercycle_algorithms.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

int main(void) {
  printf("Running ML-KEM-1024 Unit Tests...\n");

  uint8_t pk[HC_ML_KEM_1024_PUBLIC_KEY_SIZE];
  uint8_t sk[HC_ML_KEM_1024_SECRET_KEY_SIZE];
  uint8_t ct[HC_ML_KEM_1024_CIPHERTEXT_SIZE];
  uint8_t ss_enc[HC_ML_KEM_1024_SHARED_SECRET_SIZE];
  uint8_t ss_dec[HC_ML_KEM_1024_SHARED_SECRET_SIZE];

  // Test Key Generation
  if (hc_ml_kem_1024_keypair(pk, sk) != hc_SUCCESS) {
    fprintf(stderr, "Key generation failed\n");
    return 1;
  }
  printf("[PASS] Keypair Generation\n");

  // Test Encapsulation
  if (hc_ml_kem_1024_encapsulate(ct, ss_enc, pk) != hc_SUCCESS) {
    fprintf(stderr, "Encapsulation failed\n");
    return 1;
  }
  printf("[PASS] Encapsulation\n");

  // Test Decapsulation
  if (hc_ml_kem_1024_decapsulate(ss_dec, ct, sk) != hc_SUCCESS) {
    fprintf(stderr, "Decapsulation failed\n");
    return 1;
  }

  // Verify Shared Secrets Match
  if (memcmp(ss_enc, ss_dec, HC_ML_KEM_1024_SHARED_SECRET_SIZE) != 0) {
    fprintf(stderr, "Shared secrets do not match\n");
    return 1;
  }
  printf("[PASS] Decapsulation & Shared Secret Verification\n");

  printf("All ML-KEM-1024 tests passed.\n");
  return 0;
}
