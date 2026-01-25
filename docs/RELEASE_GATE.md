# Fusion Release Gate

A release is approved only if all gates pass:

## Build Gates

1. `./install.sh` completes successfully.
2. `dist/lib/fusion/std/src` contains only `lib.fu` and `main.fu`.
3. `fusion build` uses sysroot only (no Fusion.toml).

## Security Gates

1. No `unsafe` blocks in `.fu` sources.
2. No `.rs` or `.fu` shipped in `dist/` (stdlib + core runtime + HAFT nodes).
3. Interop assets are only in `toolchain/interop` and are not linked unless wrapped.

## Compiler Correctness Gates

1. `fuc --parse-only` and `fuc --sema-only` succeed on fixture suite.
2. Fixture suite compiles and matches expected output.

## Runtime Gates

1. Bounds checks validated (safety fixture must abort).
2. Panic/abort path available in runtime (`panic` symbol).
