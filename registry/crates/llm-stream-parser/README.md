# llm-stream-parser

Robust parsing for identifying structure in streaming LLM output.

## Features

- JSON stream parsing
- XML/HTML tag detection
- Resilient recovery

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-stream-parser = "0.1.0"
```

## Usage

```rust
use llm_stream_parser::Parser;

fn main() {
    let p = Parser::new();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
