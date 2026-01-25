# Fusion Memory Management and Effect System

**Dataset Category**: Fundamentals
**Training Level**: Intermediate to Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

Fusion features a revolutionary dual-mode memory management system controlled by the **Effect System**. This allows developers to choose between garbage collection (ease of use) and manual memory management (predictable performance) within the same codebase.

## 1. Dual-Mode Memory Management

### 1.1 Garbage Collection (GC) Mode - Default

By default, Fusion uses a generational garbage collector for automatic memory management.

**Characteristics**:
- **Automatic**: No manual memory management required
- **Safe**: Prevents memory leaks and use-after-free errors
- **Ergonomic**: Natural programming style
- **Pauseless**: Concurrent GC minimizes pauses

```fusion
fn process_data() {
    let data = load_large_dataset()  // Allocated on GC heap
    let processed = transform(data)  // Old data becomes garbage
    save_results(processed)          // New data saved
}  // All memory automatically collected when no longer reachable
```text

**GC Implementation**:
- **Algorithm**: Generational garbage collection with concurrent marking
- **Generations**: Young, Old, and Permanent generations
- **Collection**: Minor collections frequent, major collections rare
- **Tuning**: Configurable via environment variables

### 1.2 Borrow Checker Mode - Opt-In

For performance-critical code, use the `@borrowed` attribute to enable Rust-style borrow checking.

**Characteristics**:
- **Zero-cost**: No runtime overhead
- **Predictable**: Deterministic memory behavior
- **Fast**: No GC pauses
- **Safe**: Compile-time ownership verification

```fusion
@borrowed
fn process_audio_buffer(buffer: &mut [f32]) {
    for sample in buffer {
        *sample *= 0.5  // In-place modification, no allocations
    }
}  // Compile-time guarantees no memory leaks
```text

## 2. Ownership and Borrowing

### 2.1 Ownership Rules

When using `@borrowed` mode, Fusion enforces ownership rules:

1. **Each value has exactly one owner**
2. **When the owner goes out of scope, the value is dropped**
3. **Values can be moved or borrowed, but not both simultaneously**

```fusion
@borrowed
fn ownership_example() {
    let s1 = String::from("hello")  // s1 owns the string
    let s2 = s1                     // Ownership moved to s2
    // println(s1)                  // ERROR: s1 no longer valid
    println(s2)                     // OK: s2 owns the string
}  // s2 dropped here, memory freed
```text

### 2.2 Borrowing (References)

Instead of transferring ownership, you can borrow a reference:

```fusion
@borrowed
fn borrowing_example() {
    let s1 = String::from("hello")

    // Immutable borrow
    let len = calculate_length(&s1)  // s1 borrowed, not moved
    println("Length of '{}' is {}", s1, len)  // s1 still valid

    // Mutable borrow
    let mut s2 = String::from("world")
    append_exclamation(&mut s2)
    println(s2)  // "world!"
}

@borrowed
fn calculate_length(s: &String) -> int {
    return s.len()
}  // s goes out of scope, but doesn't drop the string (not owner)

@borrowed
fn append_exclamation(s: &mut String) {
    s.push_str("!")
}
```text

### 2.3 Borrowing Rules

1. **Multiple immutable borrows OR one mutable borrow** (but not both)
2. **References must always be valid** (enforced by lifetime checking)

```fusion
@borrowed
fn borrowing_rules() {
    let mut s = String::from("hello")

    // Multiple immutable borrows - OK
    let r1 = &s
    let r2 = &s
    println("{} and {}", r1, r2)

    // Mutable borrow after immutable borrows - OK (r1, r2 no longer used)
    let r3 = &mut s
    r3.push_str(" world")
    println(r3)

    // ERROR: Cannot have mutable and immutable borrows simultaneously
    // let r4 = &s
    // r3.push_str("!")  // ERROR: r3 still in scope
}
```text

## 3. The Effect System

Fusion's effect system uses attributes to modify compilation behavior and enable advanced features.

### 3.1 Memory Management Effects

#### @borrowed - Borrow Checker Mode

```fusion
// Enable borrow checker for this function
@borrowed
fn zero_alloc_function(data: &[u8]) -> u64 {
    let mut sum: u64 = 0
    for byte in data {
        sum += *byte as u64
    }
    return sum
}  // Guaranteed zero allocations
```text

#### @gc - Explicit GC Mode

```fusion
// Explicitly use GC (default, rarely needed)
@gc
fn flexible_function(data: Vec<int>) {
    let processed = complex_transform(data)
    // GC handles cleanup
}
```text

### 3.2 Performance Effects

#### @gpu_accelerated - GPU Execution

