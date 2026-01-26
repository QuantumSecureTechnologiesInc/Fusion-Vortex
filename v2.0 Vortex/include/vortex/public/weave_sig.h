// weave_sig.h – Public API for Weave‑SIG

#ifndef WEAVE_SIG_H
#define WEAVE_SIG_H

#include "cemqc.h"
#include <stddef.h>


#ifdef __cplusplus
extern "C" {
#endif

// Keypair and Signature structure
// WEAVE_SIG_N = 16 quaternions
#define WEAVE_SIG_PUBLIC_KEY_SIZE 96
#define WEAVE_SIG_SECRET_KEY_SIZE 192
#define WEAVE_SIG_SIGNATURE_SIZE 96

typedef struct {
  unsigned char public_key[WEAVE_SIG_PUBLIC_KEY_SIZE];
  unsigned char secret_key[WEAVE_SIG_SECRET_KEY_SIZE];
} hc_sig_keypair_t;

// Signature structure
typedef struct {
  unsigned char data[WEAVE_SIG_SIGNATURE_SIZE];
} hc_signature_t;

// Key generation
int hc_sig_keygen(hc_sig_keypair_t *kp);

// Sign
int hc_sig_sign(const hc_sig_keypair_t *kp, const unsigned char *msg,
                size_t msg_len, hc_signature_t *sig);

// Verify
int hc_sig_verify(const hc_sig_keypair_t *kp, const unsigned char *msg,
                  size_t msg_len, const hc_signature_t *sig);

#ifdef __cplusplus
}
#endif

#endif // WEAVE_SIG_H
