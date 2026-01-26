# Fusion v2.0 Vortex Core Merge Audit

**Date**: 2025-12-15
**Purpose**: Audit all dependencies on `fusion-core` and `fusion_core` before merging into single `fusion-core` crate

---

## Current State

### Two Crates Exist:

1. **`fusion_core`** (underscore) at `registry/crates/fusion_core`
   - NEW: Tri-brid type system (Classical, Tensor, Quantum)
   - Contains: types/, ops/, compiler/semantic, traits

2. **`fusion-core`** (hyphen) at `registry/crates/core`
   - OLD: Compiler infrastructure (package name: `fusion-compiler`)
   - Contains: lexer, parser, AST, VM, type_checker

---

## Dependency Analysis

### 🟢 Crates Using `fusion_core` (Underscore) - 131+ instances

#### Registry Crates (using workspace = true):

- wasm-server
- vram-scheduler
- webasm-renderer
- transform
- trusted-anchor
- vault
- version
- tensor-optim
- tensor-parallel
- tensor-sparse
- telemetry-ingestor
- trie-search
- tree
- training
- supply-chain
- std-ext
- solver
- sec-forensics
- sec-penetration
- sentinel-tribrid
- rl-algorithms
- schema-validator
- sdk-generator
- sbom-generator
- safetensors
- safety-monitor
- retry
- resnet
- sandbox-manager
- react-hooks
- qubo
- quantum-sdk
- qaoa
- q-sim
- q-ibm-backend
- q-aws-backend
- q-algo
- ops
- nn-rbf
- nn-metrics
- nn-gan-layers
- nn-embed
- nn-attention-block
- nn-3d-conv
- model-server-core
- llm-vision-adapter
- llm-stream-parser
- llm-tokenizers
- llm-rotary-opt
- llm-rerope
- llm-rag
- llm-quantization
- llm-moe-tools
- llm-mixtral-routing
- llm-lora-manager
- llm-lora-kernel
- llm-logits-processor
- llm-llama
- llm-inference
- llm-inference-graph
- llm-gqa-kernel
- llm-distributed-training
- llm-distillation
- llm-distill
- llm-dynamic-batch
- llm-data-tokenizer
- llm-custom-tokenizer
- llm-cache-compression
- llm-beam-search
- llm-auto-prompt
- llm-attention-mask
- kv-cache
- jordan-wigner
- k8s-operator
- interop-python-pkgmgr
- interop-python
- interop-js
- interop-java
- inference-graph
- haft-fusion
- grpc
- graphql
- graph
- gpu-scheduler
- github
- gate-decomposition
- fusion_ai_core ← registry crate
- fusion_quantum ← registry crate
- fusion_net ← registry crate
- fusion_finance ← registry crate
- fusion_runtime_scheduler ← registry crate
- fusion_runtime_mem_mgr ← registry crate
- fusion_runtime_hal ← registry crate
- fusion_runtime_core ← registry crate
- (81+ more in registry/crates not shown)

#### Runtime Crates (path dependencies):

- runtime/crates/fusion_runtime_scheduler → `../../../registry/crates/fusion_core/`
- runtime/crates/fusion_runtime_core → `../../../registry/crates/fusion_core/`
- runtime/crates/fusion_runtime_hal → `../../../registry/crates/fusion_core/`
- runtime/crates/fusion_runtime_mem_mgr → `../../../registry/crates/fusion_core/`
- runtime/crates/fusion_quantum → `../fusion_core`
- runtime/crates/fusion_net → `../../../registry/crates/fusion_core/`
- runtime/crates/fusion_finance → `../../../registry/crates/fusion_core/`
- runtime/crates/fusion_cortex → `../fusion_core`
- runtime/crates/fusion_ai_core → `../../../registry/crates/fusion_core/`

#### Special Cases:

- registry/crates/std → `path = "../fusion_core"` (relative path)

---

### 🔵 Crates Using `fusion-core` (Hyphen) - 48+ instances

#### Registry Crates (using workspace = true):

- tester
- formatter
- docgen
- ai-cli

#### Ecosystem Crates:

