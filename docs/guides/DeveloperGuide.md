# Fusion Developer Guide

## Introduction

Welcome to the Fusion Programming Language development guide. This document provides comprehensive information for contributors and developers working on the Fusion project.

**Prerequisites:**
- Rust 1.70+ (stable toolchain)
- Python 3.10+ (optional, for interop and tooling)
- CUDA Toolkit 12.0+ (optional, for GPU acceleration)
- Git

## Architecture Overview

### The Unified Monolith

Fusion adopts a **Unified Monolith** architecture, consolidating compiler, runtime, standard library, AI engine, and quantum engine into a single cohesive system. This approach eliminates fragmentation and provides:

- **Zero-cost abstractions** between subsystems
- **Unified type system** across all domains
- **Single build artifact** for distribution

### Runtime Architecture

**Fusion Runtime Core v2.0 (Nebula)** provides:

- **Hybrid Async Runtime**: Fusion's native runtime replacing Tokio
- **HAFT Memory Manager**: Hierarchical Adaptive Fusion Tiering for GPU/CPU memory orchestration
- **Policy Engine**: Security policy enforcement at runtime

**Key Components:**
- `fusion_runtime_core`: Core async primitives and executor
- `fusion_runtime_scheduler`: Task scheduling with work-stealing
- `fusion_runtime_mem_mgr`: Memory allocation and lifetime management
- `fusion_runtime_hal`: Hardware Abstraction Layer for cross-platform support

### Engine Systems

#### AI Engine
- **Inference**: LLM serving with `fusion_llm_inference_engine`
- **Training**: Distributed training via `fusion_llm_distributed_training`
- **Auto-Prompting**: Automatic prompt optimization
- **Model Serving**: Production-ready model server (`fusion_llm_model_server`)

#### Quantum Engine
- **Simulators**: Classical and GPU-accelerated quantum simulators
- **Transpilers**: Circuit optimization and hardware backend translation
- **Algorithms**: QAOA, VQE, Shor's algorithm implementations

## Project Organization

```
fusion/
├── cmd/                    # Application entry points
│   ├── fusion/            # Main CLI (`fusion build`, `fusion run`)
│   ├── fusion-coder/      # AI-powered code assistant
│   └── fusion-server/     # HTTP/gRPC servers
├── crates/                # Core workspace libraries
│   ├── fusion-core/       # Foundational types and traits
│   ├── fusion-compiler/   # Compiler frontend and backend
│   ├── toolchain/         # Build system integration
│   └── ...               # ~100 core crates
├── registry/crates/       # Ecosystem packages (242+ crates)
│   ├── llm-*/            # LLM infrastructure
│   ├── q-*/              # Quantum computing
│   ├── nn-*/             # Neural network layers
│   ├── sec-*/            # Security tools
│   └── ...
├── docs/                  # Documentation
│   ├── guides/           # User and developer guides
│   └── references/       # API references
├── runtime/              # Runtime implementations
└── tests/                # Integration and E2E tests
```

### Key Directories

- **`cmd/`**: Contains binary targets. Each subdirectory is a deployable application.
- **`crates/`**: Core workspace libraries shared across all Fusion components.
- **`registry/crates/`**: Feature-specific implementations (LLM, Quantum, Security, etc.). These are published to the Fusion registry.
- **`docs/`**: All project documentation, organized by type.

## Build System & Toolchain

### Cargo Workspace

Fusion uses a massive Cargo workspace structure defined in the root `Cargo.toml`. This allows unified dependency management and build orchestration.

**Build Commands:**

```bash
# Full workspace build
cargo build --workspace --release

# Build specific crate
cargo build -p fusion-core

# Build CLI only
cargo build --bin fusion --release

# Check without building
cargo check --workspace
```

### Build Profiles

Defined in `Cargo.toml`:

- **dev**: Fast compilation, debug symbols, no optimizations
- **release**: Full optimizations, no debug info
- **dist**: Release + LTO + thin LTO for distribution

```bash
# Use distribution profile
cargo build --profile dist
```

### Fusion Build Tool

The `fusion-build` binary provides enhanced build capabilities:

```bash
# Build with Fusion-specific optimizations
fusion build --optimize-ai

# Build with quantum backend support
fusion build --features quantum-hardware
```

## Development Workflow

### Initial Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/QuantumSecureTechnologies/Fusion-Programming-Language.git
   cd Fusion-Programming-Language
   ```

2. **Set environment variables:**
   ```bash
   export FUSION_HOME=$(pwd)
   export PATH="$FUSION_HOME/target/release:$PATH"
   ```

3. **Build the project:**
   ```bash
   cargo build --workspace
   ```

### Running Tests

**Unit Tests:**
```bash
# All workspace tests
cargo test --workspace

# Specific crate
cargo test -p fusion-compiler

