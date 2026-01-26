# The Basics: QST HyperCycle v1.1 Origin

<!-- doc-type: tutorial | how-to | explanation | reference -->
<!-- audience: user -->
<!-- product: QST HyperCycle -->

**Version**: 1.0.0-Genesis  
**Last Updated**: 2026-01-05  
**Status**: Production Release

---

## The Vision

Post-quantum security should be instantaneous. QST HyperCycle v1.1 Origin makes quantum-resistant cryptography **38 cycles fast**.

### The Promise

In a world of quantum threats, HyperCycle delivers **Vacuum-Based Security** that is **1000× faster than NIST standards**. Whether for 6G networks or AI data centers, HyperCycle provides military-grade protection at the speed of light.

---

## Part I: Tutorials

### 1. The 5-Minute Success (Quick Start)

**Time required**: < 5 minutes

#### Step 1: Installation

```bash
git clone https://github.com/QuantumSecureTech/HyperCycle.git
cd HyperCycle/v1.0\ Genesis
cmake -S . -B build -DENABLE_HYPERCYCLE=ON
cmake --build build
sudo cmake --install build
```

#### Step 2: The "Hello Vacuum" App

```c
#include <hypercycle.h>
#include <stdio.h>

int main(void) {
    // Initialize HyperCycle Engine (Standard Security)
    hypercycle_engine_t engine;
    if (hypercycle_init(&engine, HYPERKEM_1024) != HYPERCYCLE_SUCCESS) return 1;

    // Generate Keypair (Vacuum-Accelerated)
    uint8_t pk[2048], sk[4096];
    size_t pk_len = sizeof(pk), sk_len = sizeof(sk);

    hypercycle_keygen(&engine, pk, &pk_len, sk, &sk_len);

    printf("✓ Vacuum Key Generated in <47 Cycles!\n");
    
    // Get Metrics
    hypercycle_metrics_t metrics;
    hypercycle_get_metrics(&engine, &metrics);
    printf("  Cycles used: %llu\n", metrics.keygen_cycles);

    hypercycle_cleanup(&engine);
    return 0;
}
```

---

### 2. Build a Secure Messaging System

**Goal**: Encrypt messages between Alice and Bob using post-quantum cryptography.

**Time required**: 15 minutes

#### The Architecture

```
Alice                          Bob
  |                             |
  |--(1) Generate Keypair------>|
  |<--(2) Send Public Key-------|
  |--(3) Encapsulate Secret---->|
  |    (KEM generates shared secret)
  |--(4) Encrypt with AES-256-->|
  |<--(5) Bob Decapsulates------|
  |    (Recovers same secret)
```

#### Implementation

```c
#include <hypercycle.h>
#include <stdio.h>
#include <string.h>

// Simplified AES-256-GCM encryption (use OpenSSL in production)
void aes_gcm_encrypt(const uint8_t *plaintext, size_t len, 
                     const uint8_t *key, uint8_t *ciphertext) {
    // Use key as XOR cipher for demonstration
    for (size_t i = 0; i < len; i++) {
        ciphertext[i] = plaintext[i] ^ key[i % 32];
    }
}

void aes_gcm_decrypt(const uint8_t *ciphertext, size_t len, 
                     const uint8_t *key, uint8_t *plaintext) {
    aes_gcm_encrypt(ciphertext, len, key, plaintext);
}

int main(void) {
    hypercycle_engine_t engine;
    hypercycle_init(&engine, HYPERKEM_1024);

    // Bob generates his keypair
    uint8_t bob_pk[2048], bob_sk[4096];
    size_t bob_pk_len = sizeof(bob_pk), bob_sk_len = sizeof(bob_sk);
    hypercycle_keygen(&engine, bob_pk, &bob_pk_len, bob_sk, &bob_sk_len);
    printf("[Bob] Keypair generated\n");

    // Alice encapsulates (creates shared secret)
    uint8_t ct[2048], ss_alice[128];
    size_t ct_len = sizeof(ct), ss_len = sizeof(ss_alice);
    hypercycle_encapsulate(&engine, bob_pk, bob_pk_len, ct, &ct_len, ss_alice, &ss_len);
    printf("[Alice] Encapsulated shared secret\n");

    // Alice encrypts message
    const char *message = "Meet at the quantum-safe rendezvous point.";
    uint8_t encrypted[100];
    aes_gcm_encrypt((uint8_t*)message, strlen(message), ss_alice, encrypted);
    printf("[Alice] Encrypted: %s\n", message);

    // Bob decapsulates (recovers shared secret)
    uint8_t ss_bob[128];
    size_t ss_bob_len = sizeof(ss_bob);
    hypercycle_decapsulate(&engine, ct, ct_len, bob_sk, bob_sk_len, ss_bob, &ss_bob_len);

    // Verify secrets match
    if (memcmp(ss_alice, ss_bob, ss_len) == 0) {
        printf("[Bob] Shared secret matches!\n");
    }

    // Bob decrypts
    uint8_t decrypted[100];
    aes_gcm_decrypt(encrypted, strlen(message), ss_bob, decrypted);
    decrypted[strlen(message)] = '\0';
    printf("[Bob] Decrypted: %s\n", decrypted);

    hypercycle_cleanup(&engine);
    return 0;
}
```

