# Fusion Programming Language: User Guide

## Introduction

Fusion is designed to be accessible yet powerful, combining the ease of Python with the performance of C and the safety of Rust.

## Getting Started

### Hello World

```fusion
fn main():
    print("Hello, world!")
```

### Variables and Types

Fusion supports both declared and inferred types.

```fusion
let x = 10              // Inferred int
let y: float = 3.14     // Explicit float
let name: string = "Fusion"
```

## Control Flow

### If/Else

```fusion
if x > 5:
    print("Greater than 5")
else:
    print("Less or equal")
```

### Loops

```fusion
for i in 0..10:
    print(i)

while x > 0:
    x = x - 1
```

## Functions

```fusion
fn add(a: int, b: int) -> int:
    return a + b
```

## Classes

```fusion
class Point:
    x: int
    y: int

    fn new(x: int, y: int) -> Point:
        return Point { x: x, y: y }
```

## Modules

Importing standard libraries:

```fusion
use fusion::math
use fusion::crypto
```text