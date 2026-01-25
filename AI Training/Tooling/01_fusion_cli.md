# Fusion CLI - Complete Command Reference

**Dataset Category**: Tooling
**Training Level**: Beginner to Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

The `fusion` CLI is the unified command-line interface for the Fusion ecosystem, providing access to compilation, building, testing, dependency management, MCP server, VSCode runtime, and more.

## 1. Installation and Setup

### 1.1 Installation Commands

```bash

# Unix-like systems (Linux/macOS)

curl -fsSL https://sh.fusion-lang.org | sh

# Windows (PowerShell)

iwr https://win.fusion-lang.org -useb | iex

# Verify installation

fusion --version
```text

### 1.2 Environment Configuration

```bash

# Add to PATH

export PATH="$HOME/.fusion/bin:$PATH"

# Set Fusion home directory

export FUSION_HOME="$HOME/.fusion"

# Enable debug logging

export FUSION_LOG=debug
```text

## 2. Project Management

### 2.1 Creating Projects

```bash

# Create new binary project

fusion new my-project

# Create new library project

fusion new --lib my-library

# Create with template

fusion new --template web-server my-api

# Initialize in existing directory

fusion init

# Available templates

fusion new --list-templates
```text

### 2.2 Project Structure

```text
my-project/
├── fusion.toml                # Project manifest (canonical)
├── src/
│   └── main.fu                # Entry point
├── tests/                     # Integration tests
├── benches/                   # Benchmarks
└── docs/                      # Documentation
```text

## 3. Build Commands

### 3.1 Building Projects

```bash

# Build in debug mode (default)

fusion build

# Build in release mode (optimized)

fusion build --release

# Build specific package

fusion build -p package-name

# Build with custom target

fusion build --target x86_64-unknown-linux-gnu

# Verbose output

fusion build -v

# Very verbose (full compiler output)

fusion build -vv
```text

### 3.2 Build Profiles

```toml

# fusion.toml

[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```text

```bash

# Use custom profile

fusion build --profile production
```text

## 4. Running Projects

```bash

# Run main binary

fusion run

# Run with arguments

fusion run -- arg1 arg2

# Run specific binary

fusion run --bin my-binary

# Run in release mode

fusion run --release

# Run example

fusion run --example hello-world
```text

## 5. Testing

### 5.1 Running Tests

```bash

# Run all tests

fusion test

# Run specific test

fusion test test_name

# Run tests matching pattern

fusion test --test pattern

# Run tests in release mode

fusion test --release

# Show test output

fusion test -- --nocapture

# Run tests in parallel

fusion test -- --test-threads=4

# Run doctests

fusion test --doc
```text

### 5.2 Writing Tests

```fusion

#[test]

fn test_addition() {
    assert_eq!(2 + 2, 4)
}

#[test]


#[should_panic]

fn test_panic() {
    panic("This test should panic")
}
```text

## 6. Flux-Resolve (Dependency Management)

### 6.1 Dependency Resolution

```bash

# Resolve dependencies

fusion flux-resolve

# Resolve with GPU acceleration

fusion flux-resolve --engine-mode gpu

# Force re-resolution

fusion flux-resolve --force

# Check for updates

fusion flux-resolve --check-updates

# Display dependency tree

fusion flux-resolve tree
```text

### 6.2 Hive Mind (Distributed Caching)

```bash

# Configure Redis for Hive Mind

fusion config flux-resolve set --redis-url redis://localhost:6379

# Enable Hive Mind

fusion flux-resolve --hive-mind

# Test Hive Mind connection

fusion flux-resolve --test-hive-mind

# View cache statistics

fusion flux-resolve stats --hive-mind
```text

##7. MCP Server Commands

### 7.1 Starting MCP Server

```bash

# Start MCP server on default port (9339)

fusion mcp serve

# Specify port

fusion mcp serve --port 3000

# Enable policy enforcement

fusion mcp serve --policy-mode strict

# Background mode

fusion mcp serve --daemon

# With specific facets

fusion mcp serve --facets "code_editing,file_search"
```text

