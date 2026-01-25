# Fusion Lint Profiles

This directory contains different linting profiles that you can use based on your preferences.

## How to Use

Copy the profile you want to use to the workspace root:

```powershell

# For minimal linting (fewer warnings)

Copy-Item .lint-profiles\minimal.toml Cargo.toml -Force

# For standard linting (current default)

Copy-Item .lint-profiles\standard.toml Cargo.toml -Force

# For strict linting (maximum code quality)

Copy-Item .lint-profiles\strict.toml Cargo.toml -Force
```text

Or on Unix/Linux:

```bash

# For minimal linting

cp .lint-profiles/minimal.toml Cargo.toml

# For standard linting

cp .lint-profiles/standard.toml Cargo.toml

# For strict linting

cp .lint-profiles/strict.toml Cargo.toml
```text

## Profiles

### Minimal

- Only critical lints
- Best for rapid prototyping
- Fewer warnings

### Standard (Default)

- Balanced approach
- Good code quality without noise
- Current workspace default

### Strict

- All recommended lints enabled
- Maximum code quality enforcement
- May produce many warnings on existing code

## Per-Crate Customization

You can also customize lints per-crate by adding a `[lints]` section to any crate's `Cargo.toml`:

```toml
[lints.clippy]

# Override workspace settings for this crate

unwrap_used = "allow"  # Allow unwrap in this specific crate
```text

## Local Developer Overrides

Create a `.cargo/config.toml` in your home directory to set personal preferences:

**Windows**: `%USERPROFILE%\.cargo\config.toml`
**Unix/Linux**: `~/.cargo/config.toml`

```toml
[build]
rustflags = ["-W", "clippy::all"]
```text