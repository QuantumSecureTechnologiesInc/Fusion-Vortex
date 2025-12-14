# Fusion Programming Language - Syntax and Grammar Reference

**Dataset Category**: Fundamentals  
**Training Level**: Beginner to Intermediate  
**Last Updated**: December 2025 (v0.2.0-beta.1)

---

## Overview

Fusion is a modern, multi-paradigm programming language with C-family syntax, designed for safety, performance, and ergonomics. This document provides complete syntax and grammar specifications for training AI systems.

## 1. Program Structure

### 1.1 Basic Program

Every Fusion program consists of declarations at the top level:

```fusion
// Basic program structure
fn main() -> int {
    println("Hello, Fusion!")
    return 0
}
```

**Key Points**:
- Entry point is `main()` function
- Returns `int` exit code (0 = success)
- No semicolons required (optional)
- Indentation-agnostic (uses braces `{}`)

### 1.2 Top-Level Declarations

```fusion
// Module/namespace declaration
module math::geometry

// Imports
import std::collections::HashMap
import fusion::ai_core as ai

// Global constants (immutable)
const PI: float = 3.14159
const MAX_SIZE: int = 1024

// Type declarations
type UserId = int
type Result<T> = Result<T, Error>

// Function declarations
fn add(a: int, b: int) -> int {
    return a + b
}

// Class declarations
class Point {
    x: float
    y: float
}

// Trait declarations
trait Drawable {
    fn draw(self)
}
```

## 2. Variables and Mutability

### 2.1 Immutable Variables (Default)

```fusion
let name = "Alice"              // Type inferred as string
let age: int = 30               // Explicit type annotation
let pi = 3.14159                // Inferred as float

// COMPILE ERROR: Cannot reassign immutable variable
// name = "Bob"  // ERROR!
```

### 2.2 Mutable Variables

```fusion
let mut counter = 0             // Mutable integer
counter += 1                    // OK: can modify
counter = 10                    // OK: can reassign

let mut scores: [int] = []      // Mutable array
scores.push(95)                 // OK: can mutate
```

### 2.3 Constants

```fusion
const MAX_CONNECTIONS: int = 100
const SERVER_URL: string = "https://api.example.com"

// Constants must be compile-time evaluable
const BUFFER_SIZE: int = 1024 * 8
```

## 3. Types

### 3.1 Primitive Types

```fusion
// Integers
let i8_val: i8 = 127
let i16_val: i16 = 32767
let i32_val: i32 = 2147483647
let i64_val: i64 = 9223372036854775807
let int_val: int = 42            // Platform-sized (i32 or i64)

// Unsigned integers
let u8_val: u8 = 255
let u16_val: u16 = 65535
let u32_val: u32 = 4294967295
let u64_val: u64 = 18446744073709551615
let uint_val: uint = 100

// Floating-point
let f32_val: f32 = 3.14
let f64_val: f64 = 2.718281828
let float_val: float = 1.618     // Defaults to f64

// Boolean
let is_active: bool = true
let is_disabled = false

// Character and String
let ch: char = 'A'
let name: string = "Fusion"
let multiline = """
    This is a
    multiline string
"""
```

### 3.2 Composite Types

#### Arrays

```fusion
// Fixed-size arrays
let numbers: [int; 5] = [1, 2, 3, 4, 5]
let zeros = [0; 100]             // 100 zeros

// Dynamic arrays (vectors)
let mut vec: [int] = []
vec.push(1)
vec.push(2)
vec.extend([3, 4, 5])
```

#### Tuples

```fusion
// Heterogeneous fixed-size collections
let point: (int, int) = (10, 20)
let person = ("Alice", 30, true)

// Destructuring
let (name, age, active) = person
let (x, y) = point
```

#### Option Type

```fusion
// Represents optional values
let some_value: Option<int> = Some(42)
let no_value: Option<int> = None

// Pattern matching
match some_value {
    Some(v) => println("Value: {}", v),
    None => println("No value")
}
```

#### Result Type

```fusion
// Represents success or failure
let success: Result<int, string> = Ok(42)
let failure: Result<int, string> = Err("Something went wrong")

// Pattern matching
match success {
    Ok(value) => println("Success: {}", value),
    Err(error) => println("Error: {}", error)
}
```

### 3.3 User-Defined Types

#### Structs/Classes

```fusion
// Class with fields
class Rectangle {
    width: float
    height: float
}

// Class with methods
class Point {
    x: float
    y: float
    
    fn new(x: float, y: float) -> Point {
        return Point { x, y }
    }
    
    fn distance_from_origin(self) -> float {
        return ((self.x ** 2) + (self.y ** 2)).sqrt()
    }
}
```

