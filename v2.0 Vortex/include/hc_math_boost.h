/*
 * HyperCycle PQC – Math Boost Layer
 *
 * This header implements a suite of quaternion arithmetic primitives and
 * optimised hash functions.  It builds on the fixed‑point chaos map
 * defined in `hc_math_core.h` and provides operations necessary for
 * constructing high‑level cryptographic primitives such as signatures and
 * key encapsulation mechanisms.  The functions are annotated with
 * `HC_DEVICE_FUNC` so they may be used both on the host and inside GPU
 * kernels.
 *
 * Author: HyperCycle Engineering Team
 * License: MIT
 */

#ifndef HC_MATH_BOOST_H
#define HC_MATH_BOOST_H

#include "hc_math_core.h"
#include <string.h>

/* --- Quaternion Arithmetic --- */

/* Add two quaternions.  This operation is naturally vectorisable and does
 * not branch. */
HC_DEVICE_FUNC void hc_quat_add(const hc_quat_t *a, const hc_quat_t *b, hc_quat_t *r) {
    r->w = a->w + b->w;
    r->x = a->x + b->x;
    r->y = a->y + b->y;
    r->z = a->z + b->z;
}

/* Subtract quaternions. */
HC_DEVICE_FUNC void hc_quat_sub(const hc_quat_t *a, const hc_quat_t *b, hc_quat_t *r) {
    r->w = a->w - b->w;
    r->x = a->x - b->x;
    r->y = a->y - b->y;
    r->z = a->z - b->z;
}

/* Hamilton product of quaternions.  This is the core of combined
 * rotations.  It uses 64‑bit intermediate values and divides once at
 * the end to preserve precision. */
HC_DEVICE_FUNC void hc_quat_mul(const hc_quat_t *a, const hc_quat_t *b, hc_quat_t *r) {
    int64_t w = a->w * b->w - a->x * b->x - a->y * b->y - a->z * b->z;
    int64_t x = a->w * b->x + a->x * b->w + a->y * b->z - a->z * b->y;
    int64_t y = a->w * b->y - a->x * b->z + a->y * b->w + a->z * b->x;
    int64_t z = a->w * b->z + a->x * b->y - a->y * b->x + a->z * b->w;
    r->w = w / HC_SCALE;
    r->x = x / HC_SCALE;
    r->y = y / HC_SCALE;
    r->z = z / HC_SCALE;
}

/* Conjugate of a quaternion: q* = (w, –x, –y, –z). */
HC_DEVICE_FUNC void hc_quat_conjugate(const hc_quat_t *a, hc_quat_t *r) {
    r->w =  a->w;
    r->x = -a->x;
    r->y = -a->y;
    r->z = -a->z;
}

/* Compose two rotations: result = second_rot * first_rot.  This function
 * performs a Hamilton product and may be inlined by the compiler. */
HC_DEVICE_FUNC void hc_quat_compose_rotations(const hc_quat_t *second_rot,
                                               const hc_quat_t *first_rot,
                                               hc_quat_t *combined_out) {
    hc_quat_mul(second_rot, first_rot, combined_out);
}

/* Rotate a vector quaternion by a rotator quaternion: v' = q v q*.  The
 * function assumes the rotator has unit norm; callers should normalise
 * beforehand if required. */
HC_DEVICE_FUNC void hc_quat_rotate(const hc_quat_t *rotator,
                                   const hc_quat_t *vector,
                                   hc_quat_t *result) {
    hc_quat_t conj;
    hc_quat_conjugate(rotator, &conj);
    hc_quat_t temp;
    hc_quat_mul(rotator, vector, &temp);
    hc_quat_mul(&temp, &conj, result);
}

/* --- Sponge Construction --- */

/* State structure for the Heisenberg–Euler sponge.  The state consists of
 * two quaternions: one holds the rate (output) portion and the other
 * holds the capacity (internal diffusion) portion. */
typedef struct {
    hc_quat_t q_rate;
    hc_quat_t q_capacity;
} hc_sponge_t;

/* Initialise the sponge state.  The capacity is seeded with HC_ALPHA to
 * avoid zero‑state attractors. */
HC_DEVICE_FUNC void hc_sponge_init(hc_sponge_t *s) {
    s->q_rate.w = 0;
    s->q_rate.x = 0;
    s->q_rate.y = 0;
    s->q_rate.z = 0;
    s->q_capacity.w = HC_ALPHA;
    s->q_capacity.x = 0;
    s->q_capacity.y = 0;
    s->q_capacity.z = 0;
}

/* Permute the sponge state.  This function applies the chaos map to both
 * the rate and capacity, with periodic mixing of the two to accelerate
 * diffusion. */
HC_DEVICE_FUNC void hc_sponge_permute(hc_sponge_t *s) {
    /* Cross‑couple bits from capacity into rate and vice versa.  These
     * XORs ensure that every bit of the state eventually influences every
     * other bit. */
    s->q_rate.w ^= s->q_capacity.z;
    s->q_capacity.x ^= s->q_rate.y;
    /* Apply the chaos map repeatedly.  Interleave occasional addition
     * operations to break symmetries. */
    for (int i = 0; i < HC_CYCLES; i++) {
        hc_chaos_map_step(&s->q_rate);
        hc_chaos_map_step(&s->q_capacity);
        if ((i % 7) == 0) {
            hc_quat_add(&s->q_rate, &s->q_capacity, &s->q_rate);
        }
    }
}

/* Absorb arbitrary data into the sponge.  Data is XORed into the rate
 * portion in 32‑byte blocks; after each block the state is permuted. */
HC_DEVICE_FUNC void hc_sponge_absorb(hc_sponge_t *s, const uint8_t *data, size_t len) {
    size_t offset = 0;
    while (offset < len) {
        uint8_t *rate_bytes = (uint8_t*)&s->q_rate;
        size_t block_size = sizeof(hc_quat_t);
        for (size_t i = 0; i < block_size && (offset + i) < len; i++) {
            rate_bytes[i] ^= data[offset + i];
        }
        hc_sponge_permute(s);
        offset += block_size;
    }
}

/* Squeeze output from the sponge.  After each permutation a block of
 * bytes from the rate is written to `out`. */
HC_DEVICE_FUNC void hc_sponge_squeeze(hc_sponge_t *s, uint8_t *out, size_t len) {
    size_t offset = 0;
    while (offset < len) {
        hc_sponge_permute(s);
        uint8_t *rate_bytes = (uint8_t*)&s->q_rate;
        size_t block_size = sizeof(hc_quat_t);
        for (size_t i = 0; i < block_size && (offset + i) < len; i++) {
            out[offset + i] = rate_bytes[i];
        }
        offset += block_size;
    }
}

#endif /* HC_MATH_BOOST_H */