#ifndef hc_CBOM_H
#define hc_CBOM_H

#include <stddef.h>

/*
 * Generate a Cryptographic Bill of Materials (CBOM) for a given project.
 * The function scans the project's include directories for symbols prefixed
 * with "hc_" and writes a simple JSON array to the output file.
 *
 * @param project_path Path to the root of the project to scan.
 * @param output_path  Path to the JSON file that will contain the CBOM.
 * @return 0 on success, non‑zero on failure.
 */
int hc_generate_cbom(const char *project_path, const char *output_path);

#endif // hc_CBOM_H
