#include "vortex/public/hc_secure_memory.h"
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
#include <windows.h>
#else
#include <sys/mman.h>
#include <unistd.h>
#endif

void *hc_secure_alloc(size_t size) {
  if (size == 0)
    return NULL;

  // Align to 64-byte cache lines
  size_t aligned_size = ((size + 63) / 64) * 64;

#ifdef _WIN32
  // Windows: VirtualAlloc + VirtualLock
  void *ptr = VirtualAlloc(NULL, aligned_size, MEM_COMMIT | MEM_RESERVE,
                           PAGE_READWRITE);
  if (!ptr)
    return NULL;

  if (!VirtualLock(ptr, aligned_size)) {
    VirtualFree(ptr, 0, MEM_RELEASE);
    return NULL;
  }
#else
  // POSIX: malloc + mlock
  void *ptr = malloc(aligned_size);
  if (!ptr)
    return NULL;

  if (mlock(ptr, aligned_size) != 0) {
    free(ptr);
    return NULL;
  }
#endif

  return ptr;
}

void hc_secure_free(void *ptr, size_t size) {
  if (!ptr || size == 0)
    return;

  size_t aligned_size = ((size + 63) / 64) * 64;

#ifdef _WIN32
  // Zeroize with SecureZeroMemory
  SecureZeroMemory(ptr, aligned_size);
  VirtualUnlock(ptr, aligned_size);
  VirtualFree(ptr, 0, MEM_RELEASE);
#else
// Zeroize with explicit_bzero or memset
#ifdef __GLIBC__
  explicit_bzero(ptr, aligned_size);
#else
  // Volatile to prevent optimizer from removing memset
  volatile uint8_t *p = (volatile uint8_t *)ptr;
  for (size_t i = 0; i < aligned_size; i++) {
    p[i] = 0;
  }
#endif
  munlock(ptr, aligned_size);
  free(ptr);
#endif
}
// Secure wipe implementation
void secure_wipe(void *ptr, size_t len) {
  if (!ptr || len == 0)
    return;
#ifdef _WIN32
  SecureZeroMemory(ptr, len);
#else
  ptr = (volatile void *)ptr;
  volatile uint8_t *p = (volatile uint8_t *)ptr;
  while (len--)
    *p++ = 0;
#endif
}
