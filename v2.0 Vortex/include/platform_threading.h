/**
 * @file platform_threading.h
 * @brief Cross-platform threading abstraction layer
 *
 * Provides a unified threading API that works on both Windows and POSIX
 * systems. On Windows, uses native Windows threading APIs (CRITICAL_SECTION,
 * CreateThread). On POSIX systems, uses pthreads.
 */

#ifndef PLATFORM_THREADING_H
#define PLATFORM_THREADING_H

#include <stdbool.h>
#include <stdint.h>


#ifdef _WIN32
/* Windows Threading */
#define WIN32_LEAN_AND_MEAN
#include <process.h>
#include <windows.h>

/* Type Definitions */
typedef CRITICAL_SECTION platform_mutex_t;
typedef HANDLE platform_thread_t;
typedef unsigned(__stdcall *platform_thread_func_t)(void *);

/* Mutex Operations */
static inline int platform_mutex_init(platform_mutex_t *mutex) {
  InitializeCriticalSection(mutex);
  return 0;
}

static inline int platform_mutex_lock(platform_mutex_t *mutex) {
  EnterCriticalSection(mutex);
  return 0;
}

static inline int platform_mutex_unlock(platform_mutex_t *mutex) {
  LeaveCriticalSection(mutex);
  return 0;
}

static inline int platform_mutex_destroy(platform_mutex_t *mutex) {
  DeleteCriticalSection(mutex);
  return 0;
}

/* Thread Operations */
static inline int platform_thread_create(platform_thread_t *thread,
                                         void *(*start_routine)(void *),
                                         void *arg) {
  *thread = (HANDLE)_beginthreadex(
      NULL, 0, (unsigned(__stdcall *)(void *))start_routine, arg, 0, NULL);
  return (*thread == NULL) ? -1 : 0;
}

static inline int platform_thread_join(platform_thread_t thread,
                                       void **retval) {
  DWORD result = WaitForSingleObject(thread, INFINITE);
  if (result == WAIT_OBJECT_0) {
    if (retval) {
      DWORD exit_code;
      GetExitCodeThread(thread, &exit_code);
      *retval = (void *)(uintptr_t)exit_code;
    }
    CloseHandle(thread);
    return 0;
  }
  return -1;
}

/* Sleep Operations */
static inline void platform_sleep_ms(uint32_t milliseconds) {
  Sleep(milliseconds);
}

static inline void platform_sleep_ns(uint64_t nanoseconds) {
  /* Windows Sleep is millisecond precision, so convert */
  Sleep((DWORD)(nanoseconds / 1000000));
}

#else
/* POSIX Threading (Linux, macOS, Unix) */
#include <pthread.h>
#include <time.h>
#include <unistd.h>

/* Type Definitions */
typedef pthread_mutex_t platform_mutex_t;
typedef pthread_t platform_thread_t;
typedef void *(*platform_thread_func_t)(void *);

/* Mutex Operations */
static inline int platform_mutex_init(platform_mutex_t *mutex) {
  return pthread_mutex_init(mutex, NULL);
}

static inline int platform_mutex_lock(platform_mutex_t *mutex) {
  return pthread_mutex_lock(mutex);
}

static inline int platform_mutex_unlock(platform_mutex_t *mutex) {
  return pthread_mutex_unlock(mutex);
}

static inline int platform_mutex_destroy(platform_mutex_t *mutex) {
  return pthread_mutex_destroy(mutex);
}

/* Thread Operations */
static inline int platform_thread_create(platform_thread_t *thread,
                                         void *(*start_routine)(void *),
                                         void *arg) {
  return pthread_create(thread, NULL, start_routine, arg);
}

static inline int platform_thread_join(platform_thread_t thread,
                                       void **retval) {
  return pthread_join(thread, retval);
}

/* Sleep Operations */
static inline void platform_sleep_ms(uint32_t milliseconds) {
  usleep(milliseconds * 1000);
}

static inline void platform_sleep_ns(uint64_t nanoseconds) {
  struct timespec ts;
  ts.tv_sec = nanoseconds / 1000000000ULL;
  ts.tv_nsec = nanoseconds % 1000000000ULL;
  nanosleep(&ts, NULL);
}

#endif /* _WIN32 */

/* Compatibility Macros for Easy Migration from pthread */
#define PLATFORM_MUTEX_INITIALIZER {0}

#endif /* PLATFORM_THREADING_H */
