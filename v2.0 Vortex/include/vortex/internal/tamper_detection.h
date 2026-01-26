/**
 * Tamper Detection and Code Integrity
 * Phase 2 Security Enhancement - Zero Overhead
 */

#ifndef hc_TAMPER_DETECTION_H
#define hc_TAMPER_DETECTION_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Compute SHA3-256 hash of function code
 * Used for runtime integrity verification
 */
void hc_compute_function_hash(const void *func_ptr, size_t func_size,
                              uint8_t *hash_out);

/**
 * Verify library integrity at startup
 * Checks critical functions against build-time hashes
 *
 * @return 0 on success, -1 if tampering detected
 */
int hc_verify_library_integrity(void);

/**
 * Enable stack canary protection
 * This is a compile-time feature, enabled via:
 * -fstack-protector-strong (GCC/Clang)
 *
 * No runtime overhead - compiler inserts guards automatically
 */
#define hc_ENABLE_STACK_PROTECTION // Marker for build system

#ifdef __cplusplus
}
#endif

#endif // hc_TAMPER_DETECTION_H
