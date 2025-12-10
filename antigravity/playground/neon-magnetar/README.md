# Fusion Programming Language CLI

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

> A next-generation programming language CLI with built-in AI assistance, post-quantum cryptography, and enterprise-grade tooling.

## Overview

Fusion is a modern programming language designed for the quantum computing era, featuring:

- **🔒 Post-Quantum Cryptography**: Built-in CRYSTALS-Kyber and CRYSTALS-Dilithium support
- **🤖 AI-Powered Development**: Integrated AI assistant for code generation, refactoring, and review
- **⚡ High Performance**: Native compilation with LLVM backend
- **🛡️ Memory Safety**: Strong type system with ownership semantics
- **🌐 Cross-Platform**: Windows, macOS, Linux support
- **📦 Modern Toolchain**: Integrated build system, package manager, formatter, and tester

## Quick Start

### Installation

```bash
# Install from source (currently the only method)
git clone https://github.com/fusion-lang/fusion-cli.git
cd fusion-cli
cargo build --release

# Add to PATH
export PATH="$PATH:$(pwd)/target/release"
```

### Create Your First Project

```bash
# Create a new project
fusion new my-project

# Navigate to project
cd my-project

# Build and run
fusion build
fusion run
```

### Try AI-Powered Development

```bash
# Generate code with AI
fusion ai generate "create a HTTP server with authentication"

# Get code explanations
fusion ai explain src/main.fu

# Review code for issues
fusion ai review --focus security

# Interactive AI assistant
fusion ai assist
```

## Features

### Core Toolchain

- **`fusion new`** - Create new projects from templates
- **`fusion build`** - Compile projects with optimisation
- **`fusion run`** - Execute compiled programs
- **`fusion test`** - Run unit and integration tests
- **`fusion fmt`** - Format code automatically
- **`fusion check`** - Type-check without building
- **`fusion lint`** - Static analysis and security lints
- **`fusion doc`** - Generate documentation

### Package Management

- **`fusion package add`** - Add dependencies
- **`fusion package update`** - Update dependencies
- **`fusion audit`** - Security vulnerability scanning
- PQC signature verification for packages

### AI Subsystem

- **`fusion ai assist`** - Interactive AI assistant
- **`fusion ai generate`** - Generate code from descriptions
- **`fusion ai refactor`** - Refactor existing code
- **`fusion ai explain`** - Explain code behaviour
- **`fusion ai review`** - Code review and suggestions
- **`fusion ai tests`** - Generate test suites
- **`fusion ai doc`** - Generate documentation

### Development Tools

- **`fusion debug`** - DAP-compatible debugger
- **`fusion profile`** - Performance profiling (CPU, memory, GPU)
- **`fusion deploy`** - Deploy to cloud platforms (AWS, Azure, GCP)

## AI Safety Features

The AI subsystem includes comprehensive safety mechanisms:

- **🔒 Safety Engine**: Detects PII, secrets, and security risks
- **👁️ Preview Mode**: Review all changes before applying
- **📋 Audit Trails**: Full provenance metadata for all AI-generated code
- **🔐 Offline Mode**: Use local models for sensitive codebases
- **✅ Human-in-the-Loop**: Mandatory review for high-risk changes

## Architecture

```
fusion-cli/
├── cmd/fusion           # CLI entry point
├── crates/
│   ├── core             # Compiler (lexer, parser, type checker)
│   ├── toolchain        # Build system and runner
│   ├── tester           # Test framework
│   ├── formatter        # Code formatter
│   ├── docgen           # Documentation generator
│   ├── pkgmgr           # Package manager (PQC signing)
│   ├── debugger         # DAP debugger
│   ├── analyzer         # Static analysis
│   ├── profiler         # Performance profiler
│   ├── audit            # Vulnerability scanner
│   ├── deploy           # Cloud deployment
│   ├── ai-core          # AI subsystem core
│   ├── ai-cli           # AI CLI commands
│   ├── ai-daemon        # Background AI daemon
│   └── ai-models        # Local model runners
├── docs/                # Documentation
├── examples/            # Example projects
└── tests/e2e            # End-to-end tests
```

## Documentation

- **[Quick Start Guide](QuickStartGuide.md)** - Get started in 5 minutes
- **[User Guide](docs/guides/UserGuide.md)** - Complete user documentation
- **[Developer Guide](docs/guides/DeveloperGuide.md)** - Contributing and architecture
- **[AI Gateway Guide](docs/guides/AI_GatewayGuide.md)** - AI subsystem details
- **[API Reference](docs/references/)** - API documentation

## Development

### Prerequisites

- Rust 1.75 or higher
- LLVM 15+ (for code generation)
- Git

### Building from Source

```bash
# Clone the repository
git clone https://github.com/fusion-lang/fusion-cli.git
cd fusion-cli

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Build release version
cargo build --workspace --release
```

### Running Tests

```bash
# Unit tests
cargo test --workspace

# Integration tests
cargo test --workspace --test '*'

# End-to-end tests
cd tests/e2e && cargo test
```

## Security

Fusion takes security seriously:

- **Post-Quantum Cryptography**: CRYSTALS-Kyber (key encapsulation) and CRYSTALS-Dilithium (signatures)
- **Package Signing**: All packages are PQC-signed
- **Dependency Auditing**: Automatic CVE scanning
- **Safe Defaults**: Offline-first AI for private codebases
- **Audit Trails**: Complete provenance for all changes

To report security vulnerabilities, please email security@fusionlang.dev.

## Roadmap

### Phase 1: Foundation (Current)
- ✅ Core CLI structure
- ✅ Basic toolchain (build, run, test)
- ✅ AI subsystem architecture
- ✅ Safety engine implementation

### Phase 2: AI Enhancement
- [ ] Cloud LLM adapters (OpenAI, Anthropic, Azure)
- [ ] Local model integration (llama.cpp, ONNX)
- [ ] Advanced code generation
- [ ] Multi-turn conversations

### Phase 3: Production Readiness
- [ ] Full compiler implementation
- [ ] LLVM code generation
- [ ] Standard library
- [ ] Package registry
- [ ] VSCode extension

### Phase 4: Enterprise Features
- [ ] Team collaboration features
- [ ] Enterprise policy management
- [ ] Compliance reporting
- [ ] Advanced profiling

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on:

- Code of Conduct
- Development workflow
- Testing requirements
- Documentation standards
- Pull request process

## License

Fusion is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

You may choose either license for your use.

## Community

- **Discord**: [Join our server](https://discord.gg/fusion-lang)
- **Forum**: [community.fusionlang.dev](https://community.fusionlang.dev)
- **Twitter**: [@fusionlang](https://twitter.com/fusionlang)
- **GitHub**: [github.com/fusion-lang](https://github.com/fusion-lang)

## Acknowledgements

Built with ❤️ by the Fusion Language Team.

Special thanks to:
- The Rust community for excellent tooling
- NIST for standardising post-quantum cryptography
- All contributors and early adopters

---

**Fusion** - Next-generation programming for a quantum future.
