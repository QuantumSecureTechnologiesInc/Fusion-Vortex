# llm-tensor-optim

Tensor optimization routines specific to transformer architectures.

## Features

- Graph rewriting
- Constant folding
- Operator fusion

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-tensor-optim = "0.1.0"
```

## Usage

```rust
use llm_tensor_optim::optimize;

fn main() {
    optimize();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
