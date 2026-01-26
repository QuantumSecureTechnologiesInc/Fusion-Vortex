#ifndef hc_SHA3_H
#define hc_SHA3_H

#include <stddef.h>
#include <stdint.h>

// FIPS 202 SHA3-256 implementation
void hc_sha3_256(const uint8_t *in, size_t inlen, uint8_t *out);

// FIPS 202 SHAKE256 implementation (XOF)
void hc_shake256(uint8_t *out, size_t outlen, const uint8_t *in, size_t inlen);

#endif
