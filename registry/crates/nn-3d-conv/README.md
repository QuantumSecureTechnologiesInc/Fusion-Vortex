# nn-3d-conv

3D convolution layers for video and volumetric data analysis.

## Features

- Conv3D implementation
- Pooling layers
- Optimized for GPU

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nn-3d-conv = "0.1.0"
```text

## Usage

```rust
use nn_3d_conv::Conv3d;

fn main() {
    let layer = Conv3d::new();
}
```text

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.