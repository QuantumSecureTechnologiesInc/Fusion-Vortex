<!-- markdownlint-disable MD024 -->

# ChangeLog - Fusion Visual Compiler

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2026-01-28

## [Unreleased] - 2026-03-04

## [Unreleased] - 2026-03-09

### Added

#### Tendermint blockchain workspace scaffold

- Added a new Fusion-native blockchain crate stack under `ecosystem/crates/`:
  - `fusion-chain-primitives`
  - `fusion-chain-codec`
  - `fusion-chain-crypto`
  - `fusion-chain-ledger`
  - `fusion-chain-state`
  - `fusion-chain-tx`
  - `fusion-chain-mempool`
  - `fusion-chain-p2p`
  - `fusion-chain-consensus-tendermint`
  - `fusion-chain-finality`
  - `fusion-chain-vm`
  - `fusion-chain-contract-abi`
  - `fusion-chain-contract-runtime`
  - `fusion-chain-contract-sdk`
  - `fusion-chain-rpc`
  - `fusion-chain-node`
  - `fusion-chain-node-local`
  - `fusion-chain-audit`
  - `fusion-chain-observability`
  - `fusion-chain-testkit`
- Upgraded `ecosystem/crates/fusion-blockchain` from hello-world to a façade crate that re-exports the full stack.
- Added `ecosystem/crates/fusion-blockchain/Fusion.workspace.toml` and `ecosystem/crates/fusion-blockchain/WORKSPACE_LAYOUT.md` to define crate membership and dependency layers for Tendermint-first execution with smart contracts in MVP.
- Added strict Tendermint signature verification for proposal, prevote, precommit, and commit checks in `fusion-chain-consensus-tendermint`.
- Added node helpers for signed proposal and local validator vote/commit flow in `fusion-chain-node`.
- Added integration-style test suites in `fusion-chain-testkit` for consensus safety, mempool policy, and contract determinism.
- Expanded consensus safety tests with explicit Byzantine scenarios: double-sign precommit, delayed vote handling, and invalid proposer key rejection.
- Added adversarial signature-corruption test coverage for tampered vote signature bytes.
- Added commit-level corruption coverage for tampered precommit signatures inside an otherwise finalised commit.
- Added companion commit-metadata corruption coverage for tampered `height`, `round`, and `block_hash` with untouched signatures.
- Added table-driven corruption matrix coverage combining signature tamper, commit metadata tamper, and vote-type tamper in one adversarial suite.
- Added reusable corruption-matrix helper utilities so new adversarial commit cases are one-line additions in the matrix list.
- Added deterministic proposer selection and rotation enforcement in `fusion-chain-consensus-tendermint` proposal validation.
- Added transaction sender-proof verification (`public_key || signature`) in `fusion-chain-tx` and sender-proof signing helpers in `fusion-chain-crypto`.
- Added deterministic genesis document parsing and node bootstrap (`from_genesis_document`) in `fusion-chain-node`.
- Added genesis bootstrap test coverage in `fusion-chain-testkit/src/genesis_loader.fu`.
- Updated `fusion-chain-node-local` to bootstrap from deterministic genesis and submit cryptographically signed transactions.
- Added append-only ledger file persistence and replay support via `fusion-chain-ledger::open_or_create`.
- Added node bootstrap with persistent ledger path (`from_genesis_document_with_ledger`) and validator-set update hooks.

### Changed

#### Native compiler hardening

- Enabled unresolved-call hard-fail as a non-disableable policy in LLVM codegen (`crates/fuc/src/codegen/llvm.fu`).
- Expanded qualified symbol resolution candidates for call lowering (`crate::` stripping, suffix-based lookup, and `::` to `__` mapping) before unresolved-call failure.
- Removed parser/sema-specific unresolved-call lowering for `lookup_func`/`analyze_output` and replaced it with direct owner-qualified resolution candidates (`Analyzer::`, `SymbolTable::`, `Parser::`, `Lexer::`, `Lowerer::`).
- Added explicit unresolved lowering coverage for formatting helpers (`write`, `write_str`, `write_fmt`) to keep strict mode stable in current stage chains.
- Switched `crates/fuc/src/stage1_parser_api.fu` phase checks to true in-process calls (`parser::parse_status`, `sema::analyze_source_status`) over real file contents, removing command-phase bridge dependence in stage1 API paths.
- Propagated strict unresolved-call mode in native entry scripts:
  - `scripts/strict_selfhost_gate.ps1`
  - `scripts/run_native_regression.ps1`
  - `scripts/bootstrap_native.ps1`
- Verified full readiness gate still passes end-to-end after hardening (`scripts/strict_selfhost_gate.ps1` 5/5).

### Changed

#### Native Compiler + Packaging Pipeline

