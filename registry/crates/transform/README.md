# Fusion Transform

**Version:** 0.2.0
**Type:** Signal Processing
**License:** MIT

## Overview

Fusion Transform (`fusion_transform`) implements fast signal and data transformations. It includes Fourier Transforms (FFT), Wavelet transforms, and other spectral analysis tools.

## Features

- **FFT**: Fast Fourier Transform (1D, 2D)
- **DCT**: Discrete Cosine Transform
- **STFT**: Short-Time Fourier Transform for spectrograms
- **Acceleration**: SIMD-optimized implementations

## Usage

```rust
use fusion_transform::fft;
use num_complex::Complex;

let mut signal = vec![Complex::new(1.0, 0.0); 1024];
fft::process(&mut signal);
```text

## Dependencies

- `fusion_core`
- `num_complex`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)