#include "cemqc.h"
#include "internal/hc_sidechannel.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#ifdef _WIN32
#include <intrin.h>
#include <windows.h>
#else
#include <time.h>
#include <x86intrin.h>

#endif

// Simple test framework
#define TEST_ASSERT(cond)                                                      \
  if (!(cond)) {                                                               \
    printf("FAILED: %s:%d: %s\n", __FILE__, __LINE__, #cond);                  \
    return 1;                                                                  \
  }

#define PASS() printf("✓ %s passed\n", __func__)

// Helper to get high-res timestamp
uint64_t get_cycles(void) {
#ifdef _WIN32
  return __rdtsc();
#else
  unsigned int lo, hi;
  __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
  return ((uint64_t)hi << 32) | lo;
#endif
}

int test_sc_memcpy(void) {
  uint8_t src[256];
  uint8_t dest[256];

  for (int i = 0; i < 256; i++)
    src[i] = (uint8_t)i;
  memset(dest, 0, 256);

  hc_sc_memcpy(dest, src, 256);

  // Check correctness
  for (int i = 0; i < 256; i++) {
    TEST_ASSERT(dest[i] == src[i]);
  }

  PASS();
  return 0;
}

int test_sc_quaternion_blinded(void) {
  hc_quaternion_t a = {1.0, 2.0, 3.0, 4.0};
  hc_quaternion_t b = {0.5, 0.5, 0.5, 0.5};
  hc_quaternion_t expected, result;

  // Standard multiplication for reference
  hc_quaternion_mul(&a, &b, &expected);

  // Mock random mask
  uint8_t mask[32];
  memset(mask, 0x42, 32);

  // Blinded multiplication
  hc_sc_quaternion_mul_blinded(&a, &b, &result, mask, 32);

  // Allow small epsilon diff due to floating point accumulation
  double diff = (expected.w - result.w) * (expected.w - result.w) +
                (expected.x - result.x) * (expected.x - result.x) +
                (expected.y - result.y) * (expected.y - result.y) +
                (expected.z - result.z) * (expected.z - result.z);

  if (diff > 1e-9) {
    printf("Blinded mul diverged: expected {%.2f, %.2f, %.2f, %.2f}, got "
           "{%.2f, %.2f, %.2f, %.2f}\n",
           expected.w, expected.x, expected.y, expected.z, result.w, result.x,
           result.y, result.z);
    return 1;
  }

  TEST_ASSERT(diff < 1e-9);

  PASS();
  return 0;
}

int main(void) {
  if (test_sc_memcpy() != 0)
    return 1;
  if (test_sc_quaternion_blinded() != 0)
    return 1;

  printf("All side-channel tests passed.\n");
  return 0;
}
