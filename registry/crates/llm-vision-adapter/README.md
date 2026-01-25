# llm-vision-adapter

Adapter layers for multimodal vision-language models.

## Features

- Projectors (Linear, MLP)
- Image embedding integration
- Cross-attention modules

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-vision-adapter = "0.1.0"
```text

## Usage

```rust
use llm_vision_adapter::Projector;

fn main() {
    let p = Projector::new();
}
```text

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.