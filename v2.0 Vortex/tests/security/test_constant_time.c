#include "internal/hc_constant_time.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>


// Simple test framework
#define TEST_ASSERT(cond)                                                      \
  if (!(cond)) {                                                               \
    printf("FAILED: %s:%d: %s\n", __FILE__, __LINE__, #cond);                  \
    return 1;                                                                  \
  }

// helper to print check marks
#define PASS() printf("✓ %s passed\n", __func__)

int test_ct_memcmp(void) {
  uint8_t a[32] = {0};
  uint8_t b[32] = {0};
  uint8_t c[32] = {0};

  // Case 1: Identical
  TEST_ASSERT(hc_ct_memcmp(a, b, 32) == 0);

  // Case 2: One byte different at start
  c[0] = 1;
  TEST_ASSERT(hc_ct_memcmp(a, c, 32) != 0);

  // Case 3: One byte different at end
  uint8_t d[32] = {0};
  d[31] = 0xFF;
  TEST_ASSERT(hc_ct_memcmp(a, d, 32) != 0);

  PASS();
  return 0;
}

int test_ct_select_u8(void) {
  uint8_t a = 0xAA;
  uint8_t b = 0x55;

  // Select a (condition != 0)
  TEST_ASSERT(hc_ct_select_u8(a, b, 1) == a);
  TEST_ASSERT(hc_ct_select_u8(a, b, -1) == a);
  TEST_ASSERT(hc_ct_select_u8(a, b, 100) == a);

  // Select b (condition == 0)
  TEST_ASSERT(hc_ct_select_u8(a, b, 0) == b);

  PASS();
  return 0;
}

int test_ct_equal_u32(void) {
  TEST_ASSERT(hc_ct_equal_u32(100, 100) == 1);
  TEST_ASSERT(hc_ct_equal_u32(100, 101) == 0);
  TEST_ASSERT(hc_ct_equal_u32(0, 0) == 1);
  TEST_ASSERT(hc_ct_equal_u32(0xFFFFFFFF, 0xFFFFFFFF) == 1);
  TEST_ASSERT(hc_ct_equal_u32(0xFFFFFFFF, 0) == 0);

  PASS();
  return 0;
}

int test_ct_lookup(void) {
  uint8_t table[256];
  for (int i = 0; i < 256; i++) {
    table[i] = (uint8_t)i;
  }

  uint8_t out;

  // Look up various indices
  hc_ct_lookup(table, 256, 1, 0, &out);
  TEST_ASSERT(out == 0);

  hc_ct_lookup(table, 256, 1, 127, &out);
  TEST_ASSERT(out == 127);

  hc_ct_lookup(table, 256, 1, 255, &out);
  TEST_ASSERT(out == 255);

  PASS();
  return 0;
}

int main(void) {
  if (test_ct_memcmp() != 0)
    return 1;
  if (test_ct_select_u8() != 0)
    return 1;
  if (test_ct_equal_u32() != 0)
    return 1;
  if (test_ct_lookup() != 0)
    return 1;

  printf("All constant-time tests passed.\n");
  return 0;
}
