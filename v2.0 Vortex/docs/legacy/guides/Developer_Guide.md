# Developer Guide: QST HyperCycle v1.1 Origin

<!-- doc-type: explanation | how-to -->
<!-- audience: developer -->
<!-- product: QST HyperCycle -->

**Version**: 1.0.0-Genesis  
**Last Updated**: 2026-01-05

---

## 1. The Philosophy

**Note**: We view code as literature. This project prioritises narrative clarity, intent, and human readability.

### The Mission

QST HyperCycle exists to provide **cryptographic superiority** in the post-quantum era. Built upon **HyperCycle v3.2 Fulminis** as its foundational cryptographic library, HyperCycle extends these battle-tested primitives with revolutionary performance optimizations.

Whilst NIST has standardised lattice-based schemes (ML-KEM, ML-DSA), HyperCycle pioneers the **Vacuum-Based Cryptography** framework (VBC). VBC utilises the 47-Cycle Vacuum Engine to generate entropy and keys at speeds physically impossible for purely algebraic approaches.

### Architectural Decisions

**Language Choice**: C was chosen for:
- **Performance**: Direct hardware access (AVX-512 intrinsics).
- **Portability**: Runs on x86, ARM, and RISC-V.
- **Interoperability**: Easy FFI terms.

**Core Patterns**:
- **Vacuum Kernel**: Physics simulation layer isolated from business logic.
- **Platform Abstraction**: OS memory locking (`VirtualLock`/`mlock`) for security.
- **SIMD Abstraction**: Runtime CPU detection (AVX-512 priority).

---

## 2. Setting the Stage (Environment Setup)

### The Toolchain

**Required:**
- **C Compiler**: GCC 11+, Clang 14+, or MSVC 2022+ (Required for AVX-512)
- **CMake**: v3.15+

### Bootstrapping

```bash
# Clone repository
git clone https://github.com/QuantumSecureTech/HyperCycle.git
cd HyperCycle/v1.0\ Genesis

# Configure with Vacuum Engine enabled
cmake -S . -B build \
    -DCMAKE_BUILD_TYPE=Release \
    -DENABLE_HYPERCYCLE=ON \
    -DENABLE_AVX512=ON

# Build
cmake --build build -j$(nproc)

# Run tests
cd build && ctest --output-on-failure
```

---

## 3. Core Concepts & Internals

### Directory Structure

```
QST HyperCycle v1.1 Origin/
├── src/
│   ├── hypercycle_core.c        # Main engine lifecycle
│   ├── hc_vacuum.c              # 47-Cycle Vacuum Engine
│   ├── hypercycle_ultra_optimizer.c # AVX-512 Logic
│   └── hc_secure_enclave.c      # Secure memory wrapper
├── include/
│   ├── public/
│   │   ├── hypercycle_algorithms.h  # Public API
│   │   └── hypercycle.h         # Core definitions
├── tests/
│   └── benchmarks/              # Performance suite
└── docs/                        # Documentation
```

### The Lifecycle of a Key Exchange

**1. Initialization** by `hc_initialize()`:
- Validates Vacuum Engine entropy source.
- Detects AVX-512 support.

**2. Key Generation** by `hypercycle_keygen()`:
- **Vacuum Phase**: Simulates 47 cycles of Heisenberg-Euler fluctuations via the Engine.
- **Extraction Phase**: Converts quantum states to 256-byte public key.
- **Speed**: **~42 CPU Cycles** (HyperCycle Engine with AVX-512).

---

## 4. Contribution Workflow

### Style Guidelines
We follow strict "Code as Literature" standards.

**Naming**:
- **Good**: `vacuum_state_evolve()`
- **Bad**: `vac_upd()`

**Commit Messages**:
```
feat(vacuum): optimise entropy extraction loop

Replaced scalar loop with AVX-512 permute, reducing cycle count from 52 to 38.
```

### Constant-Time Verification
All critical paths must be constant-time.
```bash
valgrind --tool=ctgrind --ct-check=all ./build/tests/unit/test_vacuum
```

---

## 5. Advanced Topics

### Fuzzing
We use AFL++ for fuzzing the Vacuum input interfaces.
```bash
afl-fuzz -i testcases/vacuum -o findings ./build/tests/fuzz/fuzz_vacuum
```

---

## 6. Release Process

### Version Numbering
- **Major**: Architecture changes (e.g., Vacuum -> Hybrid).
- **Minor**: New physics models.
- **Patch**: Bug fixes.

---

**Copyright © 2026 Quantum Secure Technologies Ltd.**


