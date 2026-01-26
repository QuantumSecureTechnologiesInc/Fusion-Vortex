#include "hc_math_boost.h"
#include <stdint.h>
#include <stdio.h>

#ifdef _WIN32
#include <windows.h>
typedef LARGE_INTEGER hc_timer_t;

static void timer_start(hc_timer_t *t) { QueryPerformanceCounter(t); }

static double timer_elapsed(hc_timer_t start) {
  LARGE_INTEGER end, freq;
  QueryPerformanceCounter(&end);
  QueryPerformanceFrequency(&freq);
  return (double)(end.QuadPart - start.QuadPart) / (double)freq.QuadPart;
}
#else
#include <time.h>
typedef struct timespec hc_timer_t;

static void timer_start(hc_timer_t *t) { clock_gettime(CLOCK_MONOTONIC, t); }

static double timer_elapsed(hc_timer_t start) {
  struct timespec end;
  clock_gettime(CLOCK_MONOTONIC, &end);
  return (end.tv_sec - start.tv_sec) + (end.tv_nsec - start.tv_nsec) * 1e-9;
}
#endif

int main() {
  // Increased iterations to ensure measurable time
  const int ITERATIONS = 500000000;

  // Test vectors
  hc_quat_t rot1 = {HC_SCALE, 100, 200, 300};
  hc_quat_t rot2 = {HC_SCALE, 50, -50, 50};
  hc_quat_t vec = {0, 1000, 0, 0};
  hc_quat_t result;

  // Volatile accumulator to prevent loop optimization
  volatile int64_t accumulator = 0;

  printf("Running Optimization Benchmark (%d iterations)...\n", ITERATIONS);

  /* --- Sequential Rotations --- */
  hc_timer_t start_seq;
  timer_start(&start_seq);

  for (int i = 0; i < ITERATIONS; i++) {
    hc_quat_t temp;
    // Two separate rotations
    hc_quat_rotate(&rot1, &vec, &temp);
    hc_quat_rotate(&rot2, &temp, &result);

    // Prevent optimization
    accumulator ^= result.w;
  }
  double time_seq = timer_elapsed(start_seq);
  printf("Sequential Time: %.6f s\n", time_seq);

  /* --- Combined Rotation (Optimization) --- */
  hc_timer_t start_comb;
  hc_quat_t combined;

  timer_start(&start_comb);

  // Pre-calculate composed rotation
  hc_quat_compose_rotations(&rot2, &rot1, &combined);

  for (int i = 0; i < ITERATIONS; i++) {
    // Single applied rotation
    hc_quat_rotate(&combined, &vec, &result);

    // Prevent optimization
    accumulator ^= result.w;
  }
  double time_comb = timer_elapsed(start_comb);
  printf("Combined Time:   %.6f s\n", time_comb);

  /* --- Results --- */
  if (time_comb > 0.0000001) {
    printf("Speedup Factor:  %.2fx\n", time_seq / time_comb);
  } else {
    printf("Speedup Factor:  N/A (Too fast)\n");
  }

  // Print accumulator to ensure it's used
  if (accumulator == 0xDEADBEEF)
    printf(" (Unlikely)\n");

  return 0;
}
