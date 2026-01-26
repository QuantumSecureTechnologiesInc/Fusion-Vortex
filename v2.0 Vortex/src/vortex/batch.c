// batch.c – Production Batch Operations for High-Throughput Workloads
// Implements parallel cryptographic operations with thread pooling and SIMD
// optimization Part of HyperCycle v3.2 Fulminis Pure Quaternion-Chaos
// Architecture

#include "vortex/public/batch.h"
#include "vortex/internal/system_entropy.h"
#include "vortex/public/weave_kem.h"
#include "vortex/public/weave_sig.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
#include <process.h>
#include <windows.h>

#else
#include <pthread.h>
#include <unistd.h>
#endif

// Thread pool structure
typedef struct {
  int thread_id;
  int batch_start;
  int batch_end;
  void *input_data;
  void *output_data;
  int (*work_function)(int, void *, void *);
  int status;
} hc_worker_thread_t;

// Get number of CPU cores
static unsigned int get_cpu_count(void) {
#ifdef _WIN32
  SYSTEM_INFO sysinfo;
  GetSystemInfo(&sysinfo);
  return sysinfo.dwNumberOfProcessors;
#else
  long nprocs = sysconf(_SC_NPROCESSORS_ONLN);
  return (nprocs > 0) ? (unsigned int)nprocs : 1;
#endif
}

