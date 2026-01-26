/*
 * ed25519_sha512.c
 *
 * Complete SHA-512 implementation for Ed25519.
 * RFC 3394 compliant, embedded for portability.
 */

#include "ed25519_core.h"
#include <string.h>

/* SHA-512 constants */
static const uint64_t sha512_k[80] = {
    0x428a2f98d728ae22ULL, 0x7137449123ef65cdULL, 0xb5c0fbcfec4d3b2fULL, 0xe9b5dba58189dabbULL,
    0x3956c25bf348b538ULL, 0x59f111f1b605d019ULL, 0x923f82a4af194f9bULL, 0xab1c5ed5da6d8118ULL,
    0xd807aa98a3030242ULL, 0x12835b0145706fbeULL, 0x243185be4ee4b28cULL, 0x550c7dc3d5ffb4e2ULL,
    0x72be5d74f27b896fULL, 0x80deb1fe3b1696b1ULL, 0x9bdc06a725c71235ULL, 0xc19bf174cf692694ULL,
    0xe49b69c19ef14ad2ULL, 0xefbe4786384f25e3ULL, 0x0fc19dc68b8cd5b5ULL, 0x240ca1cc77ac9c65ULL,
    0x2de92c6f592b0275ULL, 0x4a7484aa6ea6e483ULL, 0x5cb0a9dcbd41fbd4ULL, 0x76f988da831153b5ULL,
    0x983e5152ee66dfabULL, 0xa831c66d2db43210ULL, 0xb00327c898fb213fULL, 0xbf597fc7beef0ee4ULL,
    0xc6e00bf33da88fc2ULL, 0xd5a79147930aa725ULL, 0x06ca6351e003826fULL, 0x142929670a0e6e70ULL,
    0x27b70a8546d22ffcULL, 0x2e1b21385c26c926ULL, 0x4d2c6dfc5ac42aedULL, 0x53380d139d95b3dfULL,
    0x650a73548baf63deULL, 0x766a0ebb3c88b691ULL, 0x81c2c92e47edaee6ULL, 0x92722c851482353bULL,
    0xa2bfe8a14cf10364ULL, 0xa81a664bbc423001ULL, 0xc24b8b70d0f89791ULL, 0xc76c51a30654be30ULL,
    0xd192e819d6ef5218ULL, 0xd69906245565a910ULL, 0xf40e35855771202aULL, 0x106aa07032bbd1b8ULL,
    0x19a4c116b8d2d0c8ULL, 0x1e376c081b3572c0ULL, 0x2748774cdf8eeb99ULL, 0x34b0bcb5e19b48a8ULL,
    0x391c0cb3c5c95a63ULL, 0x4ed8aa4ae3418acbULL, 0x5b9cca4f7763e373ULL, 0x682e6ff3d6b2b8a3ULL,
    0x748f82ee5defb2fcULL, 0x78a5636f43172f60ULL, 0x84c87814a1f0ab72ULL, 0x8cc702081a6439ecULL,
    0x90befffa23631e28ULL, 0xa4506cebde82bde9ULL, 0xbef9a3f7b2c67915ULL, 0xc67178f2e372532bULL,
    0xca273eceea26619cULL, 0xd186b8c721c0c207ULL, 0xeada7dd6cde0eb1eULL, 0xf57d4f7fee6ed178ULL,
    0x06f067aa72176fbaULL, 0x0a637dc5a2c898a6ULL, 0x113f9804bef90daeULL, 0x1b710b35131c471bULL,
    0x28db77f523047d84ULL, 0x32caab7b40c72493ULL, 0x3c9ebe0a15c9bebcULL, 0x431d67c49c100d0cULL,
    0x4cc5d4becb3e42b6ULL, 0x597f299cfc657e2aULL, 0x5fcb6fab3ad6faecULL, 0x6c44198c4a475817ULL
};

