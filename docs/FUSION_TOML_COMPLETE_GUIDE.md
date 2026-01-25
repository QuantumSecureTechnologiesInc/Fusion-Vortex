# Fusion.toml: The All-in-One Manifest

**The Revolutionary Unified Configuration System**

---

## Overview

**Fusion.toml** is Fusion's revolutionary all-in-one project manifest that eliminates the need for multiple configuration files. Unlike traditional ecosystems where you need separate files for dependencies (Fusion.toml), package management (package.json), build configuration (CMakeLists.txt), runtime settings, security policies, and more, **Fusion.toml consolidates everything into a single, comprehensive manifest**.

This unified approach means:
- ✅ **One file** instead of 5-10 separate configuration files
- ✅ **No context switching** between different configuration formats
- ✅ **Consistent syntax** across all configuration domains
- ✅ **Automatic validation** of cross-domain constraints
- ✅ **Single source of truth** for your entire project

---

## Complete Fusion.toml Structure

Here's a comprehensive example showing all available sections:

```toml

# ============================================================================


# PACKAGE METADATA


# ============================================================================

[package]
name = "my-fusion-project"
version = "1.0.0"
edition = "2024"                    # Language edition (2021, 2024)
authors = ["Your Name <you@example.com>"]
description = "A quantum-secure AI application"
license = "MIT OR Apache-2.0"
repository = "https://github.com/username/project"
homepage = "https://project.example.com"
documentation = "https://docs.project.example.com"
readme = "README.md"
keywords = ["quantum", "ai", "cryptography"]
categories = ["science", "cryptography"]

# ============================================================================


# DEPENDENCIES


# ============================================================================

[dependencies]

# Fusion standard library

fusion_std = "0.2.0"

# Quantum computing

fusion_quantum = "0.2.0"
fusion_qaoa = "0.2.0"
fusion_vqe = "0.2.0"

# AI/ML

fusion_ai_core = "0.2.0"
fusion_transformers = "0.2.0"
fusion_llm_inference_engine = "0.2.0"

# Cryptography

fusion_crypto_pqc = "0.2.0"
sentinel_tribrid = "1.0.0"

# Web framework

fusion_web = "0.2.0"
fusion_http = "0.2.0"

# Async runtime

tokio = { version = "1.42", features = ["full"] }

# Serialization

serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Optional dependencies (enabled by features)

fusion_k8s_operator = { version = "0.2.0", optional = true }
fusion_cloud_aws = { version = "0.2.0", optional = true }

[dev-dependencies]

# Test-only dependencies

fusion_test_utils = "0.2.0"
proptest = "1.6"
criterion = "0.5"

# ============================================================================


# POLYGLOT LANGUAGE SUPPORT


# ============================================================================

[languages.cpp]
enabled = true
standard = "c++20"                  # c++17, c++20, c++23
compiler = "clang++"                # clang++, g++, msvc
sources = ["src/native/physics.cpp", "src/native/math.cpp"]
include_dirs = ["include/", "vendor/eigen/"]
libraries = ["boost", "eigen"]
compile_flags = ["-O3", "-march=native"]

[languages.python]
enabled = true
version = "3.11"                    # Minimum Python version
requirements = ["numpy>=1.24", "pytorch>=2.0", "pandas"]
entry_point = "src/ai/model.py"
virtual_env = ".venv"               # Virtual environment path
package_manager = "uv"              # pip, uv, poetry

[languages.javascript]
enabled = true
manager = "bun"                     # npm, yarn, pnpm, bun
packages = {
    react = "^18.0",
    typescript = "^5.0",
    "@types/node" = "^20.0"
}
entry_point = "src/ui/app.tsx"

# ============================================================================


# RUNTIME CONFIGURATION


# ============================================================================

[runtime]
profile = "supernova"               # Options: v1, nebula, nebula_2_1, supernova
target = "native"                   # native, wasm, wasm32-wasi

# Supernova Runtime v3.0 settings

[runtime.supernova]
heterogeneous_execution = true      # Enable CPU/GPU/QPU dispatch
gpu_backends = ["cuda", "vulkan", "metal"]
quantum_backends = ["simulator", "aws-braket", "ibm-quantum"]
max_threads = "auto"                # auto, or specific number
memory_pool_size = "4GB"            # GPU memory pool

# Thread pool configuration

[runtime.thread_pool]
worker_threads = "auto"             # Number of worker threads
stack_size = "2MB"                  # Stack size per thread
work_stealing = true                # Enable work-stealing scheduler

# ============================================================================


# BUILD CONFIGURATION


# ============================================================================

[build]
optimization_level = 2              # 0 (debug) to 3 (max optimization)
debug_info = true                   # Include debug symbols
incremental = true                  # Enable incremental compilation
parallel = true                     # Parallel compilation
lto = false                         # Link-time optimization (slow but smaller/faster)
codegen_units = 16                  # Number of codegen units (lower = better optimization, slower compile)

# Target-specific settings

[build.target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "target-cpu=native"]

[build.target.wasm32-unknown-unknown]
opt_level = "z"                     # Optimize for size
strip = true                        # Strip debug info

# ============================================================================


# MONOLITH (INCREMENTAL COMPILATION ENGINE)


# ============================================================================

[monolith]
enabled = true                      # Enable Monolith for fast builds
persistence_path = ".fusion/cache"  # AST cache location
max_cache_size = "10GB"             # Maximum cache size
daemon_timeout = "30m"              # Daemon idle timeout
incremental_threshold = 0.1         # Rebuild if >10% of files changed

# ============================================================================


# HAFT (HYPER-ADAPTIVE FLUX TENSOR)


# ============================================================================

[haft]
enabled = true                      # Enable HAFT for tensor memory management
gpu_memory_limit = "8GB"            # Maximum GPU memory
ram_memory_limit = "32GB"           # Maximum RAM
ssd_cache_path = "/tmp/haft-cache"  # SSD cache for large tensors
tiering_strategy = "adaptive"       # adaptive, manual, aggressive

# HAFT Agent Configuration

[haft.agents]
researcher_interval = "100ms"       # How often Researcher analyzes access patterns
builder_threshold = 0.7             # Move to GPU if access frequency > 70%
optimizer_prefetch = true           # Enable predictive prefetching

# ============================================================================


# SENTINEL TRIBRID (SECURITY)


# ============================================================================

[sentinel]
enabled = true                      # Enable Sentinel security
mode = "tribrid"                    # tribrid, pq-only, classical-only

# Cryptographic configuration

[sentinel.crypto]
key_exchange = "hybrid"             # hybrid (X25519+ML-KEM), pq-only, classical-only
signatures = "hybrid"               # hybrid (Ed25519+ML-DSA), pq-only, classical-only
encryption = "aes-256-gcm"          # Symmetric encryption
pq_security_level = 3               # NIST security level (1, 3, 5)

# Chaos engine settings

[sentinel.chaos]
enabled = true                      # Enable chaos-based entropy
initial_state = "random"            # random, or specific seed
rotation_interval = "1h"            # Key rotation interval
entropy_threshold = 0.95            # Minimum entropy threshold

# ============================================================================


# CORTEX AI SCHEDULER


# ============================================================================

[cortex]
enabled = true                      # Enable Cortex AI scheduler
learning_mode = "adaptive"          # adaptive, aggressive, conservative
profiling_interval = "1s"           # How often to collect metrics
optimization_threshold = 0.8        # Optimize if confidence > 80%

# GPU scheduling

[cortex.gpu]
auto_dispatch = true                # Automatically dispatch to GPU
batch_size_optimization = true      # Optimize batch sizes
kernel_fusion = true                # Fuse multiple kernels

# ============================================================================


# QUANTUM COMPUTING


# ============================================================================

[quantum]
default_backend = "simulator"       # simulator, aws-braket, ibm-quantum, google-quantum-ai
max_qubits = 25                     # Maximum qubits for simulation
shots = 1000                        # Default number of measurement shots

# Cloud quantum backends

[quantum.aws_braket]
enabled = false
region = "us-east-1"
device = "arn:aws:braket:::device/quantum-simulator/amazon/sv1"

[quantum.ibm_quantum]
enabled = false
api_token = "${IBM_QUANTUM_TOKEN}"  # Environment variable
hub = "ibm-q"
group = "open"
project = "main"

# ============================================================================


# AI/ML CONFIGURATION


# ============================================================================

[ai]
provider = "ollama"                 # ollama, gpt-oss, mistral, phi, gemma, qwen, deepseek, openai
default_device = "cuda:0"           # cuda:0, cpu, vulkan, metal
mixed_precision = true              # Enable FP16/BF16 training
gradient_checkpointing = true       # Save memory during training

# Model serving

[ai.serving]
batch_size = 32                     # Inference batch size
timeout = "5s"                      # Request timeout
quantization = "int8"               # none, int8, int4

# Distributed training

[ai.distributed]
backend = "nccl"                    # nccl, gloo, mpi
world_size = 4                      # Number of GPUs
rank = 0                            # Current process rank

# Local/hosted LLM providers (env expansion supported)

[ai.ollama]
base_url = "http://localhost:11434"
model = "llama3"

[ai.gpt_oss]
base_url = "http://localhost:11435/v1"
model = "gpt-oss"

[ai.mistral]
api_key = "${MISTRAL_API_KEY}"
base_url = "https://api.mistral.ai/v1"
model = "mistral-large-latest"

[ai.phi]
base_url = "http://localhost:11436/v1"
model = "phi-3.5-mini"

[ai.gemma]
base_url = "http://localhost:11437/v1"
model = "gemma-2-9b-it"

[ai.qwen]
api_key = "${QWEN_API_KEY}"
base_url = "https://dashscope.aliyuncs.com/compatible-mode/v1"
model = "qwen2.5-72b-instruct"

[ai.deepseek]
api_key = "${DEEPSEEK_API_KEY}"
base_url = "https://api.deepseek.com/v1"
model = "deepseek-chat"

# ============================================================================


# WEB SERVER


# ============================================================================

[web]
host = "0.0.0.0"
port = 8080
workers = "auto"                    # Number of worker threads
max_connections = 10000
request_timeout = "30s"
keep_alive = "75s"

# TLS configuration

[web.tls]
enabled = true
cert_path = "certs/server.crt"
key_path = "certs/server.key"
pq_cipher_suites = [
    "TLS_KYBER768_AES_256_GCM_SHA384",
    "TLS_ECDHE_KYBER768_AES_256_GCM_SHA384"
]

# ============================================================================


# DATABASE


# ============================================================================

[database]
url = "postgresql://localhost/mydb"
max_connections = 20
min_connections = 5
connection_timeout = "5s"
idle_timeout = "10m"

# ============================================================================


# LOGGING & TELEMETRY


# ============================================================================

[logging]
level = "info"                      # trace, debug, info, warn, error
format = "json"                     # json, text, pretty
output = "stdout"                   # stdout, stderr, file path

[telemetry]
enabled = true
endpoint = "https://telemetry.example.com"
sample_rate = 0.1                   # Sample 10% of requests
metrics = ["latency", "throughput", "errors"]

# ============================================================================


# TESTING


# ============================================================================

[test]
parallel = true                     # Run tests in parallel
timeout = "60s"                     # Test timeout
coverage = true                     # Collect code coverage
coverage_threshold = 80             # Minimum coverage percentage

# ============================================================================


# BENCHMARKING


# ============================================================================

[bench]
warmup_time = "3s"                  # Warmup duration
measurement_time = "5s"             # Measurement duration
sample_size = 100                   # Number of samples
confidence_level = 0.95             # Statistical confidence

# ============================================================================


# FEATURES (CONDITIONAL COMPILATION)


# ============================================================================

[features]
default = ["std", "async"]          # Default features
std = []                            # Standard library
async = ["tokio"]                   # Async runtime
kubernetes = ["fusion_k8s_operator"] # Kubernetes support
aws = ["fusion_cloud_aws"]          # AWS integration
full = ["std", "async", "kubernetes", "aws"] # All features

# ============================================================================


# WORKSPACE (MULTI-CRATE PROJECTS)


# ============================================================================

[workspace]
members = [
    "crates/*",
    "services/*",
    "tools/*"
]
exclude = [
    "examples/*",
    "benchmarks/*"
]

# Shared dependencies across workspace

[workspace.dependencies]
fusion_std = "0.2.0"
tokio = "1.42"
serde = "1.0"

# ============================================================================


# LINTING & CODE QUALITY


# ============================================================================

[lints.fusion]
unsafe_code = "deny"                # deny, warn, allow
unused_imports = "warn"
dead_code = "warn"
missing_docs = "warn"

[lints.flux]
all = "warn"
pedantic = "warn"
nursery = "allow"

# ============================================================================


# DOCUMENTATION


# ============================================================================

[docs]
generate = true                     # Generate documentation
output_dir = "target/doc"
private_items = false               # Include private items
examples = true                     # Include examples

# ============================================================================


# DEPLOYMENT


# ============================================================================

[deploy]
target = "kubernetes"               # kubernetes, docker, binary
registry = "ghcr.io/username"       # Container registry

[deploy.kubernetes]
namespace = "production"
replicas = 3
resources = { cpu = "1000m", memory = "2Gi" }

[deploy.docker]
base_image = "debian:bookworm-slim"
expose_ports = [8080, 9090]

# ============================================================================


# SECURITY POLICIES


# ============================================================================

[security]
audit_mode = "strict"               # strict, permissive, disabled
secrets_detection = true            # Detect hardcoded secrets
dependency_scanning = true          # Scan dependencies for vulnerabilities
sbom_generation = true              # Generate Software Bill of Materials

# ============================================================================


# CUSTOM SCRIPTS


# ============================================================================

[scripts]
pretest = "fusion check"            # Run before tests
postbuild = "fusion doc"            # Run after build
deploy = "kubectl apply -f k8s/"    # Custom deployment script
```text

