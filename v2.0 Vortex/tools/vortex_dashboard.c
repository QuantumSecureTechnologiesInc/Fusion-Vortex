/**
 * @file vortex_dashboard.c
 * @brief Real-time terminal dashboard for monitoring Vortex v2.0 chaos health
 *
 * Compile: gcc -o vortex_dashboard vortex_dashboard.c ../src/hc_vacuum_engine.c
 * \ -I../include -lpthread -lssl -lcrypto -lm -mavx512f
 *
 * Run: ./vortex_dashboard
 */

#include "hc_vacuum_engine.h"
#include "vortex_monitoring_api.h"
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>


static volatile int running = 1;

void signal_handler(int sig) { running = 0; }

void clear_screen() { printf("\033[2J\033[H"); }

void print_bar(double value, double max, int width) {
  int filled = (int)((value / max) * width);
  printf("[");
  for (int i = 0; i < width; i++) {
    if (i < filled) {
      printf("█");
    } else {
      printf("░");
    }
  }
  printf("]");
}

void print_dashboard(hc_telemetry_extended_t *stats, int iteration) {
  clear_screen();

  printf("╔════════════════════════════════════════════════════════════════════"
         "╗\n");
  printf("║          VORTEX v2.0 - CHAOS MONITORING DASHBOARD                  "
         "║\n");
  printf("╚════════════════════════════════════════════════════════════════════"
         "╝\n\n");

  /* System Status */
  hc_chaos_health_t health = get_chaos_health(stats->lyapunov_exponent);
  printf("┌─ SYSTEM STATUS "
         "────────────────────────────────────────────────────┐\n");
  printf("│ Iteration: %-10d                                               │\n",
         iteration);
  printf("│ Status:    ");
  switch (health) {
  case HC_CHAOS_HEALTHY:
    printf("\033[32m● HEALTHY  \033[0m (Chaos strong)                          "
           "    │\n");
    break;
  case HC_CHAOS_MARGINAL:
    printf("\033[33m◐ MARGINAL \033[0m (Chaos degrading)                       "
           "    │\n");
    break;
  case HC_CHAOS_CRITICAL:
    printf("\033[31m○ CRITICAL \033[0m (Chaos collapsing)                      "
           "    │\n");
    break;
  }
  printf("└────────────────────────────────────────────────────────────────────"
         "┘\n\n");

  /* Lyapunov Exponent */
  printf("┌─ LYAPUNOV EXPONENT (Chaos Indicator) "
         "──────────────────────────────┐\n");
  printf("│ Current: %.6f                                                  │\n",
         stats->lyapunov_exponent);
  printf("│ Target:  0.050000 (minimum for stable chaos)                      "
         "│\n");
  printf("│ Visual:  ");
  print_bar(stats->lyapunov_exponent, 0.15, 40);
  printf("  │\n");
  printf("└────────────────────────────────────────────────────────────────────"
         "┘\n\n");

  /* Recovery Metrics */
  printf("┌─ RECOVERY & HEALING METRICS "
         "───────────────────────────────────────┐\n");
  printf("│ Phase Shifts:      %-10llu (ergodic phase space jumps)        │\n",
         stats->phase_shifts);
  printf("│ Collapse Warnings: %-10llu (near-collapse detections)        │\n",
         stats->collapse_warnings);
  printf("│ Self-Heal Count:   %-10llu (automatic recovery operations)   │\n",
         stats->self_heal_count);
  printf("└────────────────────────────────────────────────────────────────────"
         "┘\n\n");

  /* Entropy Generation */
  printf("┌─ ENTROPY GENERATION "
         "───────────────────────────────────────────────┐\n");
  printf("│ Total Keys:        %-10llu bytes                              │\n",
         stats->total_keys_generated);
  printf("│ Total Batches:     %-10llu requests                           │\n",
         stats->total_batches);
  printf("│ Last Batch Time:   %.6f seconds                              │\n",
         stats->last_batch_time_sec);
  printf("└────────────────────────────────────────────────────────────────────"
         "┘\n\n");

  /* Real-time indicators */
  printf("┌─ LIVE INDICATORS "
         "──────────────────────────────────────────────────┐\n");
  printf("│ ");
  if (health == HC_CHAOS_HEALTHY) {
    printf("\033[32m");
  } else if (health == HC_CHAOS_MARGINAL) {
    printf("\033[33m");
  } else {
    printf("\033[31m");
  }

  for (int i = 0; i < 65; i++) {
    if (i % 5 == (iteration % 5)) {
      printf("▓");
    } else {
      printf("░");
    }
  }
  printf("\033[0m │\n");
  printf("└────────────────────────────────────────────────────────────────────"
         "┘\n\n");

  printf("Press Ctrl+C to exit...\n");
}

int main() {
  signal(SIGINT, signal_handler);

  /* Initialize Vortex context */
  hc_vac_context_t ctx;
  hc_context_config_t config = {.device_id = 1};

  printf("Initializing Vortex v2.0 monitoring...\n");

  if (hc_vacuum_init_context(&ctx, &config) != HC_SUCCESS) {
    fprintf(stderr, "Failed to initialize Vortex context\n");
    return 1;
  }

  sleep(2); /* Let background worker populate data */

  uint8_t seed[32];
  int iteration = 0;

  while (running) {
    /* Generate entropy */
    hc_vacuum_generate_seed_safe(ctx, seed);

    /* Get telemetry */
    hc_telemetry_extended_t stats;
    hc_vacuum_get_telemetry(ctx, (hc_telemetry_t *)&stats);

    /* Display dashboard */
    print_dashboard(&stats, ++iteration);

    /* Update every 500ms */
    usleep(500000);
  }

  printf("\n\nShutting down...\n");
  hc_vacuum_free_context(ctx);

  return 0;
}
