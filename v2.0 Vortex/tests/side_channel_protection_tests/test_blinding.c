#include "hc_gpu_universal.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>

int main(void) {
  printf("--- HyperCycle Blinding Test ---\n");
  const hc_gpu_backend_t *backend = hc_gpu_auto_init();
  if (!backend) {
    printf("Failed to load backend\n");
    return 1;
  }
  printf("Backend: %s\n", backend->name);
  // No context needed for universal API
  size_t count = 1;
  uint64_t seeds = 0xCAFEBABEULL;
  uint64_t blind = 0xDEADBEEFULL;
  uint8_t out_raw[32];
  uint8_t out_blind[32];

  // Generate without blinding (blind_seed = 0)
  if (backend->generate_entropy_batch(seeds, 0, out_raw, count) !=
      HC_GPU_SUCCESS) {
    printf("Failed to generate raw entropy\n");
    return 1;
  }

  // Generate with blinding
  if (backend->generate_entropy_batch(seeds, blind, out_blind, count) !=
      HC_GPU_SUCCESS) {
    printf("Failed to generate blinded entropy\n");
    return 1;
  }

  // Compare outputs
  if (memcmp(out_raw, out_blind, 32) == 0) {
    printf("[FAIL] Blinded output matches raw output (Blinding Ineffective)\n");
  } else {
    printf("[PASS] Blinded output differs from raw output (Blinding Active)\n");
  }

  hc_gpu_shutdown();
  return 0;
}
