/*
 * ed25519_field.h
 *
 * Field arithmetic header for GF(2^255 - 19).
 */

#ifndef ED25519_FIELD_H
#define ED25519_FIELD_H

#include <stdint.h>
#include <stddef.h>

/* Field element represented as 10 x uint32 limbs in radix 2^26 */
typedef uint32_t fe[10];

/* Modular addition: h = (f + g) mod p */
void fe_add(uint32_t *h, const uint32_t *f, const uint32_t *g);

/* Modular subtraction: h = (f - g) mod p */
void fe_sub(uint32_t *h, const uint32_t *f, const uint32_t *g);

/* Modular multiplication: h = (f * g) mod p */
void fe_mul(uint32_t *h, const uint32_t *f, const uint32_t *g);

/* Modular squaring: h = (f * f) mod p */
void fe_sq(uint32_t *h, const uint32_t *f);

/* Modular inversion: h = f^(p-2) mod p (Fermat's little theorem) */
void fe_inv(uint32_t *h, const uint32_t *f);

/* Load field element from 32-byte little-endian bytes */
void fe_from_bytes(uint32_t *h, const uint8_t *s);

/* Store field element to 32-byte little-endian bytes */
void fe_to_bytes(uint8_t *s, const uint32_t *h);

/* Conditional negate: if b=1, negate f */
void fe_cond_neg(uint32_t *h, const uint32_t *f, int b);

/* Check if field element is negative (LSB of encoding) */
int fe_is_negative(const uint32_t *f);

#endif /* ED25519_FIELD_H */
