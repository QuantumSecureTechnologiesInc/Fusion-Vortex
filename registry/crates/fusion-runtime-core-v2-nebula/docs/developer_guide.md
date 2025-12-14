# Fusion Runtime Core v2.0 (Nebula) - Developer Guide

## 1. Architecture Overview

Fusion v2.0 represents a paradigm shift from a monolithic process to a micro-kernel architecture.

- **The Core (Host):** A high-performance Rust server acting as the "Spine". It handles network transport (gRPC) and safety (Memory sandboxing).
- **The Plugins (Guests):** Hot-swappable units of logic compiled to WebAssembly (WASM).
- **The Interface:** A strict Protobuf contract (`fusion_core_v2.proto`) ensuring backward compatibility.

## 2. Environment Setup

To develop plugins for Fusion v2.0, you do not need the full Core source code, but you do need the Rust toolchain and the WASM target.

### 2.1 Install Rust

If you haven't already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2.2 Add the WASM Target

Fusion v2.0 uses the `wasm32-unknown-unknown` target for lightweight, stateless compute.

```bash
rustup target add wasm32-unknown-unknown
```

## 3. Building a Fusion Plugin

Plugins are Rust libraries compiled as `cdylib` (C-compatible Dynamic Libraries).

### 3.1 Project Creation

Create a new library project:

```bash
cargo new --lib my-fusion-plugin
cd my-fusion-plugin
```

### 3.2 Configure Cargo.toml

You must tell the compiler to generate a dynamic system library (.wasm).

**File: Cargo.toml**

```toml
[package]
name = "my-fusion-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"] # Critical for WASM generation

[dependencies]
# No heavy dependencies! Keep plugins lightweight.
```

### 3.3 Writing the Logic (src/lib.rs)

The Core looks for a specific entry point named `fusion_entry`. You must prevent the compiler from mangling this name.

**File: src/lib.rs**

```rust
#[no_mangle]
pub extern "C" fn fusion_entry() -> i32 {
    // 1. Perform calculation
    let result = perform_complex_logic();

    // 2. Return status code (0 = Success, >0 = Error)
    if result {
        return 0;
    } else {
        return 1;
    }
}

fn perform_complex_logic() -> bool {
    // Your business logic here
    true
}
```

### 3.4 Compiling

Run the build command:

```bash
cargo build --target wasm32-unknown-unknown --release
```

**Artifact Location:**

Your compiled binary will be located at:

```
target/wasm32-unknown-unknown/release/my_fusion_plugin.wasm
```

## 4. Deploying & Testing

Once you have your `.wasm` file, you need to upload it to the running Core. We use the Python SDK for this.

### 4.1 Start the Core

**Open Terminal 1:**

```bash
cd server
cargo run
# Output: Fusion Runtime Core v2.0 (Nebula) listening on 0.0.0.0:50051
```

### 4.2 Run the Client Script

**Open Terminal 2.** Ensure you have the SDK requirements installed (`grpcio`, `protobuf`).

Create a test script `deploy.py`:

```python
from fusion_client import FusionClient
import os

client = FusionClient()

# Check Core Health
health = client.check_health()
print(f"Core Status: {health}")

# Define path to your new plugin
wasm_path = "../my-fusion-plugin/target/wasm32-unknown-unknown/release/my_fusion_plugin.wasm"

if os.path.exists(wasm_path):
    print("Uploading Plugin...")
    result = client.execute_wasm("my-test-plugin", wasm_path, input_data="test-payload")
    print(f"Result: {result}")
else:
    print("Error: WASM file not found. Did you run cargo build?")
```

## 5. Troubleshooting & Limitations

### 5.1 "Function not found" Error

**Cause:** You forgot `#[no_mangle]` or `pub extern "C"`.

**Fix:** Ensure the function signature in `lib.rs` is exactly `pub extern "C" fn fusion_entry() -> i32`.

### 5.2 Infinite Loops

**Behaviour:** The plugin hangs and then returns a specific error.

**Mechanism:** Fusion Core v2.0 uses "Fuel" (instruction counting). If a plugin consumes more than 10,000 units of fuel, the Core kills it instantly to protect the system.

### 5.3 Memory Limits

**Current Limit:** Plugins are restricted to 4GB of addressable memory (WASM 32-bit limit), but the Core enforces a tighter 512MB limit for stability.

## 6. CI/CD Integration

To automate plugin builds in GitHub Actions:

```yaml
jobs:
  build-plugin:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Target
        run: rustup target add wasm32-unknown-unknown
      - name: Build
        run: cargo build --target wasm32-unknown-unknown --release
      - name: Upload Artifact
        uses: actions/upload-artifact@v3
        with:
          name: plugin-binary
          path: target/wasm32-unknown-unknown/release/*.wasm
```

## 7. Advanced Topics

### 7.1 Host Functions

In future versions (v2.1+), plugins will be able to call back into the host for logging, I/O, and state management.

### 7.2 Shared Memory

For high-performance data transfer, future versions will support shared linear memory between the host and plugin.

### 7.3 Multi-tenancy

The Core is designed to run multiple plugins concurrently with full isolation and resource quotas.

## 8. Support

For issues, questions, or contributions, please contact the Fusion development team.
