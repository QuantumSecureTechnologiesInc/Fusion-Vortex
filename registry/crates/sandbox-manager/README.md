# sandbox-manager

Orchestration and lifecycle management for Fusion execution sandboxes.

## Features

- Sandbox pooling
- Resource isolation
- Lifecycle hooks

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sandbox-manager = "0.1.0"
```

## Usage

```rust
use sandbox_manager::Manager;

fn main() {
    let mgr = Manager::new();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
