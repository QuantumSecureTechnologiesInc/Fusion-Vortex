# router-mesh

Core routing logic for mesh networking in Fusion.

## Features

- Dynamic route updates
- Topology discovery
- Traffic shaping

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
router-mesh = "0.1.0"
```

## Usage

```rust
use router_mesh::Router;

fn main() {
    let router = Router::new();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
