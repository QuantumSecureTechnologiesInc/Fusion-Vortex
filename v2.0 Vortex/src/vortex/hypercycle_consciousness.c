#include "vortex/public/hypercycle_consciousness.h"
#include <math.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

static bool consciousness_contains_pattern(const uint8_t *data, size_t data_len,
                                           const uint8_t *pattern,
                                           size_t pattern_len) {
  if (!data || !pattern || pattern_len == 0 || data_len < pattern_len) {
    return false;
  }

  for (size_t i = 0; i <= data_len - pattern_len; i++) {
    if (memcmp(data + i, pattern, pattern_len) == 0) {
      return true;
    }
  }
  return false;
}

static bool consciousness_is_adversarial_input(const uint8_t *input,
                                               size_t len) {
  if (!input || len < 32)
    return false;

  // Calculate Shannon entropy
  uint32_t byte_counts[256] = {0};
  for (size_t i = 0; i < len; i++) {
    byte_counts[input[i]]++;
  }

  double entropy = 0.0;
  double len_d = (double)len;

  for (int i = 0; i < 256; i++) {
    if (byte_counts[i] > 0) {
      double p = byte_counts[i] / len_d;
      entropy -= p * log2(p);
    }
  }

  // Flag suspiciously low or high entropy
  // Normal random data should have entropy between 2.0 and 7.9 bits
  return (entropy < 2.0 || entropy > 7.9);
}

int consciousness_resistance_init(consciousness_resistance_t *guard) {
  if (!guard)
    return -1;

  guard->max_patterns = HYPERCYCLE_MAX_ATTACK_PATTERNS;
  guard->pattern_count = 0;
  guard->blocked_count = 0;

  guard->attack_patterns = malloc(guard->max_patterns * sizeof(uint8_t *));
  if (!guard->attack_patterns)
    return -1;

  // Initialize to NULL
  for (size_t i = 0; i < guard->max_patterns; i++) {
    guard->attack_patterns[i] = NULL;
  }

  return 0;
}

void consciousness_resistance_cleanup(consciousness_resistance_t *guard) {
  if (!guard || !guard->attack_patterns)
    return;

  for (size_t i = 0; i < guard->pattern_count; i++) {
    free(guard->attack_patterns[i]);
  }
  free(guard->attack_patterns);
  guard->attack_patterns = NULL;
}

bool consciousness_resistance_check_attack(consciousness_resistance_t *guard,
                                           const uint8_t *input,
                                           size_t input_len) {
  if (!guard || !input)
    return false;

  // Check against known attack patterns
  for (size_t i = 0; i < guard->pattern_count; i++) {
    if (guard->attack_patterns[i] &&
        consciousness_contains_pattern(input, input_len,
                                       guard->attack_patterns[i], 32)) {
      guard->blocked_count++;
      return true; // Attack detected
    }
  }

  // Check for adversarial input patterns (entropy-based detection)
  if (consciousness_is_adversarial_input(input, input_len)) {
    guard->blocked_count++;
    return true;
  }

  return false; // No attack detected
}
