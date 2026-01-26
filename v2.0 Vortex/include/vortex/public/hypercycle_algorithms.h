// hypercycle_algorithms.h – Public API declarations

#ifndef HYPERCYCLE_ALGORITHMS_H
#define HYPERCYCLE_ALGORITHMS_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Initialize the library (must be called before any other function)
int hc_initialize(void);

// Cleanup the library (call when done)
void hc_cleanup(void);

// ============================================================================
// Error Codes
// ============================================================================
#define hc_SUCCESS 0
#define hc_ERROR_NULL_POINTER -1
#define hc_ERROR_INVALID_PARAMETER -2
#define hc_ERROR_VERIFICATION_FAILED -3
#define hc_ERROR_ENTROPY_FAILURE -4

// ============================================================================
// NIST Level 5 API (Vacuum-based implementations)
// ============================================================================

/* Key and Artifact Sizes (in bytes) */
#define hc_ML_KEM_1024_PUBLIC_KEY_SIZE 256
#define hc_ML_KEM_1024_SECRET_KEY_SIZE 512
#define hc_ML_KEM_1024_CIPHERTEXT_SIZE 192
#define hc_ML_KEM_1024_SHARED_SECRET_SIZE 32

#define hc_ML_DSA_87_PUBLIC_KEY_SIZE 320
#define hc_ML_DSA_87_SECRET_KEY_SIZE 832
#define hc_ML_DSA_87_SIGNATURE_SIZE 608

// Uppercase aliases for compatibility
#define HC_ML_DSA_87_PUBLIC_KEY_SIZE hc_ML_DSA_87_PUBLIC_KEY_SIZE
#define HC_ML_DSA_87_SECRET_KEY_SIZE hc_ML_DSA_87_SECRET_KEY_SIZE
#define HC_ML_DSA_87_SIGNATURE_SIZE hc_ML_DSA_87_SIGNATURE_SIZE

#define HC_ML_KEM_1024_PUBLIC_KEY_SIZE hc_ML_KEM_1024_PUBLIC_KEY_SIZE
#define HC_ML_KEM_1024_SECRET_KEY_SIZE hc_ML_KEM_1024_SECRET_KEY_SIZE
#define HC_ML_KEM_1024_CIPHERTEXT_SIZE hc_ML_KEM_1024_CIPHERTEXT_SIZE
#define HC_ML_KEM_1024_SHARED_SECRET_SIZE hc_ML_KEM_1024_SHARED_SECRET_SIZE

// HyperKEM-1024: 256-bit quantum security KEM
int hc_ml_kem_1024_keypair(uint8_t *pk, uint8_t *sk);
int hc_ml_kem_1024_encapsulate(uint8_t *ct, uint8_t *ss, const uint8_t *pk);
int hc_ml_kem_1024_decapsulate(uint8_t *ss, const uint8_t *ct,
                               const uint8_t *sk);

// HyperDSA-87: 256-bit quantum security signatures
int hc_ml_dsa_87_keypair(uint8_t *pk, uint8_t *sk);
int hc_ml_dsa_87_sign(uint8_t *sig, size_t *sig_len, const uint8_t *msg,
                      size_t msg_len, const uint8_t *sk);
int hc_ml_dsa_87_verify(const uint8_t *sig, size_t sig_len, const uint8_t *msg,
                        size_t msg_len, const uint8_t *pk);

// Version information
#define HYPERCYCLE_VERSION "1.0.0-Genesis"

#ifdef __cplusplus
}
#endif

#endif // HYPERCYCLE_ALGORITHMS_H
