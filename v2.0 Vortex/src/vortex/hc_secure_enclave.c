#include "vortex/public/hc_secure_enclave.h"
#include "vortex/public/hc_secure_memory.h"
#include "vortex/public/hc_vacuum_entropy.h"
#include <string.h>

int hc_enclave_store_key(const uint8_t *key, size_t len) {
  if (!key || len == 0)
    return -1;

  // Allocate secure storage (locked in RAM)
  uint8_t *secure_key = hc_secure_alloc(len);
  if (!secure_key)
    return -1;

  // Copy key to secure memory
  memcpy(secure_key, key, len);

  // In production: Store handle/reference for later retrieval
  // For now: Just demonstrate secure storage then cleanup

  // Cleanup (zeroize and free)
  hc_secure_free(secure_key, len);
  return 0;
}

int hc_enclave_generate_vacuum_key(uint8_t *out, size_t len) {
  // Generate key entirely within "enclave" (secure memory context)
  return hc_generate_vacuum_key(out, len);
}
