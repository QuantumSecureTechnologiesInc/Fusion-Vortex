// weave_kem.h – Public API for Weave‑KEM

#ifndef WEAVE_KEM_H
#define WEAVE_KEM_H

#include "cemqc.h"
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Keypair structure
// WEAVE_N = 64 (dimensions) * 4 (quat components) * 2 (bytes per component) ->
// compressed
#define WEAVE_KEM_PUBLIC_KEY_SIZE 96
#define WEAVE_KEM_SECRET_KEY_SIZE 192
#define WEAVE_KEM_CIPHERTEXT_SIZE 128
#define WEAVE_KEM_SHARED_SECRET_SIZE 32

typedef struct {
  unsigned char public_key[WEAVE_KEM_PUBLIC_KEY_SIZE];
  unsigned char secret_key[WEAVE_KEM_SECRET_KEY_SIZE];
} hc_kem_keypair_t;

// Ciphertext and shared secret structures
typedef struct {
  unsigned char data[WEAVE_KEM_CIPHERTEXT_SIZE];
} hc_ciphertext_t;

typedef struct {
  unsigned char data[WEAVE_KEM_SHARED_SECRET_SIZE];
} hc_shared_secret_t;

// Key generation
int hc_kem_keygen(hc_kem_keypair_t *kp);

// Encapsulation
int hc_kem_encaps(const hc_kem_keypair_t *kp, hc_ciphertext_t *ct,
                  hc_shared_secret_t *ss);

// Decapsulation
int hc_kem_decaps(const hc_kem_keypair_t *kp, const hc_ciphertext_t *ct,
                  hc_shared_secret_t *ss);

#ifdef __cplusplus
}
#endif

#endif // WEAVE_KEM_H
