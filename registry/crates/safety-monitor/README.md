# safety-monitor

Core safety monitoring primitives for critical systems.

## Features

- Heartbeat monitoring
- Watchdog timers
- Failure detection

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
safety-monitor = "0.1.0"
```

## Usage

```rust
use safety_monitor::Watchdog;

fn main() {
    let wd = Watchdog::new();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