#### Enums

```fusion
// Algebraic data types
enum Direction {
    North,
    South,
    East,
    West
}

// Enums with associated data
enum Message {
    Quit,
    Move { x: int, y: int },
    Write(string),
    ChangeColor(int, int, int)
}

// Pattern matching on enums
let msg = Message::Move { x: 10, y: 20 }
match msg {
    Message::Quit => println("Quit"),
    Message::Move { x, y } => println("Move to ({}, {})", x, y),
    Message::Write(text) => println("Write: {}", text),
    Message::ChangeColor(r, g, b) => println("Color: ({}, {}, {})", r, g, b)
}
```

### 3.4 Generic Types

```fusion
// Generic struct
class Box<T> {
    value: T
    
    fn new(value: T) -> Box<T> {
        return Box { value }
    }
    
    fn get(self) -> T {
        return self.value
    }
}

// Generic function
fn swap<T>(a: T, b: T) -> (T, T) {
    return (b, a)
}

// Multiple type parameters
class Pair<K, V> {
    key: K
    value: V
}
```

## 4. Functions

### 4.1 Function Declaration

```fusion
// Basic function
fn greet(name: string) -> string {
    return "Hello, " + name + "!"
}

// Multiple parameters
fn add(a: int, b: int) -> int {
    return a + b
}

// No return value (returns unit type `()`)
fn print_message(msg: string) {
    println(msg)
}

// Implicit return (last expression)
fn multiply(a: int, b: int) -> int {
    a * b  // No return keyword needed
}
```

### 4.2 Default Parameters

```fusion
fn greet(name: string, greeting: string = "Hello") -> string {
    return greeting + ", " + name + "!"
}

// Usage
greet("Alice")                  // "Hello, Alice!"
greet("Bob", "Hi")              // "Hi, Bob!"
```

### 4.3 Variadic Parameters

```fusion
fn sum(numbers: ...int) -> int {
    let mut total = 0
    for num in numbers {
        total += num
    }
    return total
}

// Usage
sum(1, 2, 3, 4, 5)              // 15
```

### 4.4 Lambda/Anonymous Functions

```fusion
// Lambda syntax
let add = |a, b| a + b
let square = |x| x * x

//Multi-line lambda
let complex = |x: int| {
    let doubled = x * 2
    return doubled + 10
}

// Higher-order functions
let numbers = [1, 2, 3, 4, 5]
let doubled = numbers.map(|x| x * 2)
let sum = numbers.reduce(|acc, x| acc + x, 0)
```

### 4.5 Method Syntax

```fusion
class Calculator {
    value: int
    
    // Constructor
    fn new(initial: int) -> Calculator {
        return Calculator { value: initial }
    }
    
    // Instance method (takes self)
    fn add(mut self, amount: int) {
        self.value += amount
    }
    
    // Getter method
    fn get_value(self) -> int {
        return self.value
    }
    
    // Static/associated function
    static fn create_default() -> Calculator {
        return Calculator::new(0)
    }
}
```

## 5. Control Flow

### 5.1 Conditional Statements

```fusion
// If-else
if x > 0 {
    println("Positive")
} else if x < 0 {
    println("Negative")
} else {
    println("Zero")
}

// If as expression
let result = if condition {
    "yes"
} else {
    "no"
}

// Guard clauses
fn divide(a: int, b: int) -> Option<int> {
    if b == 0 {
        return None
    }
    return Some(a / b)
}
```

### 5.2 Pattern Matching

```fusion
// Match expression (exhaustive)
let number = 13
match number {
    0 => println("Zero"),
    1..=10 => println("Small"),
    11..=100 => println("Medium"),
    _ => println("Large")
}

// Match with guards
match value {
    x if x < 0 => println("Negative: {}", x),
    x if x > 0 => println("Positive: {}", x),
    _ => println("Zero")
}

// Destructuring in match
match point {
    (0, 0) => println("Origin"),
    (x, 0) => println("X-axis at {}", x),
    (0, y) => println("Y-axis at {}", y),
    (x, y) => println("Point at ({}, {})", x, y)
}
```

### 5.3 Loops

