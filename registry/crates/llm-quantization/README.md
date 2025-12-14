# llm-quantization

Quantization primitives for reducing LLM memory footprint and boosting inference speed.

## Features

- 4-bit, 8-bit quantization
- GPTQ, AWQ support
- Mixed-precision inference

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-quantization = "0.1.0"
```

## Usage

```rust
use llm_quantization::quantize;

fn main() {
    quantize();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
