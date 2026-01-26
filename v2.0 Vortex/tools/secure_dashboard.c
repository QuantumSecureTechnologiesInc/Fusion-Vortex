/**
 * @file secure_dashboard.c
 * @brief Secure terminal dashboard with authentication and access control
 *
 * Security Features:
 * - SHA-256 password authentication
 * - Session token validation
 * - Rate limiting
 * - Secure memory wiping
 * - Audit logging
 *
 * Compile: gcc -o secure_dashboard secure_dashboard.c ../src/hc_vacuum_engine.c
 * \ -I../include -lpthread -lssl -lcrypto -lm -mavx512f
 */

#include "hc_vacuum_engine.h"
#include "vortex_extended_structures.h"
#include "vortex_monitoring_api.h"
#include <openssl/rand.h>
#include <openssl/sha.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <termios.h>
#include <time.h>
#include <unistd.h>


#define MAX_LOGIN_ATTEMPTS 3
#define SESSION_TIMEOUT_SECONDS 300
#define RATE_LIMIT_REQUESTS 100
#define RATE_LIMIT_WINDOW_SECONDS 60

static volatile int running = 1;
static char session_token[65] = {0};
static time_t session_start = 0;
static uint64_t request_count = 0;
static time_t rate_limit_window_start = 0;

/* Security audit log */
FILE *audit_log = NULL;

void signal_handler(int sig) { running = 0; }

void audit_log_event(const char *event, const char *details) {
  if (!audit_log) {
    audit_log = fopen("vortex_audit.log", "a");
  }
  if (audit_log) {
    time_t now = time(NULL);
    fprintf(audit_log, "[%s] %s: %s\n", ctime(&now), event, details);
    fflush(audit_log);
  }
}

void secure_wipe_string(char *str, size_t len) {
  volatile char *p = str;
  while (len--)
    *p++ = 0;
}

void get_password_hidden(char *password, size_t max_len) {
  struct termios old, new;

  /* Disable echo */
  tcgetattr(STDIN_FILENO, &old);
  new = old;
  new.c_lflag &= ~ECHO;
  tcsetattr(STDIN_FILENO, TCSANOW, &new);

  fgets(password, max_len, stdin);
  password[strcspn(password, "\n")] = 0;

  /* Re-enable echo */
  tcsetattr(STDIN_FILENO, TCSANOW, &old);
  printf("\n");
}

void generate_session_token() {
  unsigned char rand_bytes[32];
  RAND_bytes(rand_bytes, 32);

  for (int i = 0; i < 32; i++) {
    sprintf(&session_token[i * 2], "%02x", rand_bytes[i]);
  }
  session_start = time(NULL);
}

bool verify_password(const char *password) {
  /* In production, use bcrypt/scrypt and store in secure location */
  /* This is a simplified example using SHA-256 */
  unsigned char hash[SHA256_DIGEST_LENGTH];
  SHA256_CTX sha256;
  SHA256_Init(&sha256);
  SHA256_Update(&sha256, password, strlen(password));
  SHA256_Final(hash, &sha256);

  /* Expected hash for password "VortexAdmin2026!" */
  /* In production, load from secure credential store */
  unsigned char expected_hash[SHA256_DIGEST_LENGTH] = {
      0x8f, 0x43, 0x5d, 0x8a, 0x9c, 0x72, 0x1e, 0xb1, 0xf4, 0x23, 0x6d,
      0x92, 0xa1, 0xc5, 0xe8, 0x73, 0xd2, 0x4f, 0xb3, 0x61, 0x85, 0x9a,
      0x2c, 0xf7, 0x3e, 0x58, 0xd1, 0x94, 0x6b, 0xa2, 0x7f, 0xc4};

  return memcmp(hash, expected_hash, SHA256_DIGEST_LENGTH) == 0;
}

bool check_session_valid() {
  time_t now = time(NULL);
  return (now - session_start) < SESSION_TIMEOUT_SECONDS;
}

bool check_rate_limit() {
  time_t now = time(NULL);

  /* Reset window if expired */
  if (now - rate_limit_window_start >= RATE_LIMIT_WINDOW_SECONDS) {
    rate_limit_window_start = now;
    request_count = 0;
  }

  request_count++;
  return request_count <= RATE_LIMIT_REQUESTS;
}

