# Fusion Toolchain Ext

**Version:** 0.2.0
**Type:** Compiler Plugins
**License:** MIT

## Overview

Fusion Toolchain Ext (`fusion_toolchain_ext`) provides the API for extending the Fusion compiler and toolchain. It allows developers to create custom subcommands, linters, and build steps.

## Features

- **Plugin API**: Stable interface for toolchain extensions
- **Hooks**: Compile-time and build-time hooks
- **Configuration**: Unified config loading for plugins

## Usage

```rust
use fusion_toolchain_ext::{Plugin, Context};

struct MyLinter;

impl Plugin for MyLinter {
    fn run(&self, ctx: &Context) -> anyhow::Result<()> {
        println!("Linting project: {}", ctx.project_name());
        Ok(())
    }
}
```text

## Dependencies

- `fusion_core`
- `fusion_std`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)