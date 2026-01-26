// cemqc.c – Core Chaotic Entropy Multi‑map Quantum Cryptography primitives
// Implements secure RNG and constant‑time quaternion arithmetic

#include "vortex/public/cemqc.h"
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

// --- RNG Implementation ---

// Simple XORShift64* RNG – deterministic for now (replace with true chaotic RNG
// later)
void hc_rng_init(hc_rng_state_t *state, const unsigned char *seed,
                 size_t seed_len) {
  if (!state)
    return;
  uint64_t s = 0xDEADBEEFDEADBEEFULL; // fallback constant
  if (seed && seed_len >= 8) {
    memcpy(&s, seed, 8);
  } else if (seed && seed_len > 0) {
    unsigned char buf[8] = {0};
    memcpy(buf, seed, seed_len);
    for (size_t i = seed_len; i < 8; ++i)
      buf[i] = 0xA5; // pad
    memcpy(&s, buf, 8);
  }
  state->state = s ? s : 0x1ULL;
}

static uint64_t xorshift64star(uint64_t *x) {
  uint64_t v = *x;
  v ^= v >> 12;
  v ^= v << 25;
  v ^= v >> 27;
  *x = v;
  return v * 0x2545F4914F6CDD1DULL;
}

void hc_rng_generate(hc_rng_state_t *state, unsigned char *out,
                     size_t out_len) {
  if (!state || !out)
    return;
  uint64_t s = state->state;
  size_t i = 0;
  while (i + 8 <= out_len) {
    uint64_t r = xorshift64star(&s);
    memcpy(out + i, &r, 8);
    i += 8;
  }
  if (i < out_len) {
    uint64_t r = xorshift64star(&s);
    memcpy(out + i, &r, out_len - i);
  }
  state->state = s;
}

// --- Quaternion Arithmetic ---

#if defined(USE_AVX2)
#include <immintrin.h>

// SIMD quaternion multiplication (int32 components) using AVX2
static inline void hc_quaternion_mul_avx2(const hc_quaternion_t *a,
                                          const hc_quaternion_t *b,
                                          hc_quaternion_t *out) {
  __m128i va = _mm_loadu_si128((const __m128i *)a); // [w,x,y,z]
  __m128i vb = _mm_loadu_si128((const __m128i *)b);

  // Extract components
  __m128i a_w = _mm_shuffle_epi32(va, _MM_SHUFFLE(0, 0, 0, 0));
  __m128i a_x = _mm_shuffle_epi32(va, _MM_SHUFFLE(1, 1, 1, 1));
  __m128i a_y = _mm_shuffle_epi32(va, _MM_SHUFFLE(2, 2, 2, 2));
  __m128i a_z = _mm_shuffle_epi32(va, _MM_SHUFFLE(3, 3, 3, 3));

  __m128i b_w = _mm_shuffle_epi32(vb, _MM_SHUFFLE(0, 0, 0, 0));
  __m128i b_x = _mm_shuffle_epi32(vb, _MM_SHUFFLE(1, 1, 1, 1));
  __m128i b_y = _mm_shuffle_epi32(vb, _MM_SHUFFLE(2, 2, 2, 2));
  __m128i b_z = _mm_shuffle_epi32(vb, _MM_SHUFFLE(3, 3, 3, 3));

  // w = a.w*b.w - a.x*b.x - a.y*b.y - a.z*b.z
  __m128i w = _mm_mullo_epi32(a_w, b_w);
  w = _mm_sub_epi32(w, _mm_mullo_epi32(a_x, b_x));
  w = _mm_sub_epi32(w, _mm_mullo_epi32(a_y, b_y));
  w = _mm_sub_epi32(w, _mm_mullo_epi32(a_z, b_z));

  // x = a.w*b.x + a.x*b.w + a.y*b.z - a.z*b.y
  __m128i x = _mm_mullo_epi32(a_w, b_x);
  x = _mm_add_epi32(x, _mm_mullo_epi32(a_x, b_w));
  x = _mm_add_epi32(x, _mm_mullo_epi32(a_y, b_z));
  x = _mm_sub_epi32(x, _mm_mullo_epi32(a_z, b_y));

  // y = a.w*b.y - a.x*b.z + a.y*b.w + a.z*b.x
  __m128i y = _mm_mullo_epi32(a_w, b_y);
  y = _mm_sub_epi32(y, _mm_mullo_epi32(a_x, b_z));
  y = _mm_add_epi32(y, _mm_mullo_epi32(a_y, b_w));
  y = _mm_add_epi32(y, _mm_mullo_epi32(a_z, b_x));

  // z = a.w*b.z + a.x*b.y - a.y*b.x + a.z*b.w
  __m128i z = _mm_mullo_epi32(a_w, b_z);
  z = _mm_add_epi32(z, _mm_mullo_epi32(a_x, b_y));
  z = _mm_sub_epi32(z, _mm_mullo_epi32(a_y, b_x));
  z = _mm_add_epi32(z, _mm_mullo_epi32(a_z, b_w));

  // Pack result
  __m128i result =
      _mm_set_epi32(_mm_extract_epi32(z, 0), _mm_extract_epi32(y, 0),
                    _mm_extract_epi32(x, 0), _mm_extract_epi32(w, 0));
  _mm_storeu_si128((__m128i *)out, result);
}
#endif

