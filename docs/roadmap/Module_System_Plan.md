# Module System Implementation Plan

**Date**: 2025-12-07
**Phase**: Phase 3 - Multi-file Compilation
**Status**: ⏳ In Progress
**Priority**: High (Foundation for larger projects)

---

## Overview

Implement a module system for Fusion to support multi-file projects, enabling code organization, namespace management, and dependency resolution.

## Design Goals

1. **Rust-inspired syntax** - Familiar to systems programmers
2. **Simple import model** - Easy to understand and use
3. **Namespace isolation** - Prevent naming conflicts
4. **Circular dependency detection** - Catch import cycles early
5. **Incremental compilation** - Only recompile changed modules

---

## Syntax Design

### Module Declaration

```fusion
// lib.fu - Library root
pub mod utils;     // Public module (can be imported by others)
mod internal;      // Private module (only accessible within lib.fu)

pub fn library_function() -> int {
    return utils::helper();
}
```

### Module Import

```fusion
// main.fu
use lib::utils;           // Import specific module
use lib::utils::*;        // Import all public items from module
use lib::{utils, types};  // Import multiple modules

fn main() -> int {
    let x = utils::helper();
    return x;
}
```

### Module Files

<!-- Option 1: File-based modules -->

```text
project/
├── main.fu
├── lib.fu
├── utils.fu        // Corresponds to "mod utils"
└── types.fu        // Corresponds to "mod types"
```

<!-- Option 2: Directory modules -->

```text
project/
├── main.fu
├── lib.fu
└── utils/
    ├── mod.fu      // Module entry point
    ├── string.fu
    └── math.fu
```

---

## Implementation Plan

### Phase 1: AST Extensions

**Add to `ast/mod.rs`**:

```rust
pub enum Declaration {
    // ... existing variants
    ModuleDeclaration {
        name: String,
        is_public: bool,
        path: Option<String>,  // File path if external module
    },
    UseDeclaration {
        path: Vec<String>,     // e.g., ["lib", "utils", "helper"]
        alias: Option<String>, // Optional rename
        import_all: bool,      // true for "use mod::*"
    },
}
```

### Phase 2: Module Resolution

**New module**: `src/module_resolver/mod.rs`

**Responsibilities**:

1. Find module files based on declarations
2. Build dependency graph
3. Detect circular dependencies
4. Determine compilation order

**Algorithm**:

```

1. Start from entry point (main.fu or lib.fu)
2. Parse top-level declarations
3. For each "mod" declaration:
   a. Locate corresponding .fu file
   b. Parse that file recursively
   c. Add to module graph

4. Topological sort to get compilation order
5. Detect cycles (error if found)
```

### Phase 3: Namespace Management

**New module**: `src/namespace/mod.rs`

**Symbol Resolution**:

```rust
struct Namespace {
    modules: HashMap<String, Module>,
    symbols: HashMap<String, SymbolInfo>,
}

struct Module {
    name: String,
    is_public: bool,
    exports: Vec<String>,  // Public functions, types, etc.
}
```

**Resolution Rules**:

1. Local symbols take precedence
2. Imported symbols checked next
3. Qualified names (e.g., `utils::func`) always resolve to specific module
4. Unqualified names check current namespace first

### Phase 4: Lexer/Parser Updates

**Add tokens**:

```rust
// lexer.rs

#[token("mod")]

Mod,

#[token("use")]

Use,
```

**Add parsing**:

```rust
// parser/mod.rs
fn parse_mod_declaration(&mut self) -> Result<Declaration, String>
fn parse_use_declaration(&mut self) -> Result<Declaration, String>
```

### Phase 5: Semantic Analyzer Integration

**Update `semantic_analyzer/mod.rs`**:

1. Track current module context
2. Resolve imports before analyzing function bodies
3. Check visibility (pub vs private)
4. Verify imported symbols exist

### Phase 6: Multi-File Compilation Driver

**Update `src/main.rs`**:

```rust
fn compile_project(entry_point: &str) -> Result<(), String> {
    // 1. Module resolution
    let module_graph = ModuleResolver::resolve(entry_point)?;

    // 2. Compile in dependency order
    let mut compiled_modules = HashMap::new();
    for module_path in module_graph.topological_order() {
        let ast = parse_file(&module_path)?;
        let checked_ast = analyze(ast, &compiled_modules)?;
        let ir = codegen(&checked_ast)?;
        compiled_modules.insert(module_path, ir);
    }

    // 3. Link all modules
    let final_ir = link_modules(compiled_modules)?;

    Ok(())
}
```

---

## Example Usage

### Example 1: Simple Two-File Project

**main.fu**:

```fusion
use utils;

fn main() -> int {
    let result = utils::add(5, 3);
    println("Result: ");
    return result;
}
```

**utils.fu**:

```fusion
pub fn add(a: int, b: int) -> int {
    return a + b;
}

pub fn multiply(a: int, b: int) -> int {
    return a * b;
}
```

**Compilation**:

```bash
fusion_lang -i main.fu

# Automatically finds and compiles utils.fu

```

### Example 2: Multi-Module Library

**lib.fu**:

```fusion
pub mod math;
pub mod string_utils;

pub fn library_version() -> int {
    return 1;
}
```

**math.fu**:

```fusion
pub fn abs(x: int) -> int {
    if x < 0 {
        return 0 - x;
    }
    return x;
}
```

**string_utils.fu**:

```fusion
pub fn length(s: string) -> int {
    return strlen(s);
}
```

**main.fu**:

```fusion
use lib::math;
use lib::string_utils;

fn main() -> int {
    let x = math::abs(-42);
    let len = string_utils::length("hello");
    return x + len;
}
```

---

## Error Handling

### Circular Dependency Detection

```fusion
// a.fu
use b;

// b.fu
use a;  // ERROR: Circular dependency detected: a -> b -> a
```

**Error Message**:

```text
error: circular module dependency detected
  --> b.fu:1:1
   |
1  | use a;
   | ^^^^^^ creates a cycle: a -> b -> a
   |
note: module 'a' was first imported here
  --> a.fu:1:1
   |
1  | use b;
   | ^^^^^^
```

### Missing Module

```fusion
use nonexistent;  // ERROR: Module 'nonexistent' not found
```

**Error Message**:

```text
error: could not find module 'nonexistent'
  --> main.fu:1:5
   |
1  | use nonexistent;
   |     ^^^^^^^^^^^ no file 'nonexistent.fu' found
   |
help: create a file named 'nonexistent.fu' in the same directory
```

---

## Testing Strategy

### Unit Tests

1. **Module Resolution**
   - Test finding single file modules
   - Test directory modules with mod.fu
   - Test nested modules
   - Test circular dependency detection

2. **Namespace**
   - Test symbol visibility (pub vs private)
   - Test name conflicts
   - Test qualified vs unqualified names

3. **Parsing**
   - Test `mod` declaration parsing
   - Test `use` declaration parsing
   - Test various import syntaxes

### Integration Tests

**Test Projects**:

```text
tests/multi_file/
├── simple/
│   ├── main.fu
│   └── utils.fu
├── nested/
│   ├── main.fu
│   ├── lib.fu
│   ├── math.fu
│   └── string.fu
└── circular/
    ├── a.fu
    └── b.fu
```

**Test Command**:

```bash
cargo test --test multi_file_compilation
```

---

## Performance Considerations

### Caching

**Problem**: Recompiling all files on every change is slow

**Solution**: Module-level caching

```rust
struct ModuleCache {
    ast_cache: HashMap<PathBuf, (SystemTime, AST)>,
    ir_cache: HashMap<PathBuf, (SystemTime, String)>,
}

impl ModuleCache {
    fn get_ast(&self, path: &Path) -> Option<AST> {
        let metadata = fs::metadata(path).ok()?;
        let modified = metadata.modified().ok()?;

        if let Some((cached_time, cached_ast)) = self.ast_cache.get(path) {
            if *cached_time == modified {
                return Some(cached_ast.clone());
            }
        }
        None
    }
}
```

### Parallel Compilation

**Independent modules can compile in parallel**:

```rust
use rayon::prelude::*;

let irs: Vec<String> = compilation_order
    .par_iter()  // Parallel iterator
    .map(|module_path| compile_module(module_path))
    .collect()?;
```

---

## Compatibility

### Backward Compatibility

**Single-file programs continue to work**:

```

# Old way (still works)

fusion_lang -i hello.fu

# New way (also works)

fusion_lang -i main.fu

# (automatically finds and compiles imported modules)

```

### Standard Library

**Stdlib becomes a module**:

```fusion
// Before (implicit)
let v = Vector::new();

// After (explicit import)
use std::Vector;
let v = Vector::new();

// Or use wildcard for convenience
use std::*;
let v = Vector::new();
```

**Transition**: Provide `--implicit-std` flag for compatibility

---

## Success Criteria

- [ ] Parse `mod` and `use` declarations
- [ ] Resolve module files from declarations
- [ ] Build module dependency graph
- [ ] Detect circular dependencies with clear errors
- [ ] Compile multi-file projects in correct order
- [ ] Respect pub/private visibility
- [ ] All existing single-file tests still pass
- [ ] New multi-file integration tests pass

---

## Timeline

**Estimated Time**: 2-3 hours

1. **Hour 1**: Lexer/Parser updates, AST extensions
2. **Hour 2**: Module resolver, namespace management
3. **Hour 3**: Integration, testing, documentation

---

**Status**: ⏳ Ready to Implement
**Next Step**: Update lexer with `mod` and `use` tokens