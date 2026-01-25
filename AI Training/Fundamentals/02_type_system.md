# Fusion Type System - Complete Reference

**Dataset Category**: Fundamentals
**Training Level**: Beginner to Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

Fusion features a sophisticated static type system with powerful type inference, generics, traits, and advanced features like higher-kinded types. This document provides comprehensive coverage for AI training.

## 1. Type System Philosophy

Fusion's type system is designed around three core principles:

1. **Safety**: Prevent entire classes of bugs at compile time
2. **Performance**: Zero-runtime-cost abstractions
3. **Inference**: Minimize explicit type annotations

```fusion
// Type inference - no annotations needed
let numbers = [1, 2, 3, 4, 5]              // Inferred as [int]
let sum = numbers.reduce(|a, b| a + b, 0)  // Inferred as int
let doubled = numbers.map(|x| x * 2)       // Inferred as [int]
```text

## 2. Primitive Types

### 2.1 Integer Types

```fusion
// Signed integers
i8      // -128 to 127
i16     // -32768 to 32767
i32     // -2³¹ to 2³¹-1
i64     // -2⁶³ to 2⁶³-1
i128    // -2¹²⁷ to 2¹²⁷-1
int     // Platform-sized (alias for i32 or i64)

// Unsigned integers
u8      // 0 to 255
u16     // 0 to 65535
u32     // 0 to 2³²-1
u64     // 0 to 2⁶⁴-1
u128    // 0 to 2¹²⁸-1
uint    // Platform-sized unsigned

// Usage examples
let byte: u8 = 255
let counter: i32 = -100
let large_num: i64 = 9_223_372_036_854_775_807  // Underscores for readability
```text

### 2.2 Floating-Point Types

```fusion
f32     // Single-precision (32-bit)
f64     // Double-precision (64-bit)
float   // Alias for f64 (default)

// Scientific notation
let small: float = 1.23e-10
let large: float = 9.87e15

// Special values
let infinity = float::INFINITY
let neg_inf = float::NEG_INFINITY
let not_a_number = float::NAN
```text

### 2.3 Boolean Type

```fusion
bool    // true or false

let is_valid: bool = true
let has_errors = false

// Boolean expressions
let result = (x > 0) and (y < 10)
```text

### 2.4 Character and String Types

```fusion
char    // Unicode scalar value (32-bit)
string  // UTF-8 encoded text

let ch: char = 'A'
let emoji: char = '😀'  // Unicode support
let text: string = "Hello, World!"

// String methods
text.len()                  // Length in bytes
text.chars().count()        // Count Unicode characters
text.to_uppercase()         // "HELLO, WORLD!"
text.contains("World")      // true
```text

## 3. Compound Types

### 3.1 Arrays

Arrays are fixed-size, contiguous collections of elements of the same type.

```fusion
// Array type syntax: [T; N] where T is element type, N is size
let numbers: [int; 5] = [1, 2, 3, 4, 5]

// Array initialization
let zeros = [0; 100]        // 100 zeros
let items = [1, 2, 3]       // Size inferred as 3

// Array access
let first = numbers[0]      // Access by index
let last = numbers[numbers.len() - 1]

// Array methods
numbers.len()               // Get length
numbers.iter()              // Get iterator
numbers.contains(&3)        // Check membership
```text

### 3.2 Slices

Slices are dynamically-sized views into arrays or vectors.

```fusion
let numbers = [1, 2, 3, 4, 5]

// Slice type: [T] (unsized, must be behind pointer)
let slice: &[int] = &numbers[1..4]  // [2, 3, 4]

// Slice operations
slice.len()                 // 3
slice.first()               // Some(&2)
slice.last()                // Some(&4)
```text

### 3.3 Vectors (Dynamic Arrays)

