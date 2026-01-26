#ifndef HC_CONSTANT_TIME_H
#define HC_CONSTANT_TIME_H

#include <stddef.h>
#include <stdint.h>

/**
 * Constant-Time Primitives for HyperCycle Vortex v2.0
 * PORTED FROM: CryptoGuard Core Integration
 *
 * All operations execute in time independent of input values.
 * Uses bit masking and arithmetic operations to avoid conditional branches.
 */

// Constant-time byte comparison (0=equal, non-zero=diff)
int hc_ct_memcmp(const void *a, const void *b, size_t len);

// Constant-time conditional select
uint8_t hc_ct_select_u8(uint8_t a, uint8_t b, int condition);
uint32_t hc_ct_select_u32(uint32_t a, uint32_t b, int condition);
uint64_t hc_ct_select_u64(uint64_t a, uint64_t b, int condition);

// Constant-time equality/zero checks
int hc_ct_equal_u32(uint32_t a, uint32_t b);
int hc_ct_is_zero_u32(uint32_t value);

// Constant-time array lookup (Oblivious Memory Access)
void hc_ct_lookup(const uint8_t *table, size_t table_size, size_t entry_size,
                  size_t index, uint8_t *out);

// Constant-time conditional copy
void hc_ct_copy(void *dest, const void *src, size_t len, int condition);

#endif // HC_CONSTANT_TIME_H
