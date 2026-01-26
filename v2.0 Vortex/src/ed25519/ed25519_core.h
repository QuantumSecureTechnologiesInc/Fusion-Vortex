/*
 * ed25519_core.h
 *
 * Core cryptographic operations for Ed25519.
 * Includes scalar arithmetic, SHA-512, and curve operations.
 */

#ifndef ED25519_CORE_H
#define ED25519_CORE_H

#include <stddef.h>
#include <stdint.h>

/* Point on Edwards curve (projective + extended coordinates) */
typedef struct {
  uint32_t X[10]; /* x coordinate (field element) */
  uint32_t Y[10]; /* y coordinate (field element) */
  uint32_t Z[10]; /* z coordinate (field element) */
  uint32_t T[10]; /* t = x*y/z (cached for speed) */
} ge_p3;

/* SHA-512 */
void sha512(uint8_t *digest, const uint8_t *message, size_t len);

/* Scalar operations (modulo L = 2^252 + 27742317777884353535851937790883648493)
 */
void sc_clamp(uint8_t *k);
void sc_reduce(uint8_t *s);
void sc_reduce64(uint8_t *r, const uint8_t *x);
void sc_add(uint8_t *z, const uint8_t *x, const uint8_t *y);
void sc_mul(uint8_t *z, const uint8_t *x, const uint8_t *y);

/* Group operations (twisted Edwards curve) */
void ge_scalarmult_base(uint8_t *q, const uint8_t *e);
void ge_scalarmult(uint8_t *q, const uint8_t *e, const uint8_t *p);
void ge_add(uint8_t *r, const uint8_t *p, const uint8_t *q);
void ge_p3_to_bytes(uint8_t *s, const ge_p3 *h);

#endif /* ED25519_CORE_H */