// Worker thread function
#ifdef _WIN32
static unsigned int __stdcall worker_thread_func(void *arg) {
#else
static void *worker_thread_func(void *arg) {
#endif
  hc_worker_thread_t *worker = (hc_worker_thread_t *)arg;

  if (!worker || !worker->work_function) {
    worker->status = -1;
#ifdef _WIN32
    return 1;
#else
    return NULL;
#endif
  }

  // Process batch
  for (int i = worker->batch_start; i < worker->batch_end; i++) {
    int result =
        worker->work_function(i, worker->input_data, worker->output_data);
    if (result != 0) {
      worker->status = result;
#ifdef _WIN32
      return 1;
#else
      return NULL;
#endif
    }
  }

  worker->status = 0;
#ifdef _WIN32
  return 0;
#else
  return NULL;
#endif
}

// KEM keypair generation worker
static int kem_keygen_worker(int index, void *input_data, void *output_data) {
  (void)input_data; // Unused
  hc_kem_keypair_t *keypairs = (hc_kem_keypair_t *)output_data;

  if (!keypairs) {
    return -1;
  }

  return hc_kem_keygen(&keypairs[index]);
}

// Signature keypair generation worker
static int sig_keygen_worker(int index, void *input_data, void *output_data) {
  (void)input_data; // Unused
  hc_sig_keypair_t *keypairs = (hc_sig_keypair_t *)output_data;

  if (!keypairs) {
    return -1;
  }

  return hc_sig_keygen(&keypairs[index]);
}

/**
 * @brief Execute batch operation using thread pool
 *
 * Thread Pool Architecture:
 * - Divides work evenly across threads
 * - Each thread processes a contiguous batch
 * - Automatic CPU core detection
 * - Supports Windows and POSIX threads
 *
 * @param config Batch configuration (thread count, batch size)
 * @param total_items Total number of items to process
 * @param work_function Worker function to execute per item
 * @param input_data Input data for workers
 * @param output_data Output data for workers
 * @return 0 on success, -1 on failure
 */
static int execute_batch(const hc_batch_config_t *config, int total_items,
                         int (*work_function)(int, void *, void *),
                         void *input_data, void *output_data) {
  if (!config || !work_function || total_items <= 0) {
    return -1;
  }

  // Determine thread count
  unsigned int thread_count = config->thread_count;
  if (thread_count == 0) {
    thread_count = get_cpu_count();
  }

  // Cap thread count to reasonable limits
  if (thread_count > 64) {
    thread_count = 64;
  }
  if (thread_count > (unsigned int)total_items) {
    thread_count = total_items;
  }

  fprintf(stdout, "[Batch] Processing %d items with %u threads\n", total_items,
          thread_count);

  // Allocate worker threads
  hc_worker_thread_t *workers =
      (hc_worker_thread_t *)calloc(thread_count, sizeof(hc_worker_thread_t));
  if (!workers) {
    return -1;
  }

  // Calculate batch sizes per thread
  int items_per_thread = total_items / thread_count;
  int remaining_items = total_items % thread_count;

  int current_offset = 0;
  for (unsigned int i = 0; i < thread_count; i++) {
    workers[i].thread_id = i;
    workers[i].batch_start = current_offset;
    workers[i].batch_end = current_offset + items_per_thread;

    // Distribute remaining items to first threads
    if ((int)i < remaining_items) {
      workers[i].batch_end++;
    }

    workers[i].input_data = input_data;
    workers[i].output_data = output_data;
    workers[i].work_function = work_function;
    workers[i].status = -1;

    current_offset = workers[i].batch_end;
  }

#ifdef _WIN32
  // Windows thread creation
  HANDLE *thread_handles = (HANDLE *)calloc(thread_count, sizeof(HANDLE));
  if (!thread_handles) {
    free(workers);
    return -1;
  }

  for (unsigned int i = 0; i < thread_count; i++) {
    thread_handles[i] = (HANDLE)_beginthreadex(NULL, 0, worker_thread_func,
                                               &workers[i], 0, NULL);
    if (thread_handles[i] == NULL) {
      fprintf(stderr, "[Batch] Failed to create thread %u\n", i);
      // Clean up created threads
      for (unsigned int j = 0; j < i; j++) {
        WaitForSingleObject(thread_handles[j], INFINITE);
        CloseHandle(thread_handles[j]);
      }
      free(thread_handles);
      free(workers);
      return -1;
    }
  }

  // Wait for all threads to complete
  WaitForMultipleObjects(thread_count, thread_handles, TRUE, INFINITE);

  // Clean up thread handles
  for (unsigned int i = 0; i < thread_count; i++) {
    CloseHandle(thread_handles[i]);
  }
  free(thread_handles);

#else
  // POSIX thread creation
  pthread_t *thread_ids = (pthread_t *)calloc(thread_count, sizeof(pthread_t));
  if (!thread_ids) {
    free(workers);
    return -1;
  }

  for (unsigned int i = 0; i < thread_count; i++) {
    int result =
        pthread_create(&thread_ids[i], NULL, worker_thread_func, &workers[i]);
    if (result != 0) {
      fprintf(stderr, "[Batch] Failed to create thread %u\n", i);
      // Clean up created threads
      for (unsigned int j = 0; j < i; j++) {
        pthread_join(thread_ids[j], NULL);
      }
      free(thread_ids);
      free(workers);
      return -1;
    }
  }

  // Wait for all threads to complete
  for (unsigned int i = 0; i < thread_count; i++) {
    pthread_join(thread_ids[i], NULL);
  }
  free(thread_ids);
#endif

  // Check worker status
  int overall_status = 0;
  for (unsigned int i = 0; i < thread_count; i++) {
    if (workers[i].status != 0) {
      fprintf(stderr, "[Batch] Thread %u failed with status %d\n", i,
              workers[i].status);
      overall_status = -1;
    }
  }

  free(workers);
  return overall_status;
}

/**
 * @brief Batch KEM keypair generation
 *
 * High-Performance Features:
 * - Parallel generation across multiple CPU cores
 * - Thread pool management
 * - Automatic workload distribution
 * - SIMD-optimized quaternion operations (per-thread)
 *
 * Use Case: Server initialization, key pre-generation
 *
 * @param config Batch configuration
 * @param keypairs Output array for generated keypairs
 * @param count Number of keypairs to generate
 * @return 0 on success, -1 on failure
 */
int hc_kem_generate_batch(const hc_batch_config_t *config,
                          hc_kem_keypair_t *keypairs, int count) {
  if (!config || !keypairs || count <= 0) {
    return -1;
  }

  fprintf(stdout, "[Batch] Generating %d KEM keypairs\n", count);

  return execute_batch(config, count, kem_keygen_worker, NULL, keypairs);
}

/**
 * @brief Batch signature keypair generation
 *
 * @param config Batch configuration
 * @param keypairs Output array for generated keypairs
 * @param count Number of keypairs to generate
 * @return 0 on success, -1 on failure
 */
int hc_sig_generate_batch(const hc_batch_config_t *config,
                          hc_sig_keypair_t *keypairs, int count) {
  if (!config || !keypairs || count <= 0) {
    return -1;
  }

  fprintf(stdout, "[Batch] Generating %d SIG keypairs\n", count);

  return execute_batch(config, count, sig_keygen_worker, NULL, keypairs);
}

// Encapsulation batch worker
typedef struct {
  hc_kem_keypair_t *keypairs;
  hc_ciphertext_t *ciphertexts;
  hc_shared_secret_t *shared_secrets;
} kem_encaps_batch_data_t;

static int kem_encaps_worker(int index, void *input_data, void *output_data) {
  (void)output_data; // Unused
  kem_encaps_batch_data_t *data = (kem_encaps_batch_data_t *)input_data;

  if (!data || !data->keypairs || !data->ciphertexts || !data->shared_secrets) {
    return -1;
  }

  return hc_kem_encaps(&data->keypairs[index], &data->ciphertexts[index],
                       &data->shared_secrets[index]);
}

/**
 * @brief Batch KEM encapsulation
 *
 * High-Throughput Use Cases:
 * - Bulk message encryption
 * - Multi-client key establishment
 * - Load-balanced cryptographic services
 *
 * @param config Batch configuration
 * @param keypairs Input keypairs
 * @param ciphertexts Output ciphertexts
 * @param shared_secrets Output shared secrets
 * @param count Number of operations
 * @return 0 on success, -1 on failure
 */
int hc_kem_encapsulate_batch(const hc_batch_config_t *config,
                             const hc_kem_keypair_t *keypairs,
                             hc_ciphertext_t *ciphertexts,
                             hc_shared_secret_t *shared_secrets, int count) {
  if (!config || !keypairs || !ciphertexts || !shared_secrets || count <= 0) {
    return -1;
  }

  fprintf(stdout, "[Batch] Encapsulating %d messages\n", count);

  kem_encaps_batch_data_t batch_data;
  batch_data.keypairs = (hc_kem_keypair_t *)keypairs;
  batch_data.ciphertexts = ciphertexts;
  batch_data.shared_secrets = shared_secrets;

  return execute_batch(config, count, kem_encaps_worker, &batch_data, NULL);
}

/**
 * @brief Initialize batch configuration with default values
 *
 * Defaults:
 * - thread_count: 0 (auto-detect CPU cores)
 * - batch_size_hint: 1024
 * - enable_prefetch: 1
 * - enable_streaming: 1
 *
 * @param config Configuration structure to initialize
 */
void hc_batch_config_init(hc_batch_config_t *config) {
  if (!config) {
    return;
  }

  config->thread_count = 0; // Auto-detect
  config->batch_size_hint = 1024;
  config->enable_prefetch = 1;
  config->enable_streaming = 1;
}
