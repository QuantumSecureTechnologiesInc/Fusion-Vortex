# Project Organization

Last updated: 2026-06-25 (Final cleanup — Fusion is fully native, 0 Rust dependencies)

## Language Status

- **3,643 `.fu`** source files across the active source tree
- **0 `.rs`** files in active source (all Rust duplicates removed)
- **0 `Cargo.toml`/`Cargo.lock`** in active source (all archived)
- Fusion is a fully self-contained native language

## Core Language Directories

- `crates/` — Compiler (14 `.fu` sources in `fuc/src/`) and 21 crate stubs
- `cmd/` — CLI tools (fusion, fusion-coder, fusion-visual, reactor-cli)
- `stdlib/` — Standard library (`.fu` sources in `core/`, `math/`, `net/`, `sync/`)
- `runtime/` — C runtime (`native/fusionrt.c`, cross-platform `runtime.c`)
- `src/` — Additional source modules
- `toolchain/` — Interop conversion layer
- `registry/` — Crate registry (`.fu` stubs and crates)

## Testing & Tooling

- `tests/` — Integration test fixtures
- `scripts/` — Gate scripts (security, compiler, runtime, fixtures, native build, CI)
- `grammar/` — ANTLR4 grammar (`Fusion.g4`, aligned with actual compiler)
- `editors/` — VS Code extension
- `tools/` — Development tools and utilities

## Binaries & Build

- `bin/` — Prebuilt compiler (`fuc.exe` 10.3 MB, `fuc` 3.8 MB) and runtime (`fusionrt.lib` 35 KB)
- `cmake_build/` — CMake build output (regeneratable)
- `clang+llvm-20.1.0-x86_64-pc-windows-msvc/` — LLVM toolchain (in `.gitignore`)
- `target*/` — Legacy Cargo build output (regeneratable)

## Documentation & Training

- `docs/` — All documentation
- `AI Training/` — Training datasets and guides
- `docs-truth-audit/` — Documentation audit report

## Preserved Source Archives

- `source_archives/` — Legacy source files (1,111 files)
- `v2.0 Vortex/` — Reference implementations (931 files)
- `ecosystem/` — Core runtime evolution, HAFT mesh nodes (364 files)
- `examples/` — Example programs and converted `.fu` files (1,945 files)

## Application & UI

- `fusion-visual-ui/` — Next.js-based visual UI (133 source files, excludes `node_modules/`)
- `deploy/` — Deployment configs for Linux, macOS, Windows

## Other Active Directories

- `antigravity/` — Antigravity playground (185 files)
- `artifacts/` — Build probes and misc fix scripts
- `templates/` — Project templates
- `inventory/` — Inventory tracking
- `.scripts/` — Internal scripts (registry generation, linting, etc.)
- `.github/` — CI workflows and Dependabot config
- `.vscode/` — Editor settings, tasks, syntax highlighting
- `.lint-profiles/` — Linting configurations
- `.fusion/` — Build policy configuration

## Archived (`archive/`)

- `.archive/fusion-visual-ui` — 10 legacy UI config files (replaced by restored `fusion-visual-ui/`)
- `.archive/root-junk` — Removed root-level Cargo.toml, Cargo.lock, build artifacts
- `.archive/legacy-duplicates` — Rust duplicates of ecosystem packages
- `.archive/` — Other archived sources (fuc-stubs, stale manifests, toy compiler, etc.)

## Permanently Removed

- `build/`, `dist/`, `New folder/` — Removed during restructure
- All `.rs` and `Cargo.toml`/`Cargo.lock` from active source tree

If you need a removed item, check `.archive/` or git history.
