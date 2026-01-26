#ifndef hc_ZERO_TRUST_H
#define hc_ZERO_TRUST_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Verify a Zero-Trust identity token (JWT format)
 *
 * Implements cryptographic verification of JWT tokens with:
 * - HMAC-SHA256 signature validation
 * - Timestamp validation (exp, nbf, iat)
 * - Issuer verification
 * - Constant-time comparison
 *
 * @param identity_token JWT token string (format: header.payload.signature)
 * @return 0 if valid, -1 if invalid or expired
 */
int hc_zta_verify_identity(const char *identity_token);

/**
 * @brief Generate a Zero-Trust identity token for testing/development
 *
 * @param subject Subject identifier (user/service ID)
 * @param validity_seconds Token validity duration in seconds
 * @param output_token Buffer to store generated token
 * @param output_len Length of output buffer
 * @return 0 on success, -1 on failure
 */
int hc_zta_generate_token(const char *subject, int validity_seconds,
                          char *output_token, size_t output_len);

#ifdef __cplusplus
}
#endif

#endif // hc_ZERO_TRUST_H
