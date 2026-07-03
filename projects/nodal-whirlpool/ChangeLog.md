# Changelog

All notable changes to the Fusion Programming Language CLI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure and mono-repository setup
- Cargo workspace with 16 crates
- CLI entry point with comprehensive command structure
- Core compiler skeleton (lexer, parser, type checker, AST)
- Toolchain implementation (build, run, project creation)
- Test framework foundation
- Code formatter skeleton
- Documentation generator
- Package manager with PQC foundations
- DAP debugger interface
- Static analyzer and linter
- Runtime profiler
- Security audit system
- Cloud deployment adapters
- **AI Subsystem**:
  - AI core library with model adapter interface
  - Safety engine for PII, secret, and security detection
  - Preview and apply engine for code changes
  - Workspace context loader
  - Prompt template manager
  - Policy manager for AI operations
  - Cache system with TTL
  - AI CLI commands (assist, generate, refactor, explain, review, tests, doc, config)
  - AI daemon for background LLM serving
  - Local model runner interface

### Security
- Post-quantum cryptography dependencies (CRYSTALS-Kyber, CRYSTALS-Dilithium)
- Safety engine for detecting secrets and PII in generated code
- Offline-first AI mode for sensitive codebases
- Audit trail metadata for all AI operations

### Documentation
- Comprehensive README with architecture overview
- Quick Start Guide
- Project structure documentation
- AI subsystem architecture documentation

## [0.1.0] - 2024-12-08

### Added
- Initial skeleton release
- Foundation for Phase 1 development
- Complete crate structure
- Build system integration

---

## Version History

- **v0.1.0** (2024-12-08): Initial skeleton release with complete architecture
- **Unreleased**: Active development of AI subsystem and core compiler

## Migration Guide

### From v0.0.x to v0.1.0

This is the first release. No migration needed.

## Roadmap Integration

This changelog tracks implementation of features from the roadmap:

- ✅ Phase 0: Repository structure and workspace setup
- ✅ Phase 1: Core CLI and toolchain skeleton
- ✅ Phase 2: AI subsystem foundation with safety engine
- 🔄 Phase 3: Safety and preview flow (in progress)
- ⏳ Phase 4: Local model integration
- ⏳ Phase 5: Cloud LLM adapters
- ⏳ Phase 6: Production polish and CI/CD

---

**Note**: This project is in active development. Breaking changes may occur until v1.0.0.
