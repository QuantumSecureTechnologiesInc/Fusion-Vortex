/*
 * Vortex User CLI - v2.1.0 (TUI Edition)
 * ======================================
 * Features:
 * - Lip Gloss / Bubble Tea Aesthetic (via ANSI)
 * - 2FA Token Generation
 * - "Charmbracelet" style rounded borders and colors
 *
 * Compile: gcc vortex_user_cli.c -o vortex.exe
 */

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <windows.h>


// --- Configuration ---
#define TOTP_SECRET "VORTEX_QUANTUM_SECURE_KEY_2026"
#define TOTP_STEP 30

// --- ANSI Colors (Dracula / Charm Theme) ---
#define C_RESET "\033[0m"
#define C_BOLD "\033[1m"
#define C_DIM "\033[2m"
#define C_PINK "\033[38;2;255;121;198m"
#define C_PURPLE "\033[38;2;189;147;249m"
#define C_CYAN "\033[38;2;139;233;253m"
#define C_GREEN "\033[38;2;80;250;123m"
#define C_BG "\033[48;2;40;42;54m" // Dark background

// --- Unicode Borders (Rounded "Lip Gloss" Style) ---
// Note: Your terminal must support UTF-8.
// Windows CMD needs `chcp 65001`. The code handles this setup.
char *TL = "\xE2\x95\xAD"; // ╭
char *TR = "\xE2\x95\xAE"; // ╮
char *BL = "\xE2\x95\xB0"; // ╰
char *BR = "\xE2\x95\xAF"; // ╯
char *H = "\xE2\x94\x80";  // ─
char *V = "\xE2\x94\x82";  // │

// --- Helper: TOTP Logic (Same as before) ---
uint32_t string_hash(const char *str, uint64_t time_slice) {
  char buffer[256];
  sprintf(buffer, "%s-%llu", str, time_slice);
  uint32_t hash = 5381;
  int c;
  char *p = buffer;
  while ((c = *p++))
    hash = ((hash << 5) + hash) + c;
  return hash;
}

// --- Helper: Draw Box ---
void draw_box(char *title, char *content, char *footer, int remaining_sec) {
  int width = 42;
  int pad = (width - 2 - strlen(content)) / 2;

  // Top Border
  printf("  %s%s%s", C_PURPLE, TL, H);
  printf(" %s%s%s ", C_PINK, title, C_PURPLE);
  for (int i = 0; i < width - 5 - strlen(title); i++)
    printf("%s", H);
  printf("%s%s\n", TR, C_RESET);

  // Empty Line
  printf("  %s%s%*s%s%s\n", C_PURPLE, V, width - 2, "", V, C_RESET);

  // Content (Centered)
  printf("  %s%s", C_PURPLE, V);
  printf("%*s", pad, "");
  printf("%s%s%s", C_CYAN, content, C_PURPLE); // Code in Cyan
  printf("%*s", width - 2 - pad - strlen(content), "");
  printf("%s%s\n", V, C_RESET);

  // Progress Bar (Simulated)
  int bars = (remaining_sec * (width - 6)) / TOTP_STEP;
  printf("  %s%s  ", C_PURPLE, V);
  printf("%s", C_GREEN); // Green bar
  for (int i = 0; i < bars; i++)
    printf("━");
  printf("%s", C_DIM); // Dim background bar
  for (int i = bars; i < (width - 6); i++)
    printf("─");
  printf("%s%s  %s%s\n", C_RESET, C_PURPLE, V, C_RESET);

  // Footer
  printf("  %s%s  %s%s%s", C_PURPLE, V, C_DIM, footer, C_PURPLE);
  printf("%*s", width - 6 - strlen(footer), "");
  printf("  %s%s\n", V, C_RESET);

  // Bottom Border
  printf("  %s%s", C_PURPLE, BL);
  for (int i = 0; i < width - 2; i++)
    printf("%s", H);
  printf("%s%s\n", BR, C_RESET);
}

void generate_2fa_ui() {
  time_t now = time(NULL);
  uint64_t time_slice = now / TOTP_STEP;
  uint32_t hash = string_hash(TOTP_SECRET, time_slice);
  int code = hash % 1000000;
  int remaining = TOTP_STEP - (now % TOTP_STEP);

  char code_str[32];
  sprintf(code_str, "%06d", code); // Format: 123456

  // Split with space: 123 456 for readability
  char pretty_code[32];
  sprintf(pretty_code, "%c%c%c %c%c%c", code_str[0], code_str[1], code_str[2],
          code_str[3], code_str[4], code_str[5]);

  char footer[64];
  sprintf(footer, "Valid for %ds", remaining);

  printf("\n");
  draw_box("VORTEX 2FA", pretty_code, footer, remaining);
  printf("\n");
}

int main(int argc, char *argv[]) {
  // Windows: Enable UTF-8 and ANSI sequences
  SetConsoleOutputCP(CP_UTF8);
  HANDLE hOut = GetStdHandle(STD_OUTPUT_HANDLE);
  DWORD dwMode = 0;
  GetConsoleMode(hOut, &dwMode);
  dwMode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
  SetConsoleMode(hOut, dwMode);

  if (argc < 2) {
    printf("Usage: vortex 2fa\n");
    return 0;
  }

  if (strcmp(argv[1], "2fa") == 0) {
    generate_2fa_ui();
  } else {
    printf("Unknown command.\n");
  }
  return 0;
}
