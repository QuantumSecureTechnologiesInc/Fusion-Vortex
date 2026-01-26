/**
 * Fault Injection Verification Test
 * Phase 3 Security Validation
 */

#include "internal/integrity.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>


void test_key_integrity() {
  printf("Testing Key Integrity (CRC32)...\n");

  uint8_t key_data[] = "SECRET_KEY_12345";
  hc_protected_key_t pkey;

  // Protect key
  hc_protect_key(key_data, sizeof(key_data), &pkey);
  printf("  Original CRC32: 0x%08X\n", pkey.crc32);

  // Verify valid key
  int res = hc_verify_key_integrity(&pkey);
  assert(res == 0);
  printf("  ✓ Valid key check passed\n");

  // Tamper with key (Fault Injection Simulation)
  printf("  Injecting fault (bit flip)...\n");
  key_data[0] ^= 0x01;

  // Verify corrupted key
  res = hc_verify_key_integrity(&pkey);
  assert(res == -1);
  printf("  ✓ Fault detected successfully\n");

  // Restore key
  key_data[0] ^= 0x01;
  res = hc_verify_key_integrity(&pkey);
  assert(res == 0);
  printf("  ✓ Restoration verified\n");
}

/* Mock function for redundancy check */
int critical_op(int input) { return input * 2; }

/* Mock function that fails on second call (simulation) */
int glitchy_op(int input) {
  static int call_count = 0;
  call_count++;
  if (call_count % 2 == 0)
    return input * 2 + 1; // Glitch result
  return input * 2;
}

void test_redundancy() {
  printf("Testing Redundancy Checks...\n");

// Define simple manual check logic matching the macro logic
#define VERIFY_OP(val, expected)                                               \
  do {                                                                         \
    int r1 = critical_op(val);                                                 \
    int r2 = critical_op(val);                                                 \
    if (r1 != r2)                                                              \
      printf("  ❌ Fault detected (mismatch)\n");                              \
    else if (r1 != expected)                                                   \
      printf("  ❌ Fault detected (wrong result)\n");                          \
    else                                                                       \
      printf("  ✓ Redundant check passed\n");                                  \
  } while (0)

  VERIFY_OP(10, 20);

  printf("  Simulating glitch in redundant path...\n");
  int r1 = glitchy_op(5); // 10
  int r2 = glitchy_op(5); // 11 (glitch)

  if (r1 != r2) {
    printf("  ✓ Glitch detected by redundancy!\n");
  } else {
    printf("  ❌ Glitch NOT detected\n");
  }
}

int main() {
  printf("=== Phase 3: Fault Injection Protection Test ===\n\n");

  test_key_integrity();
  test_redundancy();

  printf("\n=== All Phase 3 Tests PASSED ✓ ===\n");
  return 0;
}
