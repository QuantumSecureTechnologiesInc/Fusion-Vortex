#ifndef hc_SIDECHANNEL_H
#define hc_SIDECHANNEL_H

#include <stddef.h>
#include <stdint.h>

/**
 * Side-Channel Resistance for HyperCycle v3.2 Fulminis
 *
 * Implements cache-oblivious algorithms and scatter-gather memory access
 * to prevent cache-timing and power analysis attacks.
 */

/**
 * Cache-oblivious table lookup.
 * Accesses all table entries to prevent cache-timing leaks.
 * Uses constant-time masking to select correct entry.
 */
void hc_sc_table_lookup(const uint8_t *table, size_t table_size,
                        size_t entry_size, size_t index, uint8_t *out);

/**
 * Scatter-gather memory copy.
 * Obfuscates access patterns via controlled ordering.
 * Not randomised (deterministic for same input) but non-sequential.
 */
void hc_sc_memcpy(uint8_t *dest, const uint8_t *src, size_t len);

/**
 * Cache line flush (architecture-specific).
 * Forces data out of CPU caches to prevent residual analysis.
 */
void hc_sc_cache_flush(void *ptr, size_t len);

/**
 * Blinded quaternion multiplication.
 * Adds random masking to prevent power analysis of quaternion operations.
 */
void hc_sc_quaternion_mul_blinded(const void *a, const void *b, void *out,
                                  const uint8_t *random_mask, size_t mask_len);

#endif // hc_SIDECHANNEL_H
