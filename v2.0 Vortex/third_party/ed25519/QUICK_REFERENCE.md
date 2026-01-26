# Ed25519 Library - Quick Reference Card

## Constants

```c
ED25519_PUBLIC_KEY_BYTES    // 32
ED25519_SECRET_KEY_BYTES    // 64 (32 seed + 32 prefix)
ED25519_SIGNATURE_BYTES     // 64 (32 R + 32 S)
ED25519_SEED_BYTES          // 32
```

## Return Codes

```c
ED25519_SUCCESS              // 0    - Operation successful
ED25519_ERROR_INVALID_KEY    // -1   - Key generation or validation failed
ED25519_ERROR_INVALID_SIG    // -2   - Signature verification failed
ED25519_ERROR_BAD_INPUT      // -3   - Invalid input parameters
```

---

## Function Reference

### Initialization

```c
// Get default configuration
ed25519_config_t ed25519_config_default(void);

// Initialize library with custom configuration
ed25519_status_t ed25519_init(const ed25519_config_t *config);

// Cleanup library resources
void ed25519_cleanup(void);
```

### Key Operations

```c
// Generate new keypair (random seed)
ed25519_status_t ed25519_keygen(
    uint8_t *public_key,      // Output: 32 bytes
    uint8_t *secret_key       // Output: 64 bytes
);

// Derive public key from secret key
ed25519_status_t ed25519_public_from_secret(
    const uint8_t *secret_key,  // Input: 64 bytes
    uint8_t *public_key         // Output: 32 bytes
);
```

### Signing & Verification

```c
// Sign a message deterministically
ed25519_status_t ed25519_sign(
    const uint8_t *message,     // Message to sign (any length, may be NULL)
    size_t message_len,         // Length in bytes
    const uint8_t *secret_key,  // 64 bytes
    uint8_t *signature          // Output: 64 bytes
);

// Verify a signature
ed25519_status_t ed25519_verify(
    const uint8_t *message,     // Original message
    size_t message_len,         // Length in bytes
    const uint8_t *public_key,  // 32 bytes
    const uint8_t *signature    // 64 bytes
);
```

### Security

```c
// Securely erase sensitive data
void ed25519_zeroize(
    void *buffer,   // Buffer to erase
    size_t len      // Number of bytes
);
```

---

## Configuration Structure

```c
typedef struct {
    // Custom RNG hook
    int (*rng_hook)(uint8_t *buffer, size_t len);
    
    // PQCA entropy mixer (optional)
    void (*entropy_mixer)(uint8_t *material, size_t len, const uint8_t *seed);
    
    // Zeroization flag (1 = enabled)
    int zeroize_on_destroy;
} ed25519_config_t;
```

---

## Common Usage Patterns

### Pattern 1: Basic Sign/Verify

```c
#include "ed25519.h"

uint8_t pk[ED25519_PUBLIC_KEY_BYTES];
uint8_t sk[ED25519_SECRET_KEY_BYTES];
uint8_t msg[] = "Hello, World!";
uint8_t sig[ED25519_SIGNATURE_BYTES];

// Initialize with defaults
ed25519_init(NULL);

// Generate keypair
ed25519_keygen(pk, sk);

// Sign message
ed25519_sign(msg, sizeof(msg), sk, sig);

// Verify signature
if (ed25519_verify(msg, sizeof(msg), pk, sig) == ED25519_SUCCESS) {
    printf("Signature is valid!\n");
}

// Cleanup
ed25519_zeroize(sk, ED25519_SECRET_KEY_BYTES);
ed25519_cleanup();
```

### Pattern 2: Custom RNG + PQCA

```c
int my_rng(uint8_t *buf, size_t len) {
    // Use hardware entropy, /dev/urandom, etc.
    return 1; // success
}

void my_entropy_mixer(uint8_t *material, size_t len, const uint8_t *seed) {
    // Apply quaternion-chaos diffusion
    // ... PQCA implementation ...
}

ed25519_config_t config = {
    .rng_hook = my_rng,
    .entropy_mixer = my_entropy_mixer,
    .zeroize_on_destroy = 1
};

ed25519_init(&config);
// Now all operations use custom RNG and PQCA entropy mixer
```

