#ifndef hc_POLICY_ENGINE_H
#define hc_POLICY_ENGINE_H

#include <stddef.h>

/*
 * Dynamic policy engine API.
 * Loads a JSON/YAML policy file that maps algorithm identifiers to concrete
 * implementations and key lengths. The engine can be queried at runtime to
 * obtain the selected algorithm for a given operation.
 *
 * @param policy_path Path to the policy configuration file.
 * @return 0 on success, non‑zero on failure.
 */
int hc_load_policy_engine(const char *policy_path);

/* Query the loaded policy for a specific operation.
 * @param operation_name Name of the operation (e.g., "keygen", "sign").
 * @param out_algorithm   Buffer to receive the algorithm identifier.
 * @param out_len         Length of the out_algorithm buffer.
 * @return 0 on success, non‑zero on failure.
 */
int hc_query_policy(const char *operation_name, char *out_algorithm,
                    size_t out_len);

#endif // hc_POLICY_ENGINE_H
