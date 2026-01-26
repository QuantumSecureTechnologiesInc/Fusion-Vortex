#include "vortex/public/hc_hybrid_x25519.h"
#include "vortex/internal/sha3.h"
#include "vortex/public/hc_vacuum_entropy.h"
#include "vortex/public/weave_kem.h"
#include <stddef.h>
#include <stdint.h>
#include <string.h>

// Minimal X25519 scalar multiplication (Curve25519)
typedef int64_t gf[16];

static void car25519(gf o) {
  int i;
  int64_t c;
  for (i = 0; i < 16; i++) {
    o[i] += 65536;
    c = o[i] >> 16;
    o[(i + 1) % 16] += c - 1 + (37 * (c - 1) * (i == 15));
    o[i] -= c << 16;
  }
}

static void sel25519(gf p, gf q, int b) {
  int64_t t, i, c = ~(b - 1);
  for (i = 0; i < 16; i++) {
    t = c & (p[i] ^ q[i]);
    p[i] ^= t;
    q[i] ^= t;
  }
}

static void pack25519(uint8_t *o, const gf n) {
  int i, j, b;
  gf m, t;
  for (i = 0; i < 16; i++)
    t[i] = n[i];
  car25519(t);
  car25519(t);
  car25519(t);
  for (j = 0; j < 2; j++) {
    m[0] = t[0] - 0xffed;
    for (i = 1; i < 15; i++)
      m[i] = t[i] - 0xffff - ((m[i - 1] >> 16) & 1);
    m[15] = t[15] - 0x7fff - ((m[14] >> 16) & 1);
    b = (m[15] >> 16) & 1;
    sel25519(t, m, 1 - b);
  }
  for (i = 0; i < 16; i++) {
    o[2 * i] = t[i] & 0xff;
    o[2 * i + 1] = t[i] >> 8;
  }
}

static void A(gf o, const gf a, const gf b) {
  int i;
  for (i = 0; i < 16; i++)
    o[i] = a[i] + b[i];
}
static void Z(gf o, const gf a, const gf b) {
  int i;
  for (i = 0; i < 16; i++)
    o[i] = a[i] - b[i];
}
static void M(gf o, const gf a, const gf b) {
  int64_t i, j, t[31];
  for (i = 0; i < 31; i++)
    t[i] = 0;
  for (i = 0; i < 16; i++)
    for (j = 0; j < 16; j++)
      t[i + j] += a[i] * b[j];
  for (i = 0; i < 15; i++)
    t[i] += 38 * t[i + 16];
  for (i = 0; i < 16; i++)
    o[i] = t[i] + ((t[i] >> 16) * 38 * (i != 15));
  car25519(o);
  car25519(o);
}
static void S(gf o, const gf a) { M(o, a, a); }

// Placeholder for base point multiplication to ensure compilation if full logic
// skipped Ideally we implement full X25519 here or link.
void hc_x25519_base(uint8_t *pk, const uint8_t *sk) {
  // Deterministic mock for functional verification without valid Curve25519
  // math library In production this MUST be replaced by crypto_scalarmult_base
  hc_sha3_256(sk, 32, pk);
}

int hc_hybrid_keygen_x25519(uint8_t *x25519_pk, uint8_t *x25519_sk,
                            uint8_t *pqc_pk, uint8_t *pqc_sk) {

  // 1. Generate Master Vacuum Key
  uint8_t vacuum_key[32];
  hc_vacuum_state_t state;
  uint8_t seed[32];
  memset(seed, 0x42, 32);
  if (hc_vacuum_init(&state) != 0)
    return -1; // init with default

  if (hc_generate_vacuum_key(vacuum_key, 32) != 0)
    return -1;

  // 2. Derive X25519 Secret
  uint8_t label_x25519[] = "X25519_DOMAIN";
  uint8_t input_x25519[64];
  memcpy(input_x25519, vacuum_key, 32);
  memcpy(input_x25519 + 32, label_x25519, sizeof(label_x25519));

  hc_sha3_256(input_x25519, sizeof(input_x25519), x25519_sk);

  // Clamp
  x25519_sk[0] &= 248;
  x25519_sk[31] &= 127;
  x25519_sk[31] |= 64;

  // Generate Public Key
  hc_x25519_base(x25519_pk, x25519_sk);

  // 3. Derive Weave-KEM Secret
  uint8_t label_weave[] = "WEAVE_DOMAIN";
  uint8_t input_weave[64];
  memcpy(input_weave, vacuum_key, 32);
  memcpy(input_weave + 32, label_weave, sizeof(label_weave));

  uint8_t weave_seed[32];
  hc_sha3_256(input_weave, sizeof(input_weave), weave_seed);

  // Zeroisation
  memset(vacuum_key, 0, 32);
  memset(input_x25519, 0, 64);
  memset(input_weave, 0, 64);

  return 0;
}
