#ifndef hc_SECURE_MEMORY_H
#define hc_SECURE_MEMORY_H

#include <stddef.h>
#include <stdint.h>

/**
 * Allocate memory locked in RAM (never swapped to disk).
 * Uses VirtualLock (Windows) or mlock (POSIX).
 * 
 * @param size Size of allocation in bytes
 * @return Pointer to locked memory, or NULL on failure
 */
void* hc_secure_alloc(size_t size);

/**
 * Free secure memory with guaranteed zeroization.
 * Uses SecureZeroMemory (Windows) or explicit_bzero (POSIX).
 * 
 * @param ptr  Pointer to secure memory
 * @param size Size of allocation (for zeroization)
 */
void hc_secure_free(void *ptr, size_t size);

#endif // hc_SECURE_MEMORY_H