### 7.2 MCP Management

```bash

# Check MCP server status

fusion mcp status

# Stop MCP server

fusion mcp stop

# Restart MCP server

fusion mcp restart

# View MCP logs

fusion mcp logs

# Clear MCP cache

fusion mcp cache clear
```text

## 8. VSCode Runtime Commands

### 8.1 Extension Management

```bash

# List installed extensions

fusion extension list

# Install extension

fusion extension install ms-vscode.cpptools

# Uninstall extension

fusion extension uninstall extension-id

# Update extension

fusion extension update extension-id

# Enable/disable extension

fusion extension enable extension-id
fusion extension disable extension-id
```text

### 8.2 Extension Development

```bash

# Package extension

fusion extension package ./my-extension

# Install local extension

fusion extension install-local ./my-extension.vsix

# Debug extension

fusion extension debug ./my-extension
```text

## 9. AI Commands

### 9.1 AI Assistant

```bash

# Start AI assistant session

fusion ai assist

# One-shot AI query

fusion ai query "Explain Fusion memory management"

# Code generation

fusion ai generate --task "Create a REST API server"

# Code review

fusion ai review path/to/file.fu
```text

### 9.2 AI Configuration

```bash

# Configure AI provider

fusion config ai set --provider ollama
fusion config ai set --api-key sk-...

# Set model

fusion config ai set --model llama3.1:8b

# Enable/disable AI features

fusion config ai set --enabled true

# Supported providers


# ollama, qwen, deepseek, gpt-oss, mistral, phi, gemma, openai

```text

## 10. Diagnostics and Checking

```bash

# Check code without building

fusion check

# Check specific file

fusion check src/main.fu

# Check all workspace members

fusion check --workspace

# Check with JSON output (for IDE integration)

fusion check --message-format json
```text

## 11. Formatting and Linting

### 11.1 Code Formatting

```bash

# Format all code

fusion fmt

# Check formatting without modifying

fusion fmt --check

# Format specific files

fusion fmt src/main.fu src/lib.fu
```text

### 11.2 Linting

```bash

# Run linter

fusion clippy

# Fix lints automatically

fusion clippy --fix

# Run with specific lint level

fusion clippy -- -W warnings
```text

## 12. Documentation

```bash

# Generate documentation

fusion doc

# Open documentation in browser

fusion doc --open

# Document specific package

fusion doc -p package-name

# Include private items

fusion doc --document-private-items

# Generate book from docs/

fusion book build

# Serve book locally

fusion book serve --port 8080
```text

## 13. Benchmarking

```bash

# Run benchmarks

fusion bench

# Run specific benchmark

fusion bench bench_name

# Save baseline

fusion bench --save-baseline baseline-name

# Compare against baseline

fusion bench --baseline baseline-name

# Generate benchmark report

fusion bench --output-format html
```text

## 14. Profiling

```bash

# Record profile

fusion profile record --release

# View profile report

fusion profile report

# Generate flamegraph

fusion profile flamegraph

# Profile specific command

fusion profile record -- run --release
```text

## 15. Security and Auditing

```bash

# Audit dependencies for security issues

fusion audit

# Fix security vulnerabilities

fusion audit fix

# Generate security report

fusion audit --format json > report.json

# Scan for secrets

fusion audit secrets

# SBOM generation

fusion sbom generate --format cyclonedx
```text

## 16. Policy Management

```bash

# Check policy compliance

fusion policy check

# Audit extension capabilities

fusion policy audit extension-id

# View policy violations

fusion policy violations

# Generate policy report

fusion policy report --output policy-report.md
```text

## 17. Configuration Management

```bash

# View configuration

fusion config show

# Set configuration value

fusion config set key value

# Get configuration value

fusion config get key

# Reset to defaults

fusion config reset

# Edit configuration file

fusion config edit
```text

### 17.1 Common Configuration Keys

