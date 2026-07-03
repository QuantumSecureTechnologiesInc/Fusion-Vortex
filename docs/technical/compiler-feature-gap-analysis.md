# Compiler Feature Gap Analysis

Documenting what the bootstrap compiler (`bin/fuc.exe`) currently supports vs what the aspirational files need to become bootstrap-compatible.

Generated: 2026-06-25

---

## 1. Bootstrap-Supported Features

Verified via `fuc.exe --sema-only` on all 20 non-aspirational `.fu` files. All pass.

| Feature | Examples | Verified In |
|---|---|---|
| Struct definitions | `struct Foo { x: int, y: string }` | ast.fu, ir.fu, codegen.fu |
| Struct literal construction | `Foo { x: 5, y: "" }` | ast.fu, ir.fu |
| Functions (fn) | `fn bar(a: int) -> int { ... }` | all files |
| Extern functions | `extern fn printf(...) -> int` | all files |
| let bindings | `let x: int = 5` | all files |
| let mut bindings | `let mut i: int = 0` | sema.fu, lexer.fu |
| if/else | `if x > 0 { ... } else { ... }` | all files |
| while loops | `while i < n { i = i + 1 }` | lexer.fu, sema.fu, cli.fu |
| Arrays | `[0; 4096]` | wasm_encoder.fu |
| Pointers (`&T`, `*ptr`) | `&x`, `(*config).field` | codegen.fu, ir.fu |
| String type | `string` (synonym for char pointer) | all files |
| int type | `int` (64-bit signed integer) | all files |
| bool type | `bool` (1-byte) | ir.fu |
| const declarations | `const FOO: int = 42` | wasm_encoder.fu, ir.fu |
| Bitwise AND (`&`) | `x & 0x7F` | wasm_encoder.fu |
| Bitwise OR (`\|`) | `x \| 0x80` | wasm_encoder.fu |
| Member access | `struct.field` | all files |
| Array indexing | `arr[i]` | wasm_encoder.fu |
| Return statements | `return x` | all files |

**Struct ABI limit**: Structs >16 bytes cannot be passed by value. Must use `&T` pointers.

---

## 2. Features Needed by `pure_fusion_compiler.fu`

| Feature | Example Usage | Bootstrap Status | Priority |
|---|---|---|---|
| `impl` blocks with `Self` | `impl PureFusionCompiler { fn new(...) -> Self { ... } }` | **Missing** — bootstrap only supports standalone `fn` | High |
| `Result<T, E>` enum | `fn compile() -> Result<(), FString>` | **Missing** — needs generic enum support | High |
| Generic types (`Option<T>`) | `ast: Option<ast::Program>` | **Missing** — no generic type parameters | High |
| `match` on `Result<Ok, Err>` | `match fs::read_to_string(...) { Ok(s) => ..., Err(e) => ... }` | **Missing** — no pattern matching on enum variants | High |
| `HashMap::new()` | `function_map: HashMap::new()` | **Missing** — no generic collections | Medium |
| `std::time::Instant` | `overall_start: Instant::now()` | **Missing** — external crate FFI | Low |
| `inkwell::context::Context` | `inkwell::context::Context::create()` | **Missing** — external crate FFI | Low |
| `std::io/std::fs/std::path` | `use std::io; use std::fs;` | **Missing** — no Rust stdlib in Fusion | Low |
| `.clone()`, `.take()`, `.expect()` | `self.ast.take().expect("...")` | **Missing** — method call syntax | Medium |

**Effort estimate**: Requires a substantially more mature compiler with generics, pattern matching, and trait impl support. Not viable with current bootstrap. **Best approached after self-hosting is fully functional.**

---

## 3. Features Needed by `wasm/backend.fu`

| Feature | Example Usage | Bootstrap Status | Priority |
|---|---|---|---|
| `impl Trait for Struct` | `impl Backend for WasmBackend { ... }` | **Missing** — bootstrap has basic `impl` but not trait implementations | High |
| Cross-module type imports | `use crate::codegen::{Backend, CodegenConfig, CodegenError}` | **Workaround exists** — use flat module structure + preprocessor | Medium |
| `match` on enum with variant payloads | `match ty { Type::Pointer(_) => ..., Type::Array(_, _) => ... }` | **Missing** — no destructuring patterns | High |
| `Option<T>` unwrapping | `match maybe_idx { Some(idx) => *idx, None => { return Err(...) } }` | **Missing** — no Option type | High |
| `.len()` on arrays/slices | `ir.functions.len()` | **Partially working** — needs `.len()` method | Medium |
| `for` loop | `for func in ir.functions` | **Missing** — no iterator-based loops | Medium |

**Effort estimate**: The `impl Trait` and `match` with payloads are the blockers. Could work around `match` with if/else chains on tag fields (similar to ir.fu Value/Terminator conversion), but `impl Trait for Struct` is a fundamental gap.

---

## 4. Features Needed by `wasm/codegen.fu`

| Feature | Example Usage | Bootstrap Status | Priority |
|---|---|---|---|
| `FVec<T>` generic collection | `param_types: FVec<ValType>` | **Missing** — no generics | High |
| `FMap<K, V>` generic map | `function_map: FMap<FString, u32>` | **Missing** — no generics | High |
| Iterator chains | `params.iter().filter_map(\|p\| ...).collect()` | **Missing** — no closures or iterators | High |
| Closures / lambdas | `\|p\| fusion_to_wasm_type(&p.param_type)` | **Missing** — no closure syntax | High |
| `match` on AST enums | `match decl { Declaration::Function { name, params, ... } => { ... } }` | **Missing** — no variant destructuring | High |
| `.into()` conversions | `"".into()` | **Missing** — no trait-based conversion | Low |
| `#[cfg(test)]` attributes | `#[cfg(test)] mod tests { ... }` | **Missing** — no attributes | Low |
| `.clone()`, `.insert()`, `.clear()` | `self.local_map.clear()` | **Missing** — method calls | Medium |

**Effort estimate**: The most distant aspirational file. Requires generics, closures, iterators, and full pattern matching. **Will naturally become viable as the compiler matures through self-hosting iterations.**

---

## 5. Priority Matrix

| Gap | Blocks | Compiler Maturity Distance |
|---|---|---|
| `match` with enum variant destructuring | pure_fusion_compiler, wasm/backend, wasm/codegen | Near-term (tag-field workaround exists) |
| `impl Trait for Struct` | wasm/backend | Medium-term (needs trait system) |
| Generic types (`Option<T>`, `FVec<T>`, `FMap<K,V>`) | all 3 aspirational files | Medium-term (needs generics) |
| Iterator chains / closures | wasm/codegen | Long-term (needs closures + iterators) |
| External crate FFI (inkwell, std) | pure_fusion_compiler | Very long-term (needs package system) |
| `Result<T, E>` / `Option<T>` as first-class types | pure_fusion_compiler | Medium-term (needs generic enums) |

---

## 6. Recommended Path Forward

1. **Self-hosting maturation** — continue building `fuc2` and the preprocessor. Each self-compilation cycle strengthens the bootstrap.
2. **`match` workaround** — convert remaining `match` on enums to tag-field if/else chains (pattern used in ir.fu).
3. **`impl Trait` implementation** — add trait resolution to the bootstrap compiler (or fuc2 preprocessor).
4. **Generics** — implement monomorphization in the preprocessor (replace `FVec<T>` with concrete `FVec_ValType`, etc.).
5. **Defer external FFI** — `pure_fusion_compiler.fu` with inkwell/std requires package management; defer until self-hosting is stable.