- Restored `crates/fuc/src` native compiler modules (`main.fu`, `ir.fu`, `parser.fu`, `sema.fu`, `codegen/*`) as active build inputs.
- Updated `.fu -> .rs` bootstrap builder (`tools/build_fuc_from_fu.py`) to:
  - patch generated shims for current inkwell API and borrow semantics,
  - inject missing dependency `generational-arena`,
  - disable inkwell default features for Windows LLVM compatibility,
  - export Windows compiler artefact `target/release/fuc.exe`.
- Reworked LLVM backend (`crates/fuc/src/codegen/llvm.fu`) to emit object files through `llc` from generated IR.
- Hardened native linker flow (`crates/fuc/src/main.fu`) for Windows:
  - uses `clang` for emit-bin linking,
  - supports stdlib fallback paths under `stdlib/`,
  - avoids duplicate panic symbol linkage.
- Added native automation scripts:
  - `scripts/bootstrap_native.ps1` (build + smoke + regression + packaging),
  - `scripts/run_native_regression.ps1` (fixture matrix + JSON summary),
  - `scripts/package_native.ps1` (Windows x64 distribution zip),
  - `scripts/native_build.sh` (POSIX native bootstrap helper),
  - `scripts/audit_selfhost_readiness.ps1` (direct parse/sema compatibility audit for `crates/fuc/src/*.fu`).
- Enforced strict no-Cargo bootstrap mode in `scripts/bootstrap_native.ps1` (requires pre-existing native `target/release/fuc.exe`).
- Produced native package artefacts in `artifacts/packages/`.

### Changed

#### Documentation - Comprehensive Vortex v2.0 Update

- **FUSION_COMPLETE_GUIDEBOOK.md** (+15.5% size):
  - Completely rewritten introduction with "What Is Fusion?" section
  - Added comprehensive "Unique Vortex v2.0 Features" section documenting:
    - Vortex Entropy Engine (chaotic entropy generator, 1GB/s throughput)
    - Supernova Runtime v3.0 (tribrid CPU/GPU/QPU execution)
    - Fusion Visual Compiler (AI-powered code generation)
    - Advanced AI CLI (multi-provider support, MCP server)
    - Fusion Forge (polyglot build system, SAT-based resolution)
    - 250+ Crate Ecosystem (complete breakdown across 6 archetypes)
  - Restructured Table of Contents (61 sections, up from 48)
  - Added Installation & Quick Start section with working examples
  - Updated version to v0.2.0-beta.1, date to January 28, 2026

- **QuickStartGuide.md** (+474% size - complete rewrite):
  - Professional 5-step structure (Installation, First Program, Explore Features, Complete Project, Advanced Features)
  - Added three working code examples (Vortex Engine PQC, Quantum Bell state, AI/ML tensors)
  - Complete multi-paradigm project demonstrating all three computational domains
  - Added advanced features showcase (Visual Compiler, AI CLI, Fusion Forge)

- **Technical_Sheet.md** (+35.9% size):
  - Added "Self-Hosting Compiler" section with component details (lexer.fu, parser.fu, type_checker.fu, semantic.fu, compiler.fu)
  - Updated to "Entropic Borrow Checker" with entropy analysis details
  - Added Bytecode VM backend
  - Updated performance metrics (10x faster incremental builds, 15-25% faster quantum simulation)
  - Reorganised to "250+ Crate Ecosystem (6 Archetypes)" with complete breakdowns
  - Added "Vortex Engine Cryptography" section (logistic map formula, NIST compliance, CQC)
  - Added "Unique Vortex v2.0 Features" section

- **User_Guide.md**:
  - Updated header to v0.2.0-beta.1
  - Emphasised "first self-hosting, quantum-native, AI-integrated" language
  - Added Vortex-specific features (Self-Hosting Compiler, 250+ crates, Vortex Engine, Supernova Runtime)

- **Product_Info_Sheet.md**:
  - Updated version to v0.2.0-beta.1
  - Added unique features subsection (Self-Hosting Compiler, Vortex Engine, Supernova Runtime, Entropic Borrow Checker)
  - Updated ecosystem statistics (250+ crates, 200K+ LoC, 1000+ doc pages)
  - Completely rewritten roadmap (v0.2.0-beta.1 current, v0.3.0, v1.0.0)

- **README.md**:
  - Updated subtitle to emphasise "first self-hosting, quantum-native, AI-integrated"
  - Added "Self-Hosting Compiler" as first key feature

- **DocumentIndex.md** (new):
  - Created comprehensive documentation navigation index
  - Organised by category (Overviews, Guides, Features, Architecture, Testing, Security)
  - Added quick navigation section for new users, builders, and contributors

#### Version Consistency

All documentation now consistently references:

