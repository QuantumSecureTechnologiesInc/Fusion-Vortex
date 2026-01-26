#include "internal/hc_constant_time.h"
#include "public/hc_telecom.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>


// Optimised for 1500-byte MTU (Ethernet)
#define HC_MAX_PACKET_SIZE 1500

// Internal struct definition needed for testing context state
// Copying typedef from hc_packet_crypto.c for visibility in test
typedef struct {
  hc_quaternion_t key_quat;  // Quaternion encryption key
  uint32_t sequence;         // Packet sequence number
  hc_rng_state_t stream_gen; // Chaotic stream generator
  uint8_t mac_key[32];       // CEMQC-based MAC key
} hc_packet_crypto_ctx_t;

// Extern declarations (since they are in .c file without public header for
// structs)
int hc_packet_crypto_init(hc_packet_crypto_ctx_t *ctx, const uint8_t *key,
                          size_t key_len, const uint8_t *mac_key);

int hc_packet_encrypt(hc_packet_crypto_ctx_t *ctx, uint8_t *packet,
                      size_t packet_len, uint8_t *mac_out);

int hc_packet_decrypt(hc_packet_crypto_ctx_t *ctx, uint8_t *packet,
                      size_t packet_len, const uint8_t *mac_tag);

#define TEST_ASSERT(cond)                                                      \
  if (!(cond)) {                                                               \
    printf("FAILED: %s:%d: %s\n", __FILE__, __LINE__, #cond);                  \
    return 1;                                                                  \
  }

#define PASS() printf("✓ %s passed\n", __func__)

int test_5g_session_flow(void) {
  hc_5g_session_t alice, bob;
  uint8_t master_key[32];

  // Deterministic master key for testing
  memset(master_key, 0xAB, 32);

  // 1. Initialise Sessions
  TEST_ASSERT(hc_5g_session_init(&alice, master_key) == 0);
  TEST_ASSERT(hc_5g_session_init(&bob, master_key) == 0);

  // Verify session IDs match
  TEST_ASSERT(memcmp(alice.session_id, bob.session_id, 16) == 0);

  // 2. Derive Handoff Key 1
  uint8_t alice_k1[32], bob_k1[32];
  TEST_ASSERT(hc_5g_derive_handoff_key(&alice, alice_k1) == 0);
  TEST_ASSERT(hc_5g_derive_handoff_key(&bob, bob_k1) == 0);

  TEST_ASSERT(memcmp(alice_k1, bob_k1, 32) == 0);

  // 3. Ratchet Forward
  TEST_ASSERT(hc_5g_ratchet_forward(&alice) == 0);
  TEST_ASSERT(hc_5g_ratchet_forward(&bob) == 0);

  // 4. Derive Handoff Key 2 (post-ratchet)
  uint8_t alice_k2[32], bob_k2[32];
  TEST_ASSERT(hc_5g_derive_handoff_key(&alice, alice_k2) == 0);
  TEST_ASSERT(hc_5g_derive_handoff_key(&bob, bob_k2) == 0);

  TEST_ASSERT(memcmp(alice_k2, bob_k2, 32) == 0);

  // K2 must be different from K1
  TEST_ASSERT(memcmp(alice_k1, alice_k2, 32) != 0);

  PASS();
  return 0;
}

int test_packet_encryption(void) {
  hc_packet_crypto_ctx_t sender, receiver;
  uint8_t key[32], mac_key[32];

  memset(key, 0x11, 32);
  memset(mac_key, 0x22, 32);

  TEST_ASSERT(hc_packet_crypto_init(&sender, key, 32, mac_key) == 0);
  TEST_ASSERT(hc_packet_crypto_init(&receiver, key, 32, mac_key) == 0);

  // Test data
  uint8_t plaintext[100];
  uint8_t buffer[100];
  uint8_t mac[16];

  for (int i = 0; i < 100; i++)
    plaintext[i] = (uint8_t)i;
  memcpy(buffer, plaintext, 100);

  // Encrypt
  TEST_ASSERT(hc_packet_encrypt(&sender, buffer, 100, mac) == 0);

  // Ciphertext should differ from plaintext
  TEST_ASSERT(memcmp(buffer, plaintext, 100) != 0);

  // Decrypt
  TEST_ASSERT(hc_packet_decrypt(&receiver, buffer, 100, mac) == 0);

  // Should match original
  TEST_ASSERT(memcmp(buffer, plaintext, 100) == 0);

  // Tamper test
  buffer[50] ^= 0x01;
  TEST_ASSERT(hc_packet_decrypt(&receiver, buffer, 100, mac) !=
              0); // Should fail auth

  PASS();
  return 0;
}

int main(void) {
  if (test_5g_session_flow() != 0)
    return 1;
  if (test_packet_encryption() != 0)
    return 1;

  printf("All telecom tests passed.\n");
  return 0;
}
