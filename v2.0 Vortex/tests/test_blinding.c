#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include "hc_gpu_universal.h"

int main(void) {
    printf("--- HyperCycle Blinding Test ---\n");
    const hc_gpu_backend_t *backend = hc_gpu_auto_init();
    if (!backend) {
        printf("Failed to load backend\n");
        return 1;
    }
    printf("Backend: %s\n", backend->name);
    hc_context_config_t cfg = {0};
    cfg.device_id = -1;
    cfg.stream_priority = 0;
    cfg.memory_pool_size = 0;
    cfg.enable_profiling = false;
    cfg.numa_node = 0;
    hc_context_t ctx;
    if (backend->init_context(&ctx, &cfg) != HC_GPU_SUCCESS) {
        printf("Failed to init context\n");
        return 1;
    }
    size_t count = 1;
    uint64_t seeds[1] = {0xCAFEBABEULL};
    uint64_t blind[1] = {0xDEADBEEFULL};
    uint8_t out_raw[32];
    uint8_t out_blind[32];
    // Generate without blinding
    if (backend->generate_batch(ctx, seeds, NULL, out_raw, count, 0) != HC_GPU_SUCCESS) {
        printf("Failed to generate raw entropy\n");
        backend->free_context(ctx);
        return 1;
    }
    // Generate with blinding
    if (backend->generate_batch(ctx, seeds, blind, out_blind, count, HC_FLAG_ENABLE_BLINDING) != HC_GPU_SUCCESS) {
        printf("Failed to generate blinded entropy\n");
        backend->free_context(ctx);
        return 1;
    }
    // Compare outputs
    if (memcmp(out_raw, out_blind, 32) == 0) {
        printf("[FAIL] Blinded output matches raw output\n");
    } else {
        printf("[PASS] Blinded output differs from raw output\n");
    }
    backend->free_context(ctx);
    hc_gpu_shutdown();
    return 0;
}
