# Fusion Crate Registry - Complete Reference

**Dataset Category**: API Reference
**Training Level**: Intermediate to Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

Fusion's ecosystem consists of 250+ crates organized into functional categories. This reference provides a complete catalog for AI training on the entire Fusion crate ecosystem.

## Crate Organization

### Core Language Crates

#### `fusion_core` (v1.0.0)

**Description**: Core compiler and language implementation
**Key Modules**:
- `compiler` - Main compiler pipeline
- `lexer` - Tokenization
- `parser` - AST generation
- `typechecker` - Type inference and checking
- `vm` - Virtual machine execution

**Common Imports**:

```fusion
use fusion_core::{Compiler, CompilerOptions, Error}
```text

#### `fusion_std` (v1.0.0)

**Description**: Standard library
**Key Modules**:
- `collections` - HashMap, HashSet, Vec, LinkedList
- `io` - Input/output operations
- `fs` - File system operations
- `net` - Networking primitives
- `sync` - Synchronization primitives (Mutex, RwLock, channels)
- `time` - Time and date utilities

**Common Imports**:

```fusion
use std::collections::HashMap
use std::io::{Read, Write}
use std::fs::File
```text

### Runtime and Execution

#### `fusion_runtime_core` (v1.0.0)

**Description**: Core runtime system
**Key Features**:
- Async executor
- Task scheduling
- Memory management
- Thread pool

**Common Imports**:

```fusion
use fusion::runtime::{Runtime, spawn}
```text

#### `fusion_runtime_core_v2_0_nebula` (v2.0)

**Description**: Next-generation runtime with AI scheduling
**Key Features**:
- **Cortex**: AI-driven task scheduler
- **HAL**: Hardware Abstraction Layer
- **QEM**: Quantum Error Mitigation for memory management

**Common Imports**:

```fusion
use fusion::runtime::nebula::{Cortex, HAL, QEM}
```text

#### `fusion_runtime_scheduler` (v1.0.0)

**Description**: Advanced task scheduling
**Key Features**:
- Work-stealing scheduler
- Priority queues
- CPU affinity
- Load balancing

### AI and Machine Learning

#### `fusion_ai_core` / `fusion-ai-core` (v1.0.0)

**Description**: Core AI/ML functionality
**Key Modules**:
- `nn` - Neural network layers
- `tensor` - Tensor operations
- `optim` - Optimizers (SGD, Adam, etc.)
- `loss` - Loss functions

**Common Imports**:

```fusion
use fusion::ai_core::nn::{Linear, ReLU, Sequential}
use fusion::ai_core::tensor::Tensor
```text

#### `fusion_ai_core_adapters` (v1.0.0)

**Description**: Adapters for external AI frameworks
**Supports**: PyTorch, TensorFlow, ONNX

#### `haft-fusion` / HAFT Crates

**Description**: Hyper-Adaptive Flux Tensors
**Related Crates**:
- `fusion-haft` - Core HAFT implementation
- `haft-researcher` - Access pattern analysis agent
- `haft-builder` - Memory tier management agent
- `haft-optimizer` - Layout optimization agent

### Large Language Models (17 crates)

- `llm-attention-mask` - Attention mask generation
- `llm-auto-prompt` - Automatic prompt engineering
- `llm-beam-search` - Beam search decoding
- `llm-cache-compression` - KV cache compression
- `llm-custom-tokenizer` - Custom tokenization
- `llm-data-tokenizer` - Data tokenizers
- `llm-distill` - Model distillation
- `llm-distributed-training` - Distributed training
- `llm-dynamic-batch` - Dynamic batching
- `llm-gqa-kernel` - Grouped-query attention kernels
- `llm-inference` - Inference engine
- `llm-llama` - LLaMA model implementation
- `llm-lora-kernel` - LoRA adaptation kernels
- `llm-lora-manager` - LoRA weight management
- `llm-quantization` - Model quantization
- `llm-rag` - Retrieval-Augmented Generation
- `llm-tensor-parallel` - Tensor parallelism

**Example Usage**:

```fusion
use llm_inference::{LLM, GenerationConfig}
use llm_llama::LlamaModel

let model = LlamaModel::from_pretrained("llama-2-70b")?
let config = GenerationConfig {
    max_tokens: 100,
    temperature: 0.7,
    top_p: 0.9
}

let output = model.generate("Once upon a time", config)?
```text

### Neural Network Layers (15 crates)

