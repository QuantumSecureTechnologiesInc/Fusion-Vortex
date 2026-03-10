# Fusion CUDA Driver

Low-level CUDA driver bindings for the Fusion Runtime.

## Features

- Direct FFI bindings to CUDA Driver API (`libcuda.so`)
- Stream management
- Kernel launch support
- Device initialization

## Platform Support

- **Linux**: Full CUDA support via FFI
- **Windows/macOS**: Fallback mode (returns `NotAvailable` error)

## Usage

```rust
use fusion_cuda_driver::CudaDriver;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = CudaDriver::new(0)?; // Device 0

    driver.launch_kernel("my_kernel")?;
    driver.synchronize()?;

    Ok(())
}
```text

## Requirements

- CUDA Toolkit 11.0+ (Linux only)
- `libcuda.so` in library path

## License

MIT OR Apache-2.0
