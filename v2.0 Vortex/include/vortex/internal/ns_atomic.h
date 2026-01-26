#ifndef NS_ATOMIC_H
#define NS_ATOMIC_H

#if defined(__cplusplus)
#include <atomic>
// Map C++ atomics to C-style names if needed, or use std::atomic
typedef std::atomic<size_t> ns_atomic_size_t;
typedef std::atomic<bool> ns_atomic_bool;
typedef std::atomic<uint64_t> ns_atomic_uint64_t;

#define ns_atomic_store(ptr, val) std::atomic_store(ptr, val)
#define ns_atomic_load(ptr) std::atomic_load(ptr)
#define ns_atomic_fetch_add(ptr, val) std::atomic_fetch_add(ptr, val)

#else
// C Implementation

#if defined(_MSC_VER) && !defined(__STDC_NO_ATOMICS__) &&                      \
    defined(__STDC_VERSION__) && __STDC_VERSION__ >= 201112L
// MSVC in C11 mode with atomics enabled
#include <stdatomic.h>
typedef atomic_size_t ns_atomic_size_t;
typedef atomic_bool ns_atomic_bool;
typedef atomic_uint_fast64_t ns_atomic_uint64_t;

static inline void ns_atomic_store_size(ns_atomic_size_t *ptr, size_t val) {
  atomic_store(ptr, val);
}
static inline size_t ns_atomic_load_size(ns_atomic_size_t *ptr) {
  return atomic_load(ptr);
}
static inline size_t ns_atomic_fetch_add_size(ns_atomic_size_t *ptr,
                                              size_t val) {
  return atomic_fetch_add(ptr, val);
}

static inline void ns_atomic_store_bool(ns_atomic_bool *ptr, bool val) {
  atomic_store(ptr, val);
}
static inline bool ns_atomic_load_bool(ns_atomic_bool *ptr) {
  return atomic_load(ptr);
}

#elif defined(_MSC_VER)
// MSVC legacy / no C11 atomics -> Use Windows Interlocked API
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <windows.h>

// volatile is important for Interlocked ops
typedef volatile LONG64 ns_atomic_size_t; // size_t is 64-bit on x64
typedef volatile LONG ns_atomic_bool; // Treat as 32-bit for simple Interlocked
typedef volatile LONG64 ns_atomic_uint64_t;

static inline void ns_atomic_store_size(ns_atomic_size_t *ptr, size_t val) {
  InterlockedExchange64(ptr, (LONG64)val);
}

static inline size_t ns_atomic_load_size(ns_atomic_size_t *ptr) {
  return (size_t)InterlockedCompareExchange64(ptr, 0, 0);
}

static inline size_t ns_atomic_fetch_add_size(ns_atomic_size_t *ptr,
                                              size_t val) {
  return (size_t)InterlockedExchangeAdd64(ptr, (LONG64)val);
}

static inline void ns_atomic_store_bool(ns_atomic_bool *ptr, bool val) {
  InterlockedExchange(ptr, val ? 1 : 0);
}

static inline bool ns_atomic_load_bool(ns_atomic_bool *ptr) {
  return InterlockedCompareExchange(ptr, 0, 0) != 0;
}

#else
// GCC/Clang (non-MSVC) -> Use built-in atomics or <stdatomic.h>
#include <stdatomic.h>
typedef atomic_size_t ns_atomic_size_t;
typedef atomic_bool ns_atomic_bool;
typedef atomic_uint_fast64_t ns_atomic_uint64_t;

static inline void ns_atomic_store_size(ns_atomic_size_t *ptr, size_t val) {
  atomic_store(ptr, val);
}
static inline size_t ns_atomic_load_size(ns_atomic_size_t *ptr) {
  return atomic_load(ptr);
}
static inline size_t ns_atomic_fetch_add_size(ns_atomic_size_t *ptr,
                                              size_t val) {
  return atomic_fetch_add(ptr, val);
}

static inline void ns_atomic_store_bool(ns_atomic_bool *ptr, bool val) {
  atomic_store(ptr, val);
}
static inline bool ns_atomic_load_bool(ns_atomic_bool *ptr) {
  return atomic_load(ptr);
}
#endif

#endif

#endif // NS_ATOMIC_H
