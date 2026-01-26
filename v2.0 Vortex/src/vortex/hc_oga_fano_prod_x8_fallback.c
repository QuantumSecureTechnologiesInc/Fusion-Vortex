// hc_oga_fano_prod_x8_fallback.c
// Portable fallback for 8-way octonion multiplication used by optimized keygen.
//
// This implementation is intentionally conservative: it reuses the existing
// scalar hc_oga_mul() logic lane-by-lane to guarantee correctness.
// If AVX-512 is available, higher-level code can still vectorize other parts.

#include "vortex/public/hc_octonion.h"
#include "vortex/public/hypercycle_v1.h"
#include <string.h>

void hc_oga_fano_prod_x8_fallback(hc_oct_x8_t *res, const hc_oct_x8_t *a,
                                  const hc_oct_x8_t *b) {
  for (int lane = 0; lane < 8; lane++) {
    hc_octonion_t A;
    hc_octonion_t B;
    hc_octonion_t R;

    A.s = (int64_t)a->s[lane];
    B.s = (int64_t)b->s[lane];
    for (int i = 0; i < 7; i++) {
      A.v[i] = (int64_t)a->v[i][lane];
      B.v[i] = (int64_t)b->v[i][lane];
    }

    hc_oga_mul(&A, &B, &R);

    res->s[lane] = (uint64_t)R.s;
    for (int i = 0; i < 7; i++) {
      res->v[i][lane] = (uint64_t)R.v[i];
    }
  }
}