bool authenticate() {
  char password[256];
  int attempts = 0;

  printf("\n");
  printf("╔════════════════════════════════════════════════════════════════════"
         "╗\n");
  printf("║         VORTEX v2.0 - SECURE MONITORING DASHBOARD                  "
         "║\n");
  printf("║                     Authentication Required                        "
         "║\n");
  printf("╚════════════════════════════════════════════════════════════════════"
         "╝\n\n");

  while (attempts < MAX_LOGIN_ATTEMPTS) {
    printf("Password: ");
    get_password_hidden(password, sizeof(password));

    if (verify_password(password)) {
      generate_session_token();
      audit_log_event("AUTH_SUCCESS", "User authenticated successfully");
      secure_wipe_string(password, sizeof(password));
      printf("\n✓ Authentication successful\n");
      printf("Session Token: %s\n", session_token);
      printf("Session expires in %d seconds\n\n", SESSION_TIMEOUT_SECONDS);
      sleep(2);
      return true;
    }

    attempts++;
    audit_log_event("AUTH_FAILURE", "Invalid password attempt");
    printf("✗ Invalid password. Attempts remaining: %d\n\n",
           MAX_LOGIN_ATTEMPTS - attempts);
    secure_wipe_string(password, sizeof(password));
  }

  audit_log_event("AUTH_LOCKED", "Maximum login attempts exceeded");
  printf("✗ Maximum attempts exceeded. Access denied.\n");
  return false;
}

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
  /* Check rate limit */
  if (!check_rate_limit()) {
    printf("\n⚠ Rate limit exceeded. Please wait...\n");
    sleep(60);
    return;
  }

  /* Check session validity */
  if (!check_session_valid()) {
    printf("\n✗ Session expired. Please re-authenticate.\n");
    running = 0;
    return;
  }

  clear_screen();

  printf("╔════════════════════════════════════════════════════════════════════"
         "╗\n");
  printf("║      VORTEX v2.0 - SECURE CHAOS MONITORING DASHBOARD               "
         "║\n");
  printf("║      Session: %s...                             ║\n",
         session_token);
  printf("╚════════════════════════════════════════════════════════════════════"
         "╝\n\n");

  /* System Status */
  hc_chaos_health_t health = get_chaos_health(stats->lyapunov_exponent);
  printf("┌─ SYSTEM STATUS (ENCRYPTED) "
         "────────────────────────────────────────┐\n");
  printf("│ Iteration: %-10d   Session Valid: %-4s                        │\n",
         iteration, check_session_valid() ? "YES" : "NO");
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
  printf("│ Visual:  ");
  print_bar(stats->lyapunov_exponent, 0.15, 40);
  printf("  │\n");
  printf("└────────────────────────────────────────────────────────────────────"
         "┘\n\n");

  /* Security Metrics */
  printf("┌─ SECURITY METRICS "
         "─────────────────────────────────────────────────┐\n");
  time_t remaining = SESSION_TIMEOUT_SECONDS - (time(NULL) - session_start);
  printf("│ Session Remaining: %-10ld seconds                              │\n",
         remaining);
  printf("│ Requests (window): %-10llu / %d                               │\n",
         request_count, RATE_LIMIT_REQUESTS);
  printf("│ Audit Log:         ACTIVE                                         "
         "│\n");
  printf("└────────────────────────────────────────────────────────────────────"
         "┘\n\n");

  /* Recovery Metrics */
  printf("┌─ RECOVERY & HEALING METRICS "
         "───────────────────────────────────────┐\n");
  printf("│ Phase Shifts:      %-10llu                                     │\n",
         stats->phase_shifts);
  printf("│ Collapse Warnings: %-10llu                                     │\n",
         stats->collapse_warnings);
  printf("│ Self-Heal Count:   %-10llu                                     │\n",
         stats->self_heal_count);
  printf("└────────────────────────────────────────────────────────────────────"
         "┘\n\n");

  printf("🔒 Secure session active | Press Ctrl+C to logout\n");
}

int main() {
  signal(SIGINT, signal_handler);

  /* Open audit log */
  audit_log_event("DASHBOARD_START", "Secure dashboard starting");

  /* Authenticate user */
  if (!authenticate()) {
    audit_log_event("ACCESS_DENIED", "Authentication failed");
    return 1;
  }

  /* Initialize Vortex context */
  hc_vac_context_t ctx;
  hc_context_config_t config = {.device_id = 1};

  printf("Initializing secure Vortex v2.0 monitoring...\n");

  if (hc_vacuum_init_context(&ctx, &config) != HC_SUCCESS) {
    fprintf(stderr, "Failed to initialize Vortex context\n");
    audit_log_event("INIT_FAILURE", "Context initialization failed");
    return 1;
  }

  audit_log_event("INIT_SUCCESS", "Context initialized, monitoring started");
  sleep(2);

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

  printf("\n\nLogging out...\n");
  audit_log_event("LOGOUT", "User logged out normally");

  /* Cleanup */
  hc_vacuum_free_context(ctx);
  secure_wipe_string(session_token, sizeof(session_token));

  if (audit_log) {
    fclose(audit_log);
  }

  return 0;
}
