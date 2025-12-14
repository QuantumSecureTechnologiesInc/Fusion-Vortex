# llm-prompt-prefill

Optimized prompt prefilling and management for LLM inference.

## Features

- KV-cache pre-population
- Prompt template management
- Efficient context handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-prompt-prefill = "0.1.0"
```

## Usage

```rust
use llm_prompt_prefill::Prefiller;

fn main() {
    let p = Prefiller::new();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
