#include "vortex/public/hc_quaternion.h"

hc_quat_f64_t hc_quat_add(hc_quat_f64_t a, hc_quat_f64_t b) {
  return (hc_quat_f64_t){a.w + b.w, a.x + b.x, a.y + b.y, a.z + b.z};
}

hc_quat_f64_t hc_quat_sub(hc_quat_f64_t a, hc_quat_f64_t b) {
  return (hc_quat_f64_t){a.w - b.w, a.x - b.x, a.y - b.y, a.z - b.z};
}

hc_quat_f64_t hc_quat_mul(hc_quat_f64_t a, hc_quat_f64_t b) {
  hc_quat_f64_t r;
  r.w = a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z;
  r.x = a.w * b.x + a.x * b.w + a.y * b.z - a.z * b.y;
  r.y = a.w * b.y - a.x * b.z + a.y * b.w + a.z * b.x;
  r.z = a.w * b.z + a.x * b.y - a.y * b.x + a.z * b.w;
  return r;
}

hc_quat_f64_t hc_quat_conjugate(hc_quat_f64_t q) {
  return (hc_quat_f64_t){q.w, -q.x, -q.y, -q.z};
}

hc_quat_f64_t hc_quat_compose_rotations(hc_quat_f64_t second_rot,
                                        hc_quat_f64_t first_rot) {
  return hc_quat_mul(second_rot, first_rot);
}

hc_quat_f64_t hc_quat_rotate(hc_quat_f64_t rotator, hc_quat_f64_t vector) {
  hc_quat_f64_t conj = hc_quat_conjugate(rotator);
  hc_quat_f64_t tmp = hc_quat_mul(rotator, vector);
  return hc_quat_mul(tmp, conj);
}
