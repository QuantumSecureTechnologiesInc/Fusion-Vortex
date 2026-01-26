#ifndef hc_OGA_KEM_H
#define hc_OGA_KEM_H

#include <stddef.h>
#include <stdint.h>


#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Constants & Sizes
// ============================================================================

// Private Key: 1 Octonion Rotor (8 doubles = 64 bytes)
#define hc_OGA_SECRET_KEY_SIZE 64

// Public Key: Twisted Basis of R7 (7 Octonions = 7 * 64 = 448 bytes)
#define hc_OGA_PUBLIC_KEY_SIZE 448

// Ciphertext: Encapsulated Message + Ephemeral Public Key
// Ephemeral PK (448 bytes) + Masked Message (32 bytes) + Tag (32 bytes) = 512
// bytes
#define hc_OGA_CIPHERTEXT_SIZE 512

// Shared Secret Size (standard 256-bit key)
#define hc_OGA_SHARED_SECRET_SIZE 32

// ============================================================================
// API
// ============================================================================

int hc_oga_keypair(uint8_t *pk, uint8_t *sk);

int hc_oga_encapsulate(uint8_t *ct, uint8_t *ss, const uint8_t *pk);

int hc_oga_decapsulate(uint8_t *ss, const uint8_t *ct, const uint8_t *sk);

#ifdef __cplusplus
}
#endif

#endif // hc_OGA_KEM_H
