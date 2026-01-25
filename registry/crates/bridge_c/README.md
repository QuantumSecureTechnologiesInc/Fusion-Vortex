# Fusion Bridge C

**Version:** 0.2.0
**Type:** FFI Utility
**License:** MIT

## Overview

Fusion Bridge C (`fusion_bridge_c`) provides essential utilities for safe and efficient interoperability between Fusion and C/C++ libraries. It handles string conversion, memory management for FFI boundaries, and type mapping.

## Features

- **String Conversion**: Zero-cost (where possible) `CString` <-> `String` conversion
- **Memory Safety**: Helpers for ensuring ownership is correctly transferred or borrowed
- **Nullable Pointers**: Safe abstractions for C-style nullable pointers

## Usage

```rust
use fusion_bridge_c::{c_str_to_string, string_to_c_str, free_c_str};
use std::os::raw::c_char;

unsafe fn call_c_lib(ptr: *const c_char) {
    let rs_string = c_str_to_string(ptr).unwrap_or_default();
    println!("Received from C: {}", rs_string);
}
```text

## Dependencies

- `libc`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)