```fusion
use fusion::hal::Tensor

// Automatically compile to GPU kernel
@gpu_accelerated
fn matrix_multiply(a: Tensor<f32>, b: Tensor<f32>) -> Tensor<f32> {
    return a.matmul(b)
}  // Executes on GPU if available, falls back to CPU
```text

#### @inline - Inline Hint

```fusion
// Suggest inlining (performance optimization)
@inline
fn fast_add(a: int, b: int) -> int {
    return a + b
}
```text

#### @inline(always) - Force Inlining

```fusion
// Force inlining (used for critical hot paths)
@inline(always)
fn critical_operation(x: int) -> int {
    return x * 2 + 1
}
```text

### 3.3 Security Effects

#### @constant_time - Timing Attack Prevention

```fusion
use fusion::crypto

// Prevent timing side-channels (critical for cryptography)
@constant_time
fn secure_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false
    }
    let mut diff: u8 = 0
    for i in 0..a.len() {
        diff |= a[i] ^ b[i]
    }
    return diff == 0
}  // Execution time independent of input values
```text

#### @trusted - Unsafe Code Boundary

```fusion
// Mark trusted unsafe code
@trusted
fn raw_pointer_manipulation(ptr: *mut u8, len: int) {
    unsafe {
        for i in 0..len {
            *ptr.offset(i) = 0
        }
    }
}  // Requires security audit
```text

### 3.4 Concurrency Effects

#### @atomic - Atomic Operations

```fusion
use std::sync::AtomicInt

@atomic
fn lockfree_increment(counter: &AtomicInt) {
    counter.fetch_add(1)
}  // Guaranteed atomic access
```text

#### @parallel - Parallel Execution

```fusion
// Automatically parallelize loop
@parallel
fn parallel_sum(numbers: &[int]) -> int {
    let mut sum = 0
    for num in numbers {
        sum += num
    }
    return sum
}  // Executes in parallel across CPU cores
```text

### 3.5 AI/ML Effects

#### @tensoropt - Tensor Optimization

```fusion
use fusion::haft::FluxTensor

@tensoropt
fn train_model(data: FluxTensor<f32>, labels: FluxTensor<f32>) {
    // HAFT agents automatically optimize tensor layout and memory tiers
    let predictions = model.forward(data)
    let loss = compute_loss(predictions, labels)
    gradients = loss.backward()
    optimizer.step(gradients)
}
```text

## 4 Lifetimes (Borrow Checker Mode)

Lifetimes ensure references are always valid.

### 4.1 Lifetime Syntax

```fusion
@borrowed
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x
    } else {
        return y
    }
}  // Return value's lifetime tied to inputs
```text

### 4.2 Lifetime Elision Rules

In many cases, lifetimes can be inferred:

```fusion
@borrowed
fn first_word(s: &str) -> &str {
    // Lifetime automatically inferred
    let bytes = s.as_bytes()
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i]
        }
    }
    return s
}
```text

### 4.3 Struct Lifetimes

```fusion
@borrowed
class ImportantExcerpt<'a> {
    part: &'a str  // Reference must live at least as long as struct
}

@borrowed
fn excerpt_example() {
    let novel = String::from("Call me Ishmael. Some years ago...")
    let first_sentence = novel.split('.').next().unwrap()
    let excerpt = ImportantExcerpt { part: first_sentence }
    // excerpt and novel must have compatible lifetimes
}  // novel dropped, then excerpt
```text

### 4.4 Static Lifetime

```fusion
// 'static lifetime lasts for entire program
let s: &'static str = "I have a static lifetime"

const GREETING: &'static str = "Hello, World!"
```text

## 5. Smart Pointers

Even in `@borrowed` mode, smart pointers provide flexible memory management.

### 5.1 Box<T> - Heap Allocation

```fusion
@borrowed
fn box_example() {
    let b = Box::new(42)  // Allocate on heap
    println("Boxed value: {}", *b)
}  // Box dropped, heap memory freed

// Recursive types require indirection
@borrowed
enum List {
    Cons(int, Box<List>),
    Nil
}
```text

### 5.2 Rc<T> - Reference Counted

```fusion
use std::rc::Rc

@borrowed
fn rc_example() {
    let a = Rc::new(42)
    let b = Rc::clone(&a)  // Increment reference count
    let c = Rc::clone(&a)

    println("Reference count: {}", Rc::strong_count(&a))  // 3
}  // All references dropped, memory freed
```text

### 5.3 Arc<T> - Atomic Reference Counted

```fusion
use std::sync::Arc
use std::thread

fn arc_example() {
    let data = Arc::new(vec![1, 2, 3, 4, 5])

    let mut handles = vec![]
    for i in 0..5 {
        let data_clone = Arc::clone(&data)
        let handle = thread::spawn(move || {
            println("Thread {}: {:?}", i, data_clone)
        })
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap()
    }
}  // data freed when last Arc dropped
```text

