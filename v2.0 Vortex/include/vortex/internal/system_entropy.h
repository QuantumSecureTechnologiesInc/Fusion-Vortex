// system_entropy.h – System entropy collection interface
// Part of HyperCycle v3.2 Fulminis Pure Quaternion-Chaos Architecture

#ifndef hc_SYSTEM_ENTROPY_H
#define hc_SYSTEM_ENTROPY_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Gather cryptographically secure entropy from the operating system
 *
 * @param buffer Output buffer for entropy bytes
 * @param length Number of bytes to generate
 * @return 0 on success, -1 on failure
 */
int hc_system_entropy(unsigned char *buffer, size_t length);

/**
 * @brief Mix additional entropy into an existing buffer using chaotic diffusion
 *
 * @param buffer Existing buffer to mix entropy into
 * @param length Buffer length
 * @param new_entropy New entropy to mix in
 * @param new_length Length of new entropy
 * @return 0 on success, -1 on failure
 */
int hc_entropy_mix(unsigned char *buffer, size_t length,
                   const unsigned char *new_entropy, size_t new_length);

/**
 * @brief Generate high-quality entropy for cryptographic key generation
 *
 * @param buffer Output buffer for entropy
 * @param length Number of bytes to generate
 * @return 0 on success, -1 on failure
 */
int hc_cryptographic_entropy(unsigned char *buffer, size_t length);

#ifdef __cplusplus
}
#endif

#endif // hc_SYSTEM_ENTROPY_H
