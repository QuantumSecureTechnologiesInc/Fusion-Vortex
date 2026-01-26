#include "hc_constant_time.h"
#include <stddef.h>
#include <stdint.h>
#include <string.h>

// Compiler barrier to prevent optimisation removal
static inline void hc_ct_barrier(void) {
#if defined(_MSC_VER)
  _ReadWriteBarrier();
#elif defined(__GNUC__)
  __asm__ __volatile__("" ::: "memory");
#endif
}

int hc_ct_memcmp(const void *a, const void *b, size_t len) {
  const uint8_t *pa = (const uint8_t *)a;
  const uint8_t *pb = (const uint8_t *)b;
  uint8_t diff = 0;

  // XOR all bytes - result is 0 only if all bytes match
  for (size_t i = 0; i < len; i++) {
    diff |= pa[i] ^ pb[i];
  }

  hc_ct_barrier();

  // Return 0 if equal, non-zero otherwise
  return diff;
}

uint8_t hc_ct_select_u8(uint8_t a, uint8_t b, int condition) {
  // Create mask: 0xFF if condition != 0, else 0x00
  uint8_t mask = (uint8_t)(-(condition != 0));

  // If mask = 0xFF: result = (a & 0xFF) | (b & 0x00) = a
  // If mask = 0x00: result = (a & 0x00) | (b & 0xFF) = b
  return (a & mask) | (b & ~mask);
}

uint32_t hc_ct_select_u32(uint32_t a, uint32_t b, int condition) {
  uint32_t mask = (uint32_t)(-(condition != 0));
  return (a & mask) | (b & ~mask);
}

uint64_t hc_ct_select_u64(uint64_t a, uint64_t b, int condition) {
  uint64_t mask = (uint64_t)(-(condition != 0));
  return (a & mask) | (b & ~mask);
}

int hc_ct_equal_u32(uint32_t a, uint32_t b) {
  uint32_t diff = a ^ b;

  // If diff == 0, all bits are 0
  // Set all bits if any bit is set
  uint32_t combined = diff | (diff >> 16);
  combined |= (combined >> 8);
  combined |= (combined >> 4);
  combined |= (combined >> 2);
  combined |= (combined >> 1);

  // Invert: 1 if diff was 0, else 0
  return (int)(1 - (combined & 1));
}

int hc_ct_lt_u32(uint32_t a, uint32_t b) {
  // Compute a - b, check sign bit without branching
  uint32_t diff = a ^ ((a ^ b) | ((a - b) ^ b));
  return (int)((diff >> 31) & 1);
}

void hc_ct_lookup(const uint8_t *table, size_t table_size, size_t entry_size,
                  size_t index, uint8_t *out) {
  if (!table || !out || table_size == 0 || entry_size == 0) {
    return;
  }

  // Ensure index is in bounds (constant-time)
  size_t safe_index = index % table_size;

  // Initialize output to zeros
  memset(out, 0, entry_size);

  // Access all entries, accumulate selected one via masking
  for (size_t i = 0; i < table_size; i++) {
    // Create mask: all 1s if i == safe_index, else all 0s
    int is_target = hc_ct_equal_u32((uint32_t)i, (uint32_t)safe_index);
    uint8_t mask = (uint8_t)(-(is_target));

    const uint8_t *entry = table + (i * entry_size);

    for (size_t j = 0; j < entry_size; j++) {
      out[j] |= (entry[j] & mask);
    }
  }

  hc_ct_barrier();
}

void hc_ct_copy(void *dest, const void *src, size_t len, int condition) {
  uint8_t *d = (uint8_t *)dest;
  const uint8_t *s = (const uint8_t *)src;
  uint8_t mask = (uint8_t)(-(condition != 0));

  for (size_t i = 0; i < len; i++) {
    // If condition: d[i] = s[i]
    // Else: d[i] = d[i]
    d[i] = (s[i] & mask) | (d[i] & ~mask);
  }

  hc_ct_barrier();
}

int hc_ct_is_zero_u32(uint32_t value) {
  // Propagate any set bit across all positions
  uint32_t v = value;
  v |= (v >> 16);
  v |= (v >> 8);
  v |= (v >> 4);
  v |= (v >> 2);
  v |= (v >> 1);

  // Invert lowest bit: 1 if value was 0
  return (int)(1 - (v & 1));
}