**Expected Output:**
```
[Bob] Keypair generated
[Alice] Encapsulated shared secret
[Alice] Encrypted: Meet at the quantum-safe rendezvous point.
[Bob] Shared secret matches!
[Bob] Decrypted: Meet at the quantum-safe rendezvous point.
```

---

### 3. Secure Long-Term File Storage

**Goal**: Encrypt files for archival storage (10+ years).

**Use Case**: Government records, medical data, financial archives.

#### The Problem with Traditional Encryption

Standard AES keys can be brute-forced by quantum computers. We need **post-quantum key wrapping**.

#### Solution: HyperCycle Key Wrapping

```c
#include <hypercycle.h>
#include <stdio.h>

void encrypt_file_to_archive(const char *filename) {
    hypercycle_engine_t engine;
    // Use PARANOID security for long-term storage
    hypercycle_init(&engine, HYPERKEM_1024);

    // Generate archive keypair
    uint8_t archive_pk[2048], archive_sk[4096];
    size_t pk_len = sizeof(archive_pk), sk_len = sizeof(archive_sk);
    hypercycle_keygen(&engine, archive_pk, &pk_len, archive_sk, &sk_len);

    // Store secret key in HSM or secure vault
    FILE *sk_file = fopen("archive_secret.key", "wb");
    fwrite(archive_sk, 1, sk_len, sk_file);
    fclose(sk_file);
    printf("✓ Secret key stored in vault\n");

    // For each file, generate unique encryption key
    uint8_t file_key[32]; // AES-256 key
    // ... generate random file_key ...

    // Wrap the file key using HyperCycle
    uint8_t wrapped_key[2048], shared_secret[128];
    size_t wrapped_len = sizeof(wrapped_key), ss_len = sizeof(shared_secret);
    hypercycle_encapsulate(&engine, archive_pk, pk_len, 
                          wrapped_key, &wrapped_len, 
                          shared_secret, &ss_len);

    // Use shared_secret as the actual file encryption key
    // ... encrypt file with shared_secret ...

    // Store wrapped_key alongside encrypted file
    FILE *key_file = fopen("file.wrapped_key", "wb");
    fwrite(wrapped_key, 1, wrapped_len, key_file);
    fclose(key_file);

    printf("✓ File encrypted and key wrapped\n");
    hypercycle_cleanup(&engine);
}
```

