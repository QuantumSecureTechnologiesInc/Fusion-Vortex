#ifndef hc_CONSTANT_TIME_H
#define hc_CONSTANT_TIME_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


/**
 * Constant-Time Primitives for HyperCycle v3.2 Fulminis
 *
 * All operations execute in time independent of input values.
 * Uses bit masking and arithmetic operations to avoid conditional branches.
 * Includes compiler barriers to prevent optimisation removal.
 */

/**
 * Constant-time byte comparison.
 * Returns 0 if equal, non-zero otherwise.
 * Execution time independent of input values or position of first difference.
 */
int hc_ct_memcmp(const void *a, const void *b, size_t len);

/**
 * Constant-time conditional select (8-bit).
 * Returns a if condition != 0, else b.
 * No branching - uses bit masking.
 */
uint8_t hc_ct_select_u8(uint8_t a, uint8_t b, int condition);

/**
 * Constant-time conditional select (32-bit).
 */
uint32_t hc_ct_select_u32(uint32_t a, uint32_t b, int condition);

/**
 * Constant-time conditional select (64-bit).
 */
uint64_t hc_ct_select_u64(uint64_t a, uint64_t b, int condition);

/**
 * Constant-time equality test.
 * Returns 1 if a == b, 0 otherwise.
 * Uses arithmetic to avoid branching.
 */
int hc_ct_equal_u32(uint32_t a, uint32_t b);

/**
 * Constant-time less-than test.
 * Returns 1 if a < b, 0 otherwise.
 */
int hc_ct_lt_u32(uint32_t a, uint32_t b);

/**
 * Constant-time array lookup.
 * Access time independent of index value.
 * Accesses all table entries using masking.
 */
void hc_ct_lookup(const uint8_t *table, size_t table_size, size_t entry_size,
                  size_t index, uint8_t *out);

/**
 * Constant-time conditional copy.
 * Copies src to dest if condition != 0, otherwise does nothing.
 * Always performs the same memory accesses regardless of condition.
 */
void hc_ct_copy(void *dest, const void *src, size_t len, int condition);

/**
 * Constant-time zero check.
 * Returns 1 if value is zero, 0 otherwise.
 */
int hc_ct_is_zero_u32(uint32_t value);

#endif // hc_CONSTANT_TIME_H
