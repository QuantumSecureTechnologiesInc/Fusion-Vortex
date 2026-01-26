/**
 * Data Integrity Implementation
 * Implementation of CRC32 and protected key logic
 */

#include "internal/integrity.h"
#include <string.h>

// Pre-computed CRC table could be used for speed, but
// byte-by-byte is sufficient for small keys and code compactness.

// Zero-Overhead Fault Accumulator (Thread-Local)
#ifndef hc_THREAD_LOCAL
#if defined(__GNUC__) || defined(__clang__)
#define hc_THREAD_LOCAL __thread
#elif defined(_MSC_VER)
#define hc_THREAD_LOCAL __declspec(thread)
#else
#define hc_THREAD_LOCAL _Thread_local
#endif
#endif

// Global thread-local accumulator for branchless checks
hc_THREAD_LOCAL uint64_t g_hc_fault_accumulator = 0;

int hc_check_fault_status(void) {
  if (g_hc_fault_accumulator != 0) {
    return -1; // Fault detected
  }
  return 0;
}

void hc_clear_fault_status(void) { g_hc_fault_accumulator = 0; }

uint32_t hc_crc32(const uint8_t *data, size_t len) {
  uint32_t crc = 0xFFFFFFFF;

  for (size_t i = 0; i < len; i++) {
    crc ^= (uint32_t)data[i] << 24;
    for (int j = 0; j < 8; j++) {
      if (crc & 0x80000000) {
        crc = (crc << 1) ^ hc_CRC32_POLY;
      } else {
        crc <<= 1;
      }
    }
  }

  return ~crc;
}

void hc_protect_key(const uint8_t *key, size_t len, hc_protected_key_t *out) {
  if (!key || !out)
    return;

  // In a real implementation, 'data' might be copied to a secure region.
  // Here we wrap the pointer and compute the checksum.
  // NOTE: The user is responsible for the lifetime of 'data'.
  out->data = (uint8_t *)key; // Cast away const for storage, treated as const
  out->len = len;
  out->crc32 = hc_crc32(key, len);
}

int hc_verify_key_integrity(const hc_protected_key_t *key) {
  if (!key || !key->data)
    return -1;

  uint32_t current_crc = hc_crc32(key->data, key->len);

  if (current_crc != key->crc32) {
    return -1; // Curruption/Fault detected
  }

  return 0;
}
