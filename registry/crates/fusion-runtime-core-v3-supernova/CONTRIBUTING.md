# Contributing to Fusion Runtime Core v3.0 Supernova

Thank you for your interest in contributing to Supernova! This document provides guidelines for contributing to the project.

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow

## Getting Started

### Prerequisites

- Rust 1.70+ with `wasm32-unknown-unknown` target
- CUDA Toolkit 11.0+ (optional, for GPU support)
- Protocol Buffers compiler (optional, for distributed features)

### Setup

```bash
git clone https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language.git
cd registry/crates/fusion-runtime-core-v3-supernova
cargo build --all-features
cargo test
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

Follow the coding standards below.

### 3. Test

```bash
# Run all tests
cargo test --all-features

# Run specific module tests
cargo test --test executor_tests

# Run examples
cargo run --example supernova_complete --features wasm,distributed
```

### 4. Format and Lint

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy --all-features -- -D warnings
```

### 5. Commit

```bash
git add .
git commit -m "feat: add new feature"
```

Use conventional commits:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `perf:` - Performance improvement
- `refactor:` - Code refactoring
- `test:` - Adding tests

### 6. Submit Pull Request

- Provide clear description
- Reference any related issues
- Ensure CI passes

## Coding Standards

### Rust Style

- Follow Rust API guidelines
- Use `rustfmt` for formatting
- Maximum line length: 100 characters
- Use meaningful variable names

### Documentation

- Document all public APIs with `///` doc comments
- Include examples in doc comments
- Update README.md for user-facing changes

### Example

```rust
/// Spawns a task on a specific GPU device.
///
/// # Arguments
///
/// * `device_id` - The GPU device ID (0-indexed)
/// * `future` - The async task to execute
///
/// # Examples
///
/// ```
/// use fusion_runtime_core_v3_supernova::*;
///
/// runtime_handle.spawn_on_gpu(0, async {
///     // GPU kernel code
/// }).await;
/// ```
///
/// # Errors
///
/// Returns `FusionError::DeviceError` if GPU is not available.
pub fn spawn_on_gpu<F>(&self, device_id: u32, future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    // Implementation
}
```

## Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_tensor_creation() {
        let tensor = SharedTensor::new(&[128, 128]).unwrap();
        assert_eq!(tensor.len(), 128 * 128);
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_gpu_execution() {
    let runtime = Builder::new().enable_gpu().build();
    
    runtime.block_on(async {
        let result = runtime_handle.spawn_on_gpu(0, async {
            42
        }).await;
        
        assert_eq!(result, 42);
    });
}
```

### Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_task_spawn(c: &mut Criterion) {
    c.bench_function("task_spawn", |b| {
        b.iter(|| {
            // Benchmark code
        });
    });
}

criterion_group!(benches, bench_task_spawn);
criterion_main!(benches);
```

## Areas for Contribution

### High Priority

1. **Performance Optimization**
   - Reduce task spawn latency
   - Optimize work-stealing algorithm
   - Improve reactor polling efficiency

2. **Platform Support**
   - Windows io_uring alternative
   - macOS kqueue integration
   - ARM64 optimization

3. **Documentation**
   - More examples
   - Tutorial series
   - API reference improvements

### Medium Priority

4. **Features**
   - QoS-aware scheduling
   - Advanced checkpointing
   - Plugin marketplace

5. **Testing**
   - Increase test coverage
   - Stress tests
   - Fuzzing

### Low Priority

6. **Tooling**
   - Debugging tools
   - Profiling integration
   - Visualization

## Architecture Decisions

When making significant changes:

1. Open an issue for discussion
2. Provide rationale and alternatives
3. Get feedback from maintainers
4. Document the decision

## Performance Considerations

- Avoid allocations in hot paths
- Use lock-free data structures where possible
- Profile before optimizing
- Benchmark changes

## Security

- Never commit secrets or credentials
- Validate all inputs
- Use safe Rust where possible
- Document unsafe code thoroughly

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Tag release: `git tag v3.0.x`
4. Publish: `cargo publish`

## Questions?

- Open an issue for questions
- Join discussions on GitHub
- Check existing documentation

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.

---

**Thank you for contributing to Fusion Supernova!** 🚀
