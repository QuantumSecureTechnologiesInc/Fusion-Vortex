# QST NeuralSeal PQ Crypto Library - External Audit Environment

## Overview

This directory contains everything needed for external security auditors and cryptographers to review the QST NeuralSeal Post-Quantum Cryptography Library.

## Contents

### 1. Source Code
- `src/` - All implementation files
- `include/` - Public API headers  
- `impl/` - Internal implementation helpers

### 2 Documentation
- `docs/` - Technical whitepapers, research papers, compliance docs
- `README.md` - Comprehensive usage guide
- `SECURITY.md` - Security policy and vulnerability reporting
- `LICENSE` - Copyright and licensing terms

### 3. Test Infrastructure
- `tests/` - Test suite with Known Answer Tests (KATs)
- `tests/comprehensive_test.c` - Full feature test suite
-  `tools/lint_ct.py` - Constant-time linter

### 4. Build System
- `CMakeLists.txt` - Cross-platform build configuration  
- `build_linux.sh`, `build_mac.sh`, `build_windows.bat` - Platform scripts

## Quick Audit Checklist

### Security Review Focus Areas

1. **Cryptographic Correctness**
   - [ ] Quaternion arithmetic implementation (`src/cemqc_arithmet ic.c`)
   - [ ] SHAKE256 implementation (`src/cemqc_shake.c`)
   - [ ] Chaos generator security (`src/cemqc_chaos.c`)
   - [ ] KEM correctness (`src/cemqc_kem_core.c`)
   - [ ] Trapdoor security (`src/cemqc_trapdoor.c`)

2. **Side-Channel Resistance**
   - [ ] Run constant-time linter: `python tools/lint_ct.py --paths src include`
   - [ ] Review all conditional branches in crypto code
   - [ ] Verify memory access patterns are data-independent
   - [ ] Check for timing variations in key operations

3. **Memory Safety**
   - [ ] Secure memory zeroing (`src/cemqc_arithmetic.c`, `impl/cemqc_zeroize.c`)
   - [ ] No use-after-free vulnerabilities
   - [ ] Buffer overflow protection
   - [ ] Stack protection enabled

4. **Error Handling**
   - [ ] All API functions validate inputs
   - [ ] Error codes are comprehensive (`include/cemqc_errors.h`)
   - [ ]No information leakage through errors

5. **Cryptographic Assumptions**
   - [ ] Review CEMQC hard problem definition
   - [ ] Validate quaternion security claims
   - [ ] Assess chaos mixing contribution
   - [ ] Verify domain separation implementation

6. **Key Management**
   - [ ] Key generation randomness quality
   - [ ] Key storage recommendations
   - [ ] Key lifecycle management

7. **Implementation Quality**
   - [ ] Code follows secure coding standards
   - [ ] No undefined behavior
   - [ ] Compiler warnings addressed
   - [ ] Thread safety considered

## Building for Audit

### Linux/macOS
```bash
# Debug build with all diagnostics
mkdir audit-build && cd audit-build
cmake .. -DCMAKE_BUILD_TYPE=Debug \
         -DCEMQC_SELFTEST=ON \
         -DCMAKE_C_FLAGS="-Wall -Wextra -Werror -fsanitize=address,undefined"
make -j$(nproc)
./cemqc_selftest_runner
```

### Windows
```cmd
mkdir audit-build && cd audit-build
cmake .. -G "NMake Makefiles" -DCMAKE_BUILD_TYPE=Debug -DCEMQC_SELFTEST=ON
nmake
cemqc_selftest_runner.exe
```

## Running Tests

### Self-Tests
```bash
./cemqc_selftest_runner
```

### Comprehensive Tests
```bash
gcc -std=c11 -I include tests/comprehensive_test.c libcemqc_v2_enhanced.a -o comprehensive_test
./comprehensive_test
```

### Known Answer Tests (KATs)
```bash
cd tests
QST_LIB_PATH=../build/libcemqc_core.a python3 run_kat.py --kat kat/kat_vectors_native_L1.json
```

### Constant-Time Verification
```bash
python tools/lint_ct.py --paths src include
```

## Audit Tools Recommended

