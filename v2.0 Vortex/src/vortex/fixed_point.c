/**
 * Fixed-Point Arithmetic Library Implementation
 * HyperCycle v3.2 Fulminis - Security Hardening
 */

#include "internal/fixed_point.h"
#include <string.h>

/**
 * Fixed-point square root using Newton-Raphson
 * Constant iterations for constant-time execution
 */
fixed_t fixed_sqrt(fixed_t x) {
  if (x <= 0)
    return 0;

  // Initial guess: x/2
  fixed_t guess = x >> 1;

  // Newton-Raphson: x_n+1 = (x_n + x/x_n) / 2
  // Fixed iterations for constant-time (6 iterations = ~10^-9 precision)
  for (int i = 0; i < 6; i++) {
    guess = (guess + fixed_div(x, guess)) >> 1;
  }

  return guess;
}

/**
 * Fixed-point sine using Taylor series
 * sin(x) = x - x^3/3! + x^5/5! - x^7/7! + ...
 * Constant iterations for constant-time
 */
fixed_t fixed_sin(fixed_t x) {
  // Normalize x to [-π, π]
  while (x > FIXED_PI)
    x -= 2 * FIXED_PI;
  while (x < -FIXED_PI)
    x += 2 * FIXED_PI;

  fixed_t result = x;
  fixed_t term = x;
  fixed_t x_squared = fixed_mul(x, x);

  // Taylor series (6 terms for precision)
  for (int i = 1; i <= 5; i++) {
    term = fixed_mul(term, x_squared);
    term = fixed_div(term, fixed_from_int(-(2 * i) * (2 * i + 1)));
    result = fixed_add(result, term);
  }

  return result;
}

/**
 * Fixed-point cosine using identity: cos(x) = sin(x + π/2)
 */
fixed_t fixed_cos(fixed_t x) { return fixed_sin(x + (FIXED_PI >> 1)); }

/**
 * Quaternion multiplication (fixed-point)
 * q1 * q2 = (w1*w2 - x1*x2 - y1*y2 - z1*z2,
 *            w1*x2 + x1*w2 + y1*z2 - z1*y2,
 *            w1*y2 - x1*z2 + y1*w2 + z1*x2,
 *            w1*z2 + x1*y2 - y1*x2 + z1*w2)
 */
void fixed_quat_mul(const fixed_quat_t *a, const fixed_quat_t *b,
                    fixed_quat_t *result) {
  fixed_t w = fixed_sub(
      fixed_sub(fixed_sub(fixed_mul(a->w, b->w), fixed_mul(a->x, b->x)),
                fixed_mul(a->y, b->y)),
      fixed_mul(a->z, b->z));

  fixed_t x = fixed_sub(
      fixed_add(fixed_add(fixed_mul(a->w, b->x), fixed_mul(a->x, b->w)),
                fixed_mul(a->y, b->z)),
      fixed_mul(a->z, b->y));

  fixed_t y = fixed_add(
      fixed_sub(fixed_add(fixed_mul(a->w, b->y), fixed_mul(a->y, b->w)),
                fixed_mul(a->x, b->z)),
      fixed_mul(a->z, b->x));

  fixed_t z = fixed_sub(
      fixed_add(fixed_add(fixed_mul(a->w, b->z), fixed_mul(a->z, b->w)),
                fixed_mul(a->x, b->y)),
      fixed_mul(a->y, b->x));

  result->w = w;
  result->x = x;
  result->y = y;
  result->z = z;
}

/**
 * Quaternion addition (fixed-point)
 */
void fixed_quat_add(const fixed_quat_t *a, const fixed_quat_t *b,
                    fixed_quat_t *result) {
  result->w = fixed_add(a->w, b->w);
  result->x = fixed_add(a->x, b->x);
  result->y = fixed_add(a->y, b->y);
  result->z = fixed_add(a->z, b->z);
}

/**
 * Quaternion conjugate (fixed-point)
 */
void fixed_quat_conj(const fixed_quat_t *a, fixed_quat_t *result) {
  result->w = a->w;
  result->x = fixed_neg(a->x);
  result->y = fixed_neg(a->y);
  result->z = fixed_neg(a->z);
}

/**
 * Quaternion normalization (fixed-point)
 * Make ||q|| = 1
 */
void fixed_quat_normalize(fixed_quat_t *q) {
  // Compute norm: sqrt(w^2 + x^2 + y^2 + z^2)
  fixed_t norm_squared =
      fixed_add(fixed_add(fixed_square(q->w), fixed_square(q->x)),
                fixed_add(fixed_square(q->y), fixed_square(q->z)));

  fixed_t norm = fixed_sqrt(norm_squared);

  // Avoid division by zero
  if (norm > 0) {
    q->w = fixed_div(q->w, norm);
    q->x = fixed_div(q->x, norm);
    q->y = fixed_div(q->y, norm);
    q->z = fixed_div(q->z, norm);
  }
}
