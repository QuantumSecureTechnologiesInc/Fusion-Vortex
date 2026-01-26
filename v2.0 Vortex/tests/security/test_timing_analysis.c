#include "internal/hc_constant_time.h"
#include <stdio.h>
#include <stdlib.h>
#include <time.h>


// Simple statistical test for timing
// Collects standard deviation of execution times
// Note: Real "Constant Time" verification needs higher precision (cycles)
// and strict environment control. This is a basic functional check.

uint64_t rdtsc(void) {
  unsigned int lo, hi;
  __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
  return ((uint64_t)hi << 32) | lo;
}

void test_timing_memcmp(void) {
  uint8_t a[32], b[32];
  for (int i = 0; i < 32; i++) {
    a[i] = i;
    b[i] = i;
  }

  // Warmup
  hc_ct_memcmp(a, b, 32);

  uint64_t t_equal = 0;
  int samples = 10000;
  for (int i = 0; i < samples; i++) {
    uint64_t start = rdtsc();
    hc_ct_memcmp(a, b, 32);
    uint64_t end = rdtsc();
    t_equal += (end - start);
  }

  // Change last byte
  b[31] ^= 0xFF;
  uint64_t t_diff = 0;
  for (int i = 0; i < samples; i++) {
    uint64_t start = rdtsc();
    hc_ct_memcmp(a, b, 32);
    uint64_t end = rdtsc();
    t_diff += (end - start);
  }

  double avg_eq = (double)t_equal / samples;
  double avg_diff = (double)t_diff / samples;

  printf("Constant Time check (memcmp):\n");
  printf("  Equal avg cycles: %.2f\n", avg_eq);
  printf("  Diff avg cycles:  %.2f\n", avg_diff);
  printf("  Delta: %.4f%%\n", 100.0 * (avg_diff - avg_eq) / avg_eq);
}

int main(void) {
  test_timing_memcmp();
  return 0;
}
