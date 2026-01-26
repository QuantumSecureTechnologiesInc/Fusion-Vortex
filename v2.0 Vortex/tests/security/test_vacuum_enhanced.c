#include <stdint.h>
#include <stdio.h>
#include <string.h>


// External declarations for testing (assuming linkage)
extern int hc_generate_vacuum_key(uint8_t *out, size_t out_len);

int main(void) {
  printf("=== Testing Enhanced Vacuum Key Generation ===\n");

  uint8_t key[32];

  printf("Test 1: Generate 32-byte key with health tests...\n");
  int result = hc_generate_vacuum_key(key, 32);

  if (result == 0) {
    printf("  ✓ SUCCESS: Key generated\n");
    printf("  First 8 bytes: ");
    for (int i = 0; i < 8; i++) {
      printf("%02x ", key[i]);
    }
    printf("\n");
  } else if (result == -101) {
    printf("  ✗ FAILED: Repetition Count Test (RCT) failure\n");
    return 1;
  } else if (result == -102) {
    printf("  ✗ FAILED: Adaptive Proportion Test (APT) failure\n");
    return 1;
  } else {
    printf("  ✗ FAILED: Unknown error code %d\n", result);
    return 1;
  }

  printf("\nTest 2: Multiple key generation (stress test)...\n");
  for (int i = 0; i < 10; i++) {
    result = hc_generate_vacuum_key(key, 32);
    if (result != 0) {
      printf("  ✗ FAILED on iteration %d (error %d)\n", i + 1, result);
      return 1;
    }
  }
  printf("  ✓ SUCCESS: 10 keys generated without health test failures\n");

  printf("\n=== All Vacuum Tests PASSED ===\n");
  return 0;
}
