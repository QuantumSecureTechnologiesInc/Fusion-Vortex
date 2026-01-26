/*
 * NeuralSeal v3.2 Fulminis Entropy Pool Implementation
 *
 * This source file implements the asynchronous entropy pooling
 * architecture described in the Fulminis design notes.  A background
 * thread continuously runs the vacuum simulation and populates a ring
 * buffer with raw 2KB entropy batches.  When callers request a random
 * key, the consumer extracts one batch, runs it through a SHA3‑256
 * conditioner, and returns a 32‑byte key.  The buffer capacity and
 * batch sizes are defined in ns_api.h.
 */

#include "vortex/internal/sha3.h"
#include "vortex/public/hc_vacuum_entropy.h"
#include "vortex/public/ns_api.h"

#include <errno.h>
#include <stdatomic.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
#include <windows.h>
/* Define platform neutral thread primitives */
#define NS_THREAD_HANDLE HANDLE
#define NS_THREAD_RETURN DWORD WINAPI
#define NS_THREAD_RETURN_TYPE DWORD
#else
#include <pthread.h>
#include <unistd.h>
#define NS_THREAD_HANDLE pthread_t
#define NS_THREAD_RETURN void *
#define NS_THREAD_RETURN_TYPE void *
#endif

// ============================================================================
// Global State
// ============================================================================

/*
 * Entropy pool instance.  This ring buffer stores NS_POOL_CAPACITY
 * ns_raw_entropy_batch_t structures.  The producer thread appends to
 * head while the consumer pops from tail.  Atomic counters track
 * usage statistics.
 */
static ns_entropy_pool_t g_entropy_pool;

/* Background worker thread that fills the entropy pool. */
static NS_THREAD_HANDLE g_filler_thread;

/* Indicates whether the pool has been initialized.  Atomic for thread safety.
 */
static atomic_bool g_pool_initialized = false;
/* Running flag used to signal the background thread to exit on shutdown. */
static atomic_bool s_running_flag = false;

// ============================================================================
// Background Entropy Filler Thread
// ============================================================================

/*
 * ns_sha3_256_conditioner
 *
 * Applies a SHA3‑256 hash to the provided raw entropy.  The output is
 * always NS_KEY_SIZE_BYTES bytes in length.  This wrapper uses the
 * existing hc_sha3_256 implementation from the HyperCycle codebase to
 * produce a uniformly distributed output suitable for post‑quantum
 * cryptography.
 */
static void ns_sha3_256_conditioner(const uint8_t *raw_input, size_t input_len,
                                    uint8_t output[NS_KEY_SIZE_BYTES]) {
  /*
   * The hc_sha3_256 function is constant‑time and will process any
   * input length.  Passing 2KB of raw vacuum noise ensures a
   * high‑entropy digest.  We ignore the return value as hc_sha3_256
   * does not indicate errors.
   */
  hc_sha3_256(raw_input, input_len, output);
}

/*
 * entropy_filler_thread
 *
 * Producer loop that continuously generates raw entropy batches via
 * hc_generate_vacuum_key() and stores them into the ring buffer.  When the
 * buffer is full, the thread yields briefly to avoid busy waiting.  The
 * thread exits when s_running_flag is cleared by ns_cleanup_entropy_pool().
 */
static NS_THREAD_RETURN entropy_filler_thread(void *arg) {
  (void)arg; // Unused

  /* Temporary buffer for one raw entropy batch. */
  ns_raw_entropy_batch_t new_batch;

  while (atomic_load_explicit(&s_running_flag, memory_order_acquire)) {
    /* Load current head and tail to determine if the buffer is full.  We
     * intentionally leave one slot empty to distinguish between full and
     * empty states. */
    unsigned int head =
        atomic_load_explicit(&g_entropy_pool.head, memory_order_relaxed);
    unsigned int tail =
        atomic_load_explicit(&g_entropy_pool.tail, memory_order_acquire);
    unsigned int next_head = (head + 1U) % NS_POOL_CAPACITY;

    if (next_head == tail) {
      /* Buffer is full.  Briefly yield the CPU to reduce contention. */
#ifdef _WIN32
      Sleep(1);
#else
/* Use a pause instruction if available to hint to the CPU. */
#if defined(__x86_64__) || defined(__i386__)
      __builtin_ia32_pause();
#else
      usleep(1000);
#endif
#endif
      continue;
    }

    /*
     * Generate raw entropy.  Each batch consists of NS_RAW_ENTROPY_SIZE
     * bytes.  The hc_generate_vacuum_key function outputs 32 bytes per
     * invocation.  We call it repeatedly to fill the 2KB buffer.
     */
    size_t offset = 0;
    const size_t chunk_size = NS_KEY_SIZE_BYTES;
    while (offset < NS_RAW_ENTROPY_SIZE) {
      if (hc_generate_vacuum_key(new_batch.data + offset, chunk_size) != 0) {
        /* Generation failed; skip this iteration and try again later. */
#ifdef _WIN32
        Sleep(10);
#else
        usleep(10000);
#endif
        offset = 0;
        continue;
      }
      offset += chunk_size;
    }

    /* Copy the newly generated batch into the ring buffer at the current
     * head position.  This copy must occur before updating the head
     * pointer to ensure that the consumer sees a fully populated batch. */
    memcpy(g_entropy_pool.buffer[head].data, new_batch.data,
           NS_RAW_ENTROPY_SIZE);

    /* Atomically advance the head pointer and increment the production
     * counter.  memory_order_release pairs with the acquire load in
     * ns_get_random_key(). */
    atomic_store_explicit(&g_entropy_pool.head, next_head,
                          memory_order_release);
    atomic_fetch_add_explicit(&g_entropy_pool.total_produced, 1U,
                              memory_order_relaxed);
  }

  /* Clear sensitive data before exiting. */
  memset(new_batch.data, 0, sizeof(new_batch.data));

#ifdef _WIN32
  return 0;
#else
  return NULL;
#endif
}

