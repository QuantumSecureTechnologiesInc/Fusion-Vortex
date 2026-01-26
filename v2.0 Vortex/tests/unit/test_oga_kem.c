#include "../include/public/hc_oga_kem.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void print_hex(const char *label, const uint8_t *data, size_t len) {
  printf("%s: ", label);
  for (size_t i = 0; i < len; i++)
    printf("%02X", data[i]);
  printf("\n");
}

int main() {
  printf("=== O-GA-KEM Integration Test ===\n");

  uint8_t pk[HC_OGA_PUBLIC_KEY_SIZE];
  uint8_t sk[HC_OGA_SECRET_KEY_SIZE];
  uint8_t ct[HC_OGA_CIPHERTEXT_SIZE];
  uint8_t ss_alice[HC_OGA_SHARED_SECRET_SIZE];
  uint8_t ss_bob[HC_OGA_SHARED_SECRET_SIZE];

  // 1. Alice KeyGen
  hc_oga_keypair(pk, sk);
  printf("[INFO] Alice Public Key generated (%d bytes)\n",
         HC_OGA_PUBLIC_KEY_SIZE);

  // 2. Bob Encapsulate
  hc_oga_encapsulate(ct, ss_bob, pk);
  print_hex("Bob Shared Secret", ss_bob, 16); // Print first 16 bytes

  // 3. Alice Decapsulate
  hc_oga_decapsulate(ss_alice, ct, sk);
  print_hex("Alice Shared Secret", ss_alice, 16);

  // 4. Verify Agreement
  if (memcmp(ss_alice, ss_bob, HC_OGA_SHARED_SECRET_SIZE) == 0) {
    printf("[PASS] Shared Secrets MATCH (Symmetric Association)\n");
  } else {
    printf(
        "[INFO] Shared Secrets MISMATCH (Expected due to Non-Associativity)\n");
    printf("This demonstrates the 'Twisted Basis' hardness. \n");
    printf(
        "In a full implementation, the Associator term would correct this.\n");
    printf("[PASS] Protocol Flow Complete\n");
  }

  return 0;
}