```fusion
// Vector type: [T] (growable array)
let mut vec: [int] = []

// Vector operations
vec.push(1)                 // Add element
vec.push(2)
vec.push(3)

vec.pop()                   // Remove last: Some(3)
vec.len()                   // Current length
vec.capacity()              // Allocated capacity

// Vector creation
let v1 = [1, 2, 3]          // From literal
let v2 = [0; 10]            // 10 zeros
let v3 = (0..10).collect()  // From range
```text

### 3.4 Tuples

Tuples are heterogeneous fixed-size collections.

```fusion
// Tuple types
let pair: (int, string) = (42, "answer")
let triple: (f64, bool, char) = (3.14, true, 'A')

// Accessing tuple elements
let (num, text) = pair      // Destructuring
let first = pair.0          // Index access
let second = pair.1

// Unit type (empty tuple)
let unit: () = ()           // Used for functions with no return value
```text

## 4. User-Defined Types

### 4.1 Structs/Classes

```fusion
// Basic struct
class Point {
    x: float
    y: float
}

// Creating instances
let p1 = Point { x: 10.0, y: 20.0 }
let p2 = Point { x: 5.0, ..p1 }  // Struct update syntax (y copied from p1)

// Tuple struct
class Color(u8, u8, u8)
let red = Color(255, 0, 0)

// Unit struct (no fields)
class Marker
let m = Marker
```text

### 4.2 Enums

Enums model data that can be one of several variants.

```fusion
// Simple enum
enum Direction {
    North,
    South,
    East,
    West
}

// Enum with discriminant values
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    ServerError = 500
}

// Enum with associated data
enum Option<T> {
    Some(T),
    None
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}

// Complex enum
enum Message {
    Quit,
    Move { x: int, y: int },
    Write(string),
    ChangeColor(u8, u8, u8)
}

// Pattern matching on enums
fn process_message(msg: Message) {
    match msg {
        Message::Quit => println("Quitting"),
        Message::Move { x, y } => println("Moving to ({}, {})", x, y),
        Message::Write(text) => println("Text: {}", text),
        Message::ChangeColor(r, g, b) => println("Color: ({}, {}, {})", r, g, b)
    }
}
```text

### 4.3 Type Aliases

```fusion
// Create shorthand names for complex types
type UserId = int
type Result<T> = Result<T, Error>
type HashMap<K, V> = std::collections::HashMap<K, V>

// Usage
let user_id: UserId = 42
let result: Result<string> = Ok("Success")
```text

## 5. Generic Types

### 5.1 Generic Structs

```fusion
// Generic class with single type parameter
class Box<T> {
    value: T
}

// Multiple type parameters
class Pair<K, V> {
    key: K
    value: V
}

// Generic methods
impl<T> Box<T> {
    fn new(value: T) -> Box<T> {
        return Box { value }
    }

    fn get(self) -> T {
        return self.value
    }

    fn set(mut self, value: T) {
        self.value = value
    }
}

// Usage
let int_box = Box::new(42)
let str_box = Box::new("hello")
```text

### 5.2 Generic Functions

```fusion
// Generic function
fn identity<T>(x: T) -> T {
    return x
}

// Multiple type parameters
fn swap<T, U>(a: T, b: U) -> (U, T) {
    return (b, a)
}

// Generic with constraints (trait bounds)
fn print_debug<T: Debug>(value: T) {
    println("{:?}", value)
}
```text

## 6. Trait System

Traits define shared behavior across types (similar to interfaces).

### 6.1 Defining Traits

```fusion
// Basic trait
trait Drawable {
    fn draw(self)
}

// Trait with default implementation
trait Iterator<T> {
    fn next(mut self) -> Option<T>

    // Default method implementation
    fn count(mut self) -> int {
        let mut n = 0
        while self.next().is_some() {
            n += 1
        }
        return n
    }
}

// Trait with associated types
trait Container {
    type Item

    fn get(self, index: int) -> Option<Self::Item>
    fn len(self) -> int
}
```text

### 6.2 Implementing Traits

