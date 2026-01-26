#include "../include/public/hc_octonion.h"
#include <stdio.h>

void print_oct(const char *label, const hc_octonion_t *o) {
  printf("%s: %.2f + (%.2f e1 + %.2f e2 + ...)\n", label,
         HC_FIXED_TO_DOUBLE(o->s), HC_FIXED_TO_DOUBLE(o->v[0]),
         HC_FIXED_TO_DOUBLE(o->v[1]));
}

int main() {
  printf("=== Octonion Math Kernel Test (Q32.32) ===\n");

  // e1
  hc_octonion_t e1 = {0};
  e1.v[0] = HC_DOUBLE_TO_FIXED(1.0);

  // e2
  hc_octonion_t e2 = {0};
  e2.v[1] = HC_DOUBLE_TO_FIXED(1.0);

  // Multiplication
  hc_octonion_t res;
  hc_oga_mul(&e1, &e2, &res);

  print_oct("e1 * e2", &res);

  // Check e4 (index 3)
  // res.v[3] should be 1.0 (fixed point)
  double v3 = HC_FIXED_TO_DOUBLE(res.v[3]);
  double s = HC_FIXED_TO_DOUBLE(res.s);

  if (v3 > 0.9 && s == 0) {
    printf("[PASS] Fano multiplication e1*e2=e4\n");
  } else {
    printf("[FAIL] Fano multiplication e1*e2=%.2fe4 (Scalar=%.2f)\n", v3, s);
  }

  // Associator Loop: [e1, e2, e3]
  hc_octonion_t e3 = {0};
  e3.v[2] = HC_DOUBLE_TO_FIXED(1.0);

  hc_octonion_t assoc;
  hc_oga_associator(&e1, &e2, &e3, &assoc);

  double norm = HC_FIXED_TO_DOUBLE(hc_oga_norm_sq(&assoc));
  printf("Associator [e1, e2, e3] norm: %f\n", norm);

  if (norm > 0.001) {
    printf("[PASS] Non-Associativity Verified! (Norm > 0)\n");
  } else {
    printf("[FAIL] Result was associative? (Norm ~ 0)\n");
  }

  return 0;
}
