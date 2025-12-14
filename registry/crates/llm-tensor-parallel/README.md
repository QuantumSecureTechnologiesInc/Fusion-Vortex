# llm-tensor-parallel

Distributed tensor parallelism for training and inference across multiple devices.

## Features

- Model partitioning
- Pipeline parallelism
- Collective communications

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-tensor-parallel = "0.1.0"
```

## Usage

```rust
use llm_tensor_parallel::Distributed;

fn main() {
    Distributed::init();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
