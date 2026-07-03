# Fusion v2.0 Vortex Developer Guide

## Introduction

Welcome to the Fusion v2.0 Vortex Programming Language development guide. This document provides comprehensive information for contributors and developers working on the Fusion project.

**Prerequisites:**

- Fusion toolchain (use `./install.sh` to build the toolchain; `dist/` is the target output directory)
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

````text
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
```text

### Key Directories

- **`cmd/`**: Contains binary targets. Each subdirectory is a deployable application.
- **`crates/`**: Core workspace libraries shared across all Fusion components.
- **`registry/crates/`**: Feature-specific implementations (LLM, Quantum, Security, etc.). These are published to the Fusion registry.
- **`docs/`**: All project documentation, organized by type.

## Build System & Toolchain

### Cargo Workspace

Fusion uses a massive Cargo workspace structure defined in the root `Fusion.toml`. This allows unified dependency management and build orchestration.

**Build Commands:**

```bash

# Full workspace build

fusion build --workspace --release

# Build specific crate

fusion build -p fusion-core

# Build CLI only

fusion build --bin fusion --release

# Check without building

fusion check --workspace
```text

### Build Profiles

Defined in `Fusion.toml`:

- **dev**: Fast compilation, debug symbols, no optimizations
- **release**: Full optimizations, no debug info
- **dist**: Release + LTO + thin LTO for distribution

```bash

# Use distribution profile

fusion build --profile dist
```text

### Fusion Build Tool

The `fusion-build` binary provides enhanced build capabilities:

```bash

# Build with Fusion-specific optimizations

fusion build --optimize-ai

# Build with quantum backend support

fusion build --features quantum-hardware
```text

## Development Workflow

### Initial Setup

1. **Clone the repository:**

   ```bash
   git clone https://github.com/QuantumSecureTechnologies/Fusion-Programming-Language.git
   cd Fusion-Programming-Language
```text

2. **Set environment variables:**

   ```bash
   export FUSION_HOME=$(pwd)
   export PATH="$FUSION_HOME/target/release:$PATH"
```text

3. **Build the project:**

   ```bash
   fusion build --workspace
```text

### Running Tests

**Unit Tests:**

```bash

# All workspace tests

fusion test --workspace

# Specific crate

fusion test -p fusion-compiler

# With output

fusion test -- --nocapture
```text

**Integration Tests:**

```bash

# Using Fusion test harness

fusion test integration

# E2E compiler tests

fusion test --test compiler_e2e
```text

**Performance Tests:**

```bash
fusion bench --workspace
```text

### Code Formatting & Linting

**Format code:**

```bash
fusion fmt --all
```text

**Lint with Clippy:**

```bash
fusion flux check --workspace -- -D warnings
```text

**Custom lint profiles** are defined in `.lint-profiles/`.

## Coding Standards

### Fusion Guidelines

1. **Follow Fusion API Guidelines**: Use `.fu` interfaces and minimal externs.
2. **Use Fusion Flux**: All code must pass `fusion flux check` without warnings.
3. **Format with fusion fmt**: Use `fusion fmt` before committing.
4. **Document public APIs**: All public items require doc comments (`///`).
5. **Error Handling**: Use `thiserror` for error types, `anyhow` for applications.

### Documentation Requirements

Every crate in `registry/crates/` must have:
- **README.md**: Overview, features, usage examples
- **Fusion.toml**: Accurate description and metadata
- **Doc comments**: For all public functions and types

### Security Practices

- **No unwrap()**: Use proper error handling
- **Input validation**: All external input must be validated
- **Secrets**: Never commit secrets; use `fusion_vault`
- **Auditing**: Run `fusion-audit` before submitting PRs

## Core Components Deep Dive

### Self-Hosting Compiler (Pure Fusion)

Fusion v2.0 Vortex features a **self-hosting compiler** written entirely in Fusion (`.fu` files). Located in `src/compiler/`:

| Module | File | Purpose |
|--------|------|---------|
| Token | `token.fu` | 28 keywords, 20 operators, all literals |
| Lexer | `lexer.fu` | Hand-written tokenizer with escape sequences |
| AST | `ast.fu` | All expression, statement, and item nodes |
| Parser | `parser.fu` | Recursive descent with operator precedence |
| Types | `types.fu` | Type registry, inference, and primitives |
| Sema | `sema.fu` | Type checking, variable resolution |
| IR | `ir.fu` | Opcodes, basic blocks, SSA-style |
| Codegen | `codegen.fu` | Fusion VM bytecode + x86_64 assembly |
| Intent | `intent.fu` | Intent-driven execution scheduling |
| PQC | `pqc.fu` | Kyber768 KEM, Dilithium3 signatures |
| Driver | `driver.fu` | Compilation pipeline orchestration |

