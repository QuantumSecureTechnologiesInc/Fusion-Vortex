/*
 * test_ed25519.c
 *
 * Comprehensive test suite for Ed25519 signature library.
 *
 * This program:
 * 1. Generates a keypair
 * 2. Signs a test message
 * 3. Verifies the signature
 * 4. Runs edge case tests
 * 5. Demonstrates PQCA integration points
 *
 * Compilation:
 * ────────────
 * MSVC (Windows):
 *   cl /O2 /W4 test_ed25519.c ed25519_core.c ed25519_field.c -out:test_ed25519.exe
 *
 * GCC/Clang (Linux/macOS):
 *   gcc -O2 -Wall -Wextra test_ed25519.c ed25519_core.c ed25519_field.c -o test_ed25519
 *   clang -O2 test_ed25519.c ed25519_core.c ed25519_field.c -o test_ed25519
 *
 * Run:
 *   ./test_ed25519
 *   Expected: All tests pass, exit code 0
 */

#include "ed25519_complete.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Utility: Print hex bytes */
static void print_hex(const uint8_t *data, size_t len, const char *label) {
    size_t i;
    printf("%s: ", label);
    for (i = 0; i < len; i++) {
        printf("%02x", data[i]);
    }
    printf("\n");
}

/* Utility: Compare buffers constant-time */
static int ct_equal(const uint8_t *a, const uint8_t *b, size_t len) {
    uint32_t diff = 0;
    size_t i;
    for (i = 0; i < len; i++) {
        diff |= a[i] ^ b[i];
    }
    return diff == 0;
}

/* Test 1: Basic keypair generation and signature verification */
static int test_basic_sign_verify(void) {
    uint8_t pk[ED25519_PUBLIC_KEY_BYTES];
    uint8_t sk[ED25519_SECRET_KEY_BYTES];
    uint8_t message[100] = "The quick brown fox jumps over the lazy dog";
    uint8_t signature[ED25519_SIGNATURE_BYTES];
    ed25519_status_t status;

    printf("\n[Test 1] Basic Sign/Verify\n");
    printf("────────────────────────\n");

    /* Generate keypair */
    status = ed25519_keygen(pk, sk);
    if (status != ED25519_SUCCESS) {
        printf("FAIL: Keypair generation failed\n");
        return 0;
    }
    print_hex(pk, 32, "Public Key");

    /* Sign message */
    status = ed25519_sign(message, strlen((const char *)message), sk, signature);
    if (status != ED25519_SUCCESS) {
        printf("FAIL: Signing failed\n");
        return 0;
    }
    print_hex(signature, 32, "Signature (R)");
    print_hex(signature + 32, 32, "Signature (S)");

    /* Verify signature */
    status = ed25519_verify(message, strlen((const char *)message), pk, signature);
    if (status != ED25519_SUCCESS) {
        printf("FAIL: Verification failed for valid signature\n");
        return 0;
    }
    printf("PASS: Signature verified successfully\n");

    return 1;
}

/* Test 2: Signature rejection with modified message */
static int test_signature_rejection(void) {
    uint8_t pk[ED25519_PUBLIC_KEY_BYTES];
    uint8_t sk[ED25519_SECRET_KEY_BYTES];
    uint8_t message[50] = "Original message for testing";
    uint8_t modified_message[50] = "Modified message for testing";
    uint8_t signature[ED25519_SIGNATURE_BYTES];
    ed25519_status_t status;

    printf("\n[Test 2] Signature Rejection on Modified Message\n");
    printf("────────────────────────────────────────────────\n");

    ed25519_keygen(pk, sk);
    ed25519_sign(message, strlen((const char *)message), sk, signature);

    /* Attempt to verify with modified message */
    status = ed25519_verify(modified_message, strlen((const char *)modified_message), pk, signature);
    if (status == ED25519_ERROR_INVALID_SIG) {
        printf("PASS: Invalid signature correctly rejected\n");
        return 1;
    } else {
        printf("FAIL: Invalid signature was accepted!\n");
        return 0;
    }
}

/* Test 3: Deterministic signing (same message = same signature) */
static int test_deterministic_signing(void) {
    uint8_t pk[ED25519_PUBLIC_KEY_BYTES];
    uint8_t sk[ED25519_SECRET_KEY_BYTES];
    uint8_t message[50] = "Test message for determinism";
    uint8_t sig1[ED25519_SIGNATURE_BYTES];
    uint8_t sig2[ED25519_SIGNATURE_BYTES];

    printf("\n[Test 3] Deterministic Signing\n");
    printf("──────────────────────────────\n");

    ed25519_keygen(pk, sk);

    /* Sign same message twice */
    ed25519_sign(message, strlen((const char *)message), sk, sig1);
    ed25519_sign(message, strlen((const char *)message), sk, sig2);

    if (ct_equal(sig1, sig2, ED25519_SIGNATURE_BYTES)) {
        printf("PASS: Same message produces identical signature\n");
        return 1;
    } else {
        printf("FAIL: Same message produced different signatures\n");
        return 0;
    }
}

