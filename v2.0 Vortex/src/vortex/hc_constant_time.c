#include "../../include/hc_constant_time.h"
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

  for (size_t i = 0; i < len; i++) {
    diff |= pa[i] ^ pb[i];
  }

  hc_ct_barrier();
  return diff;
}

uint8_t hc_ct_select_u8(uint8_t a, uint8_t b, int condition) {
  uint8_t mask = (uint8_t)(-(condition != 0));
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
  // Reduce to 0/1 using bit propagation
  uint32_t v = diff;
  v |= (v >> 16);
  v |= (v >> 8);
  v |= (v >> 4);
  v |= (v >> 2);
  v |= (v >> 1);
  return (int)(1 - (v & 1));
}

int hc_ct_is_zero_u32(uint32_t value) {
  uint32_t v = value;
  v |= (v >> 16);
  v |= (v >> 8);
  v |= (v >> 4);
  v |= (v >> 2);
  v |= (v >> 1);
  return (int)(1 - (v & 1));
}

void hc_ct_lookup(const uint8_t *table, size_t table_size, size_t entry_size,
                  size_t index, uint8_t *out) {
  if (!table || !out || table_size == 0 || entry_size == 0)
    return;

  size_t safe_index = index % table_size;
  memset(out, 0, entry_size);

  for (size_t i = 0; i < table_size; i++) {
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
    d[i] = (s[i] & mask) | (d[i] & ~mask);
  }
  hc_ct_barrier();
}
