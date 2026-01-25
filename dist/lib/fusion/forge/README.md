# Fusion Forge

**Fusion Forge** is a polyglot build system and package manager for the Fusion programming language ecosystem. It provides a unified interface for building projects that combine Rust, C++, Python, and JavaScript code.

## Features

- **Polyglot Support**: Build projects using Rust, C++, Python, and JavaScript in a single unified workflow
- **Smart Dependency Resolution**: Automatically determines build order across languages (C++ → Rust → Python/JS)
- **Runtime Target Support**: Build for different Fusion runtime versions (v1, Nebula, Nebula 2.1, Supernova)
- **Build Caching**: SHA256-based intelligent caching to avoid unnecessary rebuilds
- **FFI Generation**: Automatic binding generation for cross-language interop using bindgen and PyO3

## Installation

```bash
cd C:\Projects\Fusion - Programming Language\tools\forge
cargo build --release
```

The compiled binary will be at `target/release/forge.exe`.

## Quick Start

### Create a New Project

```bash
forge new my-project --target supernova
cd my-project
```

### Build Your Project

```bash
forge build
```

## Fusion.toml Configuration

The `Fusion.toml` manifest defines your project structure:

```toml
[package]
name = "my-project"
version = "0.1.0"
language = "rust"
runtime_target = "supernova"  # v1, nebula, nebula_2_1, or supernova

[dependencies]
tokio = "1.0"

[languages.cpp]
standard = "c++20"
sources = ["src/native/physics.cpp"]
include_dirs = ["include/"]

[languages.python]
requirements = ["numpy", "pytorch"]
entry_point = "src/ai/model.py"

[languages.js]
manager = "bun"
packages = { react = "^18.0" }
```

## Runtime Targets

- **v1**: Native-only builds (Rust + C++)
- **nebula**: Strict WASM sandbox (Rust only, no C++)
- **nebula_2_1**: WASM with host access (Rust + C++ WASM)
- **supernova**: Full hybrid mesh (all languages)

## Language Support

### Rust
- Supported on all runtime targets
- Automatic WASM compilation for Nebula targets
- Native builds for v1 and Supernova

### C++
- Requires `clang++`
- Blocked on Nebula v2.0 (strict sandbox)
- Compiles to native `.so` or WASM objects

### Python
- Requires `python3` or `uv`
- **Supernova only** (v3.0)
- Automatic virtual environment management

### JavaScript
- Requires `bun` or `npm`
- Blocked on v1
- Automatic dependency installation

## Architecture

Forge orchestrates multiple language toolchains in the correct dependency order:

1. **Resolution**: Parse `Fusion.toml` and determine active toolchains
2. **Validation**: Check toolchain availability and target compatibility
3. **Execution**: Build in dependency order (C++ → Rust → Python/JS)
4. **Caching**: Track file hashes to skip unchanged components

## Commands

### `forge new <name>`

Create a new Fusion project.

Options:
- `--template <template>`: Project template (default: "fusion")
- `--target <target>`: Runtime target (default: "supernova")

### `forge build`

Build the current project.

Options:
- `--release`: Build in release mode

## License

MIT
