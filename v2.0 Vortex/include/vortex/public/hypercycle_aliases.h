/*=====================================================================
  hypercycle_aliases.h – User‑friendly API aliases for QST HyperCycle
  ------------------------------------------------------------
  This header provides brand‑aware, PascalCase aliases for the public
  HyperCycle functions. The underlying implementations remain unchanged;
  the macros simply forward calls to the original `hc_*` symbols.
  All symbols are prefixed with `NSeal` (Legacy) or `Hyper` to make the
library’s branding explicit while preserving ABI compatibility.
=====================================================================*/

#ifndef NSEAL_ALIASES_H
#define NSEAL_ALIASES_H

#include "hc_accelerator.h"
#include "hc_secure_enclave.h"
#include <stddef.h>
#include <stdint.h>

/* ------------------------------------------------------------------
   Side‑channel masking utilities
   ------------------------------------------------------------------ */
#define NSealMaskSecret hc_mask_secret
#define NSealUnmaskSecret hc_unmask_secret

/* ------------------------------------------------------------------
   Cryptographic Bill of Materials (CBOM)
   ------------------------------------------------------------------ */
#define NSealGenerateCBOM hc_generate_cbom

/* ------------------------------------------------------------------
   Quantum risk scoring
   ------------------------------------------------------------------ */
#define NSealComputeRiskScore hc_compute_risk_score

/* ------------------------------------------------------------------
   Automated key rotation
   ------------------------------------------------------------------ */
#define NSealKeyRotate hc_rotate_keys

/* ------------------------------------------------------------------
   Dynamic policy engine
   ------------------------------------------------------------------ */
#define NSealLoadPolicyEngine hc_load_policy_engine
#define NSealQueryPolicy hc_query_policy

/* ------------------------------------------------------------------
   Zero‑Trust integration
   ------------------------------------------------------------------ */
#define NSealZeroTrustVerify hc_zta_verify_identity

/* ------------------------------------------------------------------
   Accelerator hooks – thin inline wrappers to avoid macro‑only expansion.
   ------------------------------------------------------------------ */
static inline int NSealAcceleratorProcess(const char *algorithm_name,
                                          const uint8_t *input,
                                          size_t input_len, uint8_t *output,
                                          size_t output_len) {
  return hc_accelerator_process(algorithm_name, input, input_len, output,
                                output_len);
}

/* ------------------------------------------------------------------
   Secure enclave integration – thin inline wrapper.
   ------------------------------------------------------------------ */
static inline int NSealEnclaveStoreKey(const uint8_t *key, size_t len) {
  return hc_enclave_store_key(key, len);
}

#endif /* NSEAL_ALIASES_H */
