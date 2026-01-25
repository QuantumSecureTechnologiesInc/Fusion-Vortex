# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-12-12 - "Fusion VSC CLI"

### Represents a major pivot to "Fusion VSC CLI" with full MCP Bridge support.

### Added

- **VS Code <-> MCP Bridge**: Fully implemented `extension_bridge.rs` (in `fusion-mcp`) to route MCP tool calls to the Extension Host.
- **Extension Host Registry**: Replaced stubbed execution with a dynamic `CommandRegistry` in `fusion-vscode-runtime`.
- **Node.js Integration**: Added `fusion-vscode-runtime` connecting to Boa engine for JS extension support.
- **Stream Support**: Enabled `reqwest` streaming in `fusion-ai-core` for real-time AI responses.
- **Quick Start Guide**: New guide tailored for the VSC CLI workflows (`docs/guides/QuickStartGuide.md`).
- **Document Index**: Central index for all project documentation (`docs/DocumentIndex.md`).

### Changed

- **Renamed**: Project renamed from "Fusion Advanced AI CLI" to **"Fusion VSC CLI"** to reflect its role as the IDE bridge.
- **Refactored**: `cmd/fusion` command structure flattened to direct `commands.rs` implementation to resolve module conflicts.
- **Updated**: `crates/ai-enhanced` async stream types pinned correctly (`Pin<Box<dyn Stream>>`).
- **Fixed**: Ambiguous `List` variants in CLI commands by using fully qualified paths.
- **Fixed**: Invalid re-exports of crate-private types in `commands.rs`.
- **Fixed**: Type inference errors in `fusion-ai-cli` for async blocks.

### Removed

- **Stubs**: Removed placeholder logging in Extension Bridge (Violation of operational rules).
- **Unused Imports**: Cleaned up `fusion-ai-cli` and `fusion-ai-core`.

## [0.1.5] - 2025-12-11 - "Advanced AI CLI" (Stabilization)

### Fixed

- **Build System**: Resolved `impl Stream` type mismatches in `interactive_agent.rs`.
- **Dependencies**: Fixed `reqwest` feature flags to enable `stream` support in `fusion-ai-core`.
- **CLI**: Resolved module ambiguity between `commands.rs` and `commands/mod.rs`.
- **Imports**: Fixed re-export visibility for `AiCommands` and `PackageCommands` in the main binary.

## [0.1.4] - 2025-12-10 - "AI Core"

### Added

- **Unified Adapter**: Common interface for OpenAI, Anthropic, and Google Gemini models.
- **Offline Mode**: Local model fallback logic.
- **Safety Engine**: Initial implementation of PII redaction and security policy enforcement.

## [0.1.0] - 2025-12-01 - "Foundation"

### Added

- Initial release of Fusion Programming Language CLI.
- Basic toolchain (build, verify, run).
- PQC crypto integration (Kyber/Dilithium).