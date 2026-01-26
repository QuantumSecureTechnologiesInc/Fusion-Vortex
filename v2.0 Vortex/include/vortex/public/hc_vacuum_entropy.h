#ifndef hc_VACUUM_ENTROPY_H
#define hc_VACUUM_ENTROPY_H

#include <complex.h>
#include <stdbool.h> // bool
#include <stddef.h>  // size_t
#include <stdint.h>  // uint8_t, int64_t, etc.

// Physical constants
#define hc_PLANCK_REDUCED 1.054571817e-34 // ℏ (J·s)
#define hc_SPEED_OF_LIGHT 299792458.0     // c (m/s)
#define hc_FINE_STRUCTURE (1.0 / 137.036) // α

// Vacuum field parameters
#define hc_VACUUM_CYCLES 47 // Number of evolution cycles
#define hc_VACUUM_DIM 512   // Phase space dimension

// Complex number support
#if defined(_MSC_VER)
// MSVC: Use standard complex if available, otherwise _Dcomplex
// C11 support in MSVC is behind /std:c11 switch.
// If not enabled, _Complex keyword is invalid.
#include <complex.h>
#include <math.h> // Required for cos, sin, sqrt, atan2
#if defined(__STDC_NO_COMPLEX__)
// Fallback if truly no complex support
typedef struct {
  double x, y;
} hc_complex;
#define HC_CREAL(z) ((z).x)
#define HC_CIMAG(z) ((z).y)
// Simple magnitude/phase for fallback struct
#define HC_CABS(z) (sqrt((z).x * (z).x + (z).y * (z).y))
#define HC_CARG(z) (atan2((z).y, (z).x))
#else
// Try to use standard names
// Note: MSVC's complex.h usually maps 'complex' to '_Complex'.
// If '_Complex' fails, we might need to map it to _Dcomplex manually
// BUT _Dcomplex is a struct, not a keyword modifier for double.
// 'double _Complex' is invalid if _Complex == _Dcomplex.
// So we must use a typedef.
typedef _Dcomplex hc_complex;
// _Dcomplex uses _Val[0] (real), _Val[1] (imag)
#define HC_CREAL(z) (creal(z))
#define HC_CIMAG(z) (cimag(z))
// MSVC complex.h defines cabs/_cabs and carg/_carg
#define HC_CABS(z) (cabs(z))
#define HC_CARG(z) (carg(z))
#endif

