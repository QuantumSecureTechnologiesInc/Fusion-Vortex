# llm-rag

Retrieval-Augmented Generation (RAG) pipeline components.

## Features

- Document ingestion
- Vector store integration
- Context retrieval and ranking

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-rag = "0.1.0"
```

## Usage

```rust
use llm_rag::Pipeline;

fn main() {
    Pipeline::new().query("What is Fusion?");
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
