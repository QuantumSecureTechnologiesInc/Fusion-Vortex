# Fusion v1.0 Quick Start Guide

**Get up and running in 5 minutes!**

---

## Prerequisites

- **Operating System**: Linux, macOS, or Windows
- **Rust Toolchain**: Latest stable (for building from source)
- **LLVM 18+**: Required for code generation
- **Optional**: CUDA 12+ (for GPU-accelerated AI)

---

## Installation

### Option 1: Cargo Install (Recommended)

```bash
cargo install fusion-lang --version 1.0.0
```

### Option 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language.git
cd Fusion-Programming-Language

# Build the compiler
cargo build --release

# Add to PATH
export PATH="$PATH:$(pwd)/target/release"
```

### Verify Installation

```bash
fusion --version
# Output: Fusion 1.0.0
```

---

## Your First Program

### Hello, World!

Create `hello.fu`:
```fusion
fn main():
    print("Hello, Fusion v1.0!")
```

Run it:
```bash
fusion run hello.fu
# Output: Hello, Fusion v1.0!
```

---

## Quick Examples

### Variables & Types

```fusion
let x = 42              // Inferred: int
let y: float = 3.14     // Explicit: float
let mut counter = 0     // Mutable variable
counter += 1
```

### Functions

```fusion
fn add(a: int, b: int) -> int:
    return a + b

fn main():
    print(add(2, 3))  // Output: 5
```

### Quantum Computing (⚛️)

```fusion
import quantum.circuits

fn main():
    // Create qubit in superposition
    let q = Qubit::new()
    h(q)  // Hadamard gate
    
    // Measure and print result
    let result = measure(q)
    print("Quantum result: " + result)
```

### AI/ML (🧠)

```fusion
import ai.models.llama

fn main():
    let model = Llama3::load("7b-chat")
    let response = model.generate("What is quantum computing?")
    print(response)
```

---

## Project Structure

Create a new project:
```bash
fusion new my_project
cd my_project
```

Project structure:
```
my_project/
├── fusion.toml       # Project configuration
├── src/
│   └── main.fu       # Entry point
└── tests/
    └── test_main.fu  # Tests
```

---

## Building & Running

```bash
# Run directly
fusion run src/main.fu

# Build optimized binary
fusion build --release

# Run tests
fusion test
```

---

## Compilation Targets

### Native (Default)
```bash
fusion build src/main.fu
```

### WebAssembly
```bash
fusion build src/main.fu --target wasm -o app.wasm
```

---

## IDE Setup

### VS Code

Install the Fusion extension:
```bash
code --install-extension fusion-language-1.0.0.vsix
```

Features:
- ✅ Syntax highlighting
- ✅ Auto-completion
- ✅ Error diagnostics
- ✅ Go to definition
- ✅ Code formatting

---

## Getting Help

- **Documentation**: [docs/guides/User_Guide.md](docs/guides/User_Guide.md)
- **Examples**: [examples/](examples/)
- **GitHub Issues**: Report bugs and request features
- **Discord**: Join the community chat

---

## What's Next?

1. 📘 Read the [User Guide](docs/guides/User_Guide.md) for comprehensive coverage
2. ⚛️ Try [Quantum Examples](examples/quantum/) for quantum computing
3. 🧠 Explore [AI Examples](examples/ai/) for machine learning
4. 🏢 Check out [Enterprise Examples](examples/enterprise/) for cloud deployment

---

**Welcome to Fusion v1.0!** 🚀

*One language. All of computing.*
