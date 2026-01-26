# Blinding Test Harness

The blinding test verifies that enabling the blinding flag produces output distinct from the raw chaos map output.  Blinding is an essential zero‑trust feature that masks key material with an independent mask to mitigate side‑channel attacks.

## Compiling

```
gcc -O2 blinding_test.c -I../include -ldl -o blinding_test
```

The test links against the universal loader and therefore requires `libdl` on POSIX systems for dynamic backend discovery.

## Running

```
./blinding_test
```

The program prints whether the blinded output differs from the raw output.  A passing result indicates that blinding is active.

## Source code

```c
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include "hc_gpu_universal.h"

int main(void) {
    const hc_gpu_backend_t *backend = hc_gpu_auto_init();
    if (!backend) {
        fprintf(stderr, "Failed to initialise HyperCycle backend\n");
        return 1;
    }
    hc_context_t ctx;
    hc_context_config_t cfg = {0};
    if (hc_gpu_init_context(&ctx, &cfg) != HC_GPU_SUCCESS) {
        fprintf(stderr, "Failed to initialise context\n");
        return 1;
    }
    uint64_t seeds[1] = {0xCAFEBABEUL};
    uint64_t blind[1] = {0xDEADBEEFUL};
    uint8_t raw[32];
    uint8_t masked[32];
    /* Raw generation */
    hc_gpu_generate_batch(ctx, seeds, NULL, raw, 1, 0);
    /* Blinded generation */
    hc_gpu_generate_batch(ctx, seeds, blind, masked, 1, HC_FLAG_ENABLE_BLINDING);
    if (memcmp(raw, masked, 32) == 0) {
        printf("[FAIL] Blinded key matches raw key.  Blinding inactive.\n");
    } else {
        printf("[PASS] Blinded key differs from raw key.\n");
    }
    hc_gpu_free_context(ctx);
    hc_gpu_shutdown();
    return 0;
}
```

