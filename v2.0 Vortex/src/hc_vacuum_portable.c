/*
 * HyperCycle Vacuum Engine - Portable Stub for MSVC
 *
 * This is a minimal portable implementation of the vacuum entropy engine
 * for compilers that don't support AVX-512 intrinsics (e.g., MSVC).
 * Uses standard C library random for entropy generation.
 *
 * NOTE: This is a FALLBACK implementation. For production use with
 * proper post-quantum entropy, compile with GCC/Clang on Linux/macOS
 * to use the full AVX-512 implementation in hc_vacuum_engine.c.
 */

#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdint.h>

/* Forward declarations - match the real API */
typedef int hc_result_t;
#define HC_SUCCESS 0
#define HC_ERR_INVALID_ARGS -2
#define HC_ERR_KERNEL_FAILURE -3
#define HC_ERR_OUT_OF_MEMORY -1

typedef struct hc_vac_context_s *hc_vac_context_t;
#define HC_PQC_SEED_SIZE 32U

typedef struct {
    uint64_t total_batches;
    uint64_t total_keys_generated;
    double   last_batch_time_sec;
    uint64_t last_batch_count;
} hc_telemetry_t;

/* External SHA-3 from the project */
extern void hc_sha3_256(const uint8_t *in, size_t inlen, uint8_t *out);

/* Portable secure wipe - exported for vortex_pqc_api.c */
void secure_wipe(void *ptr, size_t len) {
    volatile unsigned char *p = (volatile unsigned char *)ptr;
    while (len--) {
        *p++ = 0;
    }
}

/* Portable entropy conditioning - exported for other modules */
void condition_entropy(const uint8_t *raw, size_t len, uint8_t *out32) {
    hc_sha3_256(raw, len, out32);
}

/* Simplified context for portable build */
struct hc_vac_context_s {
    uint64_t state_q[8];
    uint64_t state_p[8];
    uint64_t entropy_failures;
    uint64_t total_bytes_generated;
    uint64_t total_requests;
    double last_request_time_sec;
    int initialized;
};

/* Initialize vacuum context */
hc_result_t hc_vacuum_init_context(hc_vac_context_t *ctx, const void *config) {
    if (!ctx) {
        return HC_ERR_INVALID_ARGS;
    }

    hc_vac_context_t handle = (hc_vac_context_t)calloc(1, sizeof(struct hc_vac_context_s));
    if (!handle) {
        return HC_ERR_OUT_OF_MEMORY;
    }
    
    /* Initialize with random seed from C library */
    srand((unsigned int)time(NULL));
    for (int i = 0; i < 8; i++) {
        handle->state_q[i] = (uint64_t)rand();
        handle->state_p[i] = (uint64_t)rand();
    }

    handle->initialized = 1;
    *ctx = handle;
    return HC_SUCCESS;
}

/* Generate seed from vacuum entropy */
hc_result_t hc_vacuum_generate_seed(hc_vac_context_t ctx,
                                     uint8_t out_seed[HC_PQC_SEED_SIZE]) {
    if (!ctx || !out_seed) {
        return HC_ERR_INVALID_ARGS;
    }

    if (!ctx->initialized) {
        return HC_ERR_KERNEL_FAILURE;
    }

    /* Generate random bytes */
    uint8_t raw[64];
    for (size_t i = 0; i < sizeof(raw); i++) {
        raw[i] = (uint8_t)(rand() & 0xFF);
    }

    /* Condition the entropy */
    condition_entropy(raw, sizeof(raw), out_seed);
    secure_wipe(raw, sizeof(raw));

    ctx->total_bytes_generated += HC_PQC_SEED_SIZE;
    ctx->total_requests++;

    return HC_SUCCESS;
}

/* Generate seed with additional safety checks */
hc_result_t hc_vacuum_generate_seed_safe(hc_vac_context_t ctx,
                                          uint8_t out_seed[HC_PQC_SEED_SIZE]) {
    /* For the portable stub, just call the regular generate */
    return hc_vacuum_generate_seed(ctx, out_seed);
}

/* Get telemetry */
hc_result_t hc_vacuum_get_telemetry(hc_vac_context_t ctx, hc_telemetry_t *out) {
    if (!ctx || !out) {
        return HC_ERR_INVALID_ARGS;
    }
    
    memset(out, 0, sizeof(hc_telemetry_t));
    out->total_batches = ctx->total_requests;
    out->total_keys_generated = ctx->total_bytes_generated;
    
    return HC_SUCCESS;
}

/* Free vacuum context */
void hc_vacuum_free_context(hc_vac_context_t ctx) {
    if (!ctx) {
        return;
    }

    secure_wipe(ctx->state_q, sizeof(ctx->state_q));
    secure_wipe(ctx->state_p, sizeof(ctx->state_p));
    
    free(ctx);
}
