# Fusion ABI Specification

> **Version:** 1.0 – 2026‑01‑16
> **Author:** Antigravity (AI assistant)

---

## 1. Overview

The Fusion Application Binary Interface (ABI) defines how compiled Fusion modules interact at the binary level. It covers data‑type representation, calling conventions, stack layout, name‑mangling, exception handling, and foreign‑function‑interface (FFI) rules for interoperability with C, Rust and LLVM‑based back‑ends. The ABI is platform‑agnostic where possible, with explicit extensions for Windows, Linux and macOS.

---

## 2. Data Types & Alignment

| Fusion Type   | Size (bytes)                                              | Alignment (bytes) | Description                          |
| ------------- | --------------------------------------------------------- | ----------------- | ------------------------------------ |
| `i8` / `u8`   | 1                                                         | 1                 | 8‑bit integer (signed/unsigned)      |
| `i16` / `u16` | 2                                                         | 2                 | 16‑bit integer                       |
| `i32` / `u32` | 4                                                         | 4                 | 32‑bit integer                       |
| `i64` / `u64` | 8                                                         | 8                 | 64‑bit integer                       |
| `f32`         | 4                                                         | 4                 | IEEE‑754 single precision            |
| `f64`         | 8                                                         | 8                 | IEEE‑754 double precision            |
| `bool`        | 1                                                         | 1                 | Boolean (0 = false, 1 = true)        |
| `ptr<T>`      | 8 (on 64‑bit)                                             | 8                 | Native pointer, opaque to the caller |
| `struct`      | Sum of field sizes, padded to the maximum field alignment |

*All structures are **packed** to the natural alignment of their most‑strict field unless the `#[repr(packed)]` attribute is explicitly used, in which case the alignment is 1 byte.*

---

## 3. Calling Convention


### 3.1 Parameter Passing

| Position | Register (Windows) | Register (System V) | Stack (if needed)                  |
| -------- | ------------------ | ------------------- | ---------------------------------- |
| 1        | `rcx`              | `rdi`               | –                                  |
| 2        | `rdx`              | `rsi`               | –                                  |
| 3        | `r8`               | `rdx`               | –                                  |
| 4        | `r9`               | `rcx`               | –                                  |
| 5‑N      | –                  | –                   | Right‑to‑left push, 8‑byte aligned |

- **Floating‑point arguments** use `xmm0‑xmm7` (Windows) or `xmm0‑xmm7` (System V) before spilling to the stack.
- Arguments larger than 16 bytes are passed by reference (pointer to the value).

### 3.2 Return Values

| Type                     | Register (Windows)                                         | Register (System V) |
| ------------------------ | ---------------------------------------------------------- | ------------------- |
| Integer ≤ 64 bits        | `rax`                                                      | `rax`               |
| Floating‑point ≤ 64 bits | `xmm0`                                                     | `xmm0`              |
| Struct ≤ 16 bytes        | `rax` (low) / `rdx` (high)                                 | `rax` / `rdx`       |
| Larger structs           | Pointer to caller‑allocated memory (first hidden argument) |

### 3.3 Register Preservation

| Register                                    | Preserved (callee) |
| ------------------------------------------- | ------------------ |
| `rbx`, `rbp`, `r12‑r15`                     | Yes                |
| `rax`, `rcx`, `rdx`, `rsi`, `rdi`, `r8‑r11` | No                 |
| `xmm0‑xmm5`                                 | No                 |
| `xmm6‑xmm15`                                | Yes                |

---

## 4. Stack Frame Layout

```text
+---------------------------+  <-- High address (stack grows down)
| Caller‑saved registers    |
+---------------------------+
| Return address            |
+---------------------------+
| Home space (Windows)      |
+---------------------------+
| Aligned local variables   |
+---------------------------+
| Shadow space (Linux)      |
+---------------------------+
| ...                       |
+---------------------------+  <-- rsp after prologue
```text