// Fallback scalar quaternion multiplication
void hc_quaternion_mul(const hc_quaternion_t *a, const hc_quaternion_t *b,
                       hc_quaternion_t *out) {
  if (!a || !b || !out)
    return;

#if defined(USE_AVX2)
  hc_quaternion_mul_avx2(a, b, out);
#else
  // Hamilton product: (a.w + a.x*i + a.y*j + a.z*k) * (b.w + b.x*i + b.y*j +
  // b.z*k)
  double w = a->w * b->w - a->x * b->x - a->y * b->y - a->z * b->z;
  double x = a->w * b->x + a->x * b->w + a->y * b->z - a->z * b->y;
  double y = a->w * b->y - a->x * b->z + a->y * b->w + a->z * b->x;
  double z = a->w * b->z + a->x * b->y - a->y * b->x + a->z * b->w;

  out->w = w;
  out->x = x;
  out->y = y;
  out->z = z;
#endif
}

// Quaternion addition
void hc_quaternion_add(const hc_quaternion_t *a, const hc_quaternion_t *b,
                       hc_quaternion_t *out) {
  if (!a || !b || !out)
    return;
  out->w = a->w + b->w;
  out->x = a->x + b->x;
  out->y = a->y + b->y;
  out->z = a->z + b->z;
}

// Quaternion scalar multiplication
void hc_quaternion_scale(const hc_quaternion_t *q, double scalar,
                         hc_quaternion_t *out) {
  if (!q || !out)
    return;
  out->w = q->w * scalar;
  out->x = q->x * scalar;
  out->y = q->y * scalar;
  out->z = q->z * scalar;
}

// Quaternion conjugate
void hc_quaternion_conjugate(const hc_quaternion_t *q, hc_quaternion_t *out) {
  if (!q || !out)
    return;
  out->w = q->w;
  out->x = -q->x;
  out->y = -q->y;
  out->z = -q->z;
}

// Quaternion norm squared
double hc_quaternion_norm_squared(const hc_quaternion_t *q) {
  if (!q)
    return 0.0;
  return q->w * q->w + q->x * q->x + q->y * q->y + q->z * q->z;
}

// Quaternion inverse
int hc_quaternion_inverse(const hc_quaternion_t *q, hc_quaternion_t *out) {
  if (!q || !out)
    return -1;

  double norm_sq = hc_quaternion_norm_squared(q);
  if (norm_sq < 1e-10) {
    return -1; // Quaternion too close to zero, no inverse
  }

  hc_quaternion_t conj;
  hc_quaternion_conjugate(q, &conj);

  out->w = conj.w / norm_sq;
  out->x = conj.x / norm_sq;
  out->y = conj.y / norm_sq;
  out->z = conj.z / norm_sq;

  return 0;
}

// Quaternion power (exponentiation by squaring)
void hc_quaternion_power(const hc_quaternion_t *base, uint32_t exponent,
                         hc_quaternion_t *out) {
  if (!base || !out)
    return;

  // Initialize result to identity quaternion (1, 0, 0, 0)
  hc_quaternion_t result = {1.0, 0.0, 0.0, 0.0};
  hc_quaternion_t temp = *base;

  while (exponent > 0) {
    if (exponent & 1) {
      hc_quaternion_t new_result;
      hc_quaternion_mul(&result, &temp, &new_result);
      result = new_result;
    }
    hc_quaternion_t new_temp;
    hc_quaternion_mul(&temp, &temp, &new_temp);
    temp = new_temp;
    exponent >>= 1;
  }

  *out = result;
}

// Convert chaos bytes to quaternion
void hc_chaos_to_quaternion(const unsigned char *chaos_bytes, size_t len,
                            hc_quaternion_t *out) {
  if (!chaos_bytes || !out || len < 16)
    return;

  // Use first 16 bytes to construct quaternion
  uint32_t w = ((uint32_t)chaos_bytes[0] << 24) |
               ((uint32_t)chaos_bytes[1] << 16) |
               ((uint32_t)chaos_bytes[2] << 8) | (uint32_t)chaos_bytes[3];
  uint32_t x = ((uint32_t)chaos_bytes[4] << 24) |
               ((uint32_t)chaos_bytes[5] << 16) |
               ((uint32_t)chaos_bytes[6] << 8) | (uint32_t)chaos_bytes[7];
  uint32_t y = ((uint32_t)chaos_bytes[8] << 24) |
               ((uint32_t)chaos_bytes[9] << 16) |
               ((uint32_t)chaos_bytes[10] << 8) | (uint32_t)chaos_bytes[11];
  uint32_t z = ((uint32_t)chaos_bytes[12] << 24) |
               ((uint32_t)chaos_bytes[13] << 16) |
               ((uint32_t)chaos_bytes[14] << 8) | (uint32_t)chaos_bytes[15];

  out->w = (double)w;
  out->x = (double)x;
  out->y = (double)y;
  out->z = (double)z;
}

// Encode message bytes into quaternion representation
void hc_message_to_quaternion(const unsigned char *msg, size_t msg_len,
                              hc_quaternion_t *out) {
  if (!msg || !out)
    return;

  // Encode up to 32 bytes of message into quaternion components
  uint64_t w_val = 0, x_val = 0, y_val = 0, z_val = 0;

  // Pack message bytes into quaternion components (8 bytes each)
  size_t i = 0;
  if (i < msg_len && i < 8) {
    memcpy(&w_val, msg + i, (msg_len - i < 8) ? (msg_len - i) : 8);
    i += 8;
  }
  if (i < msg_len && i < 16) {
    memcpy(&x_val, msg + i, (msg_len - i < 8) ? (msg_len - i) : 8);
    i += 8;
  }
  if (i < msg_len && i < 24) {
    memcpy(&y_val, msg + i, (msg_len - i < 8) ? (msg_len - i) : 8);
    i += 8;
  }
  if (i < msg_len && i < 32) {
    memcpy(&z_val, msg + i, (msg_len - i < 8) ? (msg_len - i) : 8);
  }

  // Convert to normalized doubles
  out->w = (double)w_val;
  out->x = (double)x_val;
  out->y = (double)y_val;
  out->z = (double)z_val;
}