# With output
cargo test -- --nocapture
```

**Integration Tests:**
```bash
# Using Fusion test harness
fusion test integration

# E2E compiler tests
cargo test --test compiler_e2e
```

**Performance Tests:**
```bash
cargo bench --workspace
```

### Code Formatting & Linting

**Format code:**
```bash
cargo fmt --all
```

**Lint with Clippy:**
```bash
cargo clippy --workspace -- -D warnings
```

**Custom lint profiles** are defined in `.lint-profiles/`.

## Coding Standards

### Rust Guidelines

1. **Follow Rust API Guidelines**: https://rust-lang.github.io/api-guidelines/
2. **Use Clippy**: All code must pass `cargo clippy` without warnings.
3. **Format with rustfmt**: Use `cargo fmt` before committing.
4. **Document public APIs**: All public items require doc comments (`///`).
5. **Error Handling**: Use `thiserror` for error types, `anyhow` for applications.

### Documentation Requirements

Every crate in `registry/crates/` must have:
- **README.md**: Overview, features, usage examples
- **Cargo.toml**: Accurate description and metadata
- **Doc comments**: For all public functions and types

### Security Practices

- **No unwrap()**: Use proper error handling
- **Input validation**: All external input must be validated
- **Secrets**: Never commit secrets; use `fusion_vault`
- **Auditing**: Run `fusion-audit` before submitting PRs

## Core Components Deep Dive

### Compiler Pipeline

1. **Lexer** (`crates/fusion-compiler/src/lexer.rs`):
   - Tokenizes source code
   - Handles UTF-8 and special characters
   - Outputs `Token` stream

2. **Parser** (`crates/fusion-compiler/src/parser.rs`):
   - Recursive descent parser
   - Produces Abstract Syntax Tree (AST)
   - Error recovery for diagnostics

3. **Semantic Analysis** (`crates/fusion-compiler/src/semantic.rs`):
   - Symbol table construction
   - Scope resolution
   - Name binding

4. **Type Checker** (`crates/fusion-compiler/src/type_checker.rs`):
   - Hindley-Milner type inference
   - Trait resolution
   - Lifetime checking

5. **IR Generation** (`crates/fusion-compiler/src/ir/`):
   - Lowers AST to Fusion IR
   - SSA form construction
   - Optimization passes

6. **Code Generation** (`crates/fusion-compiler/src/codegen/`):
   - LLVM backend (primary)
   - WASM backend (experimental)
   - Native code emission

### Runtime System

**Core Loop:**
```rust
// Simplified runtime pseudocode
loop {
    tasks = scheduler.poll_ready_tasks();
    for task in tasks {
        executor.spawn(task);
    }
    handle_io_events();
}
```

**Key Traits:**
- `Future`: Async computation primitive
- `Task`: Schedulable unit of work
- `Waker`: Notification mechanism

### Interoperability

**Python Bridge** (`registry/crates/interop-python/`):
- PyO3-based bindings
- Zero-copy NumPy tensor sharing
- GIL management

**JavaScript Bridge** (`registry/crates/interop-js/`):
- V8 or QuickJS embedding
- ES Module support

**Java Bridge** (`registry/crates/interop-java/`):
- JNI interface
- Dynamic class loading

## Contributing

### Pull Request Process

1. **Fork and branch**: Create a feature branch from `main`
2. **Implement**: Write code following our standards
3. **Test**: Run `cargo test --workspace` and ensure all pass
4. **Lint**: Run `cargo clippy` and `cargo fmt`
5. **Document**: Update relevant documentation
6. **Commit**: Use conventional commit messages:
   ```
   feat(compiler): add trait specialization
   fix(runtime): resolve memory leak in scheduler
   docs(guide): update contributing section
   ```
7. **Push and PR**: Submit pull request with clear description

### Feature Flags

Use Cargo features for optional functionality:

```toml
[features]
default = ["std"]
std = []
gpu = ["cuda", "metal"]
quantum-hardware = ["q-ibm-backend", "q-aws-backend"]
```

### Code Review Guidelines

Reviewers check for:
- **Correctness**: Does it work as intended?
- **Performance**: Any regressions?
- **Safety**: No unsafe code without justification
- **Style**: Adheres to project standards
- **Tests**: Adequate test coverage

## Resources

- **Main Documentation**: `docs/guides/FUSION_COMPLETE_GUIDEBOOK.md`
- **API Reference**: Generated via `cargo doc --open`
- **Quick Start**: `QuickStartGuide.md`
- **Contributing**: `CONTRIBUTING.md`
- **Build Policy**: `BUILD_POLICY.md`

## Getting Help

- **Issues**: https://github.com/QuantumSecureTechnologies/Fusion-Programming-Language/issues
- **Discussions**: GitHub Discussions
- **Discord**: (If applicable)

---

**Thank you for contributing to Fusion!** Your work helps build the future of programming.
