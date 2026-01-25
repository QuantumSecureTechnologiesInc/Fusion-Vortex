# Fusion AI CLI

**Version:** Workspace
**Type:** Command Line Interface
**License:** MIT / Apache 2.0 Dual License

## Overview

The Fusion AI CLI (`fusion-ai-cli`) extends the standard Fusion toolchain with powerful AI capabilities. It provides commands for code generation, explanation, refactoring, and project analysis directly from the terminal.

## Installation

This crate is typically installed as part of the Fusion toolchain, but can be built independently:

```bash
cargo install --path registry/crates/ai-cli
```text

## Commands

- `fusion ai code <prompt>` - Generate code from natural language
- `fusion ai explain <file>` - Explain functionality of a source file
- `fusion ai refactor <file>` - Suggest and apply refactorings
- `fusion ai review <pr>` - Perform code review on changes
- `fusion ai test <file>` - Generate tests for a module

## Usage Examples

### Generate Code

```bash
fusion ai code "Create a thread-safe LRU cache with generic types"
```text

### Explain Complex Logic

```bash
fusion ai explain src/algorithm.fu --detail high
```text

### Automated Code Review

```bash
fusion ai review --branch feature/new-api
```text

## Configuration

The CLI uses the standard Fusion AI configuration from `fusion.toml`:

```toml
[ai]
model = "claude-3-opus"
temperature = 0.2
max_tokens = 4096
```text

## Integration

- **Git Integration**: Aware of git status and diffs for context
- **Workspace Aware**: Understands project structure and dependencies
- **Interactive Mode**: Supports conversational refinement

## Dependencies

- `fusion-ai-core`
- `fusion-core`
- `git2` for VCS integration

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)