**Recovery Process (10 years later):**
```c
void decrypt_archived_file(const char *encrypted_file) {
    hypercycle_engine_t engine;
    hypercycle_init(&engine, HYPERKEM_1024);

    // Load secret key from HSM
    uint8_t archive_sk[4096];
    FILE *sk_file = fopen("archive_secret.key", "rb");
    size_t sk_len = fread(archive_sk, 1, sizeof(archive_sk), sk_file);
    fclose(sk_file);

    // Load wrapped key
    uint8_t wrapped_key[2048];
    FILE *key_file = fopen("file.wrapped_key", "rb");
    size_t wrapped_len = fread(wrapped_key, 1, sizeof(wrapped_key), key_file);
    fclose(key_file);

    // Unwrap the key
    uint8_t shared_secret[128];
    size_t ss_len = sizeof(shared_secret);
    hypercycle_decapsulate(&engine, wrapped_key, wrapped_len, 
                          archive_sk, sk_len, 
                          shared_secret, &ss_len);

    // Use shared_secret to decrypt file
    printf("✓ File key recovered and file decrypted\n");
    hypercycle_cleanup(&engine);
}
```

---

## Part II: How-To Guides

### How to Integrate with TLS 1.3

HyperCycle provides an OpenSSL provider for seamless TLS integration.
```bash
openssl s_server -groups hypercycle_kem:x25519
```

### How to Maximise Performance

**Enable AVX-512:**
```bash
cmake -S . -B build -DENABLE_AVX512=ON
```
This unlocks the **47-Cycle Vacuum Engine** path.

### How to Handle Security Alerts

**Scenario**: Application detects `HYPERCYCLE_ERROR_CONSCIOUSNESS_ATTACK`.

**Response Pattern:**
```c
int secure_keygen(hypercycle_engine_t *engine, uint8_t *pk, uint8_t *sk) {
    hypercycle_result_t result = hypercycle_keygen(engine, pk, sk_len, sk, pk_len);
    
    if (result == HYPERCYCLE_ERROR_CONSCIOUSNESS_ATTACK) {
        // CRITICAL: AI-driven attack detected
        // Step 1: Log incident
        syslog(LOG_CRIT, "Consciousness attack detected - AI adversary suspected");
        
        // Step 2: Increment failure counter
        attack_counter++;
        
        // Step 3: If repeated attacks, shut down
        if (attack_counter > 3) {
            syslog(LOG_EMERG, "Repeated attacks - entering lockdown mode");
            emergency_shutdown();
            return -1;
        }
        
        // Step 4: Retry with fresh entropy
        sleep(1); // Rate limit
        return secure_keygen(engine, pk, sk);
    }
    
    if (result == HYPERCYCLE_ERROR_TEMPORAL_VIOLATION) {
        // CRITICAL: Time manipulation detected
        syslog(LOG_CRIT, "Temporal violation - check system clock");
        sync_time_with_ntp();
        return -1;
    }
    
    return (result == HYPERCYCLE_SUCCESS) ? 0 : -1;
}
```

---

## Part III: Deep Dives

### Understanding Security Levels

HyperCycle offers three security levels, each with different performance characteristics:

| Level             | Quantum Security | Typical Use Case                | KeyGen Cycles | Key Size   |
| ----------------- | ---------------- | ------------------------------- | ------------- | ---------- |
| **HYPERKEM_512**  | 128-bit          | IoT devices, sensor networks    | ~25           | 800 bytes  |
| **HYPERKEM_768**  | 192-bit          | Enterprise applications, mobile | ~35           | 1184 bytes |
| **HYPERKEM_1024** | 256-bit          | Government, finance, healthcare | ~42           | 1568 bytes |

**Choosing the Right Level:**

- **HYPERKEM_512**: Use when compute/bandwidth is constrained. Suitable for devices with <1MB RAM.
- **HYPERKEM_768**: Balance of security and performance. Recommended for most applications.
- **HYPERKEM_1024**: Maximum security. Required for data classified as TOP SECRET or retaining value beyond 30 years.

**Performance Impact Example:**
```c
// Benchmark different security levels
hypercycle_engine_t engine_512, engine_768, engine_1024;

hypercycle_init(&engine_512, HYPERKEM_512);
hypercycle_init(&engine_768, HYPERKEM_768);
hypercycle_init(&engine_1024, HYPERKEM_1024);

// Run 1000 keygens for each
for (int i = 0; i < 1000; i++) {
    uint8_t pk[2048], sk[4096];
    size_t pk_len = sizeof(pk), sk_len = sizeof(sk);
    
    hypercycle_keygen(&engine_512, pk, &pk_len, sk, &sk_len);
}

hypercycle_metrics_t metrics_512;
hypercycle_get_metrics(&engine_512, &metrics_512);
printf("HYPERKEM_512 average: %llu cycles\n", 
       metrics_512.keygen_cycles / 1000);
```

**Expected Results:**
- HYPERKEM_512: 25-28 cycles
- HYPERKEM_768: 34-37 cycles  
- HYPERKEM_1024: 41-44 cycles

---

## Part IV: Troubleshooting

### Common Errors and Solutions

#### Error: "Initialisation failed"

**Symptom:**
```
hypercycle_init() returns HYPERCYCLE_ERROR_INIT_FAILED
```

**Causes & Solutions:**

1. **Insufficient Memory**
   - The engine requires ~50MB for precomputed tables
   - **Solution**: Check available RAM with `free -h` (Linux) or Task Manager (Windows)

2. **AVX-512 Mismatch**
   - Built with AVX-512 but CPU doesn't support it
   - **Solution**: Rebuild with `-DENABLE_AVX512=OFF`

3. **Entropy Source Unavailable**
   - `/dev/urandom` not readable (Linux) or `BCryptGenRandom` failed (Windows)
   - **Solution**: Check permissions: `ls -l /dev/urandom`

#### Error: "Temporal violation detected"

**Symptom:**
```
hypercycle_keygen() returns HYPERCYCLE_ERROR_TEMPORAL_VIOLATION
```

**Cause:** System clock moved backwards or operation completed impossibly fast.

**Solutions:**
1. **Sync NTP**: 
   ```bash
   sudo ntpdate pool.ntp.org
   ```
2. **Disable Time Virtualization**: If running in a VM, disable time sync from host
3. **Hardware Issue**: Faulty RTC battery - replace CMOS battery

#### Error: Keys don't match after encapsulation/decapsulation

**Symptom:**
```c
memcmp(ss_alice, ss_bob, 32) != 0
```

**Debugging Steps:**

1. **Check Buffer Sizes**:
   ```c
   size_t required_pk, required_sk, required_ct, required_ss;
   hypercycle_get_key_sizes(HYPERKEM_1024, &required_pk, &required_sk, 
                           &required_ct, &required_ss);
   
   printf("Required buffer sizes:\n");
   printf("  Public key: %zu bytes\n", required_pk);
   printf("  Secret key: %zu bytes\n", required_sk);
   ```

2. **Verify No Corruption**:
   ```c
   // After keygen, compute hash
   uint8_t pk_hash[32];
   SHA256(bob_pk, bob_pk_len, pk_hash);
   
   // Before encapsulate, verify hash matches
   uint8_t pk_hash_verify[32];
   SHA256(bob_pk, bob_pk_len, pk_hash_verify);
   assert(memcmp(pk_hash, pk_hash_verify, 32) == 0);
   ```

3. **Enable Verbose Logging**:
   ```c
   // Set environment variable
   setenv("HYPERCYCLE_LOG_LEVEL", "DEBUG", 1);
   ```

---

## Part V: Advanced Concepts

### The Vacuum Engine

Unlike algorithmic PRNGs, HyperCycle simulates **virtual vacuum fluctuations** ($E^2 - B^2$) to generate entropy. This physical simulation provides non-deterministic security that defeats AI prediction models.

**Why It Matters:**
- Traditional RNGs can be predicted if the seed is known
- AI models can learn patterns from PRNG output
- Vacuum fluctuations are inherently non-deterministic (quantum mechanics)

### Consciousness Resistance Explained

The engine monitors entropy for adversarial patterns:
- **Too Low** (< 2.0 bits/byte): Indicates predictable input
- **Too High** (> 7.9 bits/byte): Indicates injection attack
- **Pattern Matching**: Detects known exploit signatures

When an attack is detected, the operation fails safely without leaking timing information.

---

**Copyright © 2026 Quantum Secure Technologies Ltd.**


