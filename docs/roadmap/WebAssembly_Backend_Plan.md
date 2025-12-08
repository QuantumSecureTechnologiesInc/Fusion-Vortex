# WebAssembly Backend Implementation Plan

**Date**: 2025-12-07
**Phase**: Phase 3 - WebAssembly Backend
**Status**: ⏳ In Progress
**Priority**: High (Deployment flexibility)

---

## Overview

Implement a WebAssembly (WASM) code generation backend for Fusion, enabling compilation to `.wasm` modules that can run in browsers, Node.js, and edge computing environments.

## Design Goals

1. **Complete WASM Support** - Generate valid WebAssembly modules
2. **Integration** - Work alongside existing LLVM IR backend
3. **Standard Compatibility** - Follow WebAssembly MVP specification
4. **Performance** - Optimize for WASM execution model
5. **Interoperability** - JavaScript/WASI integration

---

## Architecture

### Code Generation Pipeline

```text
Fusion Source Code
    ↓ Parser
AST (Abstract Syntax Tree)
    ↓ Semantic Analyzer
Type-Checked AST
    ↓ Borrow Checker
Safe AST
    ↓ Backend Selection
    ├─→ LLVM IR CodeGen (existing)
    └─→ WASM CodeGen (new)
         ↓
    WebAssembly Binary (.wasm)
```

### Module Structure

```text
src/
└── wasm/
    ├── mod.rs           # Module exports
    ├── codegen.rs       # WASM code generator
    ├── builder.rs       # WASM instruction builder
    ├── types.rs         # WASM type mappings
    └── module.rs        # WASM module structure
```

---

## WASM Type Mappings

### Fusion → WASM Type Conversions

| Fusion Type  | WASM Type   | Notes                 |
| :----------- | :---------- | :-------------------- |
| `int`        | `i64`       | 64-bit signed integer |
| `float`      | `f64`       | 64-bit float          |
| `bool`       | `i32`       | 0 = false, 1 = true   |
| `string`     | `i32` (ptr) | Pointer to memory     |
| `void`       | -           | No return value       |
| Custom types | `i32` (ptr) | Heap-allocated        |

### Memory Model

**Linear Memory**:

- Single contiguous array of bytes
- Grow dynamically with `memory.grow`
- Accessed via `i32.load` / `i32.store`

**Stack**:

- WASM operand stack
- Function-local variables
- No explicit stack management needed

---

## Implementation Strategy

### Phase 1: Basic Types and Functions (Est. 1 hour)

**Goal**: Generate WASM for simple functions

**Example**:

```fusion
fn add(a: int, b: int) -> int {
    return a + b;
}
```

**WASM Output** (WAT format):

```wasm
(module
  (func $add (param $a i64) (param $b i64) (result i64)
    local.get $a
    local.get $b
    i64.add
  )
  (export "add" (func $add))
)
```

**Implementation**:

- WASM module builder
- Function code generation
- Basic arithmetic operations
- Local variable handling

### Phase 2: Control Flow (Est. 30 min)

**Goal**: Handle if/else, loops, return

**WASM Instructions**:

- `if` / `else` / `end`
- `block` / `loop` / `br` / `br_if`
- `return`

**Example**:

```fusion
fn max(a: int, b: int) -> int {
    if a > b {
        return a;
    } else {
        return b;
    }
}
```

### Phase 3: Memory Management (Est. 45 min)

**Goal**: Heap allocation and strings

**Components**:

- Memory section definition
- `malloc` / `free` implementation
- String handling
- Array/Vector support

**Example**:

```fusion
fn create_string() -> string {
    return "Hello, WASM!";
}
```

### Phase 4: Function Calls (Est. 30 min)

**Goal**: Function calls and imports

**WASM Features**:

- Call table
- Imported functions
- Exported functions
- Indirect calls

### Phase 5: Integration (Est. 30 min)

**Goal**: CLI flag and build system

**Updates**:

- Add `--target wasm` flag
- Binary output to `.wasm` file
- Optional WAT text output

---

## WASM Instruction Reference

### Arithmetic

| Operation      | Fusion  | WASM Instruction |
| :------------- | :------ | :--------------- |
| Addition       | `a + b` | `i64.add`        |
| Subtraction    | `a - b` | `i64.sub`        |
| Multiplication | `a * b` | `i64.mul`        |
| Division       | `a / b` | `i64.div_s`      |
| Modulo         | `a % b` | `i64.rem_s`      |

### Comparison

