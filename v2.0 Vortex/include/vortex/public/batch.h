// batch.h – Public API for batch operations
// High-throughput parallel cryptographic operations
// Part of HyperCycle v3.2 Fulminis

#ifndef BATCH_H
#define BATCH_H

#include "hypercycle_algorithms.h"
#include "weave_kem.h"
#include "weave_sig.h"

#ifdef __cplusplus
extern "C" {
#endif

// Configuration for batch processing
typedef struct {
  unsigned int thread_count;    // 0 = auto-detect CPU cores
  unsigned int batch_size_hint; // Hint for optimal batch size
  int enable_prefetch;          // Enable memory prefetching
  int enable_streaming;         // Enable streaming operations
} hc_batch_config_t;

/**
 * @brief Initialize batch configuration with default values
 * @param config Configuration structure to initialize
 */
void hc_batch_config_init(hc_batch_config_t *config);

/**
 * @brief Batch KEM keypair generation
 * @param config Batch configuration
 * @param keypairs Output array for generated keypairs
 * @param count Number of keypairs to generate
 * @return 0 on success, -1 on failure
 */
int hc_kem_generate_batch(const hc_batch_config_t *config,
                          hc_kem_keypair_t *keypairs, int count);

/**
 * @brief Batch signature keypair generation
 * @param config Batch configuration
 * @param keypairs Output array for generated keypairs
 * @param count Number of keypairs to generate
 * @return 0 on success, -1 on failure
 */
int hc_sig_generate_batch(const hc_batch_config_t *config,
                          hc_sig_keypair_t *keypairs, int count);

/**
 * @brief Batch KEM encapsulation
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
                             hc_shared_secret_t *shared_secrets, int count);

#ifdef __cplusplus
}
#endif

#endif // BATCH_H
