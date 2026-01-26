#include "vortex/public/hypercycle_temporal.h"
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef _WIN32
#include <windows.h>
#else
#include <time.h>
#endif

int temporal_protection_init(temporal_protection_t *guard) {
  if (!guard)
    return -1;

  guard->violations_prevented = 0;

#ifdef _WIN32
  LARGE_INTEGER freq, counter;
  QueryPerformanceFrequency(&freq);
  QueryPerformanceCounter(&counter);
  // Store in timespec format for consistency
  guard->last_timestamp.tv_sec = counter.QuadPart / freq.QuadPart;
  guard->last_timestamp.tv_nsec =
      ((counter.QuadPart % freq.QuadPart) * 1000000000) / freq.QuadPart;
#else
  clock_gettime(CLOCK_MONOTONIC, &guard->last_timestamp);
#endif

  return 0;
}

bool temporal_protection_check_violation(temporal_protection_t *guard) {
  if (!guard)
    return false;

#ifdef _WIN32
  LARGE_INTEGER freq, current_counter;
  QueryPerformanceFrequency(&freq);
  QueryPerformanceCounter(&current_counter);

  struct timespec current_time;
  current_time.tv_sec = current_counter.QuadPart / freq.QuadPart;
  current_time.tv_nsec =
      ((current_counter.QuadPart % freq.QuadPart) * 1000000000) / freq.QuadPart;

  // Check if current time is before last time (temporal violation)
  if (current_time.tv_sec < guard->last_timestamp.tv_sec ||
      (current_time.tv_sec == guard->last_timestamp.tv_sec &&
       current_time.tv_nsec < guard->last_timestamp.tv_nsec)) {
    guard->violations_prevented++;
    return true;
  }

  // Check for impossibly fast operations (<1ns)
  uint64_t elapsed_ns =
      (current_time.tv_sec - guard->last_timestamp.tv_sec) * 1000000000ULL +
      (current_time.tv_nsec - guard->last_timestamp.tv_nsec);

  if (elapsed_ns < 1) {
    guard->violations_prevented++;
    return true;
  }

  guard->last_timestamp = current_time;
#else
  struct timespec current_time;
  clock_gettime(CLOCK_MONOTONIC, &current_time);

  // Check if current time is before last time
  if (current_time.tv_sec < guard->last_timestamp.tv_sec ||
      (current_time.tv_sec == guard->last_timestamp.tv_sec &&
       current_time.tv_nsec < guard->last_timestamp.tv_nsec)) {
    guard->violations_prevented++;
    return true;
  }

  // Check for impossibly fast operations (<1ns)
  uint64_t elapsed_ns =
      (current_time.tv_sec - guard->last_timestamp.tv_sec) * 1000000000ULL +
      (current_time.tv_nsec - guard->last_timestamp.tv_nsec);

  if (elapsed_ns < 1) {
    guard->violations_prevented++;
    return true;
  }

  guard->last_timestamp = current_time;
#endif

  return false;
}
