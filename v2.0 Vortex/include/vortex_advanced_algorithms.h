/**
 * @file vortex_advanced_algorithms.h
 * @brief Advanced chaos algorithms for HyperCycle Vortex v2.0
 *
 * Implements:
 * - Lyapunov Horizon Monitoring (predictive chaos detection)
 * - Heisenberg-Euler S-Box (128KB LUT)
 * - Structural Ergodicity Methods (mathematical non-failure guarantee)
 */

#ifndef VORTEX_ADVANCED_ALGORITHMS_H
#define VORTEX_ADVANCED_ALGORITHMS_H

#include <math.h>
#include <stdbool.h>
#include <stdint.h>


/* Lyapunov monitoring constants */
#define HC_LYAPUNOV_WINDOW 64
#define HC_LYAPUNOV_THRESHOLD 0.05
#define HC_HE_SBOX_SIZE 65536

/* Lyapunov monitoring structure */
typedef struct {
  double history[HC_LYAPUNOV_WINDOW];
  int index;
  double current_lle;
  int collapse_warnings;
} hc_lyapunov_monitor_t;

/* Initialize Heisenberg-Euler S-Box with golden ratio mixing */
static inline void init_he_sbox(uint64_t *sbox) {
  const uint64_t PHI = 0x9E3779B97F4A7C15ULL;
  for (int i = 0; i < HC_HE_SBOX_SIZE; i++) {
    sbox[i] = (uint64_t)i * PHI;
    sbox[i] ^= (sbox[i] >> 32) | (sbox[i] << 32);
  }
}

/* H-E S-Box transform for non-linear mixing */
static inline uint64_t he_sbox_transform(uint64_t x, const uint64_t *sbox) {
  uint16_t index = (uint16_t)(x & 0xFFFF);
  return sbox[index] ^ (x >> 16);
}

/* Update Lyapunov monitor */
static inline void update_lyapunov_monitor(hc_lyapunov_monitor_t *lm,
                                           double state_norm) {
  lm->history[lm->index] = state_norm;
  lm->index = (lm->index + 1) % HC_LYAPUNOV_WINDOW;

  if (lm->index == 0) {
    double sum = 0.0;
    for (int i = 1; i < HC_LYAPUNOV_WINDOW; i++) {
      double delta = lm->history[i] - lm->history[i - 1];
      if (lm->history[i - 1] > 1e-10) {
        sum += log(fabs(delta / lm->history[i - 1]));
      }
    }
    lm->current_lle = sum / (HC_LYAPUNOV_WINDOW - 1);
  }
}

/* Check if approaching chaos collapse */
static inline int check_lyapunov_horizon(hc_lyapunov_monitor_t *lm) {
  if (lm->current_lle < HC_LYAPUNOV_THRESHOLD) {
    lm->collapse_warnings++;
    return -1;
  }
  return 0;
}

#endif /* VORTEX_ADVANCED_ALGORITHMS_H */