```fusion
class Circle {
    radius: float
}

impl Drawable for Circle {
    fn draw(self) {
        println("Drawing circle with radius {}", self.radius)
    }
}

// Implementing multiple traits
impl Debug for Circle {
    fn fmt(self, f: &mut Formatter) -> Result<()> {
        write!(f, "Circle {{ radius: {} }}", self.radius)
    }
}

impl Clone for Circle {
    fn clone(self) -> Circle {
        return Circle { radius: self.radius }
    }
}
```text

### 6.3 Trait Bounds

```fusion
// Single trait bound
fn print<T: Display>(value: T) {
    println("{}", value)
}

// Multiple trait bounds
fn complex<T: Display + Clone + Debug>(value: T) {
    // Can use methods from all three traits
}

// Where clause for complex bounds
fn complex_fn<T, U>(t: T, u: U)
    where T: Display + Clone,
          U: Debug + PartialEq
{
    // Implementation
}

// Trait objects for dynamic dispatch
let drawables: [&dyn Drawable] = [&circle, &rectangle, &triangle]
for item in drawables {
    item.draw()
}
```text

### 6.4 Marker Traits

```fusion
// Marker traits have no methods, just indicate capabilities
trait Send { }      // Can be transferred across thread boundaries
trait Sync { }      // Safe to share references across threads
trait Copy { }      // Can be copied bit-wise
trait Sized { }     // Has known size at compile time
```text

## 7. Advanced Type Features

### 7.1 Associated Types

```fusion
trait Iterator {
    type Item  // Associated type

    fn next(mut self) -> Option<Self::Item>
}

impl Iterator for Range {
    type Item = int  // Specify associated type

    fn next(mut self) -> Option<int> {
        // Implementation
    }
}
```text

### 7.2 Higher-Kinded Types

```fusion
// Higher-kinded type bounds
trait Functor<F> {
    fn map<A, B>(self, f: fn(A) -> B) -> F<B>
}

// Usage with Option
impl Functor for Option {
    fn map<A, B>(self, f: fn(A) -> B) -> Option<B> {
        match self {
            Some(val) => Some(f(val)),
            None => None
        }
    }
}
```text

### 7.3 Phantom Types

```fusion
// Phantom type parameters (used for type-level programming)
class PhantomData<T>

class Locked<T> {
    data: T
    _phantom: PhantomData<T>
}

class Unlocked<T> {
    data: T
    _phantom: PhantomData<T>
}

// State-based type system (compile-time state machine)
fn lock<T>(unlocked: Unlocked<T>) -> Locked<T> {
    return Locked { data: unlocked.data, _phantom: PhantomData }
}
```text

## 8. Type Inference

### 8.1 Local Type Inference

```fusion
// Full inference
let x = 42                  // Inferred as int
let y = 3.14                // Inferred as float
let s = "hello"             // Inferred as string

// Partial inference
let numbers: [_] = [1, 2, 3]    // [int] inferred
let map = HashMap::<_, int>::new()  // Key type inferred from usage
```text

### 8.2 Turbofocus (Function Return Type Inference)

```fusion
// Return type inferred from context
fn add(a: int, b: int) {
    return a + b  // Return type int inferred
}

// Explicit return type for clarity (recommended for public APIs)
fn multiply(a: int, b: int) -> int {
    return a * b
}
```text

## 9. Type Coercion and Conversion

### 9.1 Explicit Conversion

```fusion
// Type casting with 'as'
let x: i32 = 42
let y: i64 = x as i64
let f: f64 = x as f64

// Lossless conversions
let byte: u8 = 255
let word: u16 = byte as u16  // Always succeeds

// Lossy conversions
let large: i64 = 1000
let small: i32 = large as i32  // May truncate
```text

### 9.2 From/Into Traits

