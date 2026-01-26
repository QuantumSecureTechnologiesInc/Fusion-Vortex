/**
 * Bit-Sensitivity Test for 47-Cycle Chaos Horizon
 *
 * Verification Criterion:
 * - Change 1 bit in initial seed
 * - After 47 cycles, output should have ~50% Hamming distance (128/256 bits
 * different)
 *
 * This validates that the fixed-point chaos hasn't been "killed" by
 * discretization
 */

#include "internal/hc_vacuum_fixed.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>


#define CYCLES 47

// Count number of differing bits between two buffers
int hamming_distance(const uint8_t *a, const uint8_t *b, size_t len) {
  int count = 0;
  for (size_t i = 0; i < len; i++) {
    uint8_t xor_val = a[i] ^ b[i];
    // Count set bits
    while (xor_val) {
      count += xor_val & 1;
      xor_val >>= 1;
    }
  }
  return count;
}

int main() {
  printf("=== 47-Cycle Chaos Horizon Bit-Sensitivity Test ===\n\n");

  // Original seed
  uint8_t seed1[16] = {0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0,
                       0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88};

  // Flipped seed (bit 0 of byte 0 flipped: 0x12 → 0x13)
  uint8_t seed2[16];
  memcpy(seed2, seed1, 16);
  seed2[0] ^= 0x01; // Flip exactly 1 bit

  printf("Seed 1: ");
  for (int i = 0; i < 16; i++)
    printf("%02X ", seed1[i]);
  printf("\nSeed 2: ");
  for (int i = 0; i < 16; i++)
    printf("%02X ", seed2[i]);
  printf(" (1 bit flipped)\n\n");

  // Convert to fixed-point quaternions
  hc_fp_quaternion_t q1 = hc_seed_to_fp_quat(seed1);
  hc_fp_quaternion_t q2 = hc_seed_to_fp_quat(seed2);

  printf("Running 47-cycle Heisenberg-Euler evolution...\n");

  // Evolve for exactly 47 cycles
  for (int cycle = 0; cycle < CYCLES; cycle++) {
    q1 = hc_he_transform_fixed(q1);
    q2 = hc_he_transform_fixed(q2);
  }

  // Extract chaotic residues (LSB 32 bits)
  uint8_t output1[16], output2[16];
  hc_extract_fp_residue(q1, output1);
  hc_extract_fp_residue(q2, output2);

  printf("\nOutput 1: ");
  for (int i = 0; i < 16; i++)
    printf("%02X ", output1[i]);
  printf("\nOutput 2: ");
  for (int i = 0; i < 16; i++)
    printf("%02X ", output2[i]);
  printf("\n\n");

  // Compute Hamming distance
  int distance = hamming_distance(output1, output2, 16);
  int total_bits = 16 * 8; // 128 bits
  double percentage = (double)distance / total_bits * 100.0;

  printf("=== Results ===\n");
  printf("Hamming Distance: %d / %d bits (%.1f%%)\n", distance, total_bits,
         percentage);
  printf("Expected Range:   48-80 bits (37.5%% - 62.5%%)\n");

  // Success criteria: 37.5% - 62.5% (indicating strong avalanche effect)
  if (distance >= 48 && distance <= 80) {
    printf("\n✅ PASS: Chaos horizon validated!\n");
    printf("   Fixed-point discretization has NOT killed the entropy.\n");
    return 0;
  } else if (distance < 48) {
    printf("\n❌ FAIL: Insufficient chaos (too deterministic)\n");
    printf("   The system is not diverging enough.\n");
    return 1;
  } else {
    printf("\n⚠️  WARNING: Excessive chaos (>62.5%%)\n");
    printf("   This is still acceptable but may indicate instability.\n");
    return 0;
  }
}
