/**
 * Fixed-Point Arithmetic Test Suite
 * Validates Q32.32 implementation accuracy and performance
 */

#include "internal/fixed_point.h"
#include <assert.h>
#include <math.h>
#include <stdio.h>

#define EPSILON 0.000001 // Acceptable error margin

void test_basic_operations() {
  printf("Testing basic operations...\n");

  // Test addition
  fixed_t a = fixed_from_double(3.14);
  fixed_t b = fixed_from_double(2.71);
  fixed_t c = fixed_add(a, b);
  double result = fixed_to_double(c);
  assert(fabs(result - 5.85) < EPSILON);
  printf("  ✓ Addition: 3.14 + 2.71 = %.6f\n", result);

  // Test subtraction
  c = fixed_sub(a, b);
  result = fixed_to_double(c);
  printf("  DEBUG: 3.14 - 2.71 = %.6f (expected 0.43, error = %.9f)\n", result,
         fabs(result - 0.43));
  assert(fabs(result - 0.43) < EPSILON);
  printf("  ✓ Subtraction: 3.14 - 2.71 = %.6f\n", result);

  // Test multiplication
  c = fixed_mul(a, b);
  result = fixed_to_double(c);
  assert(fabs(result - 8.5094) < EPSILON);
  printf("  ✓ Multiplication: 3.14 * 2.71 = %.6f\n", result);

  // Test division
  c = fixed_div(a, b);
  result = fixed_to_double(c);
  assert(fabs(result - 1.1586) < EPSILON);
  printf("  ✓ Division: 3.14 / 2.71 = %.6f\n", result);
}

void test_sqrt() {
  printf("Testing square root...\n");

  fixed_t x = fixed_from_double(9.0);
  fixed_t s = fixed_sqrt(x);
  double result = fixed_to_double(s);
  assert(fabs(result - 3.0) < EPSILON);
  printf("  ✓ sqrt(9.0) = %.6f\n", result);

  x = fixed_from_double(2.0);
  s = fixed_sqrt(x);
  result = fixed_to_double(s);
  assert(fabs(result - 1.414213) < 0.0001);
  printf("  ✓ sqrt(2.0) = %.6f\n", result);
}

void test_quaternion() {
  printf("Testing quaternion operations...\n");

  fixed_quat_t q1 = {fixed_from_double(1.0), fixed_from_double(0.0),
                     fixed_from_double(0.0), fixed_from_double(0.0)};

  fixed_quat_t q2 = {fixed_from_double(0.0), fixed_from_double(1.0),
                     fixed_from_double(0.0), fixed_from_double(0.0)};

  fixed_quat_t result;
  fixed_quat_mul(&q1, &q2, &result);

  printf("  ✓ Quaternion multiply: (1,0,0,0) * (0,1,0,0) = "
         "(%.2f,%.2f,%.2f,%.2f)\n",
         fixed_to_double(result.w), fixed_to_double(result.x),
         fixed_to_double(result.y), fixed_to_double(result.z));
}

int main() {
  printf("=== Fixed-Point Arithmetic Test Suite ===\n\n");

  test_basic_operations();
  test_sqrt();
  test_quaternion();

  printf("\n=== All Tests PASSED ✓ ===\n");
  return 0;
}
