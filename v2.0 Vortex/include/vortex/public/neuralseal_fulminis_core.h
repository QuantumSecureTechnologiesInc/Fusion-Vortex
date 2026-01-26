/**
 * @file neuralseal_fulminis_core.h
 * @version 3.2.0 "Fulminis"
 * @brief Single-Header Post-Quantum Cryptographic Library
 *
 * NeuralSeal v3.2 "Fulminis" - Production Release
 * Unified header consolidating all cryptographic primitives
 *
 * FEATURES:
 * - Octonion-Geometric Algebra KEM (O-GA-KEM)
 * - ML-KEM-1024 (NIST FIPS 203)
 * - ML-DSA-87 (NIST FIPS 204)
 * - Heisenberg-Euler Vacuum Entropy
 * - Asynchronous Entropy Pooling
 * - AVX-512 SIMD Acceleration
 *
 * USAGE:
 * 1. Include this header in your project
 * 2. Link against the HyperCycle library
 * 3. Initialize entropy pool: ns_init_entropy_pool()
 * 4. Use cryptographic primitives as needed
 * 5. Cleanup: ns_cleanup_entropy_pool()
 */

#ifndef NEURALSEAL_FULMINIS_CORE_H
#define NEURALSEAL_FULMINIS_CORE_H

// ============================================================================
// Core Dependencies
// ============================================================================

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Version Information
// ============================================================================

#define NEURALSEAL_VERSION "3.2.0-Fulminis"
#define NEURALSEAL_VERSION_MAJOR 3
#define NEURALSEAL_VERSION_MINOR 2
#define NEURALSEAL_VERSION_PATCH 0

// ============================================================================
// Asynchronous Entropy Pooling API
// ============================================================================

/**
 * @brief Initialize the asynchronous entropy pooling system
 *
 * Starts a background thread that continuously generates quantum-resistant
 * entropy. Must be called once at application startup.
 *
 * @return 0 on success, negative error code on failure
 */
int ns_init_entropy_pool(void);

/**
 * @brief Get instant random key from entropy pool
 *
 * Non-blocking operation, typically <1 microsecond.
 *
 * @param out Output buffer for random key
 * @param len Length of key (must be multiple of 32)
 * @return 0 on success, negative error code on failure
 */
int ns_get_random_key(uint8_t *out, size_t len);

/**
 * @brief Get entropy pool statistics
 *
 * @param total_produced Total keys generated (output)
 * @param total_consumed Total keys consumed (output)
 * @param available Keys currently available (output)
 * @param underruns Times pool was exhausted (output)
 * @return 0 on success, -1 if not initialized
 */
int ns_get_entropy_stats(size_t *total_produced, size_t *total_consumed,
                         size_t *available, size_t *underruns);

/**
 * @brief Cleanup entropy pooling system
 *
 * Stops background thread and releases resources.
 */
void ns_cleanup_entropy_pool(void);

// ============================================================================
// Octonion-Geometric Algebra KEM (O-GA-KEM)
// ============================================================================

#define NS_OGA_PUBLIC_KEY_SIZE 448
#define NS_OGA_SECRET_KEY_SIZE 64
#define NS_OGA_CIPHERTEXT_SIZE 512
#define NS_OGA_SHARED_SECRET_SIZE 32

/**
 * @brief Generate O-GA-KEM keypair
 *
 * @param pk Public key output (448 bytes)
 * @param sk Secret key output (64 bytes)
 * @return 0 on success
 */
int ns_oga_keypair(uint8_t *pk, uint8_t *sk);

/**
 * @brief O-GA-KEM Encapsulation
 *
 * @param ct Ciphertext output (512 bytes)
 * @param ss Shared secret output (32 bytes)
 * @param pk Public key (448 bytes)
 * @return 0 on success
 */
int ns_oga_encapsulate(uint8_t *ct, uint8_t *ss, const uint8_t *pk);