- ecosystem/crates/fusion_finance
- ecosystem/crates/tester
- ecosystem/crates/nn-rbf
- ecosystem/crates/nn-gan-layers
- ecosystem/crates/nn-metrics
- ecosystem/crates/nn-embed
- ecosystem/crates/nn-3d-conv
- ecosystem/crates/nn-attention-block
- ecosystem/crates/llm-stream-parser
- ecosystem/crates/llm-tokenizers
- ecosystem/crates/llm-vision-adapter
- ecosystem/crates/llm-model-server
- ecosystem/crates/llm-quantization
- ecosystem/crates/llm-rerope
- ecosystem/crates/llm-rotary-opt
- ecosystem/crates/llm-lora-kernel
- ecosystem/crates/llm-moe-tools
- ecosystem/crates/llm-mixtral-routing
- ecosystem/crates/llm-rag
- ecosystem/crates/llm-inference
- ecosystem/crates/llm-data-tokenizer
- ecosystem/crates/llm-logits-processor
- ecosystem/crates/llm-gqa-kernel
- ecosystem/crates/llm-lora-manager
- ecosystem/crates/llm-attention-mask
- ecosystem/crates/llm-cache-compression
- ecosystem/crates/fusion_quantum
- ecosystem/crates/fusion_ai_core
- ecosystem/crates/fusion_net
- ecosystem/crates/docgen
- ecosystem/crates/formatter
- ecosystem/crates/ai-cli
- ecosystem/crates/ai-core

#### Direct Path Dependencies:

- crates/toolchain → `../../registry/crates/core/`
- crates/analyzer → `../../registry/crates/core`
- cmd/fusion → workspace (after recent fix)
- cmd/fusion/fusion → `../../../registry/crates/core/`
- cmd/fusion/fusion/fusion → `../../../../registry/crates/core/`

#### Antigravity Playground (testing/old):

- antigravity/playground/nodal-whirlpool/cmd/fusion
- antigravity/playground/Fusion VSC CLi/* (multiple)

---

## Merge Strategy

### Phase 1: Create Merged `fusion-core` Crate

1. Keep `registry/crates/fusion_core` as location
2. Rename package to `fusion-core` (hyphen)
3. Merge compiler modules from `registry/crates/core`
4. Update Fusion.toml with all necessary dependencies

### Phase 2: Update Workspace Dependencies

Update root `Fusion.toml`:

```toml
[workspace.dependencies]
fusion-core = { path = "registry/crates/fusion_core", version = "0.2.0" }

# REMOVE: fusion_core (old reference)


# REMOVE: fusion-compiler (alias)

```text

### Phase 3: Update All Dependents

#### Auto-Fix (workspace = true users):

These will automatically resolve once workspace.dependencies is updated:
- All 131+ `fusion_core = { workspace = true }` references
- All 48+ `fusion-core = { workspace = true }` references

#### Manual Fix Required:

1. **Runtime crates**: Update paths from `../../../registry/crates/fusion_core/` → kept (still valid)
2. **std crate**: Update `path = "../fusion_core"` → stays (relative path still works)
3. **Toolchain**: Change path `../../registry/crates/core/` → `../../registry/crates/fusion_core/`
4. **Analyzer**: Change path `../../registry/crates/core` → `../../registry/crates/fusion_core`
5. **cmd/fusion variants**: Already using workspace = true (fixed)

### Phase 4: Remove Old Crate

Delete `registry/crates/core` directory entirely

---

## Import Statement Changes

### For Code Using `fusion_core`:

**BEFORE:**

```rust
use fusion_core::types::Tensor;
```text

**AFTER (no change needed if using underscore):**

```rust
use fusion_core::types::Tensor;
```text

### For Code Using `fusion-core`:

**BEFORE:**

```rust
use fusion_core_compiler::lexer::Lexer;
use fusion_core_compiler::parser::Parser;
```text

**AFTER:**

```rust
use fusion_core::compiler::lexer::Lexer;
use fusion_core::compiler::parser::Parser;
```text

---

## Risk Assessment

### Low Risk:

- ✅ Workspace-based dependencies (auto-resolve)
- ✅ Path-based that point to fusion_core (unchanged location)

### Medium Risk:

- ⚠️ Path-based pointing to `core/` directory (need manual update)
- ⚠️ Import statements in .rs files (need grep + replace)

### Safe to Ignore:

- 🔶 antigravity/playground/* (test/development sandboxes)
- 🔶 Source Files/Ecosystem/Epoch (external/reference)

---

## Next Steps

1. ✅ **Audit Complete** (this document)
2. ⬜ Merge crate contents
3. ⬜ Update root Fusion.toml workspace.dependencies
4. ⬜ Fix path-based dependencies (toolchain, analyzer)
5. ⬜ Search and replace import statements in .rs files
6. ⬜ Run `fusion check --workspace`
7. ⬜ Delete old `registry/crates/core`
8. ⬜ Run full test suite

---

## Summary Statistics

- **Total `fusion_core` references**: 131+
- **Total `fusion-core` references**: 48+
- **Workspace-managed (auto-fix)**: ~170+
- **Manual path updates needed**: ~5-7 crates
- **Import statement updates**: TBD (next grep pass)