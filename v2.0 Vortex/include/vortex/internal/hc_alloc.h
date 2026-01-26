#ifndef HC_ALLOC_H
#define HC_ALLOC_H

#include <stddef.h>
#include <stdlib.h>

#if defined(_WIN32)
#include <malloc.h> // _aligned_malloc, _aligned_free

static inline void *hc_aligned_malloc(size_t alignment, size_t size) {
  return _aligned_malloc(size, alignment);
}

static inline void hc_aligned_free(void *p) { _aligned_free(p); }
#else
// POSIX fallback
static inline void *hc_aligned_malloc(size_t alignment, size_t size) {
  void *p = NULL;
  if (posix_memalign(&p, alignment, size) != 0)
    return NULL;
  return p;
}

static inline void hc_aligned_free(void *p) { free(p); }
#endif

#endif /* HC_ALLOC_H */
