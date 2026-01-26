// cemqc.h – Public API for core CEMQC primitives

#ifndef CEMQC_H
#define CEMQC_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// RNG state (concrete implementation)
typedef struct {
  uint64_t state; // 64‑bit XORShift state
} hc_rng_state_t;

// Initialise RNG with seed
void hc_rng_init(hc_rng_state_t *state, const unsigned char *seed,
                 size_t seed_len);

// Generate random bytes
void hc_rng_generate(hc_rng_state_t *state, unsigned char *out, size_t out_len);

// Quaternion type
typedef struct {
  double w, x, y, z;
} hc_quaternion_t;

// Constant‑time quaternion multiplication
void hc_quaternion_mul(const hc_quaternion_t *a, const hc_quaternion_t *b,
                       hc_quaternion_t *out);

// Constant‑time quaternion addition
void hc_quaternion_add(const hc_quaternion_t *a, const hc_quaternion_t *b,
                       hc_quaternion_t *out);

// Constant‑time quaternion subtraction
void hc_quaternion_sub(const hc_quaternion_t *a, const hc_quaternion_t *b,
                       hc_quaternion_t *out);

// Quaternion norm squared (constant‑time)
void hc_quaternion_norm_sq(const hc_quaternion_t *a, double *out);

// Quaternion conjugate: q* = w - xi - yj - zk
void hc_quaternion_conjugate(const hc_quaternion_t *q, hc_quaternion_t *out);

// Quaternion inverse: q^(-1) = q* / ||q||^2
// Returns 0 on success, -1 if quaternion is zero (no inverse)
int hc_quaternion_inverse(const hc_quaternion_t *q, hc_quaternion_t *out);

// Quaternion exponentiation: compute q^n using repeated multiplication
// For cryptographic use: q^secret_scalar
void hc_quaternion_power(const hc_quaternion_t *base, uint32_t exponent,
                         hc_quaternion_t *out);

// Quaternion scalar multiplication
void hc_quaternion_scale(const hc_quaternion_t *q, double scalar,
                         hc_quaternion_t *out);

// Quaternion norm squared (returns double, not void)
double hc_quaternion_norm_squared(const hc_quaternion_t *q);

// Convert chaos state bytes to quaternion for masking
void hc_chaos_to_quaternion(const unsigned char *chaos_bytes, size_t len,
                            hc_quaternion_t *out);

// Encode message bytes into quaternion representation
void hc_message_to_quaternion(const unsigned char *msg, size_t msg_len,
                              hc_quaternion_t *out);

#ifdef __cplusplus
}
#endif

#endif // CEMQC_H
