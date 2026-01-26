#ifndef hc_ACCELERATOR_H
#define hc_ACCELERATOR_H

#include <stddef.h>
#include <stdint.h>

/*
 * Accelerator hook API.
 * Allows the library to off‑load cryptographic primitives to a hardware
 * accelerator (e.g., SHA‑3 or PQC accelerator). The implementation is a stub
 * that simply forwards to the software implementation.
 *
 * @param algorithm_name Name of the algorithm to accelerate (e.g., "sha3").
 * @param input          Pointer to input data.
 * @param input_len      Length of input data in bytes.
 * @param output         Buffer to receive the result.
 * @param output_len     Length of the output buffer.
 * @return 0 on success, non‑zero on failure.
 */
int hc_accelerator_process(const char *algorithm_name, const uint8_t *input,
                           size_t input_len, uint8_t *output,
                           size_t output_len);

#endif // hc_ACCELERATOR_H
