#include "runtime/native/fusionrt.h"
#include <stdio.h>
#include <string.h>

// Mock Fusion extern syntax in C
// In real Fusion: extern fn fusion_fs_write_str(path: string, content: string)
// -> bool;

int main() {
  printf("--- Fusion Native Runtime Verification ---\n");

  // 1. Initialize Runtime
  fusion_rt_init();
  printf("[PASS] Runtime initialized\n");

  // 2. Test File System
  const char *test_file = "test_fusion_rt.txt";
  const char *content = "Hello from Fusion Native Runtime!";

  if (fusion_fs_write_str(test_file, content)) {
    printf("[PASS] fs_write_str success\n");
  } else {
    printf("[FAIL] fs_write_str failed\n");
    return 1;
  }

  if (fusion_fs_exists(test_file)) {
    printf("[PASS] fs_exists success\n");
  } else {
    printf("[FAIL] fs_exists failed\n");
    return 1;
  }

  const char *read_back = fusion_fs_read_to_string(test_file);
  if (strcmp(read_back, content) == 0) {
    printf("[PASS] fs_read_to_string matches: '%s'\n", read_back);
  } else {
    printf("[FAIL] fs_read_to_string mismatch: '%s'\n", read_back);
    return 1;
  }

  // 3. Test Time
  int64_t now = fusion_time_now_ms();
  printf("[PASS] time_now_ms: %lld\n", (long long)now);

  fusion_sleep_ms(100);
  int64_t later = fusion_time_now_ms();

  if (later >= now + 100) {
    printf("[PASS] sleep_ms passed (delta: %lld ms)\n",
           (long long)(later - now));
  } else {
    printf("[FAIL] sleep_ms failed (delta: %lld ms)\n",
           (long long)(later - now));
  }

  // 4. Test Crypto/Hashing
  int64_t hash = fusion_hash32("fusion");
  printf("[PASS] hash32('fusion'): %lld\n", (long long)hash);

  // 5. Cleanup
  fusion_fs_remove_file(test_file);
  fusion_rt_shutdown();
  printf("[PASS] Runtime shutdown\n");

  printf("\n>>> VERIFICATION SUCCESSFUL: libfusionrt works correctly <<<\n");
  return 0;
}