```fusion
// While loop
let mut count = 0
while count < 10 {
    println(count)
    count += 1
}

// Infinite loop
loop {
    let input = read_input()
    if input == "quit" {
        break
    }
    process(input)
}

// For loop (iterator-based)
for i in 0..10 {
    println(i)
}

// For loop over collection
let numbers = [1, 2, 3, 4, 5]
for num in numbers {
    println(num)
}

// For loop with index
for (index, value) in numbers.enumerate() {
    println("Index {}: {}", index, value)
}

// Loop control
for i in 0..100 {
    if i % 2 == 0 {
        continue  // Skip even numbers
    }
    if i > 50 {
        break     // Exit loop
    }
    println(i)
}
```

## 6. Operators

### 6.1 Arithmetic Operators

```fusion
let a = 10
let b = 3

a + b       // Addition: 13
a - b       // Subtraction: 7
a * b       // Multiplication: 30
a / b       // Division: 3 (integer division)
a % b       // Modulo: 1
a ** b      // Exponentiation: 1000
-a          // Negation: -10
```

### 6.2 Comparison Operators

```fusion
a == b      // Equal: false
a != b      // Not equal: true
a < b       // Less than: false
a <= b      // Less than or equal: false
a > b       // Greater than: true
a >= b      // Greater than or equal: true
```

### 6.3 Logical Operators

```fusion
true and false      // Logical AND: false
true or false       // Logical OR: true
not true            // Logical NOT: false

// Short-circuit evaluation
condition1 and condition2  // condition2 not evaluated if condition1 is false
condition1 or condition2   // condition2 not evaluated if condition1 is true
```

### 6.4 Bitwise Operators

```fusion
a & b       // Bitwise AND
a | b       // Bitwise OR
a ^ b       // Bitwise XOR
~a          // Bitwise NOT
a << 2      // Left shift
a >> 2      // Right shift
```

### 6.5 Assignment Operators

```fusion
let mut x = 10
x += 5      // x = x + 5
x -= 3      // x = x - 3
x *= 2      // x = x * 2
x /= 4      // x = x / 4
x %= 3      // x = x % 3
x **= 2     // x = x ** 2
```

### 6.6 Range Operators

```fusion
0..10       // Exclusive range: [0, 1, 2, ..., 9]
0..=10      // Inclusive range: [0, 1, 2, ..., 10]
..10        // Up to (exclusive): [0, 1, ..., 9]
5           // From: [5, 6, 7, ...]
..          // Unbounded range (all values)
```

## 7. Comments

```fusion
// Single-line comment

/*
 * Multi-line comment
 * Can span multiple lines
 */

/// Documentation comment for the following item
/// Supports Markdown formatting
fn documented_function() {
    // Implementation
}

//! Module-level documentation comment
```

## 8. Attributes

```fusion
// Compiler attributes
#[inline]
fn fast_function() { }

#[deprecated(message = "Use new_function instead")]
fn old_function() { }

// Effect system attributes
@borrowed
fn zero_alloc_function(data: &[u8]) { }

@gpu_accelerated
fn matrix_multiply(a: Tensor, b: Tensor) -> Tensor { }

@constant_time
fn crypto_compare(a: &[u8], b: &[u8]) -> bool { }

@atomic
fn lock_free_increment(counter: &AtomicInt) { }

// Test attribute
#[test]
fn test_addition() {
    assert_eq!(add(2, 3), 5)
}
```

## 9. Error Handling

```fusion
// Result type for recoverable errors
fn parse_number(s:_string) -> Result<int, string> {
    match int::parse(s) {
        Some(n) => Ok(n),
        None => Err("Invalid number format")
    }
}

// Question mark operator for error propagation
fn process_file(path: string) -> Result<string, Error> {
    let content = fs::read_to_string(path)?  // Auto-propagate errors
    let processed = transform(content)?
    return Ok(processed)
}

// Panic for unrecoverable errors
fn critical_operation(value: int) {
    if value < 0 {
        panic("Value cannot be negative!")
    }
    // Continues if value >= 0
}
```

## 10. String Interpolation

```fusion
let name = "Alice"
let age = 30

// String interpolation with {}
println("Hello, {}!", name)
println("{} is {} years old", name, age)

// Formatted output
let pi = 3.14159
println("Pi to 2 decimals: {:.2}", pi)  // "Pi to 2 decimals: 3.14"

// Named placeholders
println("{name} is {age} years old", name=name, age=age)
```

## 11. Module System

```fusion
// Declaring a module
module geometry {
    pub fn area_circle(radius: float) -> float {
        return 3.14159 * radius ** 2
    }
    
    // Private (not exported)
    fn helper() { }
}

// Nested modules
module math::advanced {
    pub fn integral(f: fn(float) -> float, a: float, b: float) -> float {
        // Implementation
    }
}

// Importing
import std::collections::HashMap
import geometry::{area_circle, area_square}
import math::advanced as adv

// Re-exporting
pub import geometry::*
```

