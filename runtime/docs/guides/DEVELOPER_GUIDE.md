# Developer Guide: Fusion Runtime Core

**Document Version:** 1.1 (Aligned with v0.3.0)  
**Audience:** Core Contributors, System Architects, Low-Latency Engineers  
**Scope:** Architecture Internals, Build Systems, CI/CD, and Hardware Integration

---

## 1. Architecture Deep-Dive

Fusion Runtime Core is not merely a task scheduler; it is a **Hybrid Compute Orchestrator**. It operates on a "Split-Plane" architecture to isolate critical financial operations from high-latency quantum tasks.

### 1.1 The Three Planes of Existence

The runtime partitions the process address space into three distinct planes:

#### 1. The Classical Plane (Host)

- **Role:** Handles networking (DPDK), file I/O, and HFT logic.
- **Constraints:** Strict zero-allocation after initialisation.
- **Thread Model:** 1:1 pinning (one thread per physical core) using `isolcpus`.

#### 2. The Accelerator Plane (Throughput)

- **Role:** Manages GPU and TPU tensor operations.
- **Mechanism:** Uses the Cortex (AI Scheduler) to predict kernel costs. If a tensor operation is predicted to take **>5ms**, it is pushed to a dedicated async stream to prevent blocking the Classical Plane.

#### 3. The Quantum Plane (Probabilistic)

- **Role:** Interfaces with QPU drivers (IBM Q, Rigetti, or Simulators).
- **Middleware:** The QEM Layer (Quantum Error Mitigation) sits here, injecting dynamical decoupling pulses automatically before submission.

### 1.2 The Cortex (AI Scheduler) Internals

The Cortex is located in `crates/fusion_cortex`. It replaces the traditional "Round-Robin" scheduler.

- **Input:** Task metadata (Complexity, Memory Footprint, Priority).
- **Model:** A quantised Reinforcement Learning (RL) model loaded at runtime.
- **Decision Loop:**
  1. User spawns a task with `Intent::HighThroughput`.
  2. Cortex queries the `CostModel`.
  3. If `Prediction(GPU) < Prediction(CPU) + TransferCost`, schedule to GPU.
  4. Otherwise, run locally on CPU thread pool.

### 1.3 Memory Model: "Entangled Pools"

We do not use `malloc` during runtime execution. Instead, we use a Slab Allocator setup at startup.

- **Unified Bridge:** A shared memory segment mapped to both GPU VRAM (via CUDA UVA) and Host RAM.
- **Safety:** Rust's ownership rules are enforced via `Arc<T>` wrappers that track device residency. Dropping a handle on the CPU automatically decrements the refcount on the GPU/QPU driver.

---

## 2. Build & Development Environment

Building a hybrid runtime requires a complex toolchain.

### 2.1 Prerequisites

| Component | Requirement                                                             |
| --------- | ----------------------------------------------------------------------- |
| Language  | Rust (Nightly channel required for `simd` and `allocator_api` features) |
| GPU       | CUDA Toolkit 12.0+ or ROCm 5.0+                                         |
| Quantum   | Python 3.9+ (for Qiskit/Cirq bindings if running simulators)            |
| System    | Linux kernel 5.15+ (required for `io_uring` and eBPF support)           |

### 2.2 Workspace Setup

```bash
# Clone the repository
git clone https://github.com/QuantumSecureTechnologiesInc/fusion.git
cd fusion/runtime

# Install system dependencies (Ubuntu/Debian)
sudo apt-get install libssl-dev pkg-config protobuf-compiler libclang-dev

# Setup the specific toolchain
rustup override set nightly
rustup component add rustfmt clippy

# Build the runtime core only
cargo build -p fusion_runtime_core --release
```

### 2.3 Feature Flags

We use feature flags to allow building on machines without specific hardware:

| Flag                    | Description                                                      |
| ----------------------- | ---------------------------------------------------------------- |
| `--features gpu`        | Enables CUDA/HIP compilation                                     |
| `--features quantum`    | Enables QPU bridge (requires Python venv)                        |
| `--features simulation` | Uses CPU simulation for Quantum/GPU (slow, but works on laptops) |
| `--features hft`        | Enables kernel-bypass networking (requires root/CAP_NET_ADMIN)   |

---