### Pattern 3: Batch Signing

```c
uint8_t pk[32], sk[64];
ed25519_keygen(pk, sk);

for (int i = 0; i < N_MESSAGES; i++) {
    uint8_t sig[64];
    ed25519_sign(messages[i], msg_lens[i], sk, sig);
    // Store sig[i]
}

// Same message always produces same signature (deterministic)
uint8_t sig1[64], sig2[64];
ed25519_sign(msg, len, sk, sig1);
ed25519_sign(msg, len, sk, sig2);
// sig1 == sig2 (byte-for-byte identical)
```

### Pattern 4: Key Persistence

```c
// Save key to file
void save_key(const char *filename, const uint8_t *sk) {
    FILE *f = fopen(filename, "wb");
    fwrite(sk, ED25519_SECRET_KEY_BYTES, 1, f);
    fclose(f);
}

// Load key from file
int load_key(const char *filename, uint8_t *sk) {
    FILE *f = fopen(filename, "rb");
    if (!f) return 0;
    fread(sk, ED25519_SECRET_KEY_BYTES, 1, f);
    fclose(f);
    return 1;
}

// Usage
uint8_t sk[64];
if (load_key("secret.key", sk)) {
    uint8_t sig[64];
    ed25519_sign(message, len, sk, sig);
    ed25519_zeroize(sk, 64);
}
```

---

## Error Handling

```c
ed25519_status_t status = ed25519_sign(msg, len, sk, sig);

if (status == ED25519_SUCCESS) {
    printf("Signing succeeded\n");
} else if (status == ED25519_ERROR_INVALID_KEY) {
    printf("Invalid secret key\n");
} else if (status == ED25519_ERROR_BAD_INPUT) {
    printf("Invalid input parameters\n");
} else {
    printf("Unknown error\n");
}

// Signature verification
status = ed25519_verify(msg, len, pk, sig);
if (status == ED25519_SUCCESS) {
    printf("Signature verified\n");
} else if (status == ED25519_ERROR_INVALID_SIG) {
    printf("Signature is invalid\n");
} else if (status == ED25519_ERROR_BAD_INPUT) {
    printf("Invalid verification parameters\n");
}
```

---

## Compile & Link

### MSVC
```batch
:: Compile
cl /O2 /W4 ed25519_field.c ed25519_core.c ed25519_api.c myapp.c

:: Or create library
lib ed25519_*.obj /out:ed25519.lib
cl /O2 myapp.c ed25519.lib
```

### GCC/Clang
```bash
# Direct compilation
gcc -O2 -Wall ed25519_field.c ed25519_core.c ed25519_api.c myapp.c -o myapp

# Or create static library
gcc -c ed25519_field.c ed25519_core.c ed25519_api.c
ar rcs libed25519.a ed25519_*.o
gcc -O2 myapp.c -L. -led25519 -o myapp
```

---

## Performance

| Operation | Time (µs) | Security Level |
|-----------|-----------|----------------|
| Keygen    | 150-200   | 128-bit |
| Signing   | 100-150   | 128-bit |
| Verify    | 200-300   | 128-bit |

---

## Security Checklist

- [ ] Always zeroize secret keys after use
- [ ] Store private keys securely (encrypted at rest)
- [ ] Use constant-time comparison for sensitive values
- [ ] Validate message length before signing
- [ ] Never reuse the same message/key pair for different signers
- [ ] Maintain key rotation policy
- [ ] Test signatures are mathematically verified
- [ ] Monitor for timing attacks in integration
- [ ] Use PQCA entropy mixer when available

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Compilation fails | Ensure C99 mode enabled (`-std=c99` or `/TC`) |
| Verification fails | Check message length matches exactly |
| Random signatures | Verify ed25519_init() is called |
| Memory issues | Ensure buffers are correct size (32/64 bytes) |
| Slow performance | Use `-O2` or `-O3` optimization flags |

---

## Resources

- **RFC 8032:** https://datatracker.ietf.org/doc/html/rfc8032
- **FIPS 186-5:** https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-5.pdf
- **NIST SP 800-186:** https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-186.pdf

---

**QuantumSecure Technologies Ltd. | Production-Grade Cryptographic Engineering**