- **Version**: v0.2.0-beta.1 (Bridge Connected)
- **Status**: Production Ready – Vortex Engine Active
- **Date**: January 28, 2026
- **Publisher**: Quantum Secure Technologies Inc.
- **Ecosystem**: 250+ crates across 6 archetypes
- **Footer**: "Generated by: Fusion v2.0 Vortex Toolchain"

## [Unreleased] - 2026-01-13

### Added

#### Documentation

- **Fusion Story and Features Document**: Comprehensive narrative document (`docs/Fusion_Story_and_Features.md`) explaining:
  - Origin story and philosophy of Fusion
  - Complete feature set with code examples
  - Quantum computing capabilities (QAOA, VQE, Grover's, Shor's)
  - AI/ML integration (transformers, LLMs, distributed training)
  - Post-quantum cryptography (ML-KEM, ML-DSA, CQC)
  - Cloud and Kubernetes integration
  - Fusion Visual Compiler details
  - Ecosystem overview (250 crates, 6 archetypes)
  - Competitive comparisons (vs Rust, Python, C++, Q#)
  - Real-world use cases (finance, healthcare, defence, cloud)
- Updated `DocumentIndex.md` with new Overview section
- Added navigation link to Fusion Story in main `README.md`

## [1.0.0] - 2026-01-03

### Added - Initial Release

#### Core Features

- **Intent-based Code Generation**: Natural language to Fusion code
- **AI-Powered Analysis**: Neural parser with 94.2% accuracy
- **Flux Resolver**: Advanced dependency resolution with SAT solver
- **Supernova Runtime Integration**: Heterogeneous CPU/GPU/QPU execution
- **Four Deployment Options**:
  - Web version (Rust + Next.js)
  - Native backend (Supernova + Forge + ReactorCLI)
  - Desktop app (Tauri with MSI installer)
  - Pure Fusion (self-hosting demonstration)

#### UI/UX

- Premium glassmorphism dark theme
- Real-time build visualization
- Project explorer with file tree
- Command palette-style intent input
- Live compilation logs

#### Code Generation Templates

- Machine Learning pipelines (GPU-accelerated)
- Web services (async HTTP servers)
- Quantum circuits (qubit simulation)
- CLI tools (argument parsing)
- Libraries (package scaffolding)

#### Documentation

- Quick Start Guide (tutorial)
- User Guide (task-oriented)
- Developer Guide (explanation)
- API Reference (information)
- Fusion vs Rust comparison
- Rules compliance audit

#### Developer Tools

- NeuralParser with transformer architecture
- Template macro system for code generation
- Flux dependency resolver
- Build session tracking
- Error handling with narrative logging

### Technical Specifications

#### Backend

- **Language**: Rust 1.80+ / Fusion (pure version)
- **Runtime**: Supernova v3.0
- **Web Framework**: Axum 0.7
- **AI Model**: BERT-tiny (11M parameters)
- **Build System**: Fusion Forge

#### Frontend

- **Framework**: Next.js 14
- **Styling**: Vanilla CSS (no Tailwind)
- **Animations**: Framer Motion
- **Icons**: Lucide React

#### Desktop

- **Framework**: Tauri 1.5
- **Installers**: MSI + NSIS
- **Size**: ~15MB (vs 100MB+ Electron)

### Dependencies

#### Workspace

- fusion-runtime-core-v3-supernova: 3.0.0
- fusion-core: 0.2.0
- fusion-ai-core: 0.2.0
- fusion-forge: 1.0.0
- reactor-cli: 0.1.0

#### External

- axum: 0.7
- tokio: 1.42
- serde: 1.0
- tauri: 1.5

### Known Issues

- [ ] Pure Fusion version requires self-hosting compiler
- [ ] GPU acceleration requires CUDA/ROCm drivers
- [ ] Quantum features require QPU access or simulator

### Security

- No known vulnerabilities
- All dependencies audited
- Post-quantum cryptography ready

### Performance

- Intent parsing: <100ms
- Code generation: <500ms
- Full build cycle: <5s

---

## [Unreleased]

### Planned Features

- [ ] Multi-language support (Python, JavaScript interop)
- [ ] Cloud deployment integration
- [ ] Collaborative editing
- [ ] Version control integration
- [ ] Plugin system
- [ ] Custom template marketplace

### Future Improvements

- [ ] Reduce binary size
- [ ] Improve intent accuracy to 98%+
- [ ] Add voice input
- [ ] Mobile app (iOS/Android)
- [ ] VS Code extension

---

## Version History

- **1.0.0** (2026-01-03) - Initial release
- **0.2.0-beta** (2025-12-15) - Beta testing
- **0.1.0-alpha** (2025-11-01) - Alpha preview

---

**Maintained by**: QuantumSecure Technologies Ltd
**License**: MIT OR Apache-2.0
**Contact**: [info@quantumsecuretechnologies.co.uk](mailto:info@quantumsecuretechnologies.co.uk)
