#include "vortex/public/hc_policy_engine.h"
#include <stdio.h>
#include <string.h>

/* Simple static storage for a single policy entry */
static char stored_operation[64] = "";
static char stored_algorithm[64] = "";

int hc_load_policy_engine(const char *policy_path) {
  if (!policy_path)
    return -1;
  /* In a real implementation this would parse JSON/YAML. Here we just
     simulate loading a fixed policy for demonstration purposes. */
  printf("[hc_policy_engine] Loading policy from %s\n", policy_path);
  /* Example: map "keygen" to "ml_kem_1024" */
  strncpy(stored_operation, "keygen", sizeof(stored_operation) - 1);
  strncpy(stored_algorithm, "ml_kem_1024", sizeof(stored_algorithm) - 1);
  return 0;
}

int hc_query_policy(const char *operation_name, char *out_algorithm,
                    size_t out_len) {
  if (!operation_name || !out_algorithm)
    return -1;
  if (strcmp(operation_name, stored_operation) == 0) {
    strncpy(out_algorithm, stored_algorithm, out_len - 1);
    out_algorithm[out_len - 1] = '\0';
    return 0;
  }
  /* No matching entry – return error */
  return -1;
}
