#include "internal/hc_health_tests.h"
#include <string.h>

// NIST recommended cutoff for RCT (NIST SP 800-90B Section 4.4.1)
#define hc_RCT_DEFAULT_CUTOFF 30

// NIST recommended cutoff for APT (W=512, C=265 for H_min >= 7.5)
#define hc_APT_WINDOW_SIZE 512
#define hc_APT_DEFAULT_CUTOFF 265

void hc_rct_init(hc_rct_state_t *rct, uint32_t cutoff) {
  if (!rct)
    return;
  memset(rct, 0, sizeof(hc_rct_state_t));
  rct->cutoff = cutoff > 0 ? cutoff : hc_RCT_DEFAULT_CUTOFF;
  rct->initialized = 0;
}

int hc_rct_test(hc_rct_state_t *rct, uint8_t sample) {
  if (!rct)
    return -1;

  if (!rct->initialized) {
    rct->last_sample = sample;
    rct->repetition_count = 1;
    rct->initialized = 1;
    return 0;
  }

  if (sample == rct->last_sample) {
    rct->repetition_count++;
    if (rct->repetition_count >= rct->cutoff) {
      return -1; // Failure: Repetition detected
    }
  } else {
    rct->last_sample = sample;
    rct->repetition_count = 1;
  }

  return 0;
}

void hc_apt_init(hc_apt_state_t *apt, uint32_t cutoff) {
  if (!apt)
    return;
  memset(apt, 0, sizeof(hc_apt_state_t));
  apt->cutoff = cutoff > 0 ? cutoff : hc_APT_DEFAULT_CUTOFF;
}

int hc_apt_test(hc_apt_state_t *apt, uint8_t sample) {
  if (!apt)
    return -1;

  apt->counters[sample]++;
  apt->window_count++;

  // Check if current sample count exceeds cutoff
  if (apt->counters[sample] >= apt->cutoff) {
    // Reset on failure for safety? Or just return error.
    // NIST says "entropy source failure".
    return -1;
  }

  // Reset window
  if (apt->window_count >= hc_APT_WINDOW_SIZE) {
    memset(apt->counters, 0, sizeof(apt->counters));
    apt->window_count = 0;
  }

  return 0;
}

// ============================================================================
// Health Monitor with Reservoir (512-sample sliding window)
// ============================================================================

int hc_health_monitor_init(hc_health_monitor_t *monitor) {
  if (!monitor) {
    return -1;
  }

  memset(monitor, 0, sizeof(hc_health_monitor_t));

  monitor->reservoir_index = 0;
  monitor->reservoir_filled = 0;
  monitor->rct_max_repetitions = 0;
  memset(monitor->apt_counts, 0, sizeof(monitor->apt_counts));

  monitor->total_samples_processed = 0;
  monitor->rct_failures = 0;
  monitor->apt_failures = 0;

  return 0;
}

int hc_health_monitor_test(hc_health_monitor_t *monitor, uint8_t sample) {
  if (!monitor) {
    return -1;
  }

  monitor->total_samples_processed++;

  // Add sample to circular reservoir
  int write_pos = monitor->reservoir_index % HC_HEALTH_WINDOW_SIZE;

  // If reservoir is full, remove old sample from APT counts
  if (monitor->reservoir_filled) {
    uint8_t old_sample = monitor->reservoir[write_pos];
    monitor->apt_counts[old_sample]--;
  }

  // Add new sample
  monitor->reservoir[write_pos] = sample;
  monitor->apt_counts[sample]++;
  monitor->reservoir_index++;

  // Mark reservoir as filled once we've reached window size
  if (monitor->reservoir_index >= HC_HEALTH_WINDOW_SIZE) {
    monitor->reservoir_filled = 1;
  }

  // Only perform tests once reservoir is filled
  if (!monitor->reservoir_filled) {
    return 0;
  }

  // ===== RCT Test: Scan reservoir for maximum consecutive repetitions =====
  int max_reps = 0;
  int current_reps = 0;
  uint8_t prev_sample = monitor->reservoir[0];

  for (int i = 1; i < HC_HEALTH_WINDOW_SIZE; i++) {
    if (monitor->reservoir[i] == prev_sample) {
      current_reps++;
      if (current_reps > max_reps) {
        max_reps = current_reps;
      }
    } else {
      current_reps = 0;
      prev_sample = monitor->reservoir[i];
    }
  }

  monitor->rct_max_repetitions = max_reps;

  // Check RCT threshold (30 consecutive repetitions)
  if (max_reps >= HC_RCT_CUTOFF) {
    monitor->rct_failures++;
    return -1; // RCT failure
  }

  // ===== APT Test: Check if any byte value exceeds frequency threshold =====
  int max_count = 0;
  for (int i = 0; i < 256; i++) {
    if (monitor->apt_counts[i] > max_count) {
      max_count = monitor->apt_counts[i];
    }
  }

  // Check APT cutoff (265 for 512-sample window)
  if (max_count >= HC_APT_CUTOFF_512) {
    monitor->apt_failures++;
    return -1; // APT failure
  }

  return 0; // Both tests passed
}

void hc_health_monitor_get_stats(const hc_health_monitor_t *monitor,
                                 uint64_t *total_samples,
                                 uint64_t *rct_failures,
                                 uint64_t *apt_failures) {
  if (!monitor) {
    return;
  }

  if (total_samples) {
    *total_samples = monitor->total_samples_processed;
  }

  if (rct_failures) {
    *rct_failures = monitor->rct_failures;
  }

  if (apt_failures) {
    *apt_failures = monitor->apt_failures;
  }
}
