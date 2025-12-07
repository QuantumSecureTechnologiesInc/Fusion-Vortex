# Quick Guide to Fusion Programming Language

## Prerequisites

- A compatible operating system (Linux, macOS, Windows).
- Rust toolchain (for building the compiler from source initially).
- LLVM 16+ installed.

## Installation

1. **Clone the Repository**

   ```bash
    git clone https://github.com/QuantumSecureTechnologies/fusion-lang.git
    cd fusion-lang
    ```

2. **Build the Compiler**

   ```bash
    cargo build --release
    ```

3. **Add to PATH**

   Add the release binary to your system PATH.

   ```bash
    export PATH=$PATH:$(pwd)/target/release
    ```

## Creating a New Project

To create a new Fusion project, use the CLI:

```bash
fusion new my_project
cd my_project
```

## Running Your First Program

Edit `src/main.fu`:

```fusion
fn main():
    print("Hello, Fusion!")
```

Run the project:

```bash
fusion run
```

## Next Steps

- Explore the [User Guide](docs/guides/User_Guide.md).
- Check out the [Examples](examples/).
