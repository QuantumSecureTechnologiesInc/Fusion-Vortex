#ifndef HYPERCYCLE_CONSCIOUSNESS_H
#define HYPERCYCLE_CONSCIOUSNESS_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


#define HYPERCYCLE_MAX_ATTACK_PATTERNS 1000

typedef struct {
  uint8_t **attack_patterns;
  size_t pattern_count;
  size_t max_patterns;
  uint64_t blocked_count;
} consciousness_resistance_t;

int consciousness_resistance_init(consciousness_resistance_t *guard);
void consciousness_resistance_cleanup(consciousness_resistance_t *guard);
bool consciousness_resistance_check_attack(consciousness_resistance_t *guard,
                                           const uint8_t *input,
                                           size_t input_len);

#endif // HYPERCYCLE_CONSCIOUSNESS_H