/* Test 4: Empty message handling */
static int test_empty_message(void) {
    uint8_t pk[ED25519_PUBLIC_KEY_BYTES];
    uint8_t sk[ED25519_SECRET_KEY_BYTES];
    uint8_t signature[ED25519_SIGNATURE_BYTES];
    ed25519_status_t status;

    printf("\n[Test 4] Empty Message\n");
    printf("─────────────────────\n");

    ed25519_keygen(pk, sk);

    /* Sign empty message */
    status = ed25519_sign(NULL, 0, sk, signature);
    if (status != ED25519_SUCCESS) {
        printf("FAIL: Could not sign empty message\n");
        return 0;
    }

    /* Verify empty message signature */
    status = ed25519_verify(NULL, 0, pk, signature);
    if (status != ED25519_SUCCESS) {
        printf("FAIL: Could not verify empty message signature\n");
        return 0;
    }

    printf("PASS: Empty message handling works correctly\n");
    return 1;
}

/* Test 5: Public key derivation from secret key */
static int test_public_key_derivation(void) {
    uint8_t pk1[ED25519_PUBLIC_KEY_BYTES];
    uint8_t pk2[ED25519_PUBLIC_KEY_BYTES];
    uint8_t sk[ED25519_SECRET_KEY_BYTES];
    ed25519_status_t status;

    printf("\n[Test 5] Public Key Derivation\n");
    printf("───────────────────────────────\n");

    ed25519_keygen(pk1, sk);

    /* Derive public key from secret key */
    status = ed25519_public_from_secret(sk, pk2);
    if (status != ED25519_SUCCESS) {
        printf("FAIL: Public key derivation failed\n");
        return 0;
    }

    if (ct_equal(pk1, pk2, ED25519_PUBLIC_KEY_BYTES)) {
        printf("PASS: Derived public key matches original\n");
        return 1;
    } else {
        printf("FAIL: Derived public key does not match\n");
        return 0;
    }
}

/* Test 6: Large message signing */
static int test_large_message(void) {
    uint8_t pk[ED25519_PUBLIC_KEY_BYTES];
    uint8_t sk[ED25519_SECRET_KEY_BYTES];
    uint8_t *large_msg;
    uint8_t signature[ED25519_SIGNATURE_BYTES];
    ed25519_status_t status;
    size_t msg_size = 10000;  /* 10 KB message */
    size_t i;

    printf("\n[Test 6] Large Message (%zu bytes)\n", msg_size);
    printf("──────────────────────────────────\n");

    large_msg = (uint8_t *)malloc(msg_size);
    if (!large_msg) {
        printf("FAIL: Could not allocate memory\n");
        return 0;
    }

    /* Fill message with pseudo-random data */
    for (i = 0; i < msg_size; i++) {
        large_msg[i] = (uint8_t)(i * 31 % 256);
    }

    ed25519_keygen(pk, sk);

    /* Sign large message */
    status = ed25519_sign(large_msg, msg_size, sk, signature);
    if (status != ED25519_SUCCESS) {
        printf("FAIL: Could not sign large message\n");
        free(large_msg);
        return 0;
    }

    /* Verify large message signature */
    status = ed25519_verify(large_msg, msg_size, pk, signature);
    if (status != ED25519_SUCCESS) {
        printf("FAIL: Could not verify large message signature\n");
        free(large_msg);
        return 0;
    }

    printf("PASS: Large message signing and verification successful\n");
    free(large_msg);
    return 1;
}

/* Test 7: Zeroization */
static int test_zeroization(void) {
    uint8_t buffer[64];
    size_t i;

    printf("\n[Test 7] Zeroization\n");
    printf("────────────────────\n");

    /* Fill buffer with non-zero data */
    memset(buffer, 0xff, 64);

    /* Zeroize */
    ed25519_zeroize(buffer, 64);

    /* Check if zeroed */
    int zeroed = 1;
    for (i = 0; i < 64; i++) {
        if (buffer[i] != 0) {
            zeroed = 0;
            break;
        }
    }

    if (zeroed) {
        printf("PASS: Buffer successfully zeroized\n");
        return 1;
    } else {
        printf("FAIL: Buffer was not properly zeroized\n");
        return 0;
    }
}

/* Main test runner */
int main(void) {
    int total = 0, passed = 0;

    printf("\n");
    printf("╔═══════════════════════════════════════════════════════════╗\n");
    printf("║        Ed25519 Signature Library Test Suite               ║\n");
    printf("║      QuantumSecure Technologies Ltd. - Coding Excellence  ║\n");
    printf("╚═══════════════════════════════════════════════════════════╝\n");

    /* Run tests */
    total++; passed += test_basic_sign_verify();
    total++; passed += test_signature_rejection();
    total++; passed += test_deterministic_signing();
    total++; passed += test_empty_message();
    total++; passed += test_public_key_derivation();
    total++; passed += test_large_message();
    total++; passed += test_zeroization();

    /* Summary */
    printf("\n");
    printf("╔═══════════════════════════════════════════════════════════╗\n");
    printf("║                       Test Results                        ║\n");
    printf("├───────────────────────────────────────────────────────────┤\n");
    printf("│ Total:  %d\n", total);
    printf("│ Passed: %d\n", passed);
    printf("│ Failed: %d\n", total - passed);
    printf("├───────────────────────────────────────────────────────────┤\n");
    if (passed == total) {
        printf("│ Status: ALL TESTS PASSED ✓\n");
        printf("╚═══════════════════════════════════════════════════════════╝\n");
        return 0;
    } else {
        printf("│ Status: SOME TESTS FAILED ✗\n");
        printf("╚═══════════════════════════════════════════════════════════╝\n");
        return 1;
    }
}
