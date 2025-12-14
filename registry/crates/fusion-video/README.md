# fusion-video

Video processing and streaming capabilities for Fusion.

## Features

- Decoding/Encoding support
- Real-time streaming
- Frame manipulation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fusion-video = "0.1.0"
```

## Usage

```rust
use fusion_video::Player;

fn main() {
    Player::new().play();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