#define ROTR64(x, n) (((x) >> (n)) | ((x) << (64 - (n))))
#define CH(x, y, z)  (((x) & (y)) ^ ((~(x)) & (z)))
#define MAJ(x, y, z) (((x) & (y)) ^ ((x) & (z)) ^ ((y) & (z)))
#define SIGMA0(x)    (ROTR64(x, 28) ^ ROTR64(x, 34) ^ ROTR64(x, 39))
#define SIGMA1(x)    (ROTR64(x, 14) ^ ROTR64(x, 18) ^ ROTR64(x, 41))
#define GAMMA0(x)    (ROTR64(x, 1)  ^ ROTR64(x, 8)  ^ ((x) >> 7))
#define GAMMA1(x)    (ROTR64(x, 19) ^ ROTR64(x, 61) ^ ((x) >> 6))

/* Load 64-bit big-endian */
static uint64_t load_be64(const uint8_t *p) {
    return ((uint64_t)p[0] << 56) | ((uint64_t)p[1] << 48) |
           ((uint64_t)p[2] << 40) | ((uint64_t)p[3] << 32) |
           ((uint64_t)p[4] << 24) | ((uint64_t)p[5] << 16) |
           ((uint64_t)p[6] << 8)  | (uint64_t)p[7];
}

/* Store 64-bit big-endian */
static void store_be64(uint8_t *p, uint64_t x) {
    p[0] = (x >> 56) & 0xff;
    p[1] = (x >> 48) & 0xff;
    p[2] = (x >> 40) & 0xff;
    p[3] = (x >> 32) & 0xff;
    p[4] = (x >> 24) & 0xff;
    p[5] = (x >> 16) & 0xff;
    p[6] = (x >> 8) & 0xff;
    p[7] = x & 0xff;
}

/* Process one 1024-bit SHA-512 block */
static void sha512_process_block(uint64_t *h, const uint8_t *data) {
    uint64_t w[80];
    uint64_t a, b, c, d, e, f, g, h_var;
    uint64_t t1, t2;
    size_t i;

    /* Load message schedule */
    for (i = 0; i < 16; i++) {
        w[i] = load_be64(data + 8 * i);
    }

    /* Expand message schedule */
    for (i = 16; i < 80; i++) {
        w[i] = GAMMA1(w[i - 2]) + w[i - 7] + GAMMA0(w[i - 15]) + w[i - 16];
    }

    /* Initialize working variables */
    a = h[0];
    b = h[1];
    c = h[2];
    d = h[3];
    e = h[4];
    f = h[5];
    g = h[6];
    h_var = h[7];

    /* Main compression loop */
    for (i = 0; i < 80; i++) {
        t1 = h_var + SIGMA1(e) + CH(e, f, g) + sha512_k[i] + w[i];
        t2 = SIGMA0(a) + MAJ(a, b, c);
        h_var = g;
        g = f;
        f = e;
        e = d + t1;
        d = c;
        c = b;
        b = a;
        a = t1 + t2;
    }

    /* Add compressed chunk to current hash value */
    h[0] += a;
    h[1] += b;
    h[2] += c;
    h[3] += d;
    h[4] += e;
    h[5] += f;
    h[6] += g;
    h[7] += h_var;
}

void sha512(uint8_t *digest, const uint8_t *message, size_t len) {
    uint64_t h[8] = {
        0x6a09e667f3bcc908ULL, 0xbb67ae8584caa73bULL, 0x3c6ef372fe94f82bULL, 0xa54ff53a5f1d36f1ULL,
        0x510e527fade682d1ULL, 0x9b05688c2b3e6c1fULL, 0x1f83d9abfb41bd6bULL, 0x5be0cd19137e2179ULL
    };

    uint8_t block[128];
    uint64_t total_bits = len * 8;
    size_t pos = 0;

    /* Process complete 128-byte blocks */
    while (pos + 128 <= len) {
        sha512_process_block(h, message + pos);
        pos += 128;
    }

    /* Process final block(s) */
    size_t remaining = len - pos;
    memcpy(block, message + pos, remaining);
    block[remaining] = 0x80;
    memset(block + remaining + 1, 0, 128 - remaining - 1);

    /* If we need more than one block for padding, process this one */
    if (remaining >= 112) {
        sha512_process_block(h, block);
        memset(block, 0, 128);
    }

    /* Append length in bits as big-endian 128-bit */
    store_be64(block + 112, total_bits);
    sha512_process_block(h, block);

    /* Output hash in big-endian */
    for (size_t i = 0; i < 8; i++) {
        store_be64(digest + 8 * i, h[i]);
    }
}