// Helper functions for MSVC's _Dcomplex or custom struct
// These functions are defined to work with the underlying structure of
// hc_complex which is either _Dcomplex (with _Val[0], _Val[1]) or a custom
// struct (x, y). The macros HC_CREAL and HC_CIMAG abstract this.
static inline hc_complex hc_cadd(hc_complex a, hc_complex b) {
  hc_complex r;
#if defined(__STDC_NO_COMPLEX__)
  r.x = HC_CREAL(a) + HC_CREAL(b);
  r.y = HC_CIMAG(a) + HC_CIMAG(b);
#else
  r._Val[0] = HC_CREAL(a) + HC_CREAL(b);
  r._Val[1] = HC_CIMAG(a) + HC_CIMAG(b);
#endif
  return r;
}
static inline hc_complex hc_csub(hc_complex a, hc_complex b) {
  hc_complex r;
#if defined(__STDC_NO_COMPLEX__)
  r.x = HC_CREAL(a) - HC_CREAL(b);
  r.y = HC_CIMAG(a) - HC_CIMAG(b);
#else
  r._Val[0] = HC_CREAL(a) - HC_CREAL(b);
  r._Val[1] = HC_CIMAG(a) - HC_CIMAG(b);
#endif
  return r;
}
static inline hc_complex hc_cmul(hc_complex a, hc_complex b) {
  hc_complex r;
#if defined(__STDC_NO_COMPLEX__)
  r.x = HC_CREAL(a) * HC_CREAL(b) - HC_CIMAG(a) * HC_CIMAG(b);
  r.y = HC_CREAL(a) * HC_CIMAG(b) + HC_CIMAG(a) * HC_CREAL(b);
#else
  r._Val[0] = HC_CREAL(a) * HC_CREAL(b) - HC_CIMAG(a) * HC_CIMAG(b);
  r._Val[1] = HC_CREAL(a) * HC_CIMAG(b) + HC_CIMAG(a) * HC_CREAL(b);
#endif
  return r;
}
static inline hc_complex hc_cscale(hc_complex a, double s) {
  hc_complex r;
#if defined(__STDC_NO_COMPLEX__)
  r.x = HC_CREAL(a) * s;
  r.y = HC_CIMAG(a) * s;
#else
  r._Val[0] = HC_CREAL(a) * s;
  r._Val[1] = HC_CIMAG(a) * s;
#endif
  return r;
}
static inline hc_complex hc_cconj(hc_complex a) {
  hc_complex r;
#if defined(__STDC_NO_COMPLEX__)
  r.x = HC_CREAL(a);
  r.y = -HC_CIMAG(a);
#else
  r._Val[0] = HC_CREAL(a);
  r._Val[1] = -HC_CIMAG(a);
#endif
  return r;
}
static inline hc_complex hc_cexp_imag(double phase) {
  hc_complex r;
#if defined(__STDC_NO_COMPLEX__)
  r.x = cos(phase);
  r.y = sin(phase);
#else
  r._Val[0] = cos(phase);
  r._Val[1] = sin(phase);
#endif
  return r;
}
static inline hc_complex hc_cbuild(double re, double im) {
  hc_complex r;
#if defined(__STDC_NO_COMPLEX__)
  r.x = re;
  r.y = im;
#else
  r._Val[0] = re;
  r._Val[1] = im;
#endif
  return r;
}

#else
typedef double _Complex hc_complex;
#define HC_CREAL(z) creal(z)
#define HC_CIMAG(z) cimag(z)
#define hc_cadd(a, b) ((a) + (b))
#define hc_csub(a, b) ((a) - (b))
#define hc_cmul(a, b) ((a) * (b))
#define hc_cscale(a, s) ((a) * (s))
#define hc_cconj(a) conj(a)
#define hc_cexp_imag(phi) cexp(I *(phi))
#define hc_cbuild(re, im) ((re) + I * (im))
#define HC_CABS(z) cabs(z)
#define HC_CARG(z) carg(z)
#endif

// Vacuum field state in Wigner-Weyl representation.
// Based on Heisenberg-Euler effective Lagrangian:
// L_HE = (α²ℏ⁴)/(90m_e⁴c⁷) [(E² - B²)² + 7(E·B)²]
typedef struct hc_vacuum_state_t {
  hc_complex psi[hc_VACUUM_DIM]; // Wigner function (phase space)
  hc_complex chi[hc_VACUUM_DIM]; // Conjugate field
  double E_field[3];             // Electric field vector
  double B_field[3];             // Magnetic field vector
  uint64_t evolution_count;      // Cycles completed
  uint8_t seed[32];              // Initial entropy seed
} hc_vacuum_state_t;

/**
 * Generate high-entropy key using 47-cycle vacuum evolution.
 *
 * @param out     Output buffer for generated key
 * @param out_len Length of output buffer
 * @return 0 on success, -1 on failure
 */
int hc_generate_vacuum_key(uint8_t *out, size_t out_len);

/**
 * Initialize vacuum state with system entropy.
 */
int hc_vacuum_init(hc_vacuum_state_t *state);

/**
 * Perform one cycle of Heisenberg-Euler evolution.
 */
void hc_vacuum_evolve_cycle(hc_vacuum_state_t *state);

/**
 * Extract bytes from vacuum state.
 */
void hc_vacuum_extract(const hc_vacuum_state_t *state, uint8_t *out,
                       size_t len);

/**
 * Cleanup vacuum state (zeroize).
 */
void hc_vacuum_cleanup(hc_vacuum_state_t *state);

#endif // hc_VACUUM_ENTROPY_H
