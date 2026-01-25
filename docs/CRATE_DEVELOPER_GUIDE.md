# Fusion Crate Ecosystem - Developer Guide

**Version**: 0.2.0
**Last Updated**: 2025-12-17

## Introduction

Welcome to the Fusion Programming Language crate ecosystem. This guide provides an overview of the architecture, organization, and best practices for working with Fusion crates.

## Ecosystem Architecture

The Fusion ecosystem follows a layered architecture organized into six archetypes:

```text
┌─────────────────────────────────────────────────────┐
│                   Applications                       │
│                 (Your Projects)                      │
└─────────────────────────────────────────────────────┘
                        ▲
                        │
┌─────────────────────────────────────────────────────┐
│            TOOLING (sec-*, generators)              │
│         CLI tools, analyzers, validators            │
└─────────────────────────────────────────────────────┘
                        ▲
                        │
┌─────────────────────────────────────────────────────┐
│     FRAMEWORKS (runtime, ai-core, mcp)              │
│    Opinionated platforms with full features         │
└─────────────────────────────────────────────────────┘
                        ▲
                        │
┌─────────────────────────────────────────────────────┐
│  INTEGRATION (cloud-*, interop-*, protocol-*)       │
│     Bridge Fusion to external systems               │
└─────────────────────────────────────────────────────┘
                        ▲
                        │
┌─────────────────────────────────────────────────────┐
│   ALGORITHMS (nn-*, q-*, llm-*, math processors)    │
│    Pure logic with complexity guarantees            │
└─────────────────────────────────────────────────────┘
                        ▲
                        │
┌─────────────────────────────────────────────────────┐
│     FOUNDATION (core, std, primitives)              │
│      Core types, minimal dependencies               │
└─────────────────────────────────────────────────────┘
```text

## Archetype Guide

### Foundation / Primitive Crates

**Purpose**: Provide zero-dependency building blocks.

**Characteristics**:
- No default features
- Panic-free (where possible)
- Minimal dependencies
- `no_std` compatible

**Examples**: `fusion_core`, `fusion_std`, `finite-fields`, `tensor-sparse`

**When to use**: Building low-level systems, embedded contexts, or when you need absolute control.

### Algorithm / Engine Crates

**Purpose**: Implement specific computational methods.

**Characteristics**:
- Documented complexity (Big-O notation in description)
- Benchmarked performance
- Deterministic behavior
- Pure functions where possible

**Examples**: `nn-lstm`, `clustering`, `qaoa`, `llm-tokenizers`

**When to use**: Implementing specific algorithms for AI, quantum, or data processing.

### Integration / Glue Crates

**Purpose**: Connect Fusion to external systems.

**Characteristics**:
- Feature flags for async/blocking modes
- Sensible defaults
- Connection pooling
- Error translation layers

**Examples**: `fusion_http`, `grpc`, `cloud-aws`, `interop-python`

**When to use**: Building services that interact with external APIs, databases, or protocols.

### Framework Crates

**Purpose**: Provide opinionated, batteries-included platforms.

**Characteristics**:
- `default = ["full"]` features
- Comprehensive documentation
- Migration guides
- Extension points

**Examples**: `fusion_runtime_core`, `fusion_ai_core`, `mcp`

**When to use**: Rapid application development with common patterns.

### Tool Crates

**Purpose**: CLI utilities and development aids.

**Characteristics**:
- Excellent error messages
- Human and machine-readable output
- Documented exit codes
- Shell completion

**Examples**: `fusion` CLI, `sec-penetration`, `sbom-generator`

**When to use**: Automation, CI/CD, security scanning, code generation.

### Experimental Crates

**Purpose**: Research and bleeding-edge features.

**Characteristics**:
- Version < 1.0
- Prominent "EXPERIMENTAL" warnings
- Roadmap documentation
- Breaking changes expected

**Examples**: (Research prototypes)

**When to use**: Exploring new ideas, academic research, early adopters.

## Common Workflows

### Quantum-Classical Hybrid Application

```toml
[dependencies]
fusion_core = { workspace = true }
fusion_quantum = { workspace = true }
fusion_runtime_core = { workspace = true, features = ["full"] }
q-sim = { workspace = true }
qaoa = { workspace = true }
```text

```rust
use fusion_core::Tensor;
use fusion_quantum::QuantumCircuit;
use qaoa::QAOAOptimizer;

fn main() -> anyhow::Result<()> {
    let circuit = QuantumCircuit::new(10)?;
    let optimizer = QAOAOptimizer::new(circuit);
    let result = optimizer.optimize()?;
    println!("Optimal solution: {:?}", result);
    Ok(())
}
```text

### LLM Inference Pipeline

```toml
[dependencies]
fusion_ai_core = { workspace = true, features = ["full"] }
llm-tokenizers = { workspace = true }
llm-quantization = { workspace = true }
llm-beam-search = { workspace = true }
```text

```rust
use llm_tokenizers::BPETokenizer;
use llm_quantization::quantize_int8;
use llm_beam_search::BeamSearch;

fn inference_pipeline(model_path: &str, prompt: &str) {
    let tokenizer = BPETokenizer::from_file("vocab.json");
    let tokens = tokenizer.encode(prompt);
    // ... model inference with quantization
}
```text

### Cloud-Native Microservice

```toml
[dependencies]
fusion_http = { workspace = true, features = ["server", "tls"] }
cloud-aws = { workspace = true, features = ["s3", "lambda"] }
fusion_runtime_core = { workspace = true }
sec-policy-engine = { workspace = true }
```text

## Dependency Management

### Workspace Dependencies

All Fusion crates use workspace dependencies for consistency:

```toml
[workspace.dependencies]
fusion_core = { path = "registry/crates/fusion_core" }
fusion_ai_core = { path = "registry/crates/fusion_ai_core" }

# ... etc

```text

### Version Compatibility

- **0.2.x**: Current stable series
- **0.3.x**: Breaking changes expected
- **1.0.0**: Planned stable API freeze

### Feature Flags Best Practices

1. **Primitives**: No defaults, explicit opt-in
2. **Algorithms**: Minimal defaults, optional SIMD/GPU
3. **Integration**: `default = ["async"]`
4. **Frameworks**: `default = ["full"]`

## Building and Testing

### Full Workspace Build

```bash
cargo build --workspace --release
```text

### Targeted Testing

```bash

# Test specific archetype

cargo test -p fusion_core -p fusion_std

# Test all quantum crates

cargo test -p q-sim -p qaoa -p q-error-correction
```text

### Benchmarking

```bash

# Run all benchmarks

cargo bench --workspace

# Specific crate benchmarks

cargo bench -p fusion_attention
```text

## Documentation Standards

Each crate should include:

1. **README.md**: Quick start, examples, API overview
2. **Inline docs**: All public APIs documented
3. **Examples**: Common use cases in `examples/`
4. **Benchmarks**: Performance characteristics in `benches/`

## Contributing

### Adding a New Crate

1. Determine the correct archetype
2. Use appropriate template from `CRATE_POLISH_GUIDE.md`
3. Follow naming conventions (`archetype-name` or `fusion_archetype_component`)
4. Add workspace dependency in root `Cargo.toml`
5. Run polish scripts for consistency

### Archetype-Specific Guidelines

See `docs/ARCHETYPE_QUICKREF.md` for detailed templates and decision trees.

## Resources

- **Ecosystem Overview**: `docs/CRATE_ECOSYSTEM_OVERVIEW.md`
- **Polish Guide**: `docs/CRATE_POLISH_GUIDE.md`
- **Quick Reference**: `docs/ARCHETYPE_QUICKREF.md`
- **Inventory**: `docs/COMPLETE_CRATE_INVENTORY.md`

---

**For support**: See individual crate README files or community forums.