# math-sparse

Sparse matrix linear algebra library.

## Features

- CSR, CSC, COO formats
- Sparse matrix-vector multiplication
- Solvers (CG, GMRES)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
math-sparse = "0.1.0"
```

## Usage

```rust
use math_sparse::Matrix;

fn main() {
    let m = Matrix::new();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
