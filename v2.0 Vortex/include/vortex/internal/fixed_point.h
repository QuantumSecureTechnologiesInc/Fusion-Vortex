/**
 * Fixed-Point Arithmetic Library (Q32.32 Format)
 * HyperCycle v3.2 Fulminis - Security Hardening
 *
 * Provides deterministic, constant-time arithmetic for cryptographic
 * operations. Eliminates FPU side-channel vulnerabilities while improving
 * performance.
 *
 * Format: Q32.32 (64-bit total: 32-bit integer, 32-bit fraction)
 * Range: ±2,147,483,648.0 with ~10^-9 precision
 */

#ifndef hc_FIXED_POINT_H
#define hc_FIXED_POINT_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Fixed-point type: Q32.32 format (32 integer bits, 32 fractional bits)
typedef int64_t fixed_t;

// Constants
#define FIXED_FRACTIONAL_BITS 32
#define FIXED_ONE (1LL << FIXED_FRACTIONAL_BITS)        // 1.0 in fixed-point
#define FIXED_HALF (1LL << (FIXED_FRACTIONAL_BITS - 1)) // 0.5

// Mathematical constants
#define FIXED_PI 3373259426LL    // π ≈ 3.141592653
#define FIXED_E 2874452256LL     // e ≈ 2.718281828
#define FIXED_SQRT2 6074000999LL // √2 ≈ 1.414213562

/**
 * Convert integer to fixed-point
 */
static inline fixed_t fixed_from_int(int32_t i) {
  return ((int64_t)i) << FIXED_FRACTIONAL_BITS;
}

/**
 * Convert fixed-point to integer (truncate)
 */
static inline int32_t fixed_to_int(fixed_t f) {
  return (int32_t)(f >> FIXED_FRACTIONAL_BITS);
}

/**
 * Convert double to fixed-point (for initialization/testing only)
 */
static inline fixed_t fixed_from_double(double d) {
  return (fixed_t)(d * (double)FIXED_ONE);
}

/**
 * Convert fixed-point to double (for testing/debugging only)
 */
static inline double fixed_to_double(fixed_t f) {
  return (double)f / (double)FIXED_ONE;
}

/**
 * Fixed-point addition (constant-time)
 */
static inline fixed_t fixed_add(fixed_t a, fixed_t b) { return a + b; }

/**
 * Fixed-point subtraction (constant-time)
 */
static inline fixed_t fixed_sub(fixed_t a, fixed_t b) { return a - b; }

/**
 * Fixed-point multiplication (constant-time)
 * Uses 128-bit intermediate to prevent overflow
 */
#if defined(_MSC_VER)
#include <intrin.h>
#endif

/**
 * Fixed-point multiplication (constant-time)
 * Uses 128-bit intermediate to prevent overflow
 */
static inline fixed_t fixed_mul(fixed_t a, fixed_t b) {
#if defined(__SIZEOF_INT128__) && !defined(_MSC_VER)
  __int128 result = (__int128)a * (__int128)b;
  return (fixed_t)(result >> FIXED_FRACTIONAL_BITS);
#elif defined(_MSC_VER) && defined(_M_X64)
  int64_t high;
  int64_t low = _mul128(a, b, &high);
  // (high << 32) | (low >> 32)
  return (fixed_t)((high << 32) | ((uint64_t)low >> 32));
#else
  // Fallback (loss of precision or overflow risk on 32-bit/MSVC-x86)
  // Simple separate multiply
  int64_t a_hi = a >> 32;
  int64_t a_lo = a & 0xFFFFFFFFLL;
  int64_t b_hi = b >> 32;
  int64_t b_lo = b & 0xFFFFFFFFLL;

  int64_t result_hi = a_hi * b_hi;
  int64_t result_mid = (a_hi * b_lo + a_lo * b_hi);
  int64_t result_lo = (a_lo * b_lo) >> FIXED_FRACTIONAL_BITS;

  return (result_hi << 32) + result_mid + result_lo;
#endif
}

/**
 * Fixed-point division (constant-time)
 */
static inline fixed_t fixed_div(fixed_t a, fixed_t b) {
#if defined(__SIZEOF_INT128__) && !defined(_MSC_VER)
  __int128 temp = ((__int128)a) << FIXED_FRACTIONAL_BITS;
  return (fixed_t)(temp / b);
#elif defined(_MSC_VER) && defined(_M_X64)
  // a << 32 division
  int64_t remainder;
  int64_t high = a >> 32; // Sign extension works for arithmetic shift
  int64_t low = a << 32;
  return _div128(high, low, b, &remainder);
#else
  // Fallback: limited precision
  return (fixed_t)((a * (1LL << 16)) / (b >> 16));
#endif
}

/**
 * Fixed-point negation
 */
static inline fixed_t fixed_neg(fixed_t a) { return -a; }

/**
 * Fixed-point absolute value
 */
static inline fixed_t fixed_abs(fixed_t a) { return (a < 0) ? -a : a; }

/**
 * Fixed-point square (optimized multiplication)
 */
static inline fixed_t fixed_square(fixed_t a) { return fixed_mul(a, a); }

/**
 * Fixed-point square root (Newton-Raphson method)
 * Constant-time approximation
 */
fixed_t fixed_sqrt(fixed_t x);

/**
 * Fixed-point sine approximation (Taylor series)
 * Input: angle in fixed-point radians
 * Constant-time implementation
 */
fixed_t fixed_sin(fixed_t x);

/**
 * Fixed-point cosine approximation (Taylor series)
 * Input: angle in fixed-point radians
 * Constant-time implementation
 */
fixed_t fixed_cos(fixed_t x);

/**
 * Quaternion operations using fixed-point arithmetic
 */
typedef struct {
  fixed_t w, x, y, z;
} fixed_quat_t;

/**
 * Quaternion multiplication (fixed-point, constant-time)
 */
void fixed_quat_mul(const fixed_quat_t *a, const fixed_quat_t *b,
                    fixed_quat_t *result);

/**
 * Quaternion addition (fixed-point, constant-time)
 */
void fixed_quat_add(const fixed_quat_t *a, const fixed_quat_t *b,
                    fixed_quat_t *result);

/**
 * Quaternion conjugate (fixed-point, constant-time)
 */
void fixed_quat_conj(const fixed_quat_t *a, fixed_quat_t *result);

/**
 * Quaternion normalization (fixed-point, constant-time)
 */
void fixed_quat_normalize(fixed_quat_t *q);

#ifdef __cplusplus
}
#endif

#endif // hc_FIXED_POINT_H