```bash

# Build settings

fusion config set build.jobs 8
fusion config set build.target x86_64-unknown-linux-gnu

# Flux-Resolve settings

fusion config set flux-resolve.engine-mode gpu
fusion config set flux-resolve.cache-dir ~/.fusion/cache

# Runtime settings

fusion config set runtime.warmup true
fusion config set runtime.cortex.enabled true

# MCP settings

fusion config set mcp.port 9339
fusion config set mcp.policy-mode strict
```text

## 18. Workspace Commands

```bash

# Create workspace

fusion workspace init

# Add member to workspace

fusion workspace add path/to/member

# Build entire workspace

fusion build --workspace

# Test entire workspace

fusion test --workspace

# List workspace members

fusion workspace list
```text

## 19. Package Registry

```bash

# Publish package

fusion publish

# Login to registry

fusion login

# Logout

fusion logout

# Search registry

fusion search query

# View package info

fusion info package-name

# Download package without installing

fusion fetch package-name
```text

## 20. Advanced Commands

### 20.1 Metadata

```bash

# Generate build metadata

fusion metadata

# View dependency graph

fusion tree

# Show package versions

fusion pkgid package-name
```text

### 20.2 Cleanup

```bash

# Clean build artifacts

fusion clean

# Clean documentation

fusion clean --doc

# Remove target directory

fusion clean --release
```text

### 20.3 Vendoring

```bash

# Vendor dependencies

fusion vendor

# Build with vendored dependencies

fusion build --frozen --offline
```text

## 21. Environment Variables

```bash

# Compiler flags

FUSION_FLAGS="--opt-level=3"

# Runtime configuration

FUSION_RUNTIME_WARMUP=true
FUSION_CORTEX_WARMUP_SECS=60

# Logging

FUSION_LOG=debug
FUSION_LOG_STYLE=always

# Build configuration

FUSION_BUILD_JOBS=8
FUSION_TARGET_DIR=./custom-target

# Cache directory

FUSION_HOME=$HOME/.fusion
FUSION_CACHE_DIR=$HOME/.fusion/cache

# GPU configuration

FUSION_GPU_DEVICE=0
FUSION_GPU_ENABLED=true
```text

## 22. Exit Codes

- `0` - Success
- `1` - General error
- `101` - Compilation error
- `102` - Test failure
- `103` - Security audit failure

## 23. Quick Reference

### Essential Commands

```bash

# Project lifecycle

fusion new my-project          # Create project
cd my-project
fusion check                   # Check code
fusion build                   # Build project
fusion run                     # Run binary
fusion test                    # Run tests

# Development workflow

fusion watch                   # Watch for changes (Monolith daemon)
fusion fmt                     # Format code
fusion clippy                  # Lint code
fusion doc --open              # Generate and view docs

# MCP integration

fusion mcp serve --daemon      # Start MCP server
fusion ai assist               # AI-powered development

# Release workflow

fusion test --release          # Test optimized build
fusion bench                   # Run benchmarks
fusion audit                   # Security audit
fusion build --release         # Final build
fusion publish                 # Publish package
```text

---

## Key Takeaways for AI Training

1. **Unified CLI**: Single `fusion` command for all operations
2. **Project Management**: `new`, `init`, `build`, `run`, `test`
3. **Flux-Resolve**: GPU-accelerated dependency resolution with Hive Mind
4. **MCP Server**: `fusion mcp serve` for AI coding assistants
5. **VSCode Runtime**: Extension management and debugging
6. **AI Integration**: Built-in AI assistant via `fusion ai`
7. **Security**: `fusion audit` and `fusion policy` for compliance
8. **Configuration**: `fusion config` for customization
9. **Profiling**: Built-in profiler for performance analysis
10. **Watch Mode**: `fusion watch` for real-time feedback

This CLI reference provides comprehensive coverage of all Fusion commands. Cross-reference with Flux-Resolve, MCP, and VSCode runtime datasets for deeper understanding of subsystems.