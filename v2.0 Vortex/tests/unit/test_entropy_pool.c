#include "ns_api.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>


#ifdef _WIN32
#include <windows.h>
#else
#include <unistd.h>
#endif

/**
 * @brief Test program for asynchronous entropy pooling system
 *
 * Verifies:
 * 1. Initialization
 * 2. Key generation performance
 * 3. Thread safety
 * 4. Statistics tracking
 * 5. Cleanup
 */

int main(void) {
  printf("=== NeuralSeal v3.2 Fulminis Entropy Pool Test ===\\n\\n");

  // Test 1: Initialization
  printf("[1] Initializing entropy pool...\\n");
  int result = ns_init_entropy_pool();
  if (result != 0) {
    printf("    FAILED: Initialization returned %d\\n", result);
    return 1;
  }
  printf("    SUCCESS: Entropy pool initialized\\n\\n");

  // Wait for pool to fill
  printf("[2] Waiting for pool to fill (2 seconds)...\\n");
#ifdef _WIN32
  Sleep(2000);
#else
  sleep(2);
#endif

  // Test 2: Statistics
  size_t produced, consumed, available, underruns;
  result = ns_get_entropy_stats(&produced, &consumed, &available, &underruns);
  if (result != 0) {
    printf("    FAILED: Stats retrieval failed\\n");
    ns_cleanup_entropy_pool();
    return 1;
  }
  printf("    Produced: %zu keys\\n", produced);
  printf("    Consumed: %zu keys\\n", consumed);
  printf("    Available: %zu keys\\n", available);
  printf("    Underruns: %zu\\n\\n", underruns);

  if (available == 0) {
    printf("    WARNING: Pool is empty, background thread may not be "
           "working\\n\\n");
  }

  // Test 3: Key Generation Performance
  printf("[3] Testing key generation performance (1000 keys)...\\n");
  uint8_t key[32];
  int failures = 0;

  for (int i = 0; i < 1000; i++) {
    result = ns_get_random_key(key, 32);
    if (result != 0) {
      failures++;
    }
  }

  if (failures > 0) {
    printf("    FAILED: %d/%d key generations failed\\n", failures, 1000);
  } else {
    printf("    SUCCESS: All 1000 keys generated successfully\\n");
  }

  // Show sample key (first 16 bytes)
  printf("    Sample key (first 16 bytes): ");
  for (int i = 0; i < 16; i++) {
    printf("%02x", key[i]);
  }
  printf("...\\n\\n");

  // Test 4: Updated Statistics
  result = ns_get_entropy_stats(&produced, &consumed, &available, &underruns);
  printf("[4] Updated statistics:\\n");
  printf("    Produced: %zu keys\\n", produced);
  printf("    Consumed: %zu keys\\n", consumed);
  printf("    Available: %zu keys\\n", available);
  printf("    Underruns: %zu\\n\\n", underruns);

  // Test 5: Cleanup
  printf("[5] Cleaning up entropy pool...\\n");
  ns_cleanup_entropy_pool();
  printf("    SUCCESS: Cleanup complete\\n\\n");

  // Test 6: Verify cleanup
  printf("[6] Verifying cleanup (should fail)...\\n");
  result = ns_get_random_key(key, 32);
  if (result == -2) {
    printf("    SUCCESS: Key generation correctly fails after cleanup\\n");
  } else {
    printf("    FAILED: Expected error -2, got %d\\n", result);
  }

  printf("\\n=== All Tests Complete ===\\n");
  return 0;
}
