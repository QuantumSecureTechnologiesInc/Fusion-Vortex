# Fusion Language Tutorial - Getting Started

Welcome to Fusion! This tutorial will guide you through the basics of the Fusion programming language, from your first program to building real applications.

---

## Table of Contents

1. [Installation & Setup](#installation--setup)
2. [Your First Program](#your-first-program)
3. [Language Basics](#language-basics)
4. [Multi-file Projects](#multi-file-projects)
5. [Collections & Iterators](#collections--iterators)
6. [WebAssembly Deployment](#webassembly-deployment)
7. [IDE Integration](#ide-integration)

---

## Installation & Setup

### Prerequisites

- Rust 1.70+ with Cargo
- LLVM 14+ (optional, for native compilation)
- VS Code (recommended for IDE features)

### Building Fusion

```

# Clone the repository

git clone https://github.com/your-org/fusion-lang
cd fusion-lang

# Build the compiler

cargo build --release

# Verify installation

./target/release/fusion_lang --version
```

### Installing the VS Code Extension

```bash
code --install-extension editors/vscode-fusion/fusion-language-0.1.0.vsix
```

---

## Your First Program

### Hello World

Create a file called `hello.fu`:

```fusion
fn main() -> int {
    println("Hello, Fusion!");
    return 0;
}
```

Compile and run:

```bash
fusion_lang -i hello.fu
```

### Understanding the Code

- `fn main() -> int` - Every Fusion program starts with a `main` function
- `println` - Built-in function to print to console
- `return 0` - Exit code (0 = success)

---

## Language Basics

### Variables

```fusion
fn variables_demo() -> int {
    // Immutable by default
    let x = 42;
    let name = "Fusion";

    // Mutable with 'mut'
    let mut counter = 0;
    counter = counter + 1;

    return counter;
}
```

### Types

```fusion
fn types_demo() {
    let num: int = 42;           // Integer
    let pi: float = 3.14159;     // Floating point
    let active: bool = true;     // Boolean
    let message: string = "Hi";  // String
}
```

### Functions

```fusion
fn add(a: int, b: int) -> int {
    return a + b;
}

fn greet(name: string) -> string {
    return "Hello, " + name;
}
```

### Control Flow

**If/Else**:

```fusion
fn check_number(x: int) -> string {
    if x > 0 {
        return "positive";
    } else if x < 0 {
        return "negative";
    } else {
        return "zero";
    }
}
```

**While Loops**:

```fusion
fn count_to_five() -> int {
    let mut i = 0;
    while i < 5 {
        println("Count: " + i);
        i = i + 1;
    }
    return i;
}
```

**For Loops** (with iterator):

```fusion
use iterator::range;

fn sum_to_ten() -> int {
    let mut total = 0;
    let iter = range(1, 11);

    while iter.has_next() {
        let num = iter.next();
        if num.is_some() {
            total = total + num.unwrap();
        }
    }

    return total;  // 55
}
```

### Classes

```fusion
class Person {
    name: string;
    age: int;
}

impl Person {
    fn new(name: string, age: int) -> Person {
        return Person {
            name: name,
            age: age
        };
    }

    fn greet(self) -> string {
        return "Hi, I'm " + self.name;
    }

    fn birthday(mut self) {
        self.age = self.age + 1;
    }
}

fn use_person() {
    let mut person = Person::new("Alice", 30);
    person.birthday();
    println(person.greet());
}
```

### Traits

```fusion
trait Printable {
    fn to_string(self) -> string;
}

impl Printable for Person {
    fn to_string(self) -> string {
        return self.name + " (age " + self.age + ")";
    }
}
```

---

## Multi-file Projects

### Project Structure

```text
my-project/
├── main.fu
└── utils.fu
```

### utils.fu

```fusion
// Utility functions
pub fn fibonacci(n: int) -> int {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

pub fn factorial(n: int) -> int {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}
```

### main.fu

```fusion
// Import the utils module
pub mod utils;

fn main() -> int {
    let fib = utils::fibonacci(10);    // 55
    let fact = utils::factorial(5);     // 120

    println("Fibonacci(10) = " + fib);
    println("Factorial(5) = " + fact);

    return 0;
}
```

### Compile Multi-file Project

```bash
fusion_lang -i main.fu --multi-file
```

---

## Collections & Iterators

### Using HashMap

```fusion
use collections::HashMap;

fn hashmap_example() {
    let mut scores = HashMap::<string, int>::new();

    // Insert key-value pairs
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);

    // Get values
    let alice_score = scores.get("Alice");

    // Check if key exists
    let has_bob = scores.contains_key("Bob");

    // Remove entry
    scores.remove("Charlie");

    // Get size
    let count = scores.len();
}
```

### Using HashSet

```fusion
use collections::HashSet;

fn hashset_example() {
    let mut primes = HashSet::<int>::new();

    // Insert values (duplicates ignored)
    primes.insert(2);
    primes.insert(3);
    primes.insert(5);
    primes.insert(2);  // Duplicate, no effect

    // Check membership
    let has_five = primes.contains(5);    // true
    let has_six = primes.contains(6);     // false

    // Set operations
    let mut evens = HashSet::<int>::new();
    evens.insert(2);
    evens.insert(4);
    evens.insert(6);

    let intersection = primes.intersection(evens);  // {2}
    let union = primes.union(evens);                // {2, 3, 4, 5, 6}
}
```

### Using Iterators

```fusion
use iterator::range;
use iterator::sum;
use iterator::count;

fn iterator_example() -> int {
    // Range iterator
    let numbers = range(1, 11);  // 1..10

    // Sum all numbers
    let total = sum(numbers);    // 55

    // Count elements
    let iter2 = range(0, 100);
    let size = count(iter2);     // 100

    return total;
}
```

---

## WebAssembly Deployment

### Creating a WASM Module

**calculator.fu**:

```fusion
fn add(a: int, b: int) -> int {
    return a + b;
}

fn subtract(a: int, b: int) -> int {
    return a - b;
}

fn multiply(a: int, b: int) -> int {
    return a * b;
}

fn divide(a: int, b: int) -> int {
    if b == 0 {
        return 0;  // Handle division by zero
    }
    return a / b;
}
```

### Compile to WebAssembly

```bash
fusion_lang -i calculator.fu --target wasm -o calculator.wasm
```

### Using in the Browser

**index.html**:

```html
<!DOCTYPE html>
<html>
<head>
    <title>Fusion Calculator</title>
</head>
<body>
    <h1>Fusion WebAssembly Calculator</h1>

    <div>
        <input type="number" id="num1" value="10">
        <select id="operation">
            <option value="add">+</option>
            <option value="subtract">-</option>
            <option value="multiply">×</option>
            <option value="divide">÷</option>
        </select>
        <input type="number" id="num2" value="5">
        <button onclick="calculate()">Calculate</button>
        <span id="result"></span>
    </div>

    <script>
        let wasmModule;

        // Load WebAssembly module
        fetch('calculator.wasm')
            .then(response => response.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes))
            .then(results => {
                wasmModule = results.instance.exports;
                console.log('WASM module loaded!');
            });

        function calculate() {
            if (!wasmModule) {
                alert('WASM not loaded yet');
                return;
            }

            const num1 = parseInt(document.getElementById('num1').value);
            const num2 = parseInt(document.getElementById('num2').value);
            const operation = document.getElementById('operation').value;

            let result;
            switch(operation) {
                case 'add':
                    result = wasmModule.add(num1, num2);
                    break;
                case 'subtract':
                    result = wasmModule.subtract(num1, num2);
                    break;
                case 'multiply':
                    result = wasmModule.multiply(num1, num2);
                    break;
                case 'divide':
                    result = wasmModule.divide(num1, num2);
                    break;
            }

            document.getElementById('result').textContent = '= ' + result;
        }
    </script>
</body>
</html>
```

### Run Locally

```

# Start a simple HTTP server

python3 -m http.server 8000

# Open browser to http://localhost:8000

```

---

## IDE Integration

### VS Code Setup

1. **Install Extension**:

   ```bash
   code --install-extension fusion-language-0.1.0.vsix

```

2. **Features Available**:
   - ✅ Syntax highlighting
   - ✅ Real-time error diagnostics
   - ✅ Auto-completion with snippets
   - ✅ Code folding
   - ✅ Comment toggling (Ctrl+/)

### Using Auto-completion

**Type `fn` and press Tab**:

```fusion
fn ${1:name}(${2:params}) -> ${3:type} {
    $0
}
```

**Type `HashMap` for instant creation**:

```fusion
HashMap<${1:K}, ${2:V}>::new()
```

**Type `class` for class template**:

```fusion
class ${1:Name} {
    $0
}
```

---

## Next Steps

### Learning More

- [Collections Library Guide](../guides/Collections_Guide.md)
- [WebAssembly Deployment Guide](../guides/WASM_Guide.md)
- [Language Reference](../guides/Language_Reference.md)

### Example Projects

- [examples/calculator/](../../examples/calculator/) - Simple calculator
- [examples/web-app/](../../examples/web-app/) - WASM web application
- [examples/multi-file/](../../examples/multi-file/) - Multi-file project

### Getting Help

- Documentation: [docs/](../../docs/)
- Issues: GitHub Issues (coming soon)
- Community: Discord (coming soon)

---

**Happy coding with Fusion!** 🚀
