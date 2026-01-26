/**
 * Data Integrity and Fault Injection Protection
 * Phase 3 Security Enhancement
 *
 * Provides CRC protection for critical key material and
 * redundancy checks for fault injection resistance.
 */

#ifndef hc_INTEGRITY_H
#define hc_INTEGRITY_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// CRC32 Polynomial (0x04C11DB7)
#define hc_CRC32_POLY 0x04C11DB7

/**
 * Protected key structure with integrity check
 * Wraps key data with a CRC32 checksum
 */
typedef struct {
  uint8_t *data;  // Pointer to actual key data
  size_t len;     // Length of key data
  uint32_t crc32; // Integrity checksum
} hc_protected_key_t;

/**
 * Compute CRC32 for data integrity
 * Hardware-accelerated where available (ARM/Intel), software fallback
 */
uint32_t hc_crc32(const uint8_t *data, size_t len);

/**
 * Protect a key by calculating its checksum
 *
 * @param key Input key bytes
 * @param len Key length
 * @param out Output protected structure (must be allocated)
 */
void hc_protect_key(const uint8_t *key, size_t len, hc_protected_key_t *out);

/**
 * Verify key integrity against checksum
 *
 * @param key Protected key structure
 * @return 0 if valid, -1 if corruption detected
 */
int hc_verify_key_integrity(const hc_protected_key_t *key);

void hc_clear_fault_status(void);
int hc_check_fault_status(void);

// Thread-Local Accumulator (exposed for inline macro speed)
#if defined(__GNUC__) || defined(__clang__)
extern __thread uint64_t g_hc_fault_accumulator;
#elif defined(_MSC_VER)
extern __declspec(thread) uint64_t g_hc_fault_accumulator;
#else
extern _Thread_local uint64_t g_hc_fault_accumulator;
#endif

/**
 * PARANOID_MODE Macro
 * If defined in build (-DPARANOID_MODE), critical operations
 * will use dual-computation redundancy.
 */
#ifdef PARANOID_MODE
// Branchless Redundant Check: Accumulates faults into thread-local variable
// Execution continues; caller must check hc_check_fault_status() at end.
#define hc_REDUNDANT_CHECK(func_call, expected)                                \
  do {                                                                         \
    int _res1 = (func_call);                                                   \
    int _res2 = (func_call);                                                   \
    /* XOR to find differences (branchless) */                                 \
    uint64_t _diff = ((uint64_t)_res1 ^ (uint64_t)_res2) |                     \
                     ((uint64_t)_res1 ^ (uint64_t)(expected));                 \
    g_hc_fault_accumulator |= _diff;                                           \
  } while (0)
#else
// Standard check (Branchless Accumulation even in standard mode?
// Or stick to return? Prompt says optimization.
// Let's use branchless accumulation here too for consistency if desired,
// but standard mode usually doesn't do double check.
// Standard mode: just check expectation.
#define hc_REDUNDANT_CHECK(func_call, expected)                                \
  do {                                                                         \
    int _res = (func_call);                                                    \
    g_hc_fault_accumulator |= ((uint64_t)_res ^ (uint64_t)(expected));         \
  } while (0)
#endif

#ifdef __cplusplus
}
#endif

#endif // hc_INTEGRITY_H
