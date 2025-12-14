# fusion-wasm-runtime

WebAssembly runtime integration for Fusion.

## Features

- WASI support
- Hot-reloading
- Secure sandboxing

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fusion-wasm-runtime = "0.1.0"
```

## Usage

```rust
use fusion_wasm_runtime::Runtime;

fn main() {
    Runtime::new().start();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