- `nn-3d-conv` - 3D convolutions
- `nn-attention-block` - Attention mechanisms
- `nn-embed` - Embedding layers
- `nn-gan-layers` - GAN-specific layers
- `nn-gcn` - Graph Convolutional Networks
- `nn-gnn` - Graph Neural Networks
- `nn-layer-norm` - Layer normalization
- `nn-lstm` - LSTM cells
- `nn-maxpool` - Max pooling
- `nn-metrics` - Training metrics
- `nn-norm` - Normalization layers
- `nn-pooling` - Pooling layers
- `nn-rbf` - Radial Basis Function networks
- `nn-resnet` - ResNet architectures
- `nn-rnn` - Recurrent layers

### Quantum Computing (15 crates)

- `fusion_quantum` / `quantum-sdk` - Main quantum SDK
- `q-algo` - Quantum algorithms (VQE, QAOA, Grover, Shor)
- `q-sim` - Quantum simulator
- `q-error-correction` - Error correction codes
- `q-gate-decomposition` - Gate decomposition
- `q-measurement-opt` - Measurement optimization
- `q-optimizer-hybrid` - Classical-quantum hybrid optimizers
- `q-pulse-seq` - Pulse sequence generation
- `q-visualization` - Circuit visualization
- `q-aws-backend` - AWS Braket integration
- `q-ibm-backend` - IBM Quantum integration
- `q-pqc-proxy` - Post-quantum cryptography proxy
- `qaoa` - QAOA implementation
- `qubo` - QUBO problem solver
- `jordan-wigner` - Jordan-Wigner transformation

**Example**:

```fusion
use fusion_quantum::{QuantumCircuit, Simulator}

let mut circuit = QuantumCircuit::new(2)
circuit.h(0)
circuit.cnot(0, 1)

let simulator = Simulator::new()
let result = simulator.run(&circuit, shots=1000)?
```text

### Security (12 crates)

- `sentinael-tribrid` - Autonomous security agent with Chaos Math Engine
- `fusion_cryptography` - Cryptographic primitives
- `fusion-policy` - Policy engine for capability enforcement
- `sec-forensics` - Security forensics
- `sec-incident-response` - Incident response automation
- `sec-network-segmentation` - Network segmentation
- `sec-os-hardener` - OS hardening
- `sec-penetration` - Penetration testing tools
- `sec-policy-compiler` - Policy compilation
- `sec-policy-engine` - Policy enforcement
- `sec-secrets-auditor` - Secrets detection and auditing
- `sec-threat-intel` - Threat intelligence integration
- `sec-trusted-anchor` - Trusted execution environment

### Cloud and Infrastructure (12 crates)

- `cloud-agent` - Cloud resource management agent
- `cloud-aws` - AWS integration
- `cloud-azure` - Azure integration
- `cloud-gcp` - Google Cloud integration
- `k8s-operator` - Kubernetes operator framework
- `fusion-faas` - Function-as-a-Service runtime
- `deploy` - Deployment orchestration
- `fusion-observability` - Observability and monitoring
- `telemetry-ingestor` - Telemetry data ingestion
- `metrics` - Metrics collection
- `fusion_telemetry` - Distributed tracing

### Networking (8 crates)

- `fusion_net` - Core networking
- `fusion_http` / `http` - HTTP client/server
- `grpc` - gRPC support
- `graphql` / `fusion-graphql` - GraphQL server
- `rest-server` / `fusion-rest-server` - REST API server
- `fusion-service-router` / `router-mesh` - Service mesh routing
- `fusion-rate-limiter` - Rate limiting
- `socket2` - Low-level sockets

### Web Development (5 crates)

- `fusion-web-server` - High-level web framework
- `fusion-wasm-runtime` - WebAssembly runtime
- `fusion-wasm-server` - WASM HTTP server
- `fusion-webasm-renderer` - WASM UI rendering
- `fusion-react-bridge` - React integration

### Data Processing (12 crates)

- `fusion_database` - Database abstraction
- `fusion-redis` - Redis client and server
- `fusion-compression` - Data compression
- `fusion-xml` - XML parsing/generation
- `fusion-yaml` - YAML parsing/generation
- `fusion-regex` - Regular expressions
- `fusion-trie-search` - Trie-based search
- `fusion-schema-validator` - Schema validation
- `safetensors` - SafeTensors format support
- `fusion-stream-monitor` - Stream processing monitoring
- `kv-cache` - Key-value cache

### Build and Tooling (15 crates)

- `flux-resolve-v2-hive-mind` - v2.0 dependency resolution with GPU and Redis
- `fusion-flux-resolve` - Original Flux-Resolve
- `cargo-converter` - Cargo.toml converter (legacy import to Fusion.toml)
- `python-converter` - Python code converter
- `fusion-crate-analyzer` / `crate-analyzer` - Crate analysis
- `fusion-sbom-generator` / `sbom-generator` - SBOM generation
- `fusion-supply-chain` - Supply chain security
- `fusion-audit` / `audit` - Security auditing
- `fusion-sandbox` - Sandbox execution
- `fusion-safety-monitor` - Runtime safety monitoring
- `fusion-fuzz-harness` - Fuzzing harness
- `tester` - Testing framework
- `profiler` - Performance profiling
- `debugger` - Debugger
- `formatter` - Code formatter

