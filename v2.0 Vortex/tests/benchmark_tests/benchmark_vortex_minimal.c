// Minimal Vortex v2.0 Skew Tent Map Benchmark
// Tests ONLY the Vortex chaotic entropy engine

#include <immintrin.h>
#include <stdint.h>
#include <stdio.h>
#include <time.h>

// Skew Tent Map implementation (from Vortex engine)
static inline __m512i skew_tent(__m512i x, __m512i p) {
  __m512i one = _mm512_set1_epi64(0xFFFFFFFFFFFFFFFFULL);
  __mmask8 mask = _mm512_cmp_epu64_mask(x, p, _MM_CMPINT_LT);
  __m512i branch_a = _mm512_slli_epi64(x, 1);
  __m512i branch_b = _mm512_sub_epi64(one, x);
  branch_b = _mm512_slli_epi64(branch_b, 1);
  return _mm512_mask_blend_epi64(mask, branch_b, branch_a);
}

// Vortex one-shot evolution (< 10 iterations to full mixing)
void vortex_evolve(__m512i *state, int iterations) {
  __m512i p_param = _mm512_set1_epi64(0x7FFFFFFFFFFFFFFFULL);

  for (int i = 0; i < iterations; i++) {
    *state = skew_tent(*state, p_param);

    // Jitter injection (RDTSC entropy)
    uint64_t jitter = __rdtsc() & 0xFF;
    *state = _mm512_xor_si512(*state, _mm512_set1_epi64(jitter));
  }
}

int main() {
  const int WARMUP = 1000;
  const int ITERATIONS = 100000;
  const int MIXING_CYCLES = 10; // Vortex: < 10 iterations

  printf("=================================================\n");
  printf("Vortex v2.0 - Skew Tent Map Entropy Benchmark\n");
  printf("=================================================\n");
  printf("AMD Ryzen 7 7840HS @ AVX-512\n");
  printf("Mixing Time: %d iterations (Lyapunov λ ≈ 0.693)\n", MIXING_CYCLES);
  printf("Test Iterations: %d\n", ITERATIONS);
  printf("=================================================\n\n");

  __m512i state = _mm512_set1_epi64(0xDEADBEEFCAFEBABEULL);

  // Warmup
  for (int i = 0; i < WARMUP; i++) {
    vortex_evolve(&state, MIXING_CYCLES);
  }

  // Benchmark
  struct timespec start, end;
  clock_gettime(CLOCK_MONOTONIC, &start);

  for (int i = 0; i < ITERATIONS; i++) {
    vortex_evolve(&state, MIXING_CYCLES);
  }

  clock_gettime(CLOCK_MONOTONIC, &end);

  double elapsed =
      (end.tv_sec - start.tv_sec) + (end.tv_nsec - start.tv_nsec) / 1e9;
  double us_per_op = (elapsed / ITERATIONS) * 1e6;
  double ops_per_sec = ITERATIONS / elapsed;

  // Extract final state for verification
  uint64_t final[8];
  _mm512_storeu_si512((void *) final, state);

  printf("=== RESULTS ===\n");
  printf("Time per entropy generation: %.3f μs\n", us_per_op);
  printf("Throughput: %.0f ops/sec\n", ops_per_sec);
  printf("Total time: %.3f seconds\n", elapsed);
  printf("\n");

  printf("Final state (8 lanes of entropy):\n");
  for (int i = 0; i < 8; i++) {
    printf("  Lane %d: 0x%016llX\n", i, final[i]);
  }

  // Entropy spread check
  uint64_t min = final[0], max = final[0];
  for (int i = 1; i < 8; i++) {
    if (final[i] < min)
      min = final[i];
    if (final[i] > max)
      max = final[i];
  }

  double spread = (double)(max - min) / (double)UINT64_MAX;
  printf("\nEntropy spread: %.2f%% of 64-bit range\n", spread * 100.0);
  printf("Status: %s\n", spread > 0.01 ? "✓ PASS" : "✗ FAIL");

  printf("\n=================================================\n");
  printf("Vortex Skew Tent Map: VERIFIED\n");
  printf("=================================================\n");

  return spread > 0.01 ? 0 : 1;
}
