# Fusion Programming Language – The Comprehensive Guidebook

**Version**: 3.4 (Monolith Era)  
**Author**: Fusion Core Team

---

## 📖 Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Language Fundamentals](#language-fundamentals)
4. [Memory Management & The Effect System](#memory-management--the-effect-system)
5. [The Fusion Unified Toolchain](#the-fusion-unified-toolchain)
6. [HAFT: Intelligent AI & Tensors](#haft-intelligent-ai--tensors)
7. [Quantum Computing & Security](#quantum-computing--security)
8. [Flux-Resolve & Package Management](#flux-resolve--package-management)
9. [Fusion Terminal Browser](#fusion-terminal-browser)
10. [Real-World Use Cases](#real-world-use-cases)
11. [Best Practices Guide](#best-practices-guide)
12. [Appendices](#appendices)

---

## 1. Introduction {#introduction}

Welcome to **Fusion**, a modern, multi-paradigm programming language designed to bridge the gap between high-level ergonomics and bare-metal performance. Fusion was born from a simple observation: modern development often requires stitching together Python for AI, Rust for performance, and C++ for legacy systems. Fusion aims to be the single language that spans these domains seamlessly.

At its core, Fusion is built on a **unified, shared-memory toolchain** known as the Monolith. Unlike traditional compilers that treat building, testing, and linting as separate steps, Fusion integrates them into a continuous intelligence loop. This architecture allows the language to "understand" your code in real-time, offering capabilities like zero-copy IDE feedback and autonomous memory optimization via **HAFT** (Hyper-Adaptive Flux Tensors).

Whether you are building a high-frequency trading engine, a quantum simulation, or a distributed AI model, Fusion provides the safety guarantees of a borrow checker with the ease of use of a dynamic language.

---

## 2. Getting Started {#getting-started}

### Installation

Fusion provides a unified installer that sets up the compiler, the Monolith toolchain, and the standard library.

**For Unix-like Systems (Linux/macOS):**
```bash
curl -fsSL https://sh.fusion-lang.org | sh
```

**For Windows (PowerShell):**
```powershell
iwr https://win.fusion-lang.org -useb | iex
```

### Your First Application

Let's create a simple project to verify the installation. Fusion's CLI handles project generation, ensuring a standard directory structure from day one.

```bash
fusion new hello-fusion
cd hello-fusion
```

Open `src/main.fu` and you'll see the entry point. Here is a slightly more advanced "Hello World" that demonstrates string interpolation and basic input:

```fusion
// src/main.fu
fn main() -> int {
    let name = "Developer"
    println("Hello, {}! Welcome to Fusion.", name)
    
    // Return explicit exit code (0 = success)
    return 0
}
```

To run this, simply execute:
```bash
fusion run
```

---

## 3. Language Fundamentals {#language-fundamentals}

Fusion syntax is designed to be familiar to users of C-family languages but stripped of unnecessary boilerplate. It uses indentation-free block delimiters (`{}`) but enforces consistent formatting via the built-in formatter.

### Variables and Mutability

By default, variables in Fusion are **immutable**. This is a deliberate design choice to prevent accidental state mutation, which is a common source of bugs in concurrent systems.

```fusion
// Immutable by default
let pi = 3.14159
// pi = 3.14  // Compiler Error!

// Explicit mutability
let mut counter = 0
counter += 1
```

### Functions and Control Flow

Functions explicitly declare return types, ensuring clarity at API boundaries. The `match` statement provides powerful pattern matching capabilities, far exceeding traditional switch statements.

```fusion
fn analyze_value(x: int) -> string {
    match x {
        0 => "Zero",
        1..=10 => "Small number",
        _ => "Large number"
    }
}
```

### Async/Await

Fusion treats asynchronous programming as a first-class citizen. Unlike some languages where async splits the ecosystem, Fusion's standard library is fully async-aware.

```fusion
async fn fetch_user_data(id: int) -> Result<User> {
    let url = fmt("https://api.example.com/users/{}", id)
    let response = await http.get(url)
    return response.json()
}
```

---

## 4. Memory Management & The Effect System {#memory-management--the-effect-system}

One of Fusion's most powerful features is its dual-mode memory management, controlled by the **Effect System**. This allows you to choose the right tool for the job within the same codebase.

### The Two Modes

1.  **Garbage Collector (Default)**: For 90% of application logic, UI code, and scripting, you want ease of use. Fusion's generational GC handles memory automatically, preventing leaks without manual intervention.
2.  **Borrow Checker (`@borrowed`)**: For performance-critical paths (e.g., inner loops, audio processing, network drivers), you can opt-in to Rust-style ownership/borrowing semantics.

### Using `@borrowed`

Applying the `@borrowed` attribute switches the compiler mode for that scope. It enforces zero-allocation policies and strict ownership rules.

```fusion
// This function runs without GC pauses
@borrowed
fn process_audio_buffer(buffer: &mut [f32]) {
    for sample in buffer {
        *sample *= 0.5  // In-place volume reduction
    }
}
```

### Other Effects

The effect system extends beyond memory:
-   `@gpu_accelerated`: Automatically compiles the function to a CUDA or OpenCL kernel for execution on the GPU.
-   `@constant_time`: Critical for cryptography; prevents compiler optimizations that could introduce timing side-channels.
-   `@atomic`: Enforces atomic memory access guarantees for lock-free data structures.

---

## 5. The Fusion Unified Toolchain {#the-fusion-unified-toolchain}

### The Monolith Architecture

In traditional workflows, you might run `cargo check`, then `cargo test`, then a linter, then a security output. Each of these tools starts from scratch, parsing your code and loading dependencies. This is inefficient.

Fusion v3.4 introduced **Fusion Monolith Core**. It is a single, long-running process that holds your project's state in shared memory (`Arc<RwLock<FusionState>>`). When you save a file, the compiler updates the Abstract Syntax Tree (AST) in memory. The auditor checks dependencies on the fly, and the Language Server Protocol (LSP) reads the *exact same memory* to provide autocomplete.

### CLI Commands

The `fusion` CLI is your gateway to the Monolith.

-   **`fusion check`**: Performs semantic analysis. Because it reuses the state from the Monolith, it is near-instantaneous.
-   **`fusion build`**: Runs the full compilation pipeline.
-   **`fusion audit`**: Scans your dependencies against the Fusion Security Database. Thanks to "Shift-Left" security, this happens *during* dependency resolution.
-   **`fusion watch`**: Starts the Monolith in daemon mode, powering your IDE extensions.

---

## 6. HAFT: Intelligent AI & Tensors {#haft-intelligent-ai--tensors}

Fusion is designed for the AI era. Instead of relying on external libraries like NumPy or PyTorch for heavy lifting, Fusion includes **HAFT** (Hyper-Adaptive Flux Tensors) as a language primitive.

### Autonomous Memory optimization

A standard array is dumb; it just sits in memory. A **FluxTensor** is intelligent. It is managed by three autonomous background agents:

1.  **The Researcher**: Continually analyzes your code's access patterns. Is it reading sequentially? Randomly? Is the matrix sparse (mostly zeros)?
2.  **The Builder**: Managing the "Hot" and "Cold" storage tiers. Based on the Researcher's findings, it moves rarely accessed data to compressed cold storage (RAM or NVMe), keeping only the active "hot" data in GPU memory or CPU cache.
3.  **The Optimizer**: Tunes the data layout in real-time, effectively rewriting memory organization to match your usage patterns.

### Example: AI Model Training

```fusion
import fusion.haft
import fusion.nn

fn train_model() {
    // 100GB Tensor - exceeds GPU memory!
    let data = FluxTensor::from_file("massive_dataset.csv")
    
    // HAFT agents activate automatically.
    // They will keep only the current batch in GPU memory.
    let model = nn::Transformer::new()
    
    // Training loop is syntax-native, no complex library calls
    model.fit(data, epochs=10)
}
```

This significantly lowers the barrier to entry for training large models on consumer hardware.

---

## 7. Quantum Computing & Security {#quantum-computing--security}

Fusion adopts a "Quantum-Native" stance. We assume that powerful quantum computers will exist during the lifetime of the code you write today.

### Hybrid Cryptography

By default, all cryptographic operations in the standard library use **Hybrid** algorithms. For example, a TLS handshake doesn't just use Elliptic Curve Diffie-Hellman (ECDH); it combines it with a Post-Quantum algorithm like Kyber-1024.

```fusion
// This automatically uses Hybrid Crypto (X25519 + Kyber)
let secure_socket = net::TcpStream::connect_secure("bank.com:443")
```

### Quantum Circuits

You can write quantum algorithms directly in Fusion. The `fusion::quantum` module provides primitives for Qubits and Gates.

```fusion
fn entangle_pair() -> Result<Measurement> {
    let q = QubitRegister::new(2)
    
    // Hadamard gate puts q[0] in superposition
    q.h(0)
    
    // CNOT gate entangles q[0] and q[1]
    q.cnot(control=0, target=1)
    
    // Collapse wave function
    return q.measure()
}
```

These circuits can run on the built-in simulator or be dispatched to a cloud QPU (IBM Q, Rigetti) by changing a simple configuration flag.

---

## 8. Flux-Resolve & Package Management {#flux-resolve--package-management}

Dependency hell is a solved problem in Fusion thanks to **Flux-Resolve**. Standard resolvers use CPU-based SAT solvers which can be slow for large graphs. Flux-Resolve offloads this constraint satisfaction problem to the GPU specifically (or uses vectorized CPU instructions).

**Shift-Left Security** is integrated here. When you try to add a dependency:
```bash
fusion add "unsafe-lib"
```
Flux-Resolve checks the vulnerability database *before* even downloading the package metadata. If a known vulnerability exists, it blocks the resolution, preventing the bad code from ever touching your disk.

---

## 9. Fusion Terminal Browser {#fusion-terminal-browser}

Developers spend half their time reading documentation. Switching context to a web browser breaks flow. Fusion includes a built-in **Terminal Browser**—a text-based web renderer optimized for technical documentation.

It renders Markdown, API references, and standard web pages directly in your terminal with full mouse support and strict Vim keybindings.

**Usage:**
```bash
fusion tool browser https://docs.fusion-lang.org/std/collections
```

You can even integrate it into your IDE setup to have documentation open in a side pane without the overhead of a Chrome instance.

---

## 10. Real-World Use Cases {#real-world-use-cases}

### Case Study: High-Frequency Trading (HFT)
**Challenge**: Process millions of market ticks per second with microsecond latency.
**Fusion Solution**: 
-   Use `@borrowed` for the order matching engine to eliminate GC pauses.
-   Use `@gpu_accelerated` to run risk analysis models in parallel on the GPU.
-   Result: A deterministic, ultra-low latency engine in a high-level language.

### Case Study: Secure Medical Records
**Challenge**: Store patient data for 50 years, ensuring it remains secure against future quantum computers.
**Fusion Solution**:
-   Use the standard library's Hybrid Cryptography for all data at rest.
-   Use `@constant_time` utilities for all custom parsing logic.
-   Result: Future-proof data compliance out of the box.

---

## 11. Best Practices Guide {#best-practices-guide}

### Do:
-   **Prefer Immutability**: Use `let` instead of `let mut` whenever possible. It makes code easier to reason about.
-   **Use GC by Default**: Don't reach for `@borrowed` optimization prematurely. The Fusion GC is highly tuned. Only optimize hot paths.
-   **Trust the Monolith**: Keep `fusion watch` running. The shared state makes your tools smarter.
-   **Annotate Asynchronously**: If a function does I/O, mark it `async`. Blocking the main thread is an anti-pattern.

### Don't:
-   **Ignore Security Warnings**: If `fusion audit` flags a dependency, do not suppress it without a rigorous manual review.
-   **Manually Manage Tensors**: Avoid writing manual loops for matrix math. Use HAFT operators (`tensor_a * tensor_b`) to let the autonomous agents optimize execution.
-   **Mix Modes Carelessly**: Be careful when passing data between `@borrowed` code and GC code. The compiler handles it, but extensive copying can hurt performance.

---

## 12. Appendices {#appendices}

### A. Glossary
-   **HAFT**: Hyper-Adaptive Flux Tensor. The intelligent array primitive.
-   **Monolith**: The unified compiler/toolchain process.
-   **Flux-Resolve**: The GPU-accelerated dependency solver.
-   **Agent**: An autonomous background thread optimizing runtime state.

### B. Cheat Sheet
-   `fusion new <name>` - Create project
-   `fusion run` - Build and run
-   `fusion audit` - Security Check
-   `@borrowed` - Zero-copy mode
-   `@gpu_accelerated` - CUDA/OpenCL target

---
*Updated for Fusion v3.4 (Monolith Era)*