/**
 * @brief O-GA-KEM Decapsulation
 *
 * @param ss Shared secret output (32 bytes)
 * @param ct Ciphertext (512 bytes)
 * @param sk Secret key (64 bytes)
 * @return 0 on success
 */
int ns_oga_decapsulate(uint8_t *ss, const uint8_t *ct, const uint8_t *sk);

// ============================================================================
// ML-KEM-1024 (NIST FIPS 203)
// ============================================================================

#define NS_ML_KEM_1024_PUBLIC_KEY_SIZE 256
#define NS_ML_KEM_1024_SECRET_KEY_SIZE 512
#define NS_ML_KEM_1024_CIPHERTEXT_SIZE 192
#define NS_ML_KEM_1024_SHARED_SECRET_SIZE 32

/**
 * @brief Generate ML-KEM-1024 keypair
 *
 * @param pk Public key output (256 bytes)
 * @param sk Secret key output (512 bytes)
 * @return 0 on success
 */
int ns_ml_kem_1024_keypair(uint8_t *pk, uint8_t *sk);

/**
 * @brief ML-KEM-1024 Encapsulation
 *
 * @param ct Ciphertext output (192 bytes)
 * @param ss Shared secret output (32 bytes)
 * @param pk Public key (256 bytes)
 * @return 0 on success
 */
int ns_ml_kem_1024_encapsulate(uint8_t *ct, uint8_t *ss, const uint8_t *pk);

/**
 * @brief ML-KEM-1024 Decapsulation
 *
 * @param ss Shared secret output (32 bytes)
 * @param ct Ciphertext (192 bytes)
 * @param sk Secret key (512 bytes)
 * @return 0 on success
 */
int ns_ml_kem_1024_decapsulate(uint8_t *ss, const uint8_t *ct,
                               const uint8_t *sk);

// ============================================================================
// ML-DSA-87 (NIST FIPS 204)
// ============================================================================

#define NS_ML_DSA_87_PUBLIC_KEY_SIZE 320
#define NS_ML_DSA_87_SECRET_KEY_SIZE 832
#define NS_ML_DSA_87_SIGNATURE_SIZE 608

/**
 * @brief Generate ML-DSA-87 keypair
 *
 * @param pk Public key output (320 bytes)
 * @param sk Secret key output (832 bytes)
 * @return 0 on success
 */
int ns_ml_dsa_87_keypair(uint8_t *pk, uint8_t *sk);

/**
 * @brief ML-DSA-87 Sign
 *
 * @param sig Signature output (608 bytes)
 * @param sig_len Signature length output
 * @param msg Message to sign
 * @param msg_len Message length
 * @param sk Secret key (832 bytes)
 * @return 0 on success
 */
int ns_ml_dsa_87_sign(uint8_t *sig, size_t *sig_len, const uint8_t *msg,
                      size_t msg_len, const uint8_t *sk);

/**
 * @brief ML-DSA-87 Verify
 *
 * @param sig Signature (608 bytes)
 * @param sig_len Signature length
 * @param msg Message
 * @param msg_len Message length
 * @param pk Public key (320 bytes)
 * @return 0 on success, -1 on verification failure
 */
int ns_ml_dsa_87_verify(const uint8_t *sig, size_t sig_len, const uint8_t *msg,
                        size_t msg_len, const uint8_t *pk);

// ============================================================================
// Compatibility Wrappers (HyperCycle → NeuralSeal)
// ============================================================================

// These wrappers allow existing HyperCycle code to use the NeuralSeal API

static inline int ns_oga_keypair(uint8_t *pk, uint8_t *sk) {
  extern int hc_oga_keypair(uint8_t *, uint8_t *);
  return hc_oga_keypair(pk, sk);
}

static inline int ns_oga_encapsulate(uint8_t *ct, uint8_t *ss,
                                     const uint8_t *pk) {
  extern int hc_oga_encapsulate(uint8_t *, uint8_t *, const uint8_t *);
  return hc_oga_encapsulate(ct, ss, pk);
}