// ============================================================================
// Public API Implementation
// ============================================================================

int ns_init_entropy_pool(void) {
  /* Prevent double initialization. */
  if (atomic_load_explicit(&g_pool_initialized, memory_order_acquire)) {
    return -1;
  }

  /* Initialize head/tail pointers and counters. */
  atomic_store_explicit(&g_entropy_pool.head, 0U, memory_order_relaxed);
  atomic_store_explicit(&g_entropy_pool.tail, 0U, memory_order_relaxed);
  atomic_store_explicit(&g_entropy_pool.total_produced, 0U,
                        memory_order_relaxed);
  atomic_store_explicit(&g_entropy_pool.total_consumed, 0U,
                        memory_order_relaxed);
  atomic_store_explicit(&g_entropy_pool.underrun_count, 0U,
                        memory_order_relaxed);

  /* Pre‑fill the buffer with a handful of entropy batches to reduce initial
   * latency.  We synchronously generate some entries before spawning the
   * thread.  Use a small number to keep startup time reasonable. */
  for (size_t i = 0; i < 16 && i < NS_POOL_CAPACITY - 1; i++) {
    ns_raw_entropy_batch_t tmp;
    size_t offset = 0;
    while (offset < NS_RAW_ENTROPY_SIZE) {
      if (hc_generate_vacuum_key(tmp.data + offset, NS_KEY_SIZE_BYTES) != 0) {
        /* If a generation fails, zero out and break to retry. */
        memset(tmp.data, 0, sizeof(tmp.data));
        offset = 0;
        continue;
      }
      offset += NS_KEY_SIZE_BYTES;
    }
    /* Copy into buffer and update head. */
    unsigned int head =
        atomic_load_explicit(&g_entropy_pool.head, memory_order_relaxed);
    memcpy(g_entropy_pool.buffer[head].data, tmp.data, NS_RAW_ENTROPY_SIZE);
    unsigned int next_head = (head + 1U) % NS_POOL_CAPACITY;
    atomic_store_explicit(&g_entropy_pool.head, next_head,
                          memory_order_relaxed);
    atomic_fetch_add_explicit(&g_entropy_pool.total_produced, 1U,
                              memory_order_relaxed);
  }

  /* Signal the background thread to run and spawn it. */
  atomic_store_explicit(&s_running_flag, true, memory_order_release);

#ifdef _WIN32
  g_filler_thread = CreateThread(NULL, 0, entropy_filler_thread, NULL, 0, NULL);
  if (g_filler_thread == NULL) {
    return -2;
  }
#else
  if (pthread_create(&g_filler_thread, NULL, entropy_filler_thread, NULL) !=
      0) {
    return -2;
  }
#endif

  atomic_store_explicit(&g_pool_initialized, true, memory_order_release);
  return 0;
}

