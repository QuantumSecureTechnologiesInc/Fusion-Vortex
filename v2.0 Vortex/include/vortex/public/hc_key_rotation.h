#ifndef hc_KEY_ROTATION_H
#define hc_KEY_ROTATION_H

#include <stddef.h>

/*
 * Automated key rotation API.
 * Reads a configuration file that specifies key lifetimes and locations,
 * generates new keys and updates the relevant certificates.
 *
 * @param config_path Path to a JSON/YAML configuration describing rotation
 * policy.
 * @return 0 on success, non‑zero on failure.
 */
int hc_rotate_keys(const char *config_path);

#endif // hc_KEY_ROTATION_H