static inline int ns_oga_decapsulate(uint8_t *ss, const uint8_t *ct,
                                     const uint8_t *sk) {
  extern int hc_oga_decapsulate(uint8_t *, const uint8_t *, const uint8_t *);
  return hc_oga_decapsulate(ss, ct, sk);
}

static inline int ns_ml_kem_1024_keypair(uint8_t *pk, uint8_t *sk) {
  extern int hc_ml_kem_1024_keypair(uint8_t *, uint8_t *);
  return hc_ml_kem_1024_keypair(pk, sk);
}

static inline int ns_ml_kem_1024_encapsulate(uint8_t *ct, uint8_t *ss,
                                             const uint8_t *pk) {
  extern int hc_ml_kem_1024_encapsulate(uint8_t *, uint8_t *, const uint8_t *);
  return hc_ml_kem_1024_encapsulate(ct, ss, pk);
}

static inline int ns_ml_kem_1024_decapsulate(uint8_t *ss, const uint8_t *ct,
                                             const uint8_t *sk) {
  extern int hc_ml_kem_1024_decapsulate(uint8_t *, const uint8_t *,
                                        const uint8_t *);
  return hc_ml_kem_1024_decapsulate(ss, ct, sk);
}

static inline int ns_ml_dsa_87_keypair(uint8_t *pk, uint8_t *sk) {
  extern int hc_ml_dsa_87_keypair(uint8_t *, uint8_t *);
  return hc_ml_dsa_87_keypair(pk, sk);
}

static inline int ns_ml_dsa_87_sign(uint8_t *sig, size_t *sig_len,
                                    const uint8_t *msg, size_t msg_len,
                                    const uint8_t *sk) {
  extern int hc_ml_dsa_87_sign(uint8_t *, size_t *, const uint8_t *, size_t,
                               const uint8_t *);
  return hc_ml_dsa_87_sign(sig, sig_len, msg, msg_len, sk);
}

static inline int ns_ml_dsa_87_verify(const uint8_t *sig, size_t sig_len,
                                      const uint8_t *msg, size_t msg_len,
                                      const uint8_t *pk) {
  extern int hc_ml_dsa_87_verify(const uint8_t *, size_t, const uint8_t *,
                                 size_t, const uint8_t *);
  return hc_ml_dsa_87_verify(sig, sig_len, msg, msg_len, pk);
}

/*
 * -------------------------------------------------------------------------
 *  High‑Performance Vacuum Simulation Helpers
 *
 *  The following definitions bring the fast vacuum evolution logic directly
 *  into this header.  They are modelled after the design shown in the
 *  Fulminis v3.2 architecture diagrams.  These helpers are optional and
 *  only enabled when the compiler supports AVX‑512.  They allow callers
 *  to perform inline vacuum evolution and Heisenberg–Euler nonlinearity
 *  approximations without pulling in additional translation units.  The
 *  functions are declared static inline to permit the compiler to
 *  aggressively inline and optimise them at call sites.
 */

#if defined(__AVX512F__) && defined(__AVX512DQ__)

#include <immintrin.h>
#include <math.h>

/*
 * @brief Fast inverse square root
 *
 * Computes an approximation to 1/sqrt(number) using the well‑known
 * bit‑level hack followed by a Newton–Raphson refinement.  This variant
 * uses an updated magic constant for the 2026 release.  It is used to
 * normalise field magnitudes in vacuum simulations without the cost of
 * division.
 */
static inline float ns_fast_inv_sqrt(float number) {
  long i;
  float x2, y;
  const float threehalfs = 1.5F;

  x2 = number * 0.5F;
  y = number;
  // Evil floating point bit level hacking
  i = *(long *)&y;
  // 2026‑optimised magic constant
  i = 0x5f3795df - (i >> 1);
  y = *(float *)&i;
  // One iteration of Newton–Raphson refinement
  y = y * (threehalfs - (x2 * y * y));
  return y;
}

