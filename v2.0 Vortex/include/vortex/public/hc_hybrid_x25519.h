#ifndef hc_HYBRID_X25519_H
#define hc_HYBRID_X25519_H

#include <stddef.h>
#include <stdint.h>


// Generates both X25519 (classical) and Weave-KEM (PQC) keys
// derived from the SAME vacuum source.
// This binds the security: to break one, you essentially need to break the
// source.
int hc_hybrid_keygen_x25519(uint8_t *x25519_pk, // 32 bytes
                            uint8_t *x25519_sk, // 32 bytes
                            uint8_t *pqc_pk,    // Weave public key
                            uint8_t *pqc_sk     // Weave secret key
);

#endif
