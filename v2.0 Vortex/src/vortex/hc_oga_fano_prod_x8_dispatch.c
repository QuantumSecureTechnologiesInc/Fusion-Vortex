// hc_oga_fano_prod_x8_dispatch.c
// Runtime dispatch for the 8-way octonion multiplication primitive.
// If AVX-512IFMA is available at runtime, the IFMA kernel is used.
// Otherwise a portable fallback is used.

#include "vortex/internal/hc_cpu_features.h"
#include "vortex/public/hypercycle_v1.h"

// Fallback always available
void hc_oga_fano_prod_x8_fallback(hc_oct_x8_t *res, const hc_oct_x8_t *a,
                                  const hc_oct_x8_t *b);

// IFMA kernel symbol (only linked when compiled on toolchain supporting it)
#if defined(__AVX512F__) && defined(__AVX512IFMA__)
void hc_oga_fano_prod_x8_ifma(hc_oct_x8_t *res, const hc_oct_x8_t *a,
                              const hc_oct_x8_t *b);
#endif

void hc_oga_fano_prod_x8(hc_oct_x8_t *res, const hc_oct_x8_t *a,
                         const hc_oct_x8_t *b) {
#if defined(__x86_64__) || defined(_M_X64) || defined(__i386) ||               \
    defined(_M_IX86)
  if (hc_cpu_has_avx512ifma()) {
#if defined(__AVX512F__) && defined(__AVX512IFMA__)
    hc_oga_fano_prod_x8_ifma(res, a, b);
    return;
#endif
  }
#endif
  hc_oga_fano_prod_x8_fallback(res, a, b);
}
