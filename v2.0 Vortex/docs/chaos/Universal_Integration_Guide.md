# Universal Integration Guide: Chaotic PQC
<!-- doc-type: how-to -->
<!-- audience: developer -->
<!-- product: HyperCycle -->

## 1. Overview
The HyperCycle Chaotic Engine is designed as a standalone, zero-dependency entropy source. It can be used as a drop-in replacement for `rand()` or SHAKE-based expanders in any PQC library.

## 2. Integration with Generic PQC Libraries
Most PQC libraries allow custom RNG callbacks. To use the chaotic engine:

```c
#include "hc_chaotic_engine.h"

// 1. Initialise context
hc_chaos_ctx_t chaos_ctx;
hc_chaos_init(&chaos_ctx, 0xSEED);

// 2. Register with PQC library (example signature)
pqc_set_random_callback(hc_chaos_univ_random, &chaos_ctx);
```

## 3. Integration with HyperCycle PQC Suite
The chaotic engine integrates natively with HyperCycle's internal PQC implementations.

```c
#include "hypercycle_pqc.h"
#include "hc_chaotic_engine.h"

hc_chaos_ctx_t ctx;
hc_chaos_init(&ctx, seed);

hc_pqc_config_t config;
hc_pqc_set_entropy_source(&config, hc_chaos_univ_random, &ctx);
```

## 4. Portability Notes
- **No Float**: Uses only integer arithmetic (Q32 fixed-point).
- **Deterministic**: Given the same seed, output is identical on ARM, x86, and RISC-V.
- **Header-Only Potential**: For ultra-constrained builds, `hc_chaotic_engine.c` logic can be inlined safely.


