#ifndef NS_RING_BUFFER_H
#define NS_RING_BUFFER_H

#include "ns_atomic.h"
#include <stdalign.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Lock-Free Ring Buffer for Entropy Pooling
// ============================================================================

/**
 * @brief Ring Buffer Configuration
 *
 * Buffer Size: 64KB (2048 keys × 32 bytes each)
 * This provides ~2000 pre-generated keys for instant retrieval
 */
#define NS_RING_BUFFER_CAPACITY 2048
#define NS_KEY_SIZE 32

// Alignment macro
#if defined(_MSC_VER)
#define NS_ALIGN_64 __declspec(align(64))
#else
#define NS_ALIGN_64 _Alignas(64)
#endif

/**
 * @brief Lock-Free Ring Buffer Structure
 *
 * Uses atomic wrapper for thread-safe, non-blocking operations.
 */
typedef struct {
  // Entropy storage (2048 × 32-byte keys = 64KB)
  NS_ALIGN_64 uint8_t keys[NS_RING_BUFFER_CAPACITY][NS_KEY_SIZE];

  // Atomic read/write pointers
  ns_atomic_size_t write_pos; // Producer writes here
  ns_atomic_size_t read_pos;  // Consumer reads here

  // Statistics (for monitoring)
  ns_atomic_size_t total_produced;
  ns_atomic_size_t total_consumed;
  ns_atomic_size_t underrun_count; // Times buffer was empty

  // Control flags
  ns_atomic_bool initialized;
  ns_atomic_bool shutdown_requested;
} ns_ring_buffer_t;

// ============================================================================
// Ring Buffer Operations
// ============================================================================

/**
 * @brief Initialize the ring buffer
 *
 * @param rb Pointer to ring buffer structure
 * @return 0 on success, -1 on failure
 */
static inline int ns_ring_buffer_init(ns_ring_buffer_t *rb) {
  if (!rb)
    return -1;

  ns_atomic_store_size(&rb->write_pos, 0);
  ns_atomic_store_size(&rb->read_pos, 0);
  ns_atomic_store_size(&rb->total_produced, 0);
  ns_atomic_store_size(&rb->total_consumed, 0);
  ns_atomic_store_size(&rb->underrun_count, 0);
  ns_atomic_store_bool(&rb->initialized, true);
  ns_atomic_store_bool(&rb->shutdown_requested, false);

  return 0;
}

/**
 * @brief Get number of available keys in buffer
 *
 * @param rb Pointer to ring buffer
 * @return Number of keys available for reading
 */
static inline size_t ns_ring_buffer_available(const ns_ring_buffer_t *rb) {
  size_t write = ns_atomic_load_size((ns_atomic_size_t *)&rb->write_pos);
  size_t read = ns_atomic_load_size((ns_atomic_size_t *)&rb->read_pos);

  if (write >= read) {
    return write - read;
  } else {
    return NS_RING_BUFFER_CAPACITY - read + write;
  }
}

/**
 * @brief Get free space in buffer
 *
 * @param rb Pointer to ring buffer
 * @return Number of free slots
 */
static inline size_t ns_ring_buffer_free_space(const ns_ring_buffer_t *rb) {
  return NS_RING_BUFFER_CAPACITY - ns_ring_buffer_available(rb) - 1;
}

/**
 * @brief Write a key to the ring buffer (Producer)
 *
 * Non-blocking operation. Returns false if buffer is full.
 *
 * @param rb Pointer to ring buffer
 * @param key Pointer to 32-byte key to write
 * @return true on success, false if buffer full
 */
static inline bool ns_ring_buffer_write(ns_ring_buffer_t *rb,
                                        const uint8_t *key) {
  size_t write_pos = ns_atomic_load_size(&rb->write_pos);
  size_t next_write = (write_pos + 1) % NS_RING_BUFFER_CAPACITY;
  size_t read_pos = ns_atomic_load_size(&rb->read_pos);

  // Check if buffer is full
  if (next_write == read_pos) {
    return false; // Buffer full
  }

  // Copy key to buffer
  for (size_t i = 0; i < NS_KEY_SIZE; i++) {
    rb->keys[write_pos][i] = key[i];
  }

  // Advance write pointer (atomic)
  ns_atomic_store_size(&rb->write_pos, next_write);
  ns_atomic_fetch_add_size(&rb->total_produced, 1);

  return true;
}

/**
 * @brief Read a key from the ring buffer (Consumer)
 *
 * Non-blocking operation. Returns false if buffer is empty.
 *
 * @param rb Pointer to ring buffer
 * @param key Pointer to buffer for 32-byte key output
 * @return true on success, false if buffer empty
 */
static inline bool ns_ring_buffer_read(ns_ring_buffer_t *rb, uint8_t *key) {
  size_t read_pos = ns_atomic_load_size(&rb->read_pos);
  size_t write_pos = ns_atomic_load_size(&rb->write_pos);

  // Check if buffer is empty
  if (read_pos == write_pos) {
    ns_atomic_fetch_add_size(&rb->underrun_count, 1);
    return false; // Buffer empty
  }

  // Copy key from buffer
  for (size_t i = 0; i < NS_KEY_SIZE; i++) {
    key[i] = rb->keys[read_pos][i];
  }

  // Advance read pointer (atomic)
  size_t next_read = (read_pos + 1) % NS_RING_BUFFER_CAPACITY;
  ns_atomic_store_size(&rb->read_pos, next_read);
  ns_atomic_fetch_add_size(&rb->total_consumed, 1);

  return true;
}

/**
 * @brief Request shutdown of ring buffer
 *
 * @param rb Pointer to ring buffer
 */
static inline void ns_ring_buffer_shutdown(ns_ring_buffer_t *rb) {
  ns_atomic_store_bool(&rb->shutdown_requested, true);
}

/**
 * @brief Check if shutdown is requested
 *
 * @param rb Pointer to ring buffer
 * @return true if shutdown requested
 */
static inline bool
ns_ring_buffer_is_shutdown_requested(const ns_ring_buffer_t *rb) {
  return ns_atomic_load_bool((ns_atomic_bool *)&rb->shutdown_requested);
}

#ifdef __cplusplus
}
#endif

#endif // NS_RING_BUFFER_H
