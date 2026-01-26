#ifndef HC_FINAL_H
#define HC_FINAL_H

#include "hc_hypercycle.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Securely wipe a buffer (anti-forensic). */
void secure_wipe(void *ptr, size_t len);

/* Hash and whiten raw entropy into a 32‑byte seed using SHA3‑256. */
void condition_entropy(const uint8_t *raw_input, size_t len, uint8_t *out_32);

/* Initialise the FIPS‑compliant vacuum context.  Runs startup tests. */
hc_result_t hc_final_init_context(hc_context_t *ctx, const hc_context_config_t *config);

/* Generate a 32‑byte post‑quantum seed from the vacuum engine. */
hc_result_t hc_final_generate_pqc_seed(hc_context_t ctx, uint8_t *out_seed);

/* Free the FIPS context and securely wipe its state. */
void hc_final_free_context(hc_context_t ctx);

#ifdef __cplusplus
}
#endif

#endif /* HC_FINAL_H */
