// tests/unit/test_ml_dsa_87.c
#include "hypercycle_algorithms.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

int main(void) {
  printf("Running ML-DSA-87 unit test...\n");

  uint8_t pk[HC_ML_DSA_87_PUBLIC_KEY_SIZE];
  uint8_t sk[HC_ML_DSA_87_SECRET_KEY_SIZE];

  if (hc_ml_dsa_87_keypair(pk, sk) != hc_SUCCESS) {
    fprintf(stderr, "Keypair generation failed\n");
    return 1;
  }
  printf("[PASS] Keypair generated\n");

  const char *msg = "HyperCycle test message";
  size_t msg_len = strlen(msg);
  uint8_t sig[HC_ML_DSA_87_SIGNATURE_SIZE];
  size_t sig_len = 0;

  if (hc_ml_dsa_87_sign(sig, &sig_len, (const uint8_t *)msg, msg_len, sk) !=
      hc_SUCCESS) {
    fprintf(stderr, "Signing failed\n");
    return 1;
  }
  printf("[PASS] Message signed (sig_len=%zu)\n", sig_len);

  if (hc_ml_dsa_87_verify(sig, sig_len, (const uint8_t *)msg, msg_len, pk) !=
      hc_SUCCESS) {
    fprintf(stderr, "Verification failed\n");
    return 1;
  }
  printf("[PASS] Signature verified\n");

  // Negative test: modify signature
  sig[0] ^= 0xFF; // Tamper with first byte
  if (hc_ml_dsa_87_verify(sig, sig_len, (const uint8_t *)msg, msg_len, pk) ==
      hc_SUCCESS) {
    fprintf(stderr, "Verification should have failed for tampered signature\n");
    return 1;
  }
  printf("[PASS] Tampered signature correctly rejected\n");

  printf("All ML-DSA-87 tests passed.\n");
  return 0;
}
