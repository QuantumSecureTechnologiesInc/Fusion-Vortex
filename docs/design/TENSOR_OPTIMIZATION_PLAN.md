# Tensor Engine Optimization Plan

**Objective**: Upgrade `fusion_lang::ml::tensor` from a reference implementation to a high-performance inference engine.

## 1. Computational Performance (Speed)

### A. Parallel Multi-threading

**Current**: Single-threaded loops.
**Target**: Use `rayon` to parallelize data-independent operations.
**Implementation**:

```rust
// Matmul outer loop parallelization
(0..m).into_par_iter().for_each(|i| { ... });
```text

**Expected Gain**: Linear scaling with CPU cores (e.g., 10x on 12-core machine).

### B. Cache Tiling (Blocking)

**Current**: Row-major iteration causes frequent cache misses on the second matrix.
**Target**: Implement Blocked Matrix Multiplication.
**Strategy**: Divide matrices into small sub-blocks ($32 \times 32$) that fit entirely in L1 Cache.
**Expected Gain**: 2-5x reduction in memory bandwidth usage.

### C. SIMD (Vectorization)

**Current**: Scalar `f32` operations.
**Target**: Use `vector` intrinsics or `autovectorize` hints to process 8 floats per cycle (AVX2).
**Expected Gain**: 4-8x FLOPs increase.

## 2. Memory Capability (Efficiency)

### A. Automatic Broadcasting

**Current**: Strict shape matching required; manual looping for bias addition.
**Target**: NumPy-style broadcasting rules.
1. Match dimensions starting from the trailing axis.
2. If dimensions differ, one must be 1 (and is virtually repeated).
**Impact**: Enables complex operations (e.g., Attention masks, Bias add) without memory copying.

### B. Zero-Copy Views

**Current**: `transpose()` clones the entire data vector.
**Target**: Split `Tensor` into `TensorView` + `Storage`.
* `Storage`: `Rc<Vec<f32>>` (Owned data)
* `Tensor`: Holds `Storage`, `offset`, `shape`, `strides`.
**Impact**: `transpose`, `slice`, `reshape` become instant $O(1)$ operations options.

## 3. Hardware Backend

**Current**: Placeholder `Device::CUDA`.
**Target**: `wgpu` Compute Shaders.
**Strategy**: Write generic WGSL kernels for `matmul` and `elementwise`.
**Impact**: Enable massive parallelism on Consumer GPUs.