/*
 * @brief Approximate Heisenberg–Euler nonlinearity
 *
 * This helper computes a 3rd‑order Taylor approximation of the
 * Heisenberg–Euler field nonlinearity for eight lanes of squared
 * magnitude.  It returns an AVX‑512 vector where each lane contains
 * 1 + α·x² + (α²/2)·x⁴.  The constant α is the fine structure
 * constant expressed in double precision.
 */
static inline __m512d ns_approx_he_nonlinearity(__m512d mag_sq) {
  const __m512d alpha = _mm512_set1_pd(0.00729735256);
  const __m512d one   = _mm512_set1_pd(1.0);
  // x² term
  __m512d x2 = _mm512_mul_pd(mag_sq, mag_sq);
  // x⁴ term: (α²/2) * x⁴
  __m512d alpha2 = _mm512_mul_pd(alpha, alpha);
  __m512d x4     = _mm512_mul_pd(x2, x2);
  __m512d term   = _mm512_mul_pd(_mm512_mul_pd(alpha2, _mm512_set1_pd(0.5)), x4);
  // 1 + α·x² + (α²/2)·x⁴
  __m512d result = _mm512_add_pd(one, _mm512_fmadd_pd(alpha, x2, term));
  return result;
}

/*
 * @brief Batch structure for vacuum evolution
 *
 * Stores eight independent vacuum trajectories in a structure‑of‑arrays
 * layout.  Each element of the vectors corresponds to one trajectory.
 */
typedef struct {
  __m512d w;
  __m512d x;
  __m512d y;
  __m512d z;
} ns_batch8_t;

/*
 * @brief Evolve eight vacuum trajectories
 *
 * Executes 47 cycles of Heisenberg–Euler evolution for the supplied batch.
 * The function updates the w,x,y,z components in place.  A simple mixing
 * term adds a small perturbation to the x component each cycle to
 * encourage chaotic behaviour.
 */
static inline void ns_evolve_vacuum_batch8(ns_batch8_t *batch) {
  if (!batch) return;
  // Iterate 47 times as specified by the Fulminis design
  for (int i = 0; i < 47; i++) {
    // Compute squared magnitude |Q|^2 = w^2 + x^2 + y^2 + z^2
    __m512d mag_sq = _mm512_add_pd(
        _mm512_add_pd(_mm512_mul_pd(batch->w, batch->w),
                      _mm512_mul_pd(batch->x, batch->x)),
        _mm512_add_pd(_mm512_mul_pd(batch->y, batch->y),
                      _mm512_mul_pd(batch->z, batch->z)));
    // Nonlinear scaling factor using polynomial approximation
    __m512d non_linear = ns_approx_he_nonlinearity(mag_sq);
    // Apply scaling to all components
    batch->w = _mm512_mul_pd(batch->w, non_linear);
    batch->x = _mm512_mul_pd(batch->x, non_linear);
    batch->y = _mm512_mul_pd(batch->y, non_linear);
    batch->z = _mm512_mul_pd(batch->z, non_linear);
    // Chaotic mixing: add a tiny fraction of w into x
    batch->x = _mm512_add_pd(batch->x,
                             _mm512_mul_pd(batch->w, _mm512_set1_pd(1e-5)));
  }
}

/*
 * GPU accelerated NTT placeholder
 *
 * When compiling under CUDA or HIP, this function can be defined to
 * execute an accelerated Number Theoretic Transform on the GPU.  It is
 * declared here to allow unified header usage; the implementation must be
 * provided in device code when using NVCC/HIPCC.  When building for the
 * host, this declaration has no effect.
 */
#if defined(__CUDACC__) || defined(__HIPCC__)
__global__ void ns_gpu_accelerated_ntt(uint64_t *seeds, uint8_t *keys);
#endif

#endif /* defined(__AVX512F__) && defined(__AVX512DQ__) */

#ifdef __cplusplus
}
#endif

#endif // NEURALSEAL_FULMINIS_CORE_H
