#ifndef HC_SBOX16_H
#define HC_SBOX16_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
  HC_MAP_POLY = 0,
  HC_MAP_TENT = 1
} hc_map_mode_t;

/**
 * Initialize the 16-bit S-Box table deterministically for a given mode.
 * Thread-safe: idempotent; repeated calls with same mode are cheap.
 */
void hc_sbox16_init(hc_map_mode_t mode);

/** Returns a pointer to the initialized 16-bit table (65536 entries). */
const uint16_t *hc_sbox16_table(void);

/** Applies one S-Box step to a 64-bit word by transforming 4x16-bit chunks. */
uint64_t hc_sbox16_step_u64(uint64_t x);

#ifdef __cplusplus
}
#endif

#endif /* HC_SBOX16_H */
