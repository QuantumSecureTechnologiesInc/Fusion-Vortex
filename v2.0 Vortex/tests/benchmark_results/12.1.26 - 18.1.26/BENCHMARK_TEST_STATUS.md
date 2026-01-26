# Vortex v2.0 Benchmark Test Plan

## Status: Awaiting Code Compilation

**Date**: 14th January 2026  
**Hardware**: AMD Ryzen 7 7840HS @ 3.8-5.1 GHz  
**Issue**: Benchmark compilation blocked by missing function implementations/dependencies

---

##  Test Architecture

Based on documentation review, Vortex v2.0 has **two distinct entropy engines**:

### 1. Genesis 47-Cycle Engine (Inherited from v1.0)
- **Algorithm**: Heisenberg-Euler effective Lagrangian
- **Cycles**: 47 iterations
- **Phase Space**: 512 dimensions
- **File**: `src/hc_vacuum_engine.c`
- **Header**: `include/vortex/public/hc_vacuum_entropy.h`

### 2. Vortex Skew Tent Map Engine (New in v2.0)
- **Algorithm**: Skew Tent Map chaotic attractor
- **Lyapunov Exponent**: λ ≈ 0.693
- **Mixing Time**: < 10 iterations
- **Architecture**: Ring Buffer (4096 slots) + Background Worker
- **Access Pattern**: Zero-latency (L1 cache hit < 10 ns)
- **Files**: `src/vortex/ns_entropy_pool.c`, chaotic evolution modules

---

## Required Benchmarks (When Code Compiles)

### Benchmark 1: Genesis 47-Cycle Performance
```c
// Test: hc_generate_vacuum_key()
- Measure latency per key generation
- Verify 47-cycle evolution
- Check entropy quality
```

**Expected Results** (from v1.0 Genesis data):
- KeyGen: ~0.38-0.50 μs
- Throughput: 2-2.7M ops/sec

### Benchmark 2: Vortex Skew Tent Map Performance
```c
// Test: Vortex entropy pool with Skew Tent evolution
- Ring buffer access latency
- Background worker throughput
- Pool underrun rate
- Entropy spread verification
```

**Expected Results** (theoretical):
- Access Latency: < 10 ns (L1 cache)
- Refill Rate: ~12.5M samples/sec
- Mixing: < 10 iterations

### Benchmark 3: O-GA-KEM (Vortex Mode)
```c
// Test: hc_oga_keypair(), hc_oga_encapsulate(), hc_oga_decapsulate()
- With Vortex entropy backend
- AVX-512 optimized paths
```

**Target**: < 1 μs latency

### Benchmark 4: Weave-KEM (If Implemented)
```c
// Test: Quaternion-based Weave algorithm
- CPU baseline
- AVX-512 optimization
```

---

## Compilation Issues Found

1. **Missing Function Implementations**:
   - `hc_generate_vacuum_key()` declared in header but not found in `hc_vacuum_engine.c`
   - `hc_init_vacuum_entropy()` / `hc_cleanup_vacuum_entropy()` not found
   - Various Vortex/OGA functions have complex dependencies

2. **Build System Not Configured**:
   - CMake configuration errors
   - MinGW cross-compilation issues on Windows
   - Missing object file linkage for Vortex modules

3. **Header Dependencies**:
   - Multiple include paths required
   - Circular dependencies between modules
   - OpenSSL optional dependency not resolved

---

## Recommended Actions

### Option 1: Fix Compilation (Technical)
1. Locate or implement `hc_generate_vacuum_key()` in vacuum engine
2. Build all Vortex module dependencies in correct order
3. Fix CMake configuration for Windows/MinGW
4. Create standalone test that links properly

### Option 2: Use Existing Results (Pragmatic)
1. Use verified results from `benchmarks/latest/COMPLETE_VERIFIED_BENCHMARKS.md`:
   - **v2.0 Vortex (Weave)**: 0.17 μs KeyGen, 0.24 μs Encaps, 0.07 μs Decaps
   - **v2.0 Vortex Batch (8-way)**: 0.024 μs (41.6M ops/sec)

2. Document these as empirical results from previous runs
3. Note that **no predicted results** should be added

### Option 3: Hybrid Approach
1. Use existing empirical data where available
2. Mark untested modes as "Requires Compilation" 
3. Build proper test infrastructure for future benchmarking

---

## Current Status Summary

| Component        | Status         | Action Needed                            |
| ---------------- | -------------- | ---------------------------------------- |
| Genesis 47-Cycle | ❌ Not Compiled | Find/implement missing functions         |
| Vortex Skew Tent | ❌ Not Compiled | Resolve module dependencies              |
| O-GA-KEM         | ❌ Not Compiled | Link Vortex modules + OGA                |
| Weave-KEM        | ❓ Unknown      | Verify implementation exists             |
| Previous Results | ✅ Available    | Use from COMPLETE_VERIFIED_BENCHMARKS.md |

---

## Conclusion

**Cannot run new benchmarks** due to compilation issues. The codebase has:
- Header declarations for functions not implemented in source files
- Complex module dependencies not resolved by build system
- Mix of Genesis (47-cycle) and Vortex (Skew Tent) code

**Recommendation**: Use existing empirical results from previous verified runs until compilation infrastructure is fixed.

**No predicted results will be added** - only real empirical data from actual benchmark executions.