| Operation    | Fusion   | WASM Instruction |
| :----------- | :------- | :--------------- |
| Equal        | `a == b` | `i64.eq`         |
| Not Equal    | `a != b` | `i64.ne`         |
| Less Than    | `a < b`  | `i64.lt_s`       |
| Greater Than | `a > b`  | `i64.gt_s`       |

### Control Flow

| Structure  | WASM Instructions                      |
| :--------- | :------------------------------------- |
| if/else    | `if (result T?) ... else ... end`      |
| while loop | `block ... loop ... br_if ... end end` |
| return     | `return`                               |
| break      | `br`                                   |

---

## Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]

# WebAssembly code generation

wasm-encoder = "0.219"  # Build WASM binary format
wasmparser = "0.219"    # Parse and validate WASM
```

Optional (for text format):

```toml
wat = "1.216"  # Convert WAT ↔ WASM
```

---

## Example Usage

### Compilation

```

# Compile to WASM

fusion_lang -i program.fu --target wasm -o program.wasm

# Compile to WAT (text format)

fusion_lang -i program.fu --target wasm --emit wat -o program.wat
```

### Running in Browser

```html
<!DOCTYPE html>
<html>
<head><title>Fusion WASM</title></head>
<body>
<script>
  WebAssembly.instantiateStreaming(fetch('program.wasm'))
    .then(obj => {
      const result = obj.instance.exports.add(5, 3);
      console.log('Result:', result); // 8
    });
</script>
</body>
</html>
```

### Running in Node.js

```javascript
const fs = require('fs');
const wasmBuffer = fs.readFileSync('program.wasm');

WebAssembly.instantiate(wasmBuffer).then(obj => {
  const result = obj.instance.exports.add(5, 3);
  console.log('Result:', result); // 8
});
```

---

## Testing Strategy

### Unit Tests

```

#[test]

fn test_wasm_simple_function() {
    let source = r#"
        fn add(a: int, b: int) -> int {
            return a + b;
        }
    "#;

    let wasm = compile_to_wasm(source).unwrap();

    // Validate WASM module
    assert!(wasmparser::validate(&wasm).is_ok());

    // Execute and test
    let result = execute_wasm(&wasm, "add", &[5, 3]).unwrap();
    assert_eq!(result, 8);
}
```

### Integration Tests

**Test Programs**:

1. Arithmetic operations
2. Control flow (if/else)
3. Loops (while, for)
4. Function calls
5. String operations
6. Memory allocation

---

## Performance Considerations

### Optimization Strategies

1. **Register Allocation**: Use WASM locals efficiently
2. **Dead Code Elimination**: Remove unused code
3. **Constant Folding**: Compute constants at compile time
4. **Tail Call Optimization**: Where possible

### Size Optimization

1. **Function Deduplication**: Merge identical functions
2. **Name Mangling**: Short symbol names
3. **Compression**: Use gzip/brotli for deployment

---

## WASI Integration (Future)

**WASI** (WebAssembly System Interface) for:

- File I/O
- Environment variables
- Command-line arguments
- Network sockets

**Example**:

```fusion
extern fn fd_write(fd: int, iovs: int, iovs_len: int, nwritten: int) -> int;

fn println(msg: string) {
    // Use WASI fd_write
}
```

---

## Success Criteria

- [ ] Generate valid WASM binary from simple functions
- [ ] Support arithmetic and comparison operations
- [ ] Handle if/else control flow
- [ ] Implement loops (while, for)
- [ ] Function calls work correctly
- [ ] Memory allocation for strings/objects
- [ ] CLI integration (`--target wasm`)
- [ ] Output `.wasm` file
- [ ] Validate generated WASM
- [ ] Run in browser successfully
- [ ] Run in Node.js successfully
- [ ] Comprehensive test coverage

---

## Timeline

**Estimated Total**: 3-4 hours

1. **Hour 1**: Setup, dependencies, basic module structure
2. **Hour 2**: Function codegen, arithmetic, comparisons
3. **Hour 3**: Control flow, loops, function calls
4. **Hour 4**: Memory management, testing, polish

---

## References

- [WebAssembly Specification](https://webassembly.github.io/spec/)
- [wasm-encoder Documentation](https://docs.rs/wasm-encoder)
- [WASM Instruction Reference](https://webassembly.github.io/spec/core/syntax/instructions.html)
- [WAT Language Guide](https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format)

---

**Status**: ⏳ Ready to Implement
**Next Step**: Add dependencies and create module structure