---

## Key Advantages of Fusion.toml

### 1. **Unified Configuration**

Instead of managing multiple files:

```text
❌ Traditional Project:
├── Fusion.toml          # Fusion dependencies
├── package.json        # JavaScript dependencies
├── requirements.txt    # Python dependencies
├── CMakeLists.txt      # C++ build config
├── .env                # Environment variables
├── docker-compose.yml  # Container config
├── k8s/                # Kubernetes manifests
└── config/             # Application config

✅ Fusion Project:
├── Fusion.toml         # Everything in one file!
└── src/
```text

### 2. **Cross-Domain Validation**

Fusion.toml validates constraints across domains:

```toml
[runtime]
profile = "nebula"      # WASM sandbox

[languages.cpp]
enabled = true          # ERROR: C++ not allowed in Nebula strict mode!
```text

The build system catches this immediately, preventing configuration errors.

### 3. **Polyglot Support**

Build projects mixing multiple languages:

```toml
[languages.cpp]
sources = ["src/physics_engine.cpp"]

[languages.python]
requirements = ["numpy", "scipy"]

[languages.javascript]
packages = { react = "^18.0" }
```text

Fusion Forge automatically:
- Determines build order (C++ → Fusion → Python/JS)
- Generates FFI bindings
- Links everything together

