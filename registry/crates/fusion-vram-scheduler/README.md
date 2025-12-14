# fusion-vram-scheduler

VRAM management and scheduling for GPU-accelerated workloads.

## Features

- Memory defragmentation
- Predictive allocation
- Multi-GPU support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fusion-vram-scheduler = "0.1.0"
```

## Usage

```rust
use fusion_vram_scheduler::schedule;

fn main() {
    schedule();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