## 12. Traits (Interfaces)

```fusion
// Trait definition
trait Drawable {
    fn draw(self)
    fn get_area(self) -> float
}

// Implementing traits
class Circle {
    radius: float
}

impl Drawable for Circle {
    fn draw(self) {
        println("Drawing circle with radius {}", self.radius)
    }
    
    fn get_area(self) -> float {
        return 3.14159 * self.radius ** 2
    }
}

// Trait bounds
fn print_drawable<T: Drawable>(item: T) {
    item.draw()
}

// Multiple trait bounds
fn complex<T: Drawable + Comparable>(item: T) {
    // Implementation
}
```

## Grammar Summary (ANTLR4 Format)

```antlr
grammar Fusion;

// Program structure
program: declaration* EOF;

declaration: 
    functionDecl
    | classDecl
    | traitDecl
    | globalVar
    | moduleDecl
    ;

// Functions
functionDecl:
    FN IDENTIFIER LPAREN paramList? RPAREN (ARROW type)? block
    ;

paramList: param (COMMA param)*;
param: IDENTIFIER COLON type;

// Types
type:
    INT | FLOAT | STRING | BOOL
    | IDENTIFIER
    | type LBRACK RBRACK           // Array
    | type QUESTION                 // Optional
    | LPAREN typeList RPAREN ARROW type  // Function type
    ;

// Expressions
expression: logicalOr;
logicalOr: logicalAnd (OR logicalAnd)*;
logicalAnd: equality (AND equality)*;
equality: comparison ((EQ | NE) comparison)*;
comparison: additive ((LT | LE | GT | GE) additive)*;
additive: multiplicative ((PLUS | MINUS) multiplicative)*;
multiplicative: unary ((MUL | DIV | MOD) unary)*;
unary: (NOT | MINUS) unary | postfix;
postfix: primary (DOT IDENTIFIER | LBRACK expression RBRACK)*;

// Keywords
FN: 'fn'; CLASS: 'class'; TRAIT: 'trait'; LET: 'let';
IF: 'if'; ELSE: 'else'; WHILE: 'while'; FOR: 'for';
IN: 'in'; RETURN: 'return'; BREAK: 'break'; CONTINUE: 'continue';
```

## Training Examples

### Example 1: Basic Calculator

```fusion
fn main() -> int {
    let a = 10
    let b = 5
    
    println("Addition: {} + {} = {}", a, b, a + b)
    println("Subtraction: {} - {} = {}", a, b, a - b)
    println("Multiplication: {} * {} = {}", a, b, a * b)
    println("Division: {} / {} = {}", a, b, a / b)
    
    return 0
}
```

### Example 2: Fibonacci Sequence

```fusion
fn fibonacci(n: int) -> int {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() -> int {
    for i in 0..10 {
        println("fib({}) = {}", i, fibonacci(i))
    }
    return 0
}
```

### Example 3: Generic Stack

```fusion
class Stack<T> {
    items: [T]
    
    fn new() -> Stack<T> {
        return Stack { items: [] }
    }
    
    fn push(mut self, item: T) {
        self.items.push(item)
    }
    
    fn pop(mut self) -> Option<T> {
        return self.items.pop()
    }
    
    fn is_empty(self) -> bool {
        return self.items.len() == 0
    }
}

fn main() -> int {
    let mut stack = Stack::<int>::new()
    stack.push(1)
    stack.push(2)
    stack.push(3)
    
    while not stack.is_empty() {
        match stack.pop() {
            Some(value) => println("Popped: {}", value),
            None => break
        }
    }
    
    return 0
}
```

---

## Key Takeaways for AI Training

1. **Syntax Philosophy**: Fusion favors C-family syntax with modern improvements (no semicolons required, pattern matching, etc.)
2. **Immutability Default**: Variables are immutable unless explicitly marked `mut`
3. **Type Safety**: Strong static typing with type inference
4. **Pattern Matching**: Exhaustive `match` expressions for control flow
5. **Effect System**: Attributes like `@borrowed`, `@gpu_accelerated` modify compilation behavior
6. **Error Handling**: `Result` and `Option` types for explicit error handling
7. **Generics**: Full support for generic programming with type parameters
8. **Traits**: Interface-based polymorphism
9. **Modules**: Hierarchical module system for code organization

This syntax reference provides the foundation for understanding and generating Fusion code. Cross-reference with type system, memory management, and standard library datasets for comprehensive knowledge.
