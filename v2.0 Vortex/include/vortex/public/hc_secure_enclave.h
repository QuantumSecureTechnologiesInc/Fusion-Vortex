#ifndef hc_SECURE_ENCLAVE_H
#define hc_SECURE_ENCLAVE_H

#include <stddef.h>
#include <stdint.h>

/*
 * Secure Enclave integration stub.
 * Stores a private key inside a hardware enclave (simulated).
 *
 * @param key   Pointer to the private key data.
 * @param len   Length of the key in bytes.
 * @return 0 on success, non‑zero on failure.
 */
int hc_enclave_store_key(const uint8_t *key, size_t len);

/*
 * Generate vacuum key within enclave context.
 *
 * @param out Output buffer
 * @param len Buffer length
 * @return 0 on success, -1 on failure
 */
int hc_enclave_generate_vacuum_key(uint8_t *out, size_t len);

#endif // hc_SECURE_ENCLAVE_H