### 5.4 RefCell<T> - Interior Mutability

```fusion
use std::cell::RefCell

@borrowed
fn refcell_example() {
    let x = RefCell::new(42)

    // Borrow mutably at runtime
    *x.borrow_mut() += 1

    println("Value: {}", *x.borrow())  // 43
}  // Runtime borrow checking
```text

## 6. Memory Layout and Optimization

### 6.1 Stack vs Heap

```fusion
@borrowed
fn stack_vs_heap() {
    // Stack allocation (fast, fixed size)
    let x: int = 42
    let array: [int; 100] = [0; 100]

    // Heap allocation (flexible size, slower)
    let vec: Vec<int> = vec![1, 2, 3]
    let boxed: Box<int> = Box::new(42)
}
```text

### 6.2 Memory Alignment

```fusion
// Control memory layout

#[repr(C)]  // C-compatible layout

class Point {
    x: f64
    y: f64
}

#[repr(packed)]  // Remove padding

class PackedStruct {
    a: u8
    b: u32
}

#[repr(align(64))]  // Cache line alignment

class CacheAligned {
    data: [u8; 64]
}
```text

## 7. Mixing GC and Borrow Checker Modes

You can mix modes in the same program:

```fusion
// Main logic uses GC for ease of use
fn main() -> int {
    let data = load_data()  // GC allocated
    let result = process_critical_section(&data)
    save_result(result)
    return 0
}

// Critical section uses borrow checker for performance
@borrowed
fn process_critical_section(data: &[f32]) -> f64 {
    let mut sum = 0.0
    for value in data {
        sum += *value
    }
    return sum / data.len() as f64
}
```text

## 8. Memory Management Best Practices

### 8.1 When to Use GC Mode

- **Default choice** for most application code
- UI logic, business logic, scripting
- Code clarity and development speed prioritized
- Acceptable latency variance

### 8.2 When to Use @borrowed Mode

- **Performance-critical paths**: Inner loops, audio/video processing
- **Real-time systems**: Hard latency requirements
- **Embedded systems**: Limited memory, no GC overhead
- **System programming**: OS kernels, device drivers

### 8.3 Optimization Strategy

1. **Start with GC mode** (default)
2. **Profile** to identify hotspots
3. **Apply @borrowed** to critical functions
4. **Measure impact** and iterate

```fusion
// Before optimization (GC)
fn process_batch(items: Vec<Item>) -> Vec<Result> {
    return items.iter().map(|item| expensive_operation(item)).collect()
}

// After optimization (borrow checker + zero-copy)
@borrowed
fn process_batch_optimized(items: &[Item], results: &mut [Result]) {
    for (i, item) in items.iter().enumerate() {
        results[i] = expensive_operation(item)
    }
}
```text

## 9. Training Examples

### Example 1: Mixed-Mode Application

```fusion
// GC mode for high-level logic
fn main() -> int {
    let config = load_configuration()
    let data = fetch_data(&config.source)

    // Switch to borrow checker for processing
    let processed = process_samples(&data)

    save_results(&config.destination, processed)
    return 0
}

@borrowed
fn process_samples(data: &[Sample]) -> Vec<ProcessedSample> {
    let mut results = Vec::with_capacity(data.len())
    for sample in data {
        results.push(transform_sample(sample))
    }
    return results
}

@borrowed
fn transform_sample(sample: &Sample) -> ProcessedSample {
    // Zero-allocation processing
    ProcessedSample {
        value: sample.value * 2,
        timestamp: sample.timestamp
    }
}
```text

---

## Key Takeaways for AI Training

1. **Dual-Mode System**: Choose GC (easy) or borrow checker (fast) per function
2. **Effect Attributes**: `@borrowed`, `@gpu_accelerated`, `@constant_time` modify behavior
3. **Ownership**: Each value has one owner; ownership can be moved or borrowed
4. **Borrowing Rules**: Multiple immutable XOR one mutable borrow
5. **Lifetimes**: Ensure references remain valid (compile-time verification)
6. **Smart Pointers**: Box (heap), Rc (refcount), Arc (thread-safe refcount)
7. **Mix Modes**: Use GC for most code, @borrowed for hotspots
8. **Zero-Cost Abstractions**: Borrow checker has no runtime overhead
9. **Memory Safety**: Prevents use-after-free, double-free, data races
10. **Gradual Optimization**: Start simple (GC), optimize selectively

Memory management is one of Fusion's most powerful features, enabling both developer productivity and maximum performance within a single language. Cross-reference with runtime core and performance optimization datasets.