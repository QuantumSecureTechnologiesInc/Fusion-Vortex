# Contributing to Fusion

Thank you for your interest in contributing to the Fusion Programming Language CLI! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful, inclusive, and collaborative environment for all contributors.

## How to Contribute

### Reporting Bugs

1. **Check existing issues** to see if the bug has already been reported
2. **Create a new issue** with:
   - Clear, descriptive title
   - Steps to reproduce
   - Expected vs. actual behaviour
   - Environment information (OS, Rust version, etc.)
   - Error messages/logs if applicable

###Suggesting Enhancements

1. **Check the roadmap** in `docs/roadmap/` to see if it's already planned
2. **Create an issue** describing:
   - The proposed enhancement
   - Use cases and benefits
   - Potential implementation approach

### Submitting Pull Requests

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/my-feature`
3. **Make your changes** following our coding standards
4. **Add tests** for new functionality
5. **Run the test suite**: `cargo test --workspace`
6. **Run the linter**: `cargo clippy --workspace -- -D warnings`
7. **Format your code**: `cargo fmt --all`
8. **Commit with descriptive messages** following conventional commits
9. **Push to your fork**
10. **Create a pull request** with:
    - Clear title and description
    - Link to related issues
    - Screenshots/examples if applicable

## Development Setup

### Prerequisites

- **Rust** 1.75 or higher
- **Git**
- **LLVM 15+** (optional, for full compiler features)

### Building

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/fusion-cli.git
cd fusion-cli

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run linter
cargo clippy --workspace

#Format code
cargo fmt --all
```

### Project Structure

```
fusion-cli/
├── cmd/fusion           # CLI entry point
├── crates/             # Core crates
│   ├── core            # Compiler (lexer, parser, typechecker)
│   ├── toolchain       # Build system
│   ├── ai-core         # AI subsystem core
│   └── ...            # Other crates
├── docs/              # Documentation
├── examples/          # Example projects
└── tests/e2e         # End-to-end tests
```

## Coding Standards

### Rust Style

- Follow **Rust API Guidelines**
- Use `cargo fmt` for consistent formatting
- Run `cargo clippy` and fix all warnings
- Write rustdoc comments for public APIs
- British English in all documentation

### Code Organisation

- One module per file when possible
- Public APIs in `lib.rs`, implementation in separate modules
- Clear separation of concerns
- Minimal public surface area

### Testing

- **Unit tests**: In the same file as the code (`#[cfg(test)]`)
- **Integration tests**: In `tests/` directory
- **Documentation tests**: In rustdoc comments
- Aim for >80% code coverage for new code

### Commit Messages

Use **Conventional Commits** format:

```
type(scope): subject

body (optional)

footer (optional)
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding/updating tests
- `chore`: Build process, dependencies, etc.

**Examples:**
```
feat(ai-core): add support for Claude API adapter

Implement ClaudeAdapter for Anthropic's Claude API with
streaming support and safety checks.

Closes #123
```

### Documentation

- **Public APIs**: Must have rustdoc comments
- **Complex functions**: Explain parameters, return values, panics, errors
- **Examples**: Include usage examples
in rustdoc
- **README**: Update if adding major features
- **Changelog**: Add entry for user-facing changes

## Testing Guidelines

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p fusion-core

# With output
cargo test --workspace -- --nocapture

# End-to-end tests
cd tests/e2e && cargo test
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_works() {
        let result = my_function("input");
        assert_eq!(result, expected_output);
    }
    
    #[test]
    #[should_panic(expected = "specific error")]
    fn test_error_handling() {
        my_function_that_panics();
    }
}
```

## AI Subsystem Development

When contributing to the AI subsystem:

1. **Safety first**: All AI operations must pass safety checks
2. **Preview mode**: Changes must be previewable before applying
3. **Offline support**: Features should work offline where possible
4. **Audit trails**: Maintain provenance metadata
5. **Testing**: Include tests with mock adapters

## Security Considerations

- **Never commit secrets** (API keys, passwords, etc.)
- **Use PQC primitives** for cryptographic operations
- **Validate all inputs** from external sources
- **No unsafe code** without explicit justification and review
- **Report vulnerabilities** to security@fusionlang.dev (not public issues)

## Release Process

(For maintainers)

1. Update version in `Cargo.toml`
2. Update `ChangeLog.md`
3. Create git tag: `v0.x.0`
4. Build release binaries
5. Publish to crates.io
6. Create GitHub release

## Getting Help

- **Discord**: [Join our server](https://discord.gg/fusion-lang)
- **Discussions**: [GitHub Discussions](https://github.com/fusion-lang/fusion-cli/discussions)
- **Documentation**: Check `docs/` directory

## Recognition

Contributors will be recognised in:
- `CONTRIBUTORS.md`
- Release notes
- Annual contributor highlights

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

---

Thank you for contributing to Fusion! 🚀