int ns_get_random_key(uint8_t *out, size_t len) {
  if (!out) {
    return -1; /* Null pointer */
  }
  if (!atomic_load_explicit(&g_pool_initialized, memory_order_acquire)) {
    return -2; /* Pool not initialized */
  }
  if (len % NS_KEY_SIZE_BYTES != 0) {
    return -4; /* Invalid length */
  }

  size_t keys_needed = len / NS_KEY_SIZE_BYTES;
  uint8_t *ptr = out;

  for (size_t i = 0; i < keys_needed; i++) {
    /* Attempt to pop a raw entropy batch from the pool. */
    while (true) {
      unsigned int tail =
          atomic_load_explicit(&g_entropy_pool.tail, memory_order_relaxed);
      unsigned int head =
          atomic_load_explicit(&g_entropy_pool.head, memory_order_acquire);
      if (head == tail) {
        /* Buffer empty.  Increment underrun counter and fall back to
         * synchronous generation. */
        atomic_fetch_add_explicit(&g_entropy_pool.underrun_count, 1U,
                                  memory_order_relaxed);
        /* Generate a 32‑byte key directly from the vacuum and
         * condition it.  We synthesise a raw buffer of one block
         * then run the SHA3 conditioner. */
        uint8_t raw_buf[NS_RAW_ENTROPY_SIZE];
        size_t off = 0;
        while (off < NS_RAW_ENTROPY_SIZE) {
          if (hc_generate_vacuum_key(raw_buf + off, NS_KEY_SIZE_BYTES) != 0) {
            /* Failure producing vacuum noise results in immediate error. */
            memset(raw_buf, 0, NS_RAW_ENTROPY_SIZE);
            return -3;
          }
          off += NS_KEY_SIZE_BYTES;
        }
        ns_sha3_256_conditioner(raw_buf, NS_RAW_ENTROPY_SIZE, ptr);
        memset(raw_buf, 0, NS_RAW_ENTROPY_SIZE);
        ptr += NS_KEY_SIZE_BYTES;
        break; /* Move to next key */
      }

      /* We have at least one batch available.  Compute next tail. */
      unsigned int next_tail = (tail + 1U) % NS_POOL_CAPACITY;
      /* Attempt to claim this entry using a CAS.  If we lose the race,
       * retry. */
      if (atomic_compare_exchange_weak_explicit(&g_entropy_pool.tail, &tail,
                                                next_tail, memory_order_release,
                                                memory_order_relaxed)) {
        /* Successfully claimed the batch at 'tail'.  Copy out raw
         * entropy, condition it, and increment counters. */
        ns_raw_entropy_batch_t current_batch = g_entropy_pool.buffer[tail];
        ns_sha3_256_conditioner(current_batch.data, NS_RAW_ENTROPY_SIZE, ptr);
        atomic_fetch_add_explicit(&g_entropy_pool.total_consumed, 1U,
                                  memory_order_relaxed);
        ptr += NS_KEY_SIZE_BYTES;
        break;
      }
      /* CAS failed – another consumer likely won.  Loop and retry. */
    }
  }

  return 0;
}

int ns_get_entropy_stats(size_t *total_produced, size_t *total_consumed,
                         size_t *available, size_t *underruns) {
  if (!atomic_load_explicit(&g_pool_initialized, memory_order_acquire)) {
    return -1;
  }
  if (total_produced) {
    *total_produced = atomic_load_explicit(&g_entropy_pool.total_produced,
                                           memory_order_relaxed);
  }
  if (total_consumed) {
    *total_consumed = atomic_load_explicit(&g_entropy_pool.total_consumed,
                                           memory_order_relaxed);
  }
  if (available) {
    unsigned int head =
        atomic_load_explicit(&g_entropy_pool.head, memory_order_acquire);
    unsigned int tail =
        atomic_load_explicit(&g_entropy_pool.tail, memory_order_relaxed);
    if (head >= tail) {
      *available = head - tail;
    } else {
      *available = NS_POOL_CAPACITY - (tail - head);
    }
  }
  if (underruns) {
    *underruns = atomic_load_explicit(&g_entropy_pool.underrun_count,
                                      memory_order_relaxed);
  }
  return 0;
}

void ns_cleanup_entropy_pool(void) {
  if (!atomic_load_explicit(&g_pool_initialized, memory_order_acquire)) {
    return;
  }

  /* Signal background thread to stop and wait for it to finish. */
  atomic_store_explicit(&s_running_flag, false, memory_order_release);
#ifdef _WIN32
  WaitForSingleObject(g_filler_thread, INFINITE);
  CloseHandle(g_filler_thread);
#else
  pthread_join(g_filler_thread, NULL);
#endif

  /* Zero out the pool to securely erase any residual entropy. */
  for (size_t i = 0; i < NS_POOL_CAPACITY; i++) {
    memset(g_entropy_pool.buffer[i].data, 0, NS_RAW_ENTROPY_SIZE);
  }
  atomic_store_explicit(&g_entropy_pool.head, 0U, memory_order_relaxed);
  atomic_store_explicit(&g_entropy_pool.tail, 0U, memory_order_relaxed);
  atomic_store_explicit(&g_entropy_pool.total_produced, 0U,
                        memory_order_relaxed);
  atomic_store_explicit(&g_entropy_pool.total_consumed, 0U,
                        memory_order_relaxed);
  atomic_store_explicit(&g_entropy_pool.underrun_count, 0U,
                        memory_order_relaxed);

  atomic_store_explicit(&g_pool_initialized, false, memory_order_release);
}