- The stack is always 16‑byte aligned at function entry.
- The **home space** (32 bytes on Windows) is reserved for the first four arguments.
- The **shadow space** (16 bytes on System V) is reserved for the caller’s use.

---

## 5. Name‑Mangling Scheme

```text
_fusion_<module>_<function>_<hash>
```text

- `<module>`: dot‑separated path of the Fusion module (e.g. `std.io`).
- `<function>`: original function name.
- `<hash>`: 8‑character SHA‑256 truncation to avoid collisions.
- Example: `fusion_std_io_print_1a2b3c4d`.

---

## 6. Exception & Error Handling

- Fusion uses **structured exception handling (SEH)** on Windows and **DWARF‑based unwinding** on Unix‑like platforms.
- Functions that may unwind must be annotated with `#[throws]`. The compiler emits a **landing pad** that restores callee‑saved registers and calls the appropriate unwind routine.
- The ABI defines a **`fusion_error_t`** structure (32 bytes) passed by hidden pointer as the last argument for `#[throws]` functions.

---

## 7. Foreign Function Interface (FFI)


### 7.1 C Interop

```c
// C header generated by `fusion-ffi-gen`
typedef struct { uint8_t data[32]; } fusion_error_t;

extern void fusion_std_io_print(const char *msg);
extern int32_t fusion_math_add(int32_t a, int32_t b);
```text

- All exported Fusion symbols use the mangling scheme described in §5.
- The generated header includes `extern "C"` guards for C++.

### 7.2 Rust Interop

```rust

#[link(name = "fusion_std")]

extern "C" {
    pub fn fusion_std_io_print(msg: *const u8);
    pub fn fusion_math_add(a: i32, b: i32) -> i32;
}
```text

- Rust `#[repr(C)]` structs must match the Fusion layout exactly.

### 7.3 LLVM IR Compatibility

- The Fusion compiler emits LLVM IR that respects the **`x86_64`** data layout string `e-m:e-i64:64-f80:128-n8:16:32:64-S128`.
- The ABI conforms to the **`SystemV`** calling convention on Unix and **`Microsoft`** convention on Windows.

---

## 8. Platform‑Specific Variations

| Platform         | Calling Convention                   | Stack Alignment | Extra Notes                                                      |
| ---------------- | ------------------------------------ | --------------- | ---------------------------------------------------------------- |
| Windows (x86_64) | Microsoft (`rcx`, `rdx`, `r8`, `r9`) | 16 bytes        | Home space reserved; use `__stdcall` for legacy APIs             |
| Linux (x86_64)   | System V                             | 16 bytes        | Shadow space reserved; registers `rdi`, `rsi`, `rdx`, `rcx` used |
| macOS (x86_64)   | System V (Apple)                     | 16 bytes        | Same as Linux, but uses Mach‑O binaries                          |

---

## 9. Diagrams

```mermaid
flowchart TD
    A[Caller] -->|Pass args in regs| B[Function Prologue]
    B --> C[Save callee‑saved regs]
    C --> D[Allocate locals on stack]
    D --> E[Body]
    E --> F[Function Epilogue]
    F -->|Return value in rax/xmm0| A
```text

---

## 10. Example: Calling Fusion from C

```c

#include "fusion_std.h"

int main(void) {
    const char *msg = "Hello from C!";
    fusion_std_io_print(msg);
    int32_t sum = fusion_math_add(42, 58);
    printf("Sum = %d\n", sum);
    return 0;
}
```text

- Compile Fusion library with `fusion build --release` producing `fusion_std.dll` (Windows) or `libfusion_std.so` (Linux).
- Link the C program against the generated library.

---

## 11. Revision History

| Version | Date       | Changes                      |
| ------- | ---------- | ---------------------------- |
| 1.0     | 2026‑01‑16 | Initial formal specification |

---

*This document is the authoritative reference for the Fusion ABI. All future compiler releases must conform to the definitions herein.*