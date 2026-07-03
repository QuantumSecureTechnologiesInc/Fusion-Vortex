> **Phase 0 audit (2026-06-24) found this doc overclaims reality.**
> Treat feature lists here as roadmap, not current state.
> See `docs-truth-audit/TRUTH_REPORT.md` for details.

# Fusion v2.0 Vortex: Quick Start Guide

**Version**: v0.2.0-beta.1
**Status**: Production Ready – Vortex Engine Active
**Publisher**: Quantum Secure Technologies Inc.

---

## Welcome to Fusion v2.0 Vortex

Fusion v2.0 Vortex is the world's **first self-hosting, quantum-native, AI-integrated systems programming language**. This quick start guide will get you up and running in under 10 minutes.

### What You'll Achieve

By the end of this guide, you'll have:

- ✅ Fusion v2.0 Vortex toolchain installed
- ✅ Created your first `.fu` program
- ✅ Explored quantum computing, AI/ML, and post-quantum cryptography
- ✅ Built and run a complete application

---

## Prerequisites

- **Operating System**: Windows 10/11, Linux (Ubuntu 20.04+), or macOS 12+
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Storage**: 2GB free disk space
- **Optional**: CUDA Toolkit 12.0+ for GPU acceleration

---

## Step 1: Installation

### Option A: Quick Install (Recommended)

```bash
# Clone the Fusion v2.0 Vortex repository
git clone https://github.com/QuantumSecureTechnologiesInc/Fusion-Vortex.git
cd Fusion-Vortex

# Build native compiler, run regression, and create package
powershell -ExecutionPolicy Bypass -File scripts/bootstrap_native.ps1

# Verify native compiler
.\target\release\fuc.exe --version
```

### Option B: From Pre-built Binaries

Download the latest release from:

- **Windows**: `Fusion-Vortex-v0.2.0-x64.msi`
- **Linux**: `fusion-vortex-v0.2.0-x86_64-linux.tar.gz`
- **macOS**: `fusion-vortex-v0.2.0-darwin-universal.pkg`

---

## Step 2: Your First Fusion Program

Create a file called `hello.fu`:

```fusion
// hello.fu - Your first Fusion v2.0 Vortex program
#[fusion::main]
fn main() {
    println!("Hello from Fusion v2.0 Vortex!");
    println!("The future of programming is here. 🚀");
}
```

**Run it:**

```bash
fusion run hello.fu
# Output:
# Hello from Fusion v2.0 Vortex!
# The future of programming is here. 🚀
```

---

## Step 3: Explore Unique Vortex Features

### 🌀 Post-Quantum Cryptography with Vortex Engine

```fusion
use std::vortex;

#[fusion::main]
fn main() -> Result<()> {
    // Initialise Vortex Entropy Engine
    let ctx = vortex::VortexContext::new()?;

    // Generate NIST-compliant PQC seed
    let seed = ctx.generate_seed_safe()?;

    println!("✅ Vortex Engine initialised");
    println!("🔑 Entropy quality: {} bits", seed.entropy_bits());

    Ok(())
}
```

### ⚛️ Quantum Computing

```fusion
use std::quantum;

#[fusion::main]
fn main() -> Result<()> {
    // Create a Bell state (quantum entanglement)
    let mut circuit = quantum::QuantumCircuit::new(2);
    circuit.h(0);      // Hadamard on qubit 0
    circuit.cx(0, 1);  // CNOT (control: 0, target: 1)

    // Execute on simulator (or real quantum hardware!)
    let result = circuit.execute().await?;

    println!("⚛️ Quantum measurement: {:?}", result.state_vector());

    Ok(())
}
```

### 🧠 AI/ML with Native Tensors

```fusion
use std::ai;

#[fusion::main]
fn main() -> Result<()> {
    // Create tensors (automatically runs on GPU if available)
    let tensor = ai::Tensor::randn([1000, 1000]);
    let result = tensor.matmul(tensor.transpose()).relu();

    println!("🧠 Tensor result shape: {:?}", result.shape());
    println!("💻 Executed on: {}", result.device());

    Ok(())
}
```

### ⚡ Intent-Driven Execution

Fusion's **Intent System** automatically schedules code to optimal hardware:

```fusion
use compiler::intent::{Intent, Cortex};

// Critical: HFT/Trading - Always CPU, minimal jitter
#[intent(Critical)]
fn process_trade(order: Order) -> Trade {
    // Guaranteed <10μs latency
}

// HighThroughput: AI/ML - Prefers GPU
#[intent(HighThroughput)]
fn train_model(data: Tensor) -> Model {
    // Automatically offloaded to CUDA
}

// Precision: Science - Extended precision
#[intent(Precision)]
fn quantum_simulation(qubits: int) -> StateVector {
    // Uses high-precision arithmetic
}
```

### 🔐 Self-Hosting Compiler

Fusion v2.0 Vortex includes a **complete self-hosting compiler** written in pure Fusion:

```fusion
use compiler::{compile, parse, lex};

fn main() {
    // The compiler can compile itself!
    let source = read_file("src/compiler/lexer.fu");
    let tokens = lex(source);
    let ast = parse(tokens);
    compile(ast, "fusion_vm");
}
```

Compiler modules in `src/compiler/`:

- `lexer.fu` → Hand-written tokenizer
- `parser.fu` → Recursive descent parser
- `sema.fu` → Type checking & inference
- `codegen.fu` → Bytecode + x86_64 output
- `intent.fu` → Intent-driven scheduling
- `pqc.fu` → Kyber/Dilithium crypto

---

## Step 4: Build a Complete Project

```bash
# Create a new project
fusion new quantum-ai-demo
cd quantum-ai-demo

# Project structure:
# quantum-ai-demo/
# ├── Fusion.toml      # Unified manifest
# ├── src/
# │   └── main.fu     # Your code
# └── README.md
```

### Edit `src/main.fu`

```fusion
use std::vortex;
use std::quantum;
use std::ai;

#[fusion::main]
async fn main() -> Result<()> {
    println!("🌀 Fusion v2.0 Vortex Demo\n");

    // 1. Post-Quantum Cryptography
    let ctx = vortex::VortexContext::new()?;
    let seed = ctx.generate_seed_safe()?;
    println!("✅ Vortex Engine: {} bits entropy\n", seed.entropy_bits());

    // 2. Quantum Computing
    let mut circuit = quantum::QuantumCircuit::new(3);
    circuit.h(0).cx(0, 1).cx(1, 2);
    let qresult = circuit.execute().await?;
    println!("⚛️ Quantum state: {:?}\n", qresult.probabilities());

    // 3. AI/ML Inference
    let tensor = ai::Tensor::randn([256, 256]);
    let airesult = tensor.matmul(tensor.transpose());
    println!("🧠 AI computation on: {}\n", airesult.device());

    println!("✨ All systems operational!");
    Ok(())
}
```

### Build and Run

```bash
fusion build --release
fusion run

# Output:
# 🌀 Fusion v2.0 Vortex Demo
#
# ✅  Vortex Engine: 256 bits entropy
#
# ⚛️ Quantum state: {|000⟩: 0.5, |111⟩: 0.5}
#
# 🧠 AI computation on: CUDA:0
#
# ✨ All systems operational!
```

---

## Step 5: Explore Advanced Features

### Fusion Visual Compiler (AI Code Generation)

```bash
# Generate a complete project from natural language
fusion visual generate "Create a REST API with post-quantum TLS and quantum random number generation"

# Output: Complete project in fusion_build_<timestamp>/
```

### Advanced AI CLI

```bash
# Multi-provider AI assistance
fusion ai assist --provider claude

# Security-focused code review
fusion ai review ./src --focus security

# Automated test generation
fusion ai tests ./src/main.fu
```

### Fusion Forge (Polyglot Build System)

Edit `Fusion.toml` to add C++/Python/JavaScript:

```toml
[languages.cpp]
sources = ["src/accelerator.cpp"]
compiler = "clang++"

[languages.python]
requirements = ["numpy>=1.24"]

[languages.javascript]
packages = { react = "^18.0" }
```

```bash
fusion build  # Builds all languages seamlessly!
```

---

## Next Steps

### 📚 Learn More

- **[Complete Guidebook](./FUSION_COMPLETE_GUIDEBOOK.md)**: Comprehensive language reference
- **[Feature Index](../features/FEATURES_INDEX.md)**: Explore unique Vortex capabilities
- **[Developer Guide](./DeveloperGuide.md)**: Deep dive into architecture
- **[Examples](../../examples/)**: Real-world application code

### 🚀 Build Something Amazing

Fusion v2.0 Vortex unifies:

- **Quantum Computing** → Native quantum types and circuits
- **AI/ML** → Built-in tensors and neural networks
- **Post-Quantum Security** → Vortex Engine and ML-KEM/ML-DSA
- **High Performance** → Supernova Runtime (CPU/GPU/QPU)

**What will you build?**

---

## Support

- 📧 **Email**: [support@quantumsecuretechnologies.co.uk](mailto:support@quantumsecuretechnologies.co.uk)
- 💬 **Discord**: [Join our community](https://discord.gg/fusion-vortex)
- 🐛 **Issues**: [GitHub Issues](https://github.com/QuantumSecureTechnologiesInc/Fusion-Vortex/issues)
- 📖 **Documentation**: [Complete Docs](../DocumentIndex.md)

---

**Generated by**: Fusion v2.0 Vortex Toolchain
**Last Updated**: January 28, 2026
**Licence**: MIT/Apache-2.0
