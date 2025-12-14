# metrics

Core metrics collection instrumentation for Fusion.

## Features

- Counters, Gauges, Histograms
- Low-overhead recording
- Exporter interfaces

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
metrics = "0.1.0"
```

## Usage

```rust
use metrics::increment_counter;

fn main() {
    increment_counter!("my_counter");
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