## 3. Testing Strategy

Testing hybrid hardware is difficult. We employ a **Three-Tier Test Pyramid**.

### Tier 1: Unit Tests (Pure Rust)

Runs on any machine. Mocks all hardware interfaces.

```bash
cargo test --lib
```

### Tier 2: Simulation Integration

Runs full flows but uses software simulators (QASM Simulator for Quantum, CPU-Tensor for AI).

```bash
cargo test --features simulation
```

### Tier 3: Hardware-in-the-Loop (HITL)

Requires actual access to QPUs and GPUs. Usually run only in CI/CD pipelines connected to lab hardware.

```bash
# Requires environment variables for API keys
export IBMQ_API_TOKEN="sk_..."
cargo test --features hardware_verify -- --ignored
```

---

## 4. Continuous Integration (CI/CD) Pipeline

Our CI pipeline (GitHub Actions / GitLab CI) is configured to ensure stability across the three planes.

### Pipeline Stages

1. **Static Analysis:** `cargo clippy`, `cargo fmt`, and security audit (`cargo audit`).
2. **Simulation Suite:** Runs the full test suite in simulation mode.
3. **Cortex Training:**
   - Fetches execution logs from the previous release.
   - Retrains the scheduler's cost prediction model.
   - Commits the new model weights to `crates/fusion_cortex/models/`.
4. **Hardware Sanity:**
   - Submits a "Hello World" circuit to a real QPU (if quota allows).
   - Fails if decoherence rates are above threshold (automated quality gate).

### Docker Environment

We provide a standard developer container to avoid toolchain hell.

```dockerfile
# Dockerfile.dev
FROM nvidia/cuda:12.0-devel-ubuntu22.04

# Install Rust & Quantum tools
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN pip3 install qiskit numpy

# Set up environment for Fusion
ENV FUSION_BACKEND="simulation"
WORKDIR /app
```

---

## 5. Contributing Guidelines

1. **Safety First:** No `unsafe` code allowed outside `fusion_hal`. All unsafe blocks must have a `// SAFETY:` comment explaining why constraints hold.

2. **Zero-Copy:** If your PR introduces a `memcpy`, it will be rejected by the linter unless marked with `#[allow(perf_violation)]`.

3. **Documentation:** All public APIs must have docstrings with examples.

4. **Testing:** All new features must include:
   - Unit tests
   - Integration tests (simulation mode)
   - Benchmarks (if performance-critical)

5. **Code Style:**
   - Follow Rust idioms
   - Use `rustfmt` for formatting
   - Maximum line length: 100 characters

---

## 6. Crate Overview

| Crate                      | Purpose                                                               |
| -------------------------- | --------------------------------------------------------------------- |
| `fusion_core`              | Core type system (FusionType, ClassicalType, TensorType, QuantumType) |
| `fusion_runtime_core`      | Main runtime orchestrator with FusionCore integration                 |
| `fusion_runtime_scheduler` | Heterogeneous task scheduler with priority queues                     |
| `fusion_runtime_mem_mgr`   | Zero-copy memory management across devices                            |
| `fusion_cortex`            | AI-powered scheduling engine (v0.3.0+)                                |
| `fusion_hal`               | Hardware Abstraction Layer with CUDA/pinned pools (v0.3.0+)           |
| `fusion_quantum`           | Quantum primitives with QEM middleware                                |
| `fusion_traits`            | Core traits for interweaving                                          |
| `fusion_tensor_core`       | Tensor operations                                                     |
| `fusion_quantum_core`      | Quantum primitives                                                    |

---

## 7. Debugging Tips

### Tracing

Enable detailed tracing with:

```bash
RUST_LOG=fusion_cortex=debug,fusion_runtime_core=trace cargo run
```

### Memory Profiling

Use the built-in memory stats:

```rust
let stats = runtime.memory_manager().stats();
println!("Peak usage: {} bytes", stats.peak_usage);
println!("Zero-copy transfers: {}", stats.zero_copy_transfers);
```

### Performance Monitoring

The Cortex exposes prediction accuracy metrics:

```rust
let cortex = CortexEngine::new();
// After training
let accuracy = cortex.evaluate_predictions(&test_logs);
```

---

*Document generated for Fusion Runtime Core v0.3.0*
