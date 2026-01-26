#ifndef hc_OCTONION_H
#define hc_OCTONION_H

#include <stdalign.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Data Structures
// ============================================================================

/**
 * @brief Octonion Multivector (Geometric Algebra of R7)
 *
 * Represents an element in the Octonion algebra O = R + R7.
 * Consists of 1 Real Scalar and 7 Imaginary Units (vectors).
 *
 * Aligned to 64 bytes for AVX-512 future-proofing.
 */
// Fixed-Point Q32.32 Format
// 32 bits integer, 32 bits fraction. Range [-2^31, 2^31). Resolution 2^-32.
#define hc_Q32_32_SCALE (1LL << 32)
#define hc_DOUBLE_TO_FIXED(x) ((int64_t)((x) * hc_Q32_32_SCALE))
#define hc_FIXED_TO_DOUBLE(x) ((double)(x) / hc_Q32_32_SCALE)

typedef struct {
  alignas(64) int64_t s; // Scalar part (Q32.32)
  int64_t v[7];          // Vector part (Q32.32)
} hc_octonion_t;

// ============================================================================
// Core Algebra API (Non-Associative)
// ============================================================================

/**
 * @brief Geometric Product of two Octonions: R = A * B
 *
 * Note: This operation is Non-Commutative AND Non-Associative.
 * (AB)C != A(BC)
 */
void hc_oga_mul(const hc_octonion_t *a, const hc_octonion_t *b,
                hc_octonion_t *out);

/**
 * @brief Conjugate of an Octonion
 * \bar{A} = s - v
 */
void hc_oga_conjugate(const hc_octonion_t *a, hc_octonion_t *out);

/**
 * @brief Norm Squared of an Octonion
 * |A|^2 = A * \bar{A} = s^2 + v1^2 + ... + v7^2
 */
int64_t hc_oga_norm_sq(const hc_octonion_t *a);

/**
 * @brief Inverse of an Octonion
 * A^-1 = \bar{A} / |A|^2
 */
void hc_oga_inverse(const hc_octonion_t *a, hc_octonion_t *out);

/**
 * @brief The Associator [A, B, C]
 * [A,B,C] = (AB)C - A(BC)
 *
 * This is the cryptographic primitive for O-GA-KEM.
 * Zero if A, B, C are associative (in a subfield).
 * Non-zero otherwise.
 */
void hc_oga_associator(const hc_octonion_t *a, const hc_octonion_t *b,
                       const hc_octonion_t *c, hc_octonion_t *out);

// ============================================================================
// Helper Operations
// ============================================================================

// 7D Dot Product
int64_t hc_dot_product_7d(const int64_t *a, const int64_t *b);

// 7D Cross Product (The core of octonion multiplication)
void hc_cross_product_7d(const int64_t *a, const int64_t *b, int64_t *out);

#ifdef __cplusplus
}
#endif

#endif // hc_OCTONION_H
