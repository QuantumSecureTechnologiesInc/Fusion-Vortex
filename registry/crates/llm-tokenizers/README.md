# llm-tokenizers

High-performance text tokenization library for LLMs.

## Features

- BPE, WordPiece, Unigram support
- Fast Rust implementation
- Hugging Face compatibility

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-tokenizers = "0.1.0"
```

## Usage

```rust
use llm_tokenizers::Tokenizer;

fn main() {
    let t = Tokenizer::from_file("vocab.json");
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
