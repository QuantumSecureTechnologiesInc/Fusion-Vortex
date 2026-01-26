#ifndef NS_API_H
#define NS_API_H

#include <stddef.h>
#include <stdint.h>
/*
 * NeuralSeal Asynchronous Entropy Pool
 *
 * To support the high performance asynchronous entropy design described
 * in the Fulminis documentation, this header exposes configuration
 * constants and lightweight data structures used by the pooling layer.
 *
 * NS_POOL_CAPACITY_LOG2 controls the size of the internal ring buffer
 * storing pre‑heated raw entropy. It must be a power of two for
 * efficient modulo arithmetic. The default value of 10 yields a
 * capacity of 1024 entropy batches. Each batch consists of
 * NS_RAW_ENTROPY_SIZE bytes (2KB) of raw chaotic data gathered from
 * the vacuum simulator prior to cryptographic conditioning.
 *
 * NS_KEY_SIZE_BYTES defines the length of a derived key returned by
 * ns_get_random_key(). This mirrors the 256‑bit output length used by
 * ML‑KEM and ML‑DSA. The macros are declared here so that external
 * modules can size their buffers appropriately without including
 * implementation headers.
 */
#include <stdatomic.h>

/* -------------------------------------------------------------------------
 * Entropy Data Structures & Constants
 * ------------------------------------------------------------------------- */

/* Log2 of the number of entries in our asynchronous entropy ring buffer.  A
 * value of 10 gives 1024 slots. The capacity must be a power of two to
 * simplify wrapping arithmetic on the head/tail indices. */
#ifndef NS_POOL_CAPACITY_LOG2
#define NS_POOL_CAPACITY_LOG2 10U
#endif

/* Number of entries in the ring buffer. */
#ifndef NS_POOL_CAPACITY
#define NS_POOL_CAPACITY (1U << NS_POOL_CAPACITY_LOG2)
#endif

/* Size in bytes of one raw entropy batch produced by the vacuum simulator
 * prior to conditioning. Each batch stores 2048 bytes (2KB) of chaotic
 * noise. */
#ifndef NS_RAW_ENTROPY_SIZE
#define NS_RAW_ENTROPY_SIZE 2048U
#endif

/* Size in bytes of the final key returned to callers.  This is the
 * output size of SHA3‑256 and matches the 256‑bit keys used by ML‑KEM
 * and ML‑DSA. */
#ifndef NS_KEY_SIZE_BYTES
#define NS_KEY_SIZE_BYTES 32U
#endif

/*
 * Raw entropy batch container.  Each batch holds a 2KB buffer of noise
 * extracted from the chaotic vacuum.  In a complete implementation this
 * structure could also carry metadata for NIST SP 800‑90B health tests
 * (e.g. repetition count or adaptive proportion test results).  The
 * current implementation focuses solely on storing the raw bytes.
 */
typedef struct {
  uint8_t data[NS_RAW_ENTROPY_SIZE];
} ns_raw_entropy_batch_t;

/*
 * Thread‑safe ring buffer for pre‑heated entropy.  The background
 * simulation thread populates the buffer at the head index while
 * ns_get_random_key() pops from the tail.  Atomic operations ensure
 * safe concurrent access in multi‑core environments.  Additional
 * counters track total production/consumption and underrun events for
 * monitoring and diagnostics.
 */
typedef struct {
  ns_raw_entropy_batch_t buffer[NS_POOL_CAPACITY];
  atomic_uint head;              /* Producer writes here */
  atomic_uint tail;              /* Consumer reads here */
  atomic_size_t total_produced;  /* Total batches generated */
  atomic_size_t total_consumed;  /* Total batches consumed */
  atomic_size_t underrun_count;  /* Number of times buffer was empty */
} ns_entropy_pool_t;

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// NeuralSeal v3.2 "Fulminis" Public API
// ============================================================================

/**
 * @brief Initialize the asynchronous entropy pooling system
 *
 * Starts a background thread that continuously generates quantum-resistant
 * entropy and fills a lock-free ring buffer. This enables instant key
 * retrieval without blocking.
 *
 * Must be called once at application startup before using ns_get_random_key().
 *
 * @return 0 on success, negative error code on failure
 *         -1: Already initialized
 *         -2: Thread creation failed
 *         -3: Memory allocation failed
 */
int ns_init_entropy_pool(void);

/**
 * @brief Get instant random key from entropy pool
 *
 * Retrieves a pre-generated quantum-resistant key from the asynchronous
 * entropy pool. This operation is non-blocking and typically completes
 * in <1 microsecond.
 *
 * The entropy is generated using the Heisenberg-Euler Vacuum Fluctuation
 * engine with SHA3-256 conditioning.
 *
 * @param out Output buffer for random key (must be at least 'len' bytes)
 * @param len Length of key to generate (typically 32 bytes)
 * @return 0 on success, negative error code on failure
 *         -1: Null pointer
 *         -2: Entropy pool not initialized
 *         -3: Entropy pool exhausted (temporary, retry recommended)
 *         -4: Invalid length (len must be multiple of 32)
 *
 * @note If the pool is temporarily exhausted, the function will attempt
 *       to generate entropy synchronously as a fallback.
 */
int ns_get_random_key(uint8_t *out, size_t len);

/**
 * @brief Get entropy pool statistics
 *
 * Retrieves statistics about the entropy pool for monitoring and debugging.
 *
 * @param total_produced Total number of keys generated (output)
 * @param total_consumed Total number of keys consumed (output)
 * @param available Number of keys currently available (output)
 * @param underruns Number of times pool was exhausted (output)
 * @return 0 on success, -1 if pool not initialized
 */
int ns_get_entropy_stats(size_t *total_produced, size_t *total_consumed,
                         size_t *available, size_t *underruns);

/**
 * @brief Cleanup and shutdown the entropy pooling system
 *
 * Stops the background thread and releases all resources.
 * Should be called at application shutdown.
 *
 * After calling this function, ns_get_random_key() will fail until
 * ns_init_entropy_pool() is called again.
 */
void ns_cleanup_entropy_pool(void);

// ============================================================================
// Compatibility Aliases (for existing HyperCycle code)
// ============================================================================

/**
 * @brief Compatibility wrapper for hc_generate_vacuum_key
 *
 * Provides the same interface as the existing HyperCycle function
 * but uses the asynchronous entropy pool for better performance.
 */
static inline int ns_generate_vacuum_key(uint8_t *key, size_t len) {
  return ns_get_random_key(key, len);
}

#ifdef __cplusplus
}
#endif

#endif // NS_API_H