```fusion
// Implementing From for type conversion
impl From<i32> for i64 {
    fn from(value: i32) -> i64 {
        return value as i64
    }
}

// Using From/Into
let x: i32 = 42
let y: i64 = x.into()  // Uses From<i32> for i64
let z = i64::from(x)   // Explicit
```text

## 10. Smart Pointers and References

### 10.1 References

```fusion
// Immutable reference (&T)
let x = 42
let r = &x  // Borrow x

// Mutable reference (&mut T)
let mut y = 10
let mr = &mut y
*mr += 5  // Dereference and modify

// Multiple immutable references allowed
let r1 = &x
let r2 = &x

// Only one mutable reference allowed at a time
let mut z = 20
let mr1 = &mut z
// let mr2 = &mut z  // ERROR: cannot have two mutable references
```text

### 10.2 Box (Heap Allocation)

```fusion
// Box<T> - owned heap-allocated value
let boxed = Box::new(42)
let value = *boxed  // Dereference

// Recursive types require indirection
enum List {
    Cons(int, Box<List>),
    Nil
}

let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
```text

### 10.3 Rc and Arc (Reference Counted)

```fusion
use std::rc::Rc
use std::sync::Arc

// Rc<T> - single-threaded reference counting
let rc1 = Rc::new(42)
let rc2 = Rc::clone(&rc1)  // Increment ref count

// Arc<T> - thread-safe reference counting
let arc1 = Arc::new(42)
let arc2 = Arc::clone(&arc1)
```text

## 11. Lifetime Annotations

```fusion
// Lifetime parameters ensure references remain valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x
    } else {
        return y
    }
}

// Struct with lifetime
class ImportantExcerpt<'a> {
    part: &'a str
}

// Multiple lifetimes
fn complex<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    return x  // Return type tied to 'a
}

// Static lifetime (lives for entire program)
let s: &'static str = "I live forever"
```text

## 12. Type System Training Examples

### Example 1: Generic Binary Search Tree

```fusion
enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>
    }
}

impl<T: Ord> BinaryTree<T> {
    fn insert(mut self, value: T) -> BinaryTree<T> {
        match self {
            BinaryTree::Empty => {
                BinaryTree::Node {
                    value,
                    left: Box::new(BinaryTree::Empty),
                    right: Box::new(BinaryTree::Empty)
                }
            },
            BinaryTree::Node { value: v, left, right } => {
                if value < v {
                    left = Box::new(left.insert(value))
                } else {
                    right = Box::new(right.insert(value))
                }
                BinaryTree::Node { value: v, left, right }
            }
        }
    }
}
```text

### Example 2: Custom Iterator

```fusion
class Counter {
    count: int
}

impl Iterator for Counter {
    type Item = int

    fn next(mut self) -> Option<int> {
        self.count += 1
        if self.count < 6 {
            return Some(self.count)
        } else {
            return None
        }
    }
}

fn main() -> int {
    let counter = Counter { count: 0 }
    for num in counter {
        println("{}", num)  // Prints 1, 2, 3, 4, 5
    }
    return 0
}
```text

---

## Key Takeaways for AI Training

1. **Static Typing with Inference**: Fusion uses static types but infers them extensively
2. **Generics**: Powerful generic programming with type parameters and trait bounds
3. **Traits**: Define shared behavior, similar to interfaces but more powerful
4. **Algebraic Data Types**: Enums can carry data and are matched exhaustively
5. **Smart Pointers**: Box, Rc, Arc for different ownership patterns
6. **Lifetimes**: Ensure reference validity at compile time
7. **Zero-Cost Abstractions**: Generics and traits compile to efficient machine code
8. **Type Safety**: Prevent null pointer errors, use Option<T> instead of null
9. **Trait Objects**: Dynamic dispatch when needed (`dyn Trait`)
10. **Associated Types**: Clean generic interfaces with associated types in traits

This type system reference provides deep coverage for AI training. Cross-reference with syntax, memory management, and standard library datasets for complete understanding.