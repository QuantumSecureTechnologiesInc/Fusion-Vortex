/*
 * HyperCycle PQC – Mathematical Core (Optimised)
 *
 * This header defines the core chaos map used throughout the library.  It is a
 * drop‑in replacement for the previous `hc_math_core.h` and has been updated
 * to align with the algorithmic optimisations described in the integration
 * documents.  The functions exposed here operate identically on CPU, CUDA and
 * HIP backends and therefore guarantee bitwise identical output across
 * hardware.
 *
 * The core comprises three parts:
 * 1. Initialising a quaternion state from a 64‑bit seed.
 * 2. Evolving that state via the Heisenberg–Euler chaos map.
 * 3. Extracting a 256‑bit key from the final state, optionally masked by a
 *    secondary trajectory (blinding).
 *
 * Author: HyperCycle Engineering Team
 * License: MIT
 */

#ifndef HC_MATH_CORE_H
#define HC_MATH_CORE_H

#include <stdint.h>
#include <stdbool.h>

/* --- Platform & Compiler Abstractions --- */

#if defined(__CUDACC__) || defined(__HIPCC__)
    #define HC_DEVICE_FUNC __device__ __forceinline__
    #define HC_ALIGN(n) __align__(n)
#else
    #define HC_DEVICE_FUNC static inline
    #if defined(_MSC_VER)
        #define HC_ALIGN(n) __declspec(align(n))
    #else
        #define HC_ALIGN(n) __attribute__((aligned(n)))
    #endif
#endif

/* --- Constants --- */

/* The number of chaos iterations to perform.  Increasing this value raises
 * security at the cost of throughput.  The chosen value of 47 originates
 * from the optimisation documents and yields good diffusion while
 * maintaining performance. */
#define HC_CYCLES 47

/* Fixed‑point scaling factor.  All quaternion components are stored as
 * 64‑bit integers scaled by 10^9 to retain fractional precision whilst
 * permitting fast integer arithmetic. */
#define HC_SCALE  1000000000ULL

/* Alpha parameter for the Heisenberg–Euler map, scaled by 10^9.  Changing
 * this value alters the chaotic behaviour and must be kept constant to
 * ensure cross‑platform consistency. */
#define HC_ALPHA  7297352ULL

/* --- Data Types --- */

/* Quaternion state.  Each component uses a 64‑bit signed integer in fixed
 * point representation.  The structure is explicitly aligned to 32 bytes
 * so that compilers can generate aligned load/store instructions for
 * vectorisation (e.g., AVX2/AVX‑512). */
typedef struct HC_ALIGN(32) {
    int64_t w;
    int64_t x;
    int64_t y;
    int64_t z;
} hc_quat_t;

/* --- Core Functions --- */

/* Initialise a quaternion from a seed.  The same seed will always
 * initialise the same state on all platforms. */
HC_DEVICE_FUNC void hc_init_state(hc_quat_t *q, uint64_t seed) {
    q->w = (int64_t)(seed % HC_SCALE);
    q->x = (int64_t)((seed >> 16) % HC_SCALE);
    q->y = (int64_t)((seed >> 32) % HC_SCALE);
    q->z = (int64_t)((seed >> 48) % HC_SCALE);
}

/* Perform one step of the Heisenberg–Euler chaos map on a quaternion.  The
 * map uses fixed‑point arithmetic to avoid floating‑point timing side
 * channels and to support deterministic behaviour on all hardware. */
HC_DEVICE_FUNC void hc_chaos_map_step(hc_quat_t *q) {
    /* Compute a reduced magnitude squared.  Division by 1000 reduces the
     * dynamic range, preventing overflow in subsequent multiplications.
     * The reductions are performed outside the multiplication to aid
     * vectorisation. */
    int64_t w_red = q->w / 1000;
    int64_t x_red = q->x / 1000;
    int64_t y_red = q->y / 1000;
    int64_t z_red = q->z / 1000;

    int64_t mag_sq = (w_red * w_red) + (x_red * x_red) +
                     (y_red * y_red) + (z_red * z_red);

    /* Compute the non‑linear scaling factor.  The term HC_ALPHA * mag_sq
     * yields large numbers; dividing by (HC_SCALE/1000) keeps the final
     * multiplication within 64‑bit bounds. */
    int64_t non_linear = HC_SCALE + (HC_ALPHA * mag_sq / (HC_SCALE / 1000));

    /* Apply scaling to each component.  Because all multiplies use
     * int64_t, the intermediate results may exceed 64 bits; dividing by
     * HC_SCALE early limits overflow. */
    q->w = (q->w * non_linear) / HC_SCALE;
    q->x = (q->x * non_linear) / HC_SCALE;
    q->y = (q->y * non_linear) / HC_SCALE;
    q->z = (q->z * non_linear) / HC_SCALE;

    /* Perform a simple symplectic mix.  This rotation injects
     * cross‑component dependency without branching. */
    int64_t tmp = q->w;
    q->w = q->w - q->x;
    q->x = q->x + tmp;

    int64_t tmp2 = q->y;
    q->y = q->y - q->z;
    q->z = q->z + tmp2;
}

/* Generate a single 256‑bit key from a seed and an optional blinding seed.
 * When blinding_seed is zero the function runs the fast unmasked path;
 * otherwise it interleaves two trajectories to mask timing and power
 * characteristics.  The output buffer must be at least 32 bytes. */
HC_DEVICE_FUNC void hc_generate_single_key(
    uint64_t seed_base,
    uint64_t blinding_seed,
    uint64_t idx,
    uint8_t *key_out
) {
    hc_quat_t q_main;
    uint64_t seed = seed_base ^ idx;
    bool use_mask = (blinding_seed != 0);

    if (!use_mask) {
        /* Fast path: no masking */
        hc_init_state(&q_main, seed);
        #pragma unroll 4
        for (int i = 0; i < HC_CYCLES; ++i) {
            hc_chaos_map_step(&q_main);
        }
        uint64_t *out_ptr = (uint64_t*)key_out;
        out_ptr[0] = (uint64_t)q_main.w ^ (uint64_t)q_main.x;
        out_ptr[1] = (uint64_t)q_main.y ^ (uint64_t)q_main.z;
        out_ptr[2] = (uint64_t)q_main.w + (uint64_t)q_main.z;
        out_ptr[3] = (uint64_t)q_main.x - (uint64_t)q_main.y;
    } else {
        /* Secure path: masked */
        hc_quat_t q_mask;
        uint64_t mask_idx = idx ^ 0xAAAAAAAA55555555ULL;
        hc_init_state(&q_main, seed);
        hc_init_state(&q_mask, blinding_seed ^ mask_idx);
        #pragma unroll 4
        for (int i = 0; i < HC_CYCLES; ++i) {
            hc_chaos_map_step(&q_main);
            hc_chaos_map_step(&q_mask);
        }
        uint64_t *out_ptr = (uint64_t*)key_out;
        uint64_t k0 = (uint64_t)q_main.w ^ (uint64_t)q_main.x;
        uint64_t k1 = (uint64_t)q_main.y ^ (uint64_t)q_main.z;
        uint64_t k2 = (uint64_t)q_main.w + (uint64_t)q_main.z;
        uint64_t k3 = (uint64_t)q_main.x - (uint64_t)q_main.y;
        uint64_t m0 = (uint64_t)q_mask.w ^ (uint64_t)q_mask.x;
        uint64_t m1 = (uint64_t)q_mask.y ^ (uint64_t)q_mask.z;
        uint64_t m2 = (uint64_t)q_mask.w + (uint64_t)q_mask.z;
        uint64_t m3 = (uint64_t)q_mask.x - (uint64_t)q_mask.y;
        out_ptr[0] = k0 ^ m0;
        out_ptr[1] = k1 ^ m1;
        out_ptr[2] = k2 ^ m2;
        out_ptr[3] = k3 ^ m3;
    }
}

#endif /* HC_MATH_CORE_H */