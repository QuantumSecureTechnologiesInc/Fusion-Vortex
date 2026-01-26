// HyperCycle.c – Core library entry point

#include "vortex/public/hypercycle_algorithms.h"
#ifdef _WIN32
#include <stdio.h>
#include <windows.h>

#ifndef WIN32_NO_STATUS
#define WIN32_NO_STATUS
#endif
#undef WIN32_NO_STATUS
#include <bcrypt.h>
#include <ntstatus.h>

#pragma comment(lib, "bcrypt.lib")
#else
#include <fcntl.h>
#include <unistd.h>
#endif

static int rng_initialized = 0;

int hc_initialize(void) {
#ifdef _WIN32
  // BCryptGenRandom does not require explicit context init
  rng_initialized = 1;
  return hc_SUCCESS;
#else
  // Verify /dev/urandom is accessible
  int fd = open("/dev/urandom", O_RDONLY);
  if (fd < 0) {
    return hc_ERROR_ENTROPY_FAILURE;
  }
  close(fd);
  rng_initialized = 1;
  return hc_SUCCESS;
#endif
}

void hc_cleanup(void) {
  // Currently no dynamic allocations; just reset state
  rng_initialized = 0;
  // Future: securely erase any allocated pools here
}
