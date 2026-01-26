/**
 * @file hc_chaos_test_harness.c
 * @brief Updated harness using the Universal API.
 */

#include "hc_chaotic_engine.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>


#define TEST_BYTES (1024 * 1024)

void run_monobit_test(uint8_t *data, size_t len) {
  long long ones = 0;
  long long total_bits = (long long)len * 8;
  for (size_t i = 0; i < len; i++) {
    uint8_t byte = data[i];
    for (int j = 0; j < 8; j++)
      if ((byte >> j) & 1)
        ones++;
  }
  double ratio = (double)ones / total_bits;
  printf("[STAT] Universal Monobit Test: %.6f (Ideal: 0.5)\n", ratio);
  if (ratio > 0.49 && ratio < 0.51)
    printf("       Result: PASS\n");
  else
    printf("       Result: WARNING\n");
}

int main() {
  hc_chaos_ctx_t ctx;
  uint8_t *buffer = malloc(TEST_BYTES);
  if (!buffer)
    return 1;

  printf("[INFO] Testing Universal API...\n");
  hc_chaos_init(&ctx, 0xABCDEF12);

  /* Use the universal callback */
  hc_chaos_univ_random(&ctx, buffer, TEST_BYTES);

  run_monobit_test(buffer, TEST_BYTES);

  FILE *f = fopen("chaos_bits.bin", "wb");
  if (f) {
    fwrite(buffer, 1, TEST_BYTES, f);
    fclose(f);
    printf("[INFO] Universal bitstream saved to chaos_bits.bin\n");
  }

  free(buffer);
  return 0;
}
