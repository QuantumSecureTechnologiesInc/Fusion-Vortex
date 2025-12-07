# Calculator Example

A simple calculator demonstrating basic Fusion language features.

## Features

- Basic arithmetic operations (add, subtract, multiply, divide, modulo)
- Power function (exponentiation)
- Error handling (division by zero)
- Control flow (if, while loops)

## Running

### Native Compilation (LLVM)

```bash
fusion_lang -i calculator.fu
```

### WebAssembly Compilation

```bash
fusion_lang -i calculator.fu --target wasm -o calculator.wasm
```

Then open `index.html` in a browser to use the calculator.

## Code Highlights

**Function Definition**:
```fusion
fn add(a: int, b: int) -> int {
    return a + b;
}
```

**Error Handling**:
```fusion
fn divide(a: int, b: int) -> int {
    if b == 0 {
        println("Error: Division by zero!");
        return 0;
    }
    return a / b;
}
```

**Loops**:
```fusion
fn power(base: int, exponent: int) -> int {
    let mut result = base;
    let mut i = 1;
    
    while i < exponent {
        result = result * base;
        i = i + 1;
    }
    
    return result;
}
```

## Learning Goals

- Understanding function definitions
- Working with parameters and return values
- Using control flow (if/else, while)
- Mutable vs immutable variables
- Basic error handling

---

**Difficulty**: Beginner  
**Concepts**: Functions, Control Flow, Variables
