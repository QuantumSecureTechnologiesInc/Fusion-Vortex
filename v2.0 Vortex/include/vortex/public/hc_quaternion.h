#ifndef HC_QUATERNION_H
#define HC_QUATERNION_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    double w, x, y, z;
} hc_quat_f64_t;

// Basic ops
hc_quat_f64_t hc_quat_add(hc_quat_f64_t a, hc_quat_f64_t b);
hc_quat_f64_t hc_quat_sub(hc_quat_f64_t a, hc_quat_f64_t b);
hc_quat_f64_t hc_quat_mul(hc_quat_f64_t a, hc_quat_f64_t b);
hc_quat_f64_t hc_quat_conjugate(hc_quat_f64_t q);

// Rotation: v' = q * v * q*
hc_quat_f64_t hc_quat_rotate(hc_quat_f64_t rotator, hc_quat_f64_t vector);

// Combine sequential rotations (apply first_rot then second_rot): q_net = second * first
hc_quat_f64_t hc_quat_compose_rotations(hc_quat_f64_t second_rot, hc_quat_f64_t first_rot);

#ifdef __cplusplus
}
#endif

#endif /* HC_QUATERNION_H */
