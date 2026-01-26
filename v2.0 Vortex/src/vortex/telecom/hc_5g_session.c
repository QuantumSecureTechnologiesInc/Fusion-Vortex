#include "cemqc.h"
#include "hc_constant_time.h"
#include "hc_telecom.h"
#include <string.h>


int hc_5g_session_init(hc_5g_session_t *session, const uint8_t *master_key) {
  if (!session || !master_key)
    return -1;

  // Convert master key bytes to quaternion
  hc_chaos_to_quaternion(master_key, 32, &session->master_key_quat);

  // Ensure quaternion is non-zero (normalized)
  double norm_sq = hc_quaternion_norm_squared(&session->master_key_quat);
  if (norm_sq < 1e-10) {
    // Use fallback values if key is degenerate
    session->master_key_quat.w = 1.0 + (double)master_key[0];
    session->master_key_quat.x = (double)master_key[1];
    session->master_key_quat.y = (double)master_key[2];
    session->master_key_quat.z = (double)master_key[3];
  }

  session->sequence_number = 0;
  session->crypto_suite = 0x01; // Quaternion-chaos suite

  // Derive session ID from master key (deterministic)
  hc_rng_state_t id_gen;
  hc_rng_init(&id_gen, master_key, 32);
  hc_rng_generate(&id_gen, session->session_id, 16);

  return 0;
}

int hc_5g_derive_handoff_key(hc_5g_session_t *session, uint8_t *key_out) {
  if (!session || !key_out)
    return -1;

  // Derive key using quaternion power with sequence number
  // K_handoff = master_key_quat ^ (sequence_number + 1)
  uint32_t exponent = (uint32_t)(session->sequence_number + 1);

  hc_quaternion_t handoff_quat;
  hc_quaternion_power(&session->master_key_quat, exponent, &handoff_quat);

  // Convert quaternion to 32-byte key
  // Use all components for maximum entropy
  uint8_t quat_bytes[32];
  memcpy(quat_bytes, &handoff_quat.w, 8);
  memcpy(quat_bytes + 8, &handoff_quat.x, 8);
  memcpy(quat_bytes + 16, &handoff_quat.y, 8);
  memcpy(quat_bytes + 24, &handoff_quat.z, 8);

  // Mix with session ID for additional security
  for (size_t i = 0; i < 16; i++) {
    quat_bytes[i] ^= session->session_id[i];
    quat_bytes[i + 16] ^= session->session_id[i];
  }

  memcpy(key_out, quat_bytes, 32);

  // Increment sequence for next handoff
  session->sequence_number++;

  return 0;
}

int hc_5g_ratchet_forward(hc_5g_session_t *session) {
  if (!session)
    return -1;

  // Generate ratchet quaternion from current state
  uint8_t ratchet_seed[32];
  memcpy(ratchet_seed, &session->master_key_quat.w, 8);
  memcpy(ratchet_seed + 8, &session->master_key_quat.x, 8);
  memcpy(ratchet_seed + 16, &session->master_key_quat.y, 8);
  memcpy(ratchet_seed + 24, &session->master_key_quat.z, 8);

  // Mix in sequence number for variation
  for (size_t i = 0; i < 8; i++) {
    ratchet_seed[i] ^= (uint8_t)((session->sequence_number >> (i * 8)) & 0xFF);
  }

  hc_quaternion_t ratchet_quat;
  hc_chaos_to_quaternion(ratchet_seed, 32, &ratchet_quat);

  // Forward ratchet: master_key = master_key ⊗ ratchet_quat
  // Non-commutativity makes this one-way
  hc_quaternion_t new_master;
  hc_quaternion_mul(&session->master_key_quat, &ratchet_quat, &new_master);

  session->master_key_quat = new_master;

  return 0;
}

int hc_5g_multi_party_agreement(const uint8_t **party_keys, size_t num_parties,
                                uint8_t *shared_key_out) {
  if (!party_keys || num_parties == 0 || num_parties > 8 || !shared_key_out) {
    return -1;
  }

  // Convert all party keys to quaternions
  hc_quaternion_t party_quats[8];
  for (size_t i = 0; i < num_parties; i++) {
    if (!party_keys[i])
      return -1;
    hc_chaos_to_quaternion(party_keys[i], 32, &party_quats[i]);
  }

  // Combine using quaternion multiplication
  // Result is commutative in the sense that all parties contribute equally
  // but individual order matters (non-commutative algebra)
  hc_quaternion_t combined = party_quats[0];

  for (size_t i = 1; i < num_parties; i++) {
    hc_quaternion_t temp;
    hc_quaternion_mul(&combined, &party_quats[i], &temp);
    combined = temp;
  }

  // Extract shared key from combined quaternion
  uint8_t combined_bytes[32];
  memcpy(combined_bytes, &combined.w, 8);
  memcpy(combined_bytes + 8, &combined.x, 8);
  memcpy(combined_bytes + 16, &combined.y, 8);
  memcpy(combined_bytes + 24, &combined.z, 8);

  // Final mixing using CEMQC RNG
  hc_rng_state_t final_mix;
  hc_rng_init(&final_mix, combined_bytes, 32);
  hc_rng_generate(&final_mix, shared_key_out, 32);

  return 0;
}