### Compiler Pipeline

1. **Lexer** (`src/compiler/lexer.fu`):
   - Hand-written tokenizer in pure Fusion
   - Handles whitespace, comments, escape sequences
   - Outputs `Token` stream

2. **Parser** (`src/compiler/parser.fu`):
   - Recursive descent parser
   - Produces Abstract Syntax Tree (AST)
   - Full operator precedence handling

3. **Semantic Analysis** (`src/compiler/sema.fu`):
   - Two-pass analysis (register types, then analyze)
   - Symbol table construction
   - Type checking and inference

4. **Type System** (`src/compiler/types.fu`):
   - Hindley-Milner type inference
   - Type registry for primitives and user types
   - Trait resolution

5. **IR Generation** (`src/compiler/ir.fu`):
   - Lowers AST to Fusion IR
   - SSA-form with basic blocks
   - Opcodes for all operations

6. **Code Generation** (`src/compiler/codegen.fu`):
   - **Fusion VM**: Bytecode for the Fusion Virtual Machine
   - **x86_64**: Native assembly output
   - **WASM**: WebAssembly target (experimental)

### Intent System

The **Intent System** (`src/compiler/intent.fu`) enables intent-driven compilation:

- **Intent Enum**: `Critical`, `HighThroughput`, `Precision`, `Background`
- **TaskProfile**: Metadata for scheduler decisions (ops, memory, intent)
- **Device**: Target hardware (Cpu, Gpu, Qpu)
- **Cortex**: AI-driven scheduler for optimal device selection

```fusion
// Intent-driven function annotation
#[intent(Critical)]  // HFT: Minimal jitter, always CPU
fn process_order(order: Order) -> Result<Trade> { ... }

#[intent(HighThroughput)]  // AI: Maximum FLOPS, prefers GPU
fn train_model(data: Tensor) -> Model { ... }
````

### Post-Quantum Cryptography

The **PQC Module** (`src/compiler/pqc.fu`) provides quantum-resistant security:

- **ML-KEM (Kyber)**: Key encapsulation at 512/768/1024 levels
- **ML-DSA (Dilithium)**: Digital signatures at 2/3/5 levels
- **Hybrid Crypto**: Combined Classical + PQC operations

```fusion
use compiler::pqc::{KyberKeypair, DilithiumSign, HybridKeypair};

// Generate hybrid keypair
let keypair = HybridKeypair::generate();

// Sign with hybrid (ECDSA + Dilithium)
let signature = hybrid_sign(message, keypair);
```

### Runtime System

**Core Loop:**

````rust
// Simplified runtime pseudocode
loop {
    tasks = scheduler.poll_ready_tasks();
    for task in tasks {
        executor.spawn(task);
    }
    handle_io_events();
}
```text

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
3. **Test**: Run `fusion test --workspace` and ensure all pass
4. **Lint**: Run `fusion flux check` and `fusion fmt`
5. **Document**: Update relevant documentation
6. **Commit**: Use conventional commit messages:

```text
   feat(compiler): add trait specialization
   fix(runtime): resolve memory leak in scheduler
   docs(guide): update contributing section
```text

7. **Push and PR**: Submit pull request with clear description

### Feature Flags

Use Cargo features for optional functionality:

```toml
[features]
default = ["std"]
std = []
gpu = ["cuda", "metal"]
quantum-hardware = ["q-ibm-backend", "q-aws-backend"]
```text

### Code Review Guidelines

Reviewers check for:
- **Correctness**: Does it work as intended?
- **Performance**: Any regressions?
- **Safety**: No unsafe code without justification
- **Style**: Adheres to project standards
- **Tests**: Adequate test coverage

## Resources

- **Main Documentation**: `docs/guides/FUSION_COMPLETE_GUIDEBOOK.md`
- **API Reference**: Generated via `fusion doc --open`
- **Quick Start**: `QuickStartGuide.md`
- **Contributing**: `CONTRIBUTING.md`
- **Build Policy**: `BUILD_POLICY.md`

## Getting Help

- **Issues**: https://github.com/QuantumSecureTechnologies/Fusion-Programming-Language/issues
- **Discussions**: GitHub Discussions
- **Discord**: (If applicable)

---

**Thank you for contributing to Fusion!** Your work helps build the future of programming.
````
