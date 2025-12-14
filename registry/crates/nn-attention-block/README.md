# nn-attention-block

Modular attention mechanisms for neural networks.

## Features

- Multi-head self-attention
- Cross-attention
- Efficient implementation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nn-attention-block = "0.1.0"
```

## Usage

```rust
use nn_attention_block::Attention;

fn main() {
    let block = Attention::new();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