### 4. **Runtime Configuration**

Configure the Supernova Runtime directly:

```toml
[runtime.supernova]
heterogeneous_execution = true
gpu_backends = ["cuda", "vulkan"]
quantum_backends = ["aws-braket"]
```text

No separate runtime config files needed.

### 5. **Security Integration**

Security policies in the same file:

```toml
[sentinel.crypto]
key_exchange = "hybrid"     # X25519 + ML-KEM
pq_security_level = 3       # NIST Level 3

[security]
audit_mode = "strict"
secrets_detection = true
```text

### 6. **Feature Flags**

Conditional compilation with features:

```toml
[features]
default = ["std"]
kubernetes = ["fusion_k8s_operator"]
aws = ["fusion_cloud_aws"]
full = ["kubernetes", "aws"]
```text

Build with specific features:

```bash
fusion build --features kubernetes,aws
```text

---

## Comparison with Other Systems

| Feature           | Fusion.toml | Fusion.toml | package.json | CMakeLists.txt |
| ----------------- | ----------- | ---------- | ------------ | -------------- |
| Dependencies      | ✅           | ✅          | ✅            | ⚠️ Manual       |
| Build Config      | ✅           | ⚠️ Limited  | ❌            | ✅              |
| Runtime Config    | ✅           | ❌          | ❌            | ❌              |
| Polyglot Support  | ✅           | ❌          | ❌            | ⚠️ Manual       |
| Security Policies | ✅           | ❌          | ❌            | ❌              |
| Deployment        | ✅           | ❌          | ⚠️ Scripts    | ❌              |
| AI/ML Config      | ✅           | ❌          | ❌            | ❌              |
| Quantum Config    | ✅           | ❌          | ❌            | ❌              |
| Cross-validation  | ✅           | ❌          | ❌            | ❌              |

