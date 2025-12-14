# Contributing to Fusion

Thank you for your interest in contributing to the Fusion Programming Language! This document provides guidelines and information for contributors.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Create a branch** for your changes
4. **Make your changes** with clear commit messages
5. **Test thoroughly** - ensure all tests pass
6. **Submit a pull request**

## Development Setup

### Prerequisites

- Rust 1.70+ with Cargo
- LLVM 14+ (for native compilation)
- Node.js 18+ (for VS Code extension development)
- Git

### Building

```

# Clone the repository

git clone https://github.com/your-org/fusion-lang
cd fusion-lang

# Build the compiler

cargo build --release

# Run tests

cargo test

# Build VS Code extension

cd editors/vscode-fusion
npm install
npm run compile
```

## Contribution Areas

We welcome contributions in these areas:

### High Priority

- **Standard Library Expansion**
  - File I/O operations
  - Networking (TCP/UDP)
  - JSON parsing
  - Regular expressions

- **Collections Library**
  - Complete runtime integration
  - Performance optimizations
  - Additional collection types

- **IDE Features**
  - Enhanced symbol navigation
  - Refactoring support
  - Code actions
  - Debugging integration

### Medium Priority

- **Package Manager**
  - Dependency resolution
  - Package registry integration
  - Build system enhancements

- **Optimizations**
  - LLVM optimization passes
  - Dead code elimination
  - Constant folding

### Future Features

- **ML Library** with GPU acceleration
- **Quantum Circuit Library**
- **Additional Backends** (SPIR-V, native ARM)

## Code Style

### Rust Code

The Fusion project enforces comprehensive linting standards to ensure code quality and consistency:

#### Formatting
- Run `cargo fmt` before committing
- Configuration is defined in `rustfmt.toml` (max width: 100, 4 spaces)
- Format on save is recommended in your editor

#### Linting
- Run `cargo clippy --workspace --all-targets` before submitting PRs
- All Clippy warnings must be addressed or explicitly allowed with justification
- Configuration is defined in `.clippy.toml` and workspace lints in `Cargo.toml`

#### Customizing Your Lint Level

You can choose your preferred lint strictness:

**Option 1: Use Preset Profiles**
```powershell
# Minimal (fewer warnings)
Copy-Item .lint-profiles\minimal-lints.toml .\Cargo.toml -Force

# Standard (default, balanced)
Copy-Item .lint-profiles\standard-lints.toml .\Cargo.toml -Force

# Strict (maximum quality)
Copy-Item .lint-profiles\strict-lints.toml .\Cargo.toml -Force
```

**Option 2: Per-Crate Customization**
Add `[lints]` to your crate's `Cargo.toml`:
```toml
[lints.clippy]
unwrap_used = "allow"  # Override for this crate
```

**Option 3: Local Developer Settings**
Create `~/.cargo/config.toml` (or `%USERPROFILE%\.cargo\config.toml` on Windows):
```toml
[build]
rustflags = ["-W", "clippy::all"]
```

See [`.lint-profiles/README.md`](../.lint-profiles/README.md) for details.


#### Enforced Standards
- **Safety**: Unsafe code requires awareness (`unsafe_code` warning)
- **Code Quality**: No unused imports, variables, or dead code
- **Clippy**: Standard lint set enabled

#### Best Practices
- Use meaningful variable and function names
- Add comments for complex logic
- Write comprehensive tests for all features
- Keep cognitive complexity reasonable (threshold: 25)
- Prefer `Result` and `Option` over panicking where appropriate
- Document public APIs when practical



### Fusion Code

- Use 4-space indentation
- Place opening braces on same line
- Use descriptive function and variable names
- Add documentation comments for public APIs

### Example

```fusion
/// Calculate the factorial of a number
/// Returns 1 for n <= 1
fn factorial(n: int) -> int {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}
```

## Testing

All contributions must include appropriate tests:

- **Unit tests** for individual functions
- **Integration tests** for feature interactions
- **End-to-end tests** for complete workflows

Run tests before submitting:

```bash
cargo test
cargo test --workspace
```

## Documentation

- Update documentation for new features
- Add examples for new APIs
- Update changelog with your changes
- Ensure all public APIs are documented

## Pull Request Process

1. **Ensure tests pass** - All tests must pass locally
2. **Update documentation** - Document new features
3. **Update changelog** - Add entry to ChangeLog.md
4. **Write clear PR description** - Explain what and why
5. **Address review feedback** - Be responsive to reviewers

### PR Title Format

```text
[Category] Brief description

Examples:
[Stdlib] Add JSON parsing support
[LSP] Implement go-to-definition
[Docs] Update getting started tutorial
[Fix] Resolve HashMap collision bug
```

### Categories

- `[Stdlib]` - Standard library additions
- `[Compiler]` - Compiler changes
- `[LSP]` - Language server improvements
- `[IDE]` - VS Code extension changes
- `[Docs]` - Documentation updates
- `[Fix]` - Bug fixes
- `[Perf]` - Performance improvements
- `[Test]` - Test additions/improvements

## Review Process

All contributions go through code review:

1. Automated tests run on PR
2. Code review by maintainers
3. Address feedback
4. Merge when approved

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inspiring community for all:

- **Be respectful** - Treat everyone with respect
- **Be collaborative** - Work together constructively
- **Be patient** - Help others learn
- **Be inclusive** - Welcome diverse perspectives

### Unacceptable Behavior

- Harassment or discrimination
- Trolling or insulting comments
- Personal or political attacks
- Publishing others' private information

## Getting Help

- **Documentation**: Check [docs/](docs/) first
- **Issues**: Search existing GitHub issues
- **Discussions**: Use GitHub Discussions for questions
- **Discord**: Join our community (link coming soon)

## Recognition

Contributors are recognized in:

- README.md contributors section
- Release notes
- Project documentation

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to Fusion!** 🚀

Your contributions help make Fusion better for everyone.
