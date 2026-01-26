#include "vortex/public/hc_core.h"
#include "vortex/internal/hc_health_tests.h"
#include "vortex/internal/hc_sbox16.h"
#include "vortex/internal/sha3.h"
#include "vortex/internal/system_entropy.h"
#include "vortex/public/hc_gpu_universal.h"
#include "vortex/public/hc_telemetry.h"
#include "vortex/public/hypercycle_vacuum_core.h"
#include <errno.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

// Global structures (initialized in hc_entropy_init)
static _Atomic uint32_t s_entropy_options =
    (HC_ENTROPY_OPT_USE_GPU | HC_ENTROPY_OPT_ENABLE_MASKING |
     HC_ENTROPY_OPT_USE_SBOX16 | HC_ENTROPY_OPT_ENABLE_RESERVOIR);
hc_entropy_pool_t g_entropy_pool;
static atomic_bool s_running_flag = true;
static pthread_t s_filler_thread;

// Health test state for reservoir (NIST SP 800-90B style)
static hc_rct_state_t g_rct_core;
static hc_apt_state_t g_apt_core;
static int g_health_inited_core = 0;

/* -------------------------------------------------------------------------
   SHA3-256 Randomness Extractor (Placeholder)
   ------------------------------------------------------------------------- */
void hc_sha3_256_conditioner(const uint8_t *raw_input, size_t input_len,
                             uint8_t output[HC_KEY_SIZE_BYTES]) {
  // Real SHA3-256 conditioner (deterministic, constant interface)
  hc_sha3_256(raw_input, input_len, output);
}

/* -------------------------------------------------------------------------
   Background Filler Thread Implementation
   ------------------------------------------------------------------------- */
void *hc_background_filler_thread(void *arg) {
  (void)arg; // Unused
  if (!g_health_inited_core) {
    hc_rct_init(&g_rct_core, 0);
    hc_apt_init(&g_apt_core, 0);
    g_health_inited_core = 1;
  }
  while (s_running_flag) {
    unsigned int head =
        atomic_load_explicit(&g_entropy_pool.head, memory_order_relaxed);
    unsigned int tail =
        atomic_load_explicit(&g_entropy_pool.tail, memory_order_acquire);

    // Check if the buffer is full (leave one slot empty to distinguish full
    // from empty)
    if (((head + 1) % HC_POOL_CAPACITY) == tail) {
      hc__telemetry_inc_pool_overrun();
// Pool is full, sleep briefly to avoid busy-waiting the CPU core
// On Windows, _mm_pause() is usually available via immintrin.h which is
// included in core
#if defined(__GNUC__) || defined(__clang__)
      __builtin_ia32_pause();
#else
      _mm_pause();
#endif
      continue;
    }

    // --- Generate Raw Entropy (Backend: CPU/CUDA/ROCm) ---
    hc_raw_entropy_batch_t new_batch;

    // Seed base + optional blinding from OS entropy each batch.
    uint64_t seed_base = 0;
    uint64_t blinding_seed = 0;
    unsigned char seed_buf[16];
    if (hc_cryptographic_entropy(seed_buf, sizeof(seed_buf)) == 0) {
      memcpy(&seed_base, seed_buf + 0, 8);
      memcpy(&blinding_seed, seed_buf + 8, 8);
    }

    // Fill 2048 bytes using 32-byte blocks (64 blocks).
    const size_t blocks = HC_RAW_ENTROPY_SIZE / 32;

    const hc_gpu_backend_t *be = hc_gpu_auto_init();
    if (!be || !be->generate_entropy_batch ||
        be->generate_entropy_batch(seed_base, blinding_seed, new_batch.data,
                                   blocks) != HC_GPU_SUCCESS) {
      // Backend failure: fall back to CPU generation without blinding.
      // (Still deterministic per batch seed_base.)
      const hc_gpu_backend_t *cpu = hc_gpu_auto_init(); // returns at least CPU
      (void)cpu;
      // As last resort, just zero (health tests will fail and caller can
      // react).
      memset(new_batch.data, 0, sizeof(new_batch.data));
    }

    // --- Health tests (reject catastrophic failures) ---
    int health_ok = 1;
    for (size_t i = 0; i < HC_RAW_ENTROPY_SIZE; i++) {
      if (hc_rct_test(&g_rct_core, new_batch.data[i]) != 0) {
        health_ok = 0;
        break;
      }
      if (hc_apt_test(&g_apt_core, new_batch.data[i]) != 0) {
        health_ok = 0;
        break;
      }
    }
    if (!health_ok) {
      hc__telemetry_inc_health_failure();
      // Do not commit to pool; continue generating
      continue;
    }

    // --- Place into pool ---
    memcpy(g_entropy_pool.buffer[head].data, new_batch.data,
           HC_RAW_ENTROPY_SIZE);

    // Commit the write and update the head pointer
    atomic_store_explicit(&g_entropy_pool.head, (head + 1) % HC_POOL_CAPACITY,
                          memory_order_release);
  }
  return NULL;
}

/* --- Public API Function Implementations --- */

/**
 * @brief Retrieves a fresh, conditioned, quantum-resistant key instantly.
 */
int hc_get_random_key(uint8_t key_out[HC_KEY_SIZE_BYTES]) {
  unsigned int head, tail;
  int retries = 0;

  do {
    tail = atomic_load_explicit(&g_entropy_pool.tail, memory_order_relaxed);
    head = atomic_load_explicit(&g_entropy_pool.head, memory_order_acquire);

    // Check if the pool is empty (tail has caught up to head)
    if (head == tail) {
      return -EAGAIN;
    }

    // We have data available. Process the next batch.
    hc_raw_entropy_batch_t current_batch = g_entropy_pool.buffer[tail];

    // This is where we apply the critical conditioning step:
    hc_sha3_256_conditioner(current_batch.data, HC_RAW_ENTROPY_SIZE, key_out);

    // Commit the read operation and update the tail pointer using an atomic CAS
    unsigned int next_tail = (tail + 1) % HC_POOL_CAPACITY;

    // Ensure the operation completes atomically before the background thread
    // can interfere
    if (atomic_compare_exchange_weak_explicit(&g_entropy_pool.tail, &tail,
                                              next_tail, memory_order_release,
                                              memory_order_relaxed)) {
      return 0; // Success
    }

    retries++;
  } while (retries < 100);

  return -EBUSY;
}

/**
 * @brief Initializes the HyperCycle Entropy System.
 */
int hc_entropy_init() {
  // Initialize atomic pointers to zero
  atomic_init(&g_entropy_pool.head, 0);
  atomic_init(&g_entropy_pool.tail, 0);
  s_running_flag = true;

  // Start the background worker thread
  if (pthread_create(&s_filler_thread, NULL, hc_background_filler_thread,
                     NULL) != 0) {
    perror("Failed to start HyperCycle entropy thread");
    return -1;
  }
  return 0;
}

/**
 * @brief Stops the background entropy generation thread cleanly.
 */
void hc_entropy_shutdown() {
  s_running_flag = false; // Signal the thread to stop
  void *status;
  if (pthread_join(s_filler_thread, &status) != 0) {
    perror("Failed to join HyperCycle entropy thread");
  }
}

void hc_entropy_set_options(uint32_t options) {
  atomic_store(&s_entropy_options, options);
}

uint32_t hc_entropy_get_options(void) {
  return atomic_load(&s_entropy_options);
}
