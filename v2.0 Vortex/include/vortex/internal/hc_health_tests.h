#ifndef hc_HEALTH_TESTS_H
#define hc_HEALTH_TESTS_H

#include <stddef.h>
#include <stdint.h>

// NIST SP 800-90B Health Test Parameters
#define HC_RCT_CUTOFF 30 // RCT threshold (NIST SP 800-90B Section 4.4.1)
#define HC_HEALTH_WINDOW_SIZE 512 // Sliding window size
#define HC_APT_CUTOFF_512 265     // APT cutoff for 512-sample window

// NIST SP 800-90B Repetition Count Test (RCT)
// Detects if the output gets "stuck" on a value.
typedef struct {
  uint8_t last_sample;
  uint32_t repetition_count;
  uint32_t cutoff; // Failure threshold (e.g., 31 for 2^-20 false positive rate)
  int initialized;
} hc_rct_state_t;

// NIST SP 800-90B Adaptive Proportion Test (APT)
// Detects if a specific value becomes too common (loss of entropy).
typedef struct {
  uint32_t counters[256];
  uint32_t window_count;
  uint32_t cutoff;               // Failure threshold
  uint8_t current_window_sample; // Sample used for counting in current window?
  // Actually per NIST, for non-binary we track frequency of ALL values or
  // standard implementation often tracks the "current sample being tested".
  // 800-90B 4.4.2: APT simply checks if any value appears too frequently in a
  // window. Optimization: We can just track max frequency, or reset every
  // window.
} hc_apt_state_t;

// Initialise health tests
void hc_rct_init(hc_rct_state_t *rct, uint32_t cutoff);
void hc_apt_init(hc_apt_state_t *apt, uint32_t cutoff);

// Process samples (returns 0 on pass, -1 on failure)
int hc_rct_test(hc_rct_state_t *rct, uint8_t sample);
int hc_apt_test(hc_apt_state_t *apt, uint8_t sample);

// ============================================================================
// Health Monitor with 512-sample Sliding Window
// ============================================================================

/**
 * @brief Health monitor with reservoir
 *
 * Implements NIST SP 800-90B continuous health testing with a 512-sample
 * reservoir for sliding window RCT and APT tests.
 */
typedef struct {
  // 512-sample reservoir for sliding window
  uint8_t reservoir[HC_HEALTH_WINDOW_SIZE];
  int reservoir_index;
  int reservoir_filled;

  // RCT state (within reservoir window)
  int rct_max_repetitions;

  // APT state (within reservoir window)
  int apt_counts[256];

  // Failure tracking
  uint64_t total_samples_processed;
  uint64_t rct_failures;
  uint64_t apt_failures;
} hc_health_monitor_t;

/**
 * @brief Initialize health monitor with reservoir
 *
 * @param monitor Health monitor structure
 * @return 0 on success, -1 on failure
 */
int hc_health_monitor_init(hc_health_monitor_t *monitor);

/**
 * @brief Test a sample using the sliding window health monitor
 *
 * @param monitor Health monitor structure
 * @param sample Entropy byte to test
 * @return 0 if passed, -1 if failed
 */
int hc_health_monitor_test(hc_health_monitor_t *monitor, uint8_t sample);

/**
 * @brief Get health monitor statistics
 *
 * @param monitor Health monitor structure
 * @param total_samples Output: total samples processed
 * @param rct_failures Output: RCT failure count
 * @param apt_failures Output: APT failure count
 */
void hc_health_monitor_get_stats(const hc_health_monitor_t *monitor,
                                 uint64_t *total_samples,
                                 uint64_t *rct_failures,
                                 uint64_t *apt_failures);

#endif