### IDE and Developer Experience (8 crates)

- `fusion-vscode-runtime` / `vscode-runtime` - VSCode extension runtime
- `fusion-mcp` / `mcp` - Model Context Protocol server
- `fusion-terminal-browser` - Terminal-based code browser
- `fusion-diagnostics` / `diagnostics` - Diagnostic messages
- `fusion-id-provider` - Language Server Protocol
- `docgen` - Documentation generator
- `fusion-layout-builder` / `layout-builder` - UI layout builder
- `fusion-component-lib` / `component-lib` - Reusable UI components

### Mathematics and Algorithms (8 crates)

- `fusion_math` / `fusion-math` - Mathematical functions
- `fusion_physics` - Physics simulations
- `math-finite-fields` - Finite field arithmetic
- `math-sparse` / `math-sparse-ops` / `math-tensor-sparse` - Sparse matrix operations
- `fusion-optimization` - Optimization algorithms
- `solver` - Equation solvers
- `clustering` / `fusion-clustering` - Clustering algorithms

### Multimedia (5 crates)

- `fusion-image` - Image processing
- `fusion-video` - Video processing
- `fusion-audio` - Audio processing
- `fusion-vision` - Computer vision
- `fusion-charts` - Data visualization charts
- `fusion-data-vis`- Data visualization

### Utilities (12 crates)

- `fusion-calendar` - Calendar and date utilities
- `fusion-geo` - Geospatial operations
- `fusion-mail` - Email client/server
- `fusion-iot` - IoT device integration
- `fusion-blockchain` - Blockchain primitives
- `fusion_finance` - Financial calculations
- `fusion-event-bus` - Event bus
- `fusion-ui` - UI framework
- `retry` / `fusion_retry` - Retry logic
- `vault` - Secrets management
- `version` - Version management
- `tree` - Tree data structures

### Interoperability (5 crates)

- `interop-java` - Java interop
- `interop-js` - JavaScript interop
- `interop-python` - Python interop
- `interop-python-pkgmgr` - Python package management integration
- `bridge_c` - C FFI bridge

### Hardware and GPU (8 crates)

- `fusion_runtime_hal` - Hardware Abstraction Layer
- `fusion-gpu-scheduler` / `gpu-scheduler` - GPU task scheduling
- `fusion-vram-scheduler` / `vram-scheduler` - VRAM management
- `fusion-cuda-kernel` / `cuda-kernels` - CUDA kernel compilation
- `cuda-interface` - CUDA interface
- `ash` - Vulkan bindings
- `metal` - Metal (macOS GPU) bindings

### Advanced Runtime Features (5 crates)

- `fusion_runtime_mem_mgr` / `fusion-memory-manager` - Memory management
- `fusion-cortex` - AI task scheduler (part of Nebula)
- `executor` - Custom executors
- `tensorweave` - Tensor orchestration
- `termblink` - Ultra-fast terminal rendering

---

## Crate Dependencies

### Common Dependency Patterns

**Most projects import**:

```toml
[dependencies]
fusion_core = "0.2.0"
fusion_std = "0.2.0"
fusion_runtime_core = "0.2.0"
```text

**AI/ML projects add**:

```toml
fusion_ai_core = "0.2.0"
haft-fusion = "0.2.0"
```text

**Web projects add**:

```toml
fusion-web-server = "0.2.0"
fusion_database = "0.2.0"
```text

**Quantum projects add**:

```toml
fusion_quantum = "0.2.0"
q-sim = "0.2.0"
```text

---

## Key Takeaways for AI Training

1. **250+ Crates**: Comprehensive ecosystem covering all domains
2. **Modular Design**: Mix and match crates as needed
3. **Consistent Versioning**: Most crates at 0.2.0
4. **Domain Coverage**: AI/ML (30+), Quantum (15+), Web (10+), Security (12+)
5. **Runtime Options**: Standard runtime or Nebula v2.0 with AI scheduling
6. **GPU Support**: Dedicated crates for CUDA, Vulkan, Metal
7. **Cloud-Native**: AWS, Azure, GCP, Kubernetes integrations
8. **Developer Tools**: IDE integration, MCP, debugging, profiling
9. **Interop**: Java, Python, JavaScript, C interoperability
10. **Security-First**: 12+ security-focused crates

This reference provides comprehensive coverage of the Fusion crate ecosystem. When generating code or answering questions, refer to specific crate capabilities and their typical usage patterns. Cross-reference with Code Examples and API Reference datasets for detailed usage.