1. **Static Analysis**
   - Coverity Scan
   - Clang Static Analyzer
   - Infer
   - CodeQL

2. **Dynamic Analysis**
   - Valgrind (memory errors)
   - AddressSanitizer (memory safety)
   - UndefinedBehaviorSanitizer
   - ThreadSanitizer (concurrency)

3. **Fuzzing**
   - AFL++
   - LibFuzzer  
   - Honggfuzz

4. **Side-Channel Analysis**
   - dudect (constant-time testing)
   - ctgrind  
   - valgrind-based timing analysis
   - Power analysis (if hardware available)

## Key Files for Review

### Critical Security Files
| File | Purpose | Priority |
|------|---------|----------|
| `src/cemqc_kem_core.c` | KEM implementation | CRITICAL |
| `src/cemqc_trapdoor.c` | Trapdoor function | CRITICAL |
| `src/cemqc_chaos.c` | Chaos generator | HIGH |
| `src/cemqc_shake.c` | SHAKE256 hash | HIGH |
| `src/cemqc_arithmetic.c` | Secure memory | HIGH |
| `impl/cemqc_zeroize.c` | Memory zeroing | MEDIUM |
| `Include/cemqc_internal.h` | Crypto constants | MEDIUM |

### Documentation for Context
| File | Purpose |
|------|---------|
| `docs/Technical_Whitepapers/` | Cryptographic design |
| `docs/Patent_Filing_Documents/` | Novel techniques |
| `SECURITY.md` | Security policy |
| `CEMQC_Novelty_Anchors_*.txt` | Innovation claims |

## Known Considerations

### Design Decisions
1. **Chaos Mixing** - Deterministic but adds entropy to SHAKE256 output
2. **Dual Modes** - Native (with chaos) and FIPS (SHAKE-only)
3. **Small Key Sizes** - 64-byte public/secret keys (verify security level)
4. **Domain Separation** - Prevents cross-protocol attacks

### Potential Review Points
1. Is the quaternion arithmetic cryptographically sound?
2. Does chaos mixing provide security benefit or just complexity?
3. Are 64-byte keys sufficient for post-quantum security?
4. Is the trapdoor function properly one-way?
5. Are there any side-channel vulnerabilities?

## Testing Profiles

### Profile: Native (Default)
- Chaos mixing: ENABLED
- Randomness: SHAKE256 + Chaos
- Use case: General deployment

### Profile: FIPS
- Chaos mixing: DISABLED
- Randomness: SHAKE256 only
- Use case: Compliance-focused
- Build: `cmake .. -DCEMQC_FIPS_MODE=ON`

## Reporting Findings

### Severity Classification
- **Critical**: Key recovery, RCE, authentication bypass
- **High**: Information disclosure, DoS, side-channel attacks
- **Medium**: Implementation weaknesses, best practice violations
- **Low**: Code quality, documentation issues

### Report Template
```
**Title**: [Concise description]
**Severity**: Critical | High | Medium | Low
**Component**: [File/module affected]
**Description**: [Detailed finding]
**Impact**: [Security consequence]
**Proof of Concept**: [Reproduction steps/code]
**Recommendation**: [How to fix]
**References**: [Related CVEs, papers, etc.]
```

### Submission
- Email: security@qst-neuralseal.com
- PGP: [Key to be published]
- Response SLA: 48 hours acknowledgment

## Audit Timeline Suggestion

| Phase | Duration | Activities |
|-------|----------|------------|
| **Week 1** | 5 days | Code review, static analysis |
| **Week 2** | 5 days | Dynamic testing, fuzzing |
| **Week 3** | 3 days | Side-channel analysis |
| **Week 4** | 2 days | Report writing |

**Total**: ~15 working days for comprehensive audit

## Contact & Support

- **Technical Questions**: support@qst-neuralseal.com
- **Security Team**: security@qst-neuralseal.com
- **Audit Coordination**: audit@qst-neuralseal.com

## License

See `LICENSE` file for copyright and usage terms.

---

**Audit Environment Version**: 2.0  
**Last Updated**: 2025-11-29  
**Maintainer**: QST NeuralSeal Security Team
