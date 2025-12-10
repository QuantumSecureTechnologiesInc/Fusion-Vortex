# Contributing to Fusion Runtime Core

Thank you for your interest in contributing to Fusion Runtime Core! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please be respectful and professional in all interactions.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [GitHub Issues](https://github.com/QuantumSecureTechnologiesInc/Fusion/issues)
2. If not, create a new issue with:
   - Clear, descriptive title
   - Steps to reproduce
   - Expected vs. actual behaviour
   - Environment details (OS, Rust version, GPU, etc.)
   - Code sample (if applicable)

### Suggesting Features

1. Open a GitHub Issue with the "feature request" label
2. Describe the feature and its use case
3. Explain how it aligns with Fusion's goals
4. Discuss implementation approach (if applicable)

### Pull Requests

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**:
   - Follow Rust conventions
   - Add tests for new functionality
   - Update documentation
4. **Test your changes**:
   ```bash
   cargo test --workspace
   cargo clippy --workspace -- -D warnings
   cargo fmt --all --check
   ```
5. **Commit with clear messages**:
   ```bash
   git commit -m "feat: Add amazing feature"
   ```
6. **Push to your fork**: `git push origin feature/amazing-feature`
7. **Open a Pull Request**

## Development Setup

### Prerequisites

- Rust 1.75+ (`rustup update`)
- LLVM 17+ (for quantum compilation)
- Optional: CUDA 12.0+, Metal SDK

### Building from Source

```bash
git clone https://github.com/QuantumSecureTechnologiesInc/Fusion.git
cd Fusion
cargo build --workspace --release
```

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test --package fusion_runtime_core

# With output
cargo test --workspace -- --nocapture
```

### Running Benchmarks

```bash
# All benchmarks
cargo bench --workspace

# Specific benchmark
cargo bench --package fusion_runtime_core
```

## Code Style

### Rust Conventions

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Maximum line length: 100 characters

### Naming Conventions

- **Types**: `PascalCase`
- **Functions**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

### Documentation

- All public APIs must have documentation comments (`///`)
- Include examples in documentation
- Use British English

**Example**:

```rust
/// Allocates memory for a specific device.
///
/// # Arguments
///
/// * `size` - Size in bytes
/// * `device` - Target device type
///
/// # Examples
///
/// ```
/// use fusion_runtime_mem_mgr::{MemoryManager, DeviceType};
///
/// let mem_mgr = MemoryManager::new(&config);
/// let allocation = mem_mgr.allocate(1024, DeviceType::Cpu);
/// ```
pub fn allocate(&self, size: usize, device: DeviceType) -> DeviceMemory {
    // Implementation
}
```

## Testing Guidelines

### Unit Tests

- Test individual components in isolation
- Use `#[cfg(test)]` modules
- Name tests descriptively: `test_feature_behavior`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_allocation() {
        let mem_mgr = MemoryManager::new(&config);
        let allocation = mem_mgr.allocate(1024, DeviceType::Cpu);
        assert_eq!(allocation.size, 1024);
    }
}
```

### Integration Tests

- Test crate interactions
- Place in `tests/` directory
- Use realistic scenarios

### Async Tests

```rust
#[tokio::test]
async fn test_task_spawn() {
    let runtime = Runtime::new();
    let handle = runtime.spawn(async { 42 });
    let result = handle.await;
    assert_eq!(result, Ok(42));
}
```

## Commit Message Format

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```text
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples**:

```text
feat(scheduler): Add QoS-aware task prioritization

Implemented three-tier priority queue system for
heterogeneous scheduling across CPU/GPU/QPU devices.

Closes #123
```

```text
fix(memory): Prevent buffer pool exhaustion

Added bounds checking to buddy allocator to prevent
allocation failures when pool is near capacity.

Fixes #456
```

## Pull Request Checklist

Before submitting a PR, ensure:

- [ ] Code builds without errors (`cargo build --workspace`)
- [ ] All tests pass (`cargo test --workspace`)
- [ ] No clippy warnings (`cargo clippy --workspace -- -D warnings`)
- [ ] Code is formatted (`cargo fmt --all`)
- [ ] Documentation is updated
- [ ] Changelog is updated (for significant changes)
- [ ] Commit messages follow conventions
- [ ] PR description explains changes clearly

## Documentation Standards

### Code Documentation

- All `pub` items must have doc comments
- Include examples where applicable
- Document panics, errors, and safety requirements

### Markdown Documentation

- Use GitHub-flavoured Markdown
- Specify language for code blocks: ` ```rust `
- British English spelling
- Line length: soft limit 100 characters

### Documentation Structure

```text
docs/
├── guides/          # User-facing guides
├── design/          # Architecture documents
├── references/      # API references
└── roadmap/         # Planning documents
```

## Release Process

1. Update version in `Cargo.toml` files
2. Update `CHANGELOG.md`
3. Create git tag: `git tag v0.x.0`
4. Push tag: `git push --tags`
5. Publish to crates.io: `cargo publish --workspace`

## License

By contributing, you agree that your contributions will be licensed under the project's MIT OR Apache-2.0 dual licence.

## Questions?

- **GitHub Discussions**: [Fusion Discussions](https://github.com/QuantumSecureTechnologiesInc/Fusion/discussions)
- **Discord**: [discord.gg/fusion-lang](https://discord.gg/fusion-lang)
- **Email**: dev@quantumsecuretech.com

---

**Thank you for contributing to Fusion Runtime Core!** 🚀