---

## Flux.lock: Dependency Lock File

Fusion.toml works with **Flux.lock**, the dependency lock file:

```toml

# Flux.lock (auto-generated, commit to version control)

version = 1

[[package]]
name = "fusion_std"
version = "0.2.0"
source = "registry+https://crates.fusion-lang.org"
checksum = "sha256:abc123..."

[[package]]
name = "fusion_quantum"
version = "0.2.0"
source = "registry+https://crates.fusion-lang.org"
checksum = "sha256:def456..."
dependencies = [
    "fusion_std 0.2.0",
    "num_complex 0.4.0"
]
```text

**Flux.lock ensures**:
- Reproducible builds across machines
- Exact dependency versions
- Cryptographic verification (checksums)
- Dependency tree resolution

---

## Environment Variables

Fusion.toml supports environment variable substitution:

```toml
[database]
url = "${DATABASE_URL}"             # From environment

[quantum.ibm_quantum]
api_token = "${IBM_QUANTUM_TOKEN}"  # Secure secrets

[deploy]
registry = "${CONTAINER_REGISTRY:-ghcr.io/default}"  # With default
```text

---

## Conclusion

**Fusion.toml** represents a fundamental rethinking of project configuration. By consolidating all configuration into a single, comprehensive manifest with cross-domain validation and intelligent defaults, Fusion eliminates the complexity and fragmentation of traditional multi-file configuration systems.

**Key Benefits**:
1. ✅ **One file** for everything
2. ✅ **Polyglot support** (Fusion, C++, Python, JavaScript)
3. ✅ **Runtime configuration** (Supernova, HAFT, Sentinel, Cortex)
4. ✅ **Security policies** built-in
5. ✅ **Cross-domain validation** prevents configuration errors
6. ✅ **Feature flags** for conditional compilation
7. ✅ **Deployment configuration** included
8. ✅ **AI/ML and Quantum** settings native

**Welcome to unified configuration. Welcome to Fusion.toml.**

---

**QuantumSecure Technologies Ltd** © 2026
**Version**: 0.2.0-beta.1
**Last Updated**: 20 January 2026
