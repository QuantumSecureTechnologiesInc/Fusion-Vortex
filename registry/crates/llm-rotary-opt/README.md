# llm-rotary-opt

Optimized kernels for Rotary Positional Embeddings (RoPE).

## Features

- CUDA/Metal acceleration
- Fused kernels
- Performance tuning

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-rotary-opt = "0.1.0"
```

## Usage

```rust
use llm_rotary_opt::rope;

fn main() {
    rope();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
