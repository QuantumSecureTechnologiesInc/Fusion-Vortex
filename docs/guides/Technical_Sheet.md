# Fusion Technical Sheet

## System Requirements

### Supported Platforms

- **Linux**: kernel 5.4+, glibc 2.31+ (x86-64, ARM64)
- **macOS**: 12+ (Intel, Apple Silicon)
- **Windows**: 10+, Server 2019+ (x86-64)

### Hardware Requirements (Minimum)

- **CPU**: 2 cores, 2GHz+
- **RAM**: 4GB
- **Storage**: 500MB for toolchain

### Hardware Requirements (Recommended for AI/Quantum)

- **CPU**: 8+ cores, AVX2/AVX-512 support
- **RAM**: 16GB+
- **GPU**: NVIDIA (CUDA 11+) or AMD (ROCm 5+)

## Compiler Specifications

- **Backend**: LLVM 16
- **Parser**: ANTLR4
- **Linking**: LLD (LLVM Linker)
- **Binary Format**: ELF, Mach-O, PE/COFF, Wasm

## Cryptographic Standards

- **Hash**: SHA-3, SHAKE256
- **Symmetric**: AES-256-GCM, ChaCha20-Poly1305
- **Asymmetric (Classical)**: X25519, Ed25519, P-256
- **Asymmetric (PQC)**: ML-KEM (Kyber), ML-DSA (Dilithium), SPHINCS+