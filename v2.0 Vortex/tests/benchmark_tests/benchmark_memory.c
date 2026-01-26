#include "../../include/public/hc_secure_memory.h"
#include <stdio.h>
#include <time.h>

void benchmark_memory_suite(void) {
  printf("=== Memory Performance Benchmark ===\n");

  int iterations = 10000;
  void *ptrs[100];
  size_t size = 1024;

  double start = (double)clock() / CLOCKS_PER_SEC;

  for (int i = 0; i < iterations; i++) {
    ptrs[i % 100] = hc_secure_alloc(size);
    hc_secure_free(ptrs[i % 100], size);
  }

  double end = (double)clock() / CLOCKS_PER_SEC;
  printf("  Alloc+Zero+Free (1KB): %.2f ns/op\n",
         (end - start) / iterations * 1e9);
}

int main(void) {
  benchmark_memory_suite();
  return 0;
}
