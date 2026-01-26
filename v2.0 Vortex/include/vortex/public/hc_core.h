#ifndef HC_CORE_H
#define HC_CORE_H

#ifdef __cplusplus
#include <atomic>
using atomic_uint = std::atomic<unsigned int>;
#else
#include <stdatomic.h> // For thread-safe ring buffer operations
#endif
#include <stddef.h>
#include <stdint.h>

// Include the core high-performance kernel header
#include "hypercycle_vacuum_core.h"


/* -------------------------------------------------------------------------
   Entropy Options (compile/runtime)
   ------------------------------------------------------------------------- */
#define HC_ENTROPY_OPT_USE_GPU          (1u << 0) /* allow CUDA/ROCm if available */
#define HC_ENTROPY_OPT_ENABLE_MASKING   (1u << 1) /* allow blinding/masking */
#define HC_ENTROPY_OPT_USE_SBOX16       (1u << 2) /* use 16-bit LUT acceleration in backends */
#define HC_ENTROPY_OPT_MAP_TENT         (1u << 3) /* use piecewise-linear tent map (vs polynomial) */
#define HC_ENTROPY_OPT_ENABLE_RESERVOIR (1u << 4) /* background reservoir thread */

/** Set process-level entropy options (call before hc_entropy_init). */
void hc_entropy_set_options(uint32_t options);

/** Get process-level entropy options. */
uint32_t hc_entropy_get_options(void);

/* -------------------------------------------------------------------------
   1. Entropy Data Structures & Constants
   ------------------------------------------------------------------------- */

// Defines the capacity of our Asynchronous Entropy Ring Buffer (must be a power
// of 2)
#define HC_POOL_CAPACITY_LOG2 10
#define HC_POOL_CAPACITY (1 << HC_POOL_CAPACITY_LOG2)
#define HC_RAW_ENTROPY_SIZE                                                    \
  2048 // Bytes of raw vacuum noise needed before conditioning
#define HC_KEY_SIZE_BYTES 32 // 256 bits for ML-KEM/ML-DSA use

typedef struct {
  uint8_t data[HC_RAW_ENTROPY_SIZE];
  // In a real implementation, add flags for NIST SP 800-90B health status
} hc_raw_entropy_batch_t;

// Thread-safe ring buffer structure for pre-heated entropy
typedef struct {
  hc_raw_entropy_batch_t buffer[HC_POOL_CAPACITY];
  atomic_uint head; // Where we push completed batches
  atomic_uint tail; // Where the API pops batches from
} hc_entropy_pool_t;

/* -------------------------------------------------------------------------
   Backend Plugin Interface
   ------------------------------------------------------------------------- */
typedef struct {
  const char *backend_name;
  int (*generate_entropy)(uint64_t *seeds, uint8_t *out_buffer, size_t count);
  void (*teardown)(void);
} hc_backend_t;

// Plugin Entry Point Prototype (dlsym target)
typedef hc_backend_t *(*hc_plugin_init_fn)(void);

/* -------------------------------------------------------------------------
   2. Function Prototypes (Public API)
   ------------------------------------------------------------------------- */

/**
 * @brief Initializes the HyperCycle Entropy System.
 * Starts the background thread for Asynchronous Entropy Pooling.
 * Must be called once at application startup (e.g., hc_initialize()).
 */
int hc_entropy_init();

/**
 * @brief Stops the background entropy generation thread cleanly.
 */
void hc_entropy_shutdown();

/**
 * @brief Retrieves a fresh, conditioned, quantum-resistant key.
 * Pops an instant key from the pre-heated pool.
 * This is the function the core ML-KEM/ML-DSA libraries will call.
 *
 * @param key_out Buffer to write the 32-byte key into.
 * @return 0 on success, -1 if the pool is empty (should never happen in HFT).
 */
int hc_get_random_key(uint8_t key_out[HC_KEY_SIZE_BYTES]);

/* -------------------------------------------------------------------------
   3. Internal Helper Functions (Private to Library Implementation)
   ------------------------------------------------------------------------- */

/**
 * @brief The background worker thread function that runs the 47-cycle
 * simulation. It continuously fills the ring buffer.
 */
void *hc_background_filler_thread(void *arg);

/**
 * @brief SHA3-256 Randomness Extractor (Conditioner).
 * Uses the raw 'vacuum noise' to produce a statistically perfect, uniform key.
 */
void hc_sha3_256_conditioner(const uint8_t *raw_input, size_t input_len,
                             uint8_t output[HC_KEY_SIZE_BYTES]);

#endif // HC_CORE_H
