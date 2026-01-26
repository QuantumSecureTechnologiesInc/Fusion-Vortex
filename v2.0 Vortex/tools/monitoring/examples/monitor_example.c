/**
 * Example: How to Monitor Lyapunov Exponent in Vortex v2.0
 *
 * Compile: gcc -o monitor_example monitor_example.c -lpthread -lssl -lcrypto
 * -mavx512f
 */

#include "hc_vacuum_engine.h"
#include "vortex_monitoring_api.h"
#include <stdio.h>
#include <unistd.h>

int main() {
  /* Initialize Vortex context */
  hc_vac_context_t ctx;
  hc_context_config_t config = {.device_id = 1};

  if (hc_vacuum_init_context(&ctx, &config) != HC_SUCCESS) {
    fprintf(stderr, "Failed to initialize context\n");
    return 1;
  }

  printf("Vortex v2.0 Monitoring Example\n");
  printf("================================\n\n");

  /* Generate some entropy to populate monitoring data */
  uint8_t seed[32];
  for (int i = 0; i < 10; i++) {
    hc_vacuum_generate_seed_safe(ctx, seed);

    /* Get extended telemetry with monitoring data */
    hc_telemetry_extended_t stats;
    hc_vacuum_get_telemetry(ctx, (hc_telemetry_t *)&stats);

    /* Display monitoring information */
    printf("Sample %d:\n", i + 1);
    printf("  Lyapunov Exponent: %.6f ", stats.lyapunov_exponent);

    /* Interpret health status */
    hc_chaos_health_t health = get_chaos_health(stats.lyapunov_exponent);
    switch (health) {
    case HC_CHAOS_HEALTHY:
      printf("[✓ HEALTHY]\n");
      break;
    case HC_CHAOS_MARGINAL:
      printf("[⚠ MARGINAL]\n");
      break;
    case HC_CHAOS_CRITICAL:
      printf("[✗ CRITICAL]\n");
      break;
    }

    printf("  Phase Shifts:      %llu\n", stats.phase_shifts);
    printf("  Collapse Warnings: %llu\n", stats.collapse_warnings);
    printf("  Self-Heal Count:   %llu\n", stats.self_heal_count);
    printf("  Keys Generated:    %llu\n\n", stats.total_keys_generated);

    usleep(100000); /* 100ms delay */
  }

  /* Cleanup */
  hc_vacuum_free_context(ctx);

  printf("Monitoring complete.\n");
  return 0;
}

/**
 * Expected Output:
 *
 * Vortex v2.0 Monitoring Example
 * ================================
 *
 * Sample 1:
 *   Lyapunov Exponent: 0.087432 [✓ HEALTHY]
 *   Phase Shifts:      0
 *   Collapse Warnings: 0
 *   Self-Heal Count:   0
 *   Keys Generated:    32
 *
 * Sample 2:
 *   Lyapunov Exponent: 0.091205 [✓ HEALTHY]
 *   Phase Shifts:      0
 *   Collapse Warnings: 0
 *   Self-Heal Count:   0
 *   Keys Generated:    64
 *
 * [... if chaos degrades ...]
 *
 * Sample 8:
 *   Lyapunov Exponent: 0.034512 [⚠ MARGINAL]
 *   Phase Shifts:      0
 *   Collapse Warnings: 3
 *   Self-Heal Count:   0
 *   Keys Generated:    256
 *
 * Sample 9:
 *   Lyapunov Exponent: 0.102384 [✓ HEALTHY]  ← Phase shift restored chaos
 *   Phase Shifts:      1
 *   Collapse Warnings: 0
 *   Self-Heal Count:   0
 *   Keys Generated:    288
 */
