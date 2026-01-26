#include "hc_sidechannel.h"
#include "cemqc.h"
#include "hc_constant_time.h"
#include <string.h>


#ifdef _WIN32
#include <intrin.h>
#include <windows.h>

#else
#include <unistd.h>
#endif

#if defined(__x86_64__) || defined(_M_X64)
#include <emmintrin.h> // For _mm_clflush
#endif

void hc_sc_table_lookup(const uint8_t *table, size_t table_size,
                        size_t entry_size, size_t index, uint8_t *out) {
  // Delegate to constant-time lookup which accesses all entries
  hc_ct_lookup(table, table_size, entry_size, index, out);
}

void hc_sc_memcpy(uint8_t *dest, const uint8_t *src, size_t len) {
  if (!dest || !src || len == 0)
    return;

  // Use a deterministic but non-sequential access pattern
  // Based on prime number stepping to avoid cache line prediction
  const size_t step = 37; // Prime number
  size_t offset = 0;

  for (size_t i = 0; i < len; i++) {
    dest[offset] = src[offset];
    offset = (offset + step) % len;
  }
}

void hc_sc_cache_flush(void *ptr, size_t len) {
  if (!ptr || len == 0)
    return;

#if defined(__x86_64__) || defined(_M_X64)
  // Flush cache lines (64 bytes each on x86-64)
  uint8_t *p = (uint8_t *)ptr;
  for (size_t i = 0; i < len; i += 64) {
    _mm_clflush((void *)(p + i));
  }

  // Memory fence to ensure completion
  _mm_mfence();
#elif defined(__aarch64__) || defined(__ARM_ARCH)
  // ARM cache operations
  // Note: Requires privileged mode on some ARM systems
  __asm__ __volatile__("dc cvau, %0" : : "r"(ptr) : "memory");
  __asm__ __volatile__("dsb ish" : : : "memory");
  __asm__ __volatile__("isb" : : : "memory");
#else
  // Fallback: touch memory to force load (may not fully flush)
  volatile uint8_t *vp = (volatile uint8_t *)ptr;
  for (size_t i = 0; i < len; i++) {
    (void)vp[i];
  }
#endif
}

void hc_sc_quaternion_mul_blinded(const void *a, const void *b, void *out,
                                  const uint8_t *random_mask, size_t mask_len) {
  const hc_quaternion_t *qa = (const hc_quaternion_t *)a;
  const hc_quaternion_t *qb = (const hc_quaternion_t *)b;
  hc_quaternion_t *qout = (hc_quaternion_t *)out;

  if (!qa || !qb || !qout || !random_mask || mask_len < 32)
    return;

  // Create blinding quaternion from random mask
  hc_quaternion_t blind;
  hc_chaos_to_quaternion(random_mask, mask_len, &blind);

  // Ensure blinding quaternion is non-zero
  if (hc_quaternion_norm_squared(&blind) < 1e-10) {
    blind.w = 1.0;
    blind.x = (double)random_mask[0];
    blind.y = (double)random_mask[1];
    blind.z = (double)random_mask[2];
  }

  // Compute inverse of blinding quaternion
  hc_quaternion_t blind_inv;
  if (hc_quaternion_inverse(&blind, &blind_inv) != 0) {
    // Fallback: use unblinded multiplication
    hc_quaternion_mul(qa, qb, qout);
    return;
  }

  // Blinded multiplication: (a ⊗ blind) ⊗ (blind^-1 ⊗ b)
  hc_quaternion_t a_blinded, b_blinded, result_blinded;

  hc_quaternion_mul(qa, &blind, &a_blinded);
  hc_quaternion_mul(&blind_inv, qb, &b_blinded);
  hc_quaternion_mul(&a_blinded, &b_blinded, &result_blinded);

  *qout = result_blinded;

  // Flush intermediate values from cache
  hc_sc_cache_flush(&a_blinded, sizeof(a_blinded));
  hc_sc_cache_flush(&b_blinded, sizeof(b_blinded));
  hc_sc_cache_flush(&blind, sizeof(blind));
  hc_sc_cache_flush(&blind_inv, sizeof(blind_inv));
}
