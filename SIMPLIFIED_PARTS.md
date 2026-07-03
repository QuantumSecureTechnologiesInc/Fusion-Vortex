# Simplified Parts — Fusion v2.0 Vortex Compiler

Known simplifications, stubs, and aspirational gaps with their actual codebases. Each entry includes the file, line(s), the code as-is, and what a full implementation would do.

---

## 1. Parser — Aspirational Constructs Skipped

**File:** `crates/fuc/src/parser.rs` L134–141

```rust
Some(Token::KwConst) | Some(Token::KwStatic) | Some(Token::KwEnum) |
Some(Token::KwImpl) | Some(Token::KwTrait) | Some(Token::KwType) => {
    // Skip aspirational constructs: consume until semicolon or brace
    self.skip_aspirational_item();
}
Some(Token::Hash) => {
    // Skip attributes: #[...]
    self.skip_attribute();
}
```

`skip_aspirational_item` (L175–189):
```rust
fn skip_aspirational_item(&mut self) {
    let mut brace_depth = 0i32;
    while self.pos < self.tokens.len() {
        match self.peek() {
            Some(Token::LBrace) => { brace_depth += 1; self.advance(); }
            Some(Token::RBrace) => {
                if brace_depth == 0 { break; }
                brace_depth -= 1;
                self.advance();
            }
            Some(Token::Semicolon) if brace_depth == 0 => { self.advance(); break; }
            _ => { self.advance(); }
        }
    }
}
```

`skip_attribute` (L191–206):
```rust
fn skip_attribute(&mut self) {
    // Skip #[ ... ]
    self.advance(); // skip Hash
    if self.peek() == Some(&Token::LBracket) {
        self.advance(); // skip LBracket
        let mut depth = 1i32;
        while self.pos < self.tokens.len() && depth > 0 {
            match self.peek() {
                Some(Token::LBracket) => { depth += 1; }
                Some(Token::RBracket) => { depth -= 1; }
                _ => {}
            }
            self.advance();
        }
    }
}
```

**Full implementation would:** Parse enum variants, impl blocks with method definitions, trait declarations, const/static initializers, type aliases, and attributes into the AST.

---

## 2. Parser — AddressOf / Dereference Are Identity

**File:** `crates/fuc/src/parser.rs` L637–648

```rust
Some(Token::Ampersand) => {
    self.advance();
    let expr = self.parse_unary()?;
    // AddressOf: skip for now, treat as identity
    Ok(expr)
}
Some(Token::Star) => {
    self.advance();
    let expr = self.parse_unary()?;
    // Dereference: skip for now, treat as identity
    Ok(expr)
}
```

**Full implementation would:** Emit `ExpressionKind::AddressOf(inner)` and `ExpressionKind::Dereference(inner)` nodes.

---

## 3. IR Lowering — For-Loop Bounds Check Missing

**File:** `crates/fuc/src/ir_lower.rs` L309–310

```rust
// TODO: proper bounds check
self.set_terminator(Terminator::Jump(body_id));
```

The for-loop header loads the index and iterates, but there is no bounds-check branch. The loop runs unconditionally.

**Full implementation would:** Insert a compare instruction between the index and the array length, with a conditional jump to the exit block when the index exceeds the bound.

---

## 4. IR Lowering — MemberAccess Always Yields Field 0

**File:** `crates/fuc/src/ir_lower.rs` L457–478

```rust
TypedExpressionKind::MemberAccess { base, field: _field } => {
    let base_val = self.lower_expression(base, var_map);
    let reg = self.next_reg();
    let dest = self.temp_val(reg, expr.ty.clone());
    if let Type::Struct(name) = &base_val.ty {
        let field_idx = 0usize; // simplified: always field 0
        let field_ptr_reg = self.next_reg();
        let field_ptr = self.temp_val(field_ptr_reg, Type::Pointer(Box::new(expr.ty.clone())));
        let name_clone = name.clone();
        self.emit(Instruction::GetFieldPtr {
            dest: field_ptr.clone(),
            base_ptr: base_val,
            field_index: field_idx,
            field_ty: expr.ty.clone(),
            struct_name: name_clone,
        });
        self.emit(Instruction::Load {
            dest: dest.clone(),
            src: Address::Pointer { val: field_ptr, pointed_to_ty: expr.ty.clone() },
        });
    }
    dest
}
```

Also in `lower_lvalue` at L575–579:
```rust
TypedExpressionKind::MemberAccess { base, field: _field } => {
    let base_addr = self.lower_lvalue(base, var_map);
    let base_val = self.lower_expression(base, var_map);
    if let Type::Struct(struct_name) = &base_val.ty {
        let field_idx = 0usize; // simplified
```

**Full implementation would:** Look up the field name in the struct's field list (from `TypedStructDefinition` or `IrStructDef`) and emit the correct index.

---

## 5. IR Lowering — Match Always Jumps to First Arm

**File:** `crates/fuc/src/ir_lower.rs` L516–547

```rust
TypedExpressionKind::Match { scrutinee: _, arms } => {
    let mut arm_blocks: Vec<usize> = Vec::new();
    let merge_id = self.new_block("match_merge".to_string());

    // Create a block for each arm
    for i in 0..arms.len() {
        arm_blocks.push(self.new_block(format!("match_arm_{}", i)));
    }
    let _default_id = self.new_block("match_default".to_string());

    // Jump to first arm (simplified: no pattern matching in IR yet)
    self.set_terminator(Terminator::Jump(arm_blocks[0]));

    // Lower each arm
    for (i, arm) in arms.iter().enumerate() {
        self.switch_to_block(arm_blocks[i]);
        let val = self.lower_expression(&arm.body, var_map);
        let reg = self.next_reg();
        let result = self.temp_val(reg, expr.ty.clone());
        self.emit(Instruction::Copy { dest: result, src: val });
        self.set_terminator(Terminator::Jump(merge_id));
    }

    // Default
    self.switch_to_block(_default_id);
    self.set_terminator(Terminator::Unreachable);

    // Merge
    self.switch_to_block(merge_id);
    let result_reg = self.next_reg();
    self.temp_val(result_reg, expr.ty.clone())
}
```

**Full implementation would:** Emit comparison instructions for each pattern (int equality, bool check, wildcard fallthrough), branching to the correct arm body or default.

---

## 6. WASM Codegen — For-Loop Emits Body Once

**File:** `crates/fuc/src/wasm/codegen.rs` L291–304

```rust
Statement::For { var, iter, body } => {
    // For loop: allocate loop var, iterate over array elements
    let local_idx = self.next_local_index;
    self.local_map.insert(var.clone(), local_idx);
    self.next_local_index += 1;
    // Generate iterator expression
    self.generate_expression(iter, func)?;
    // Store initial value
    func.instruction(&Instruction::LocalSet(local_idx));
    // For now, emit the body once (simplified for-loop)
    for s in &body.statements {
        self.generate_statement(s, func)?;
    }
}
```

**Full implementation would:** Emit a proper WASM `loop`/`block` structure with condition check, body, increment, and back-branch.

---

## 7. WASM Codegen — MemberAccess Returns Base Pointer

**File:** `crates/fuc/src/wasm/codegen.rs` L394–398

```rust
ExpressionKind::MemberAccess { base, field: _field } => {
    // For now, treat member access as returning the base pointer
    // TODO: proper GEP-like offset calculation for struct fields
    self.generate_expression(base, func)?;
}
```

**Full implementation would:** Compute the byte offset of the target field within the struct and add it to the base pointer.

---

## 8. WASM Codegen — Match Scrutinee Dropped, First Arm Used

**File:** `crates/fuc/src/wasm/codegen.rs` L411–420

```rust
ExpressionKind::Match { scrutinee, arms } => {
    // Simplified match: evaluate scrutinee, then first arm body
    self.generate_expression(scrutinee, func)?;
    func.instruction(&Instruction::Drop);
    if let Some(first_arm) = arms.first() {
        self.generate_expression(&first_arm.body, func)?;
    } else {
        func.instruction(&Instruction::I64Const(0));
    }
}
```

**Full implementation would:** Emit `br_table` or nested `if`/`else` blocks to dispatch to the correct arm based on pattern matching.

---

## 9. WASM Codegen — StructLiteral / ArrayLiteral Don't Allocate

**File:** `crates/fuc/src/wasm/codegen.rs` L399–410

```rust
ExpressionKind::StructLiteral { name: _, fields } => {
    // Stack-allocate struct: push each field value
    for (_field_name, field_expr) in fields {
        self.generate_expression(field_expr, func)?;
    }
}
ExpressionKind::ArrayLiteral(elems) => {
    // Push each element onto the stack
    for elem in elems {
        self.generate_expression(elem, func)?;
    }
}
```

**Full implementation would:** Allocate linear memory for the struct/array, store each field/element at the correct offset, and return a pointer.

---

## 10. WASM Codegen — Closures Are Inlined

**File:** `crates/fuc/src/wasm/codegen.rs` L421–424

```rust
ExpressionKind::Closure { params: _, body } => {
    // Simplified: generate closure body
    self.generate_expression(body, func)?;
}
```

**Full implementation would:** Emit a separate WASM function for the closure body, pack captured variables into a heap-allocated environment struct, and return a pair of (function pointer, environment pointer).

---

## 11. LLVM Backend — Feature-Gated

**File:** `crates/fuc/src/codegen/llvm_backend.rs` L1–2

```rust
// LLVM Backend - requires inkwell crate (feature = "llvm")
#![cfg(feature = "llvm")]
```

The entire file is behind `#![cfg(feature = "llvm")]`. Without `--features llvm`, native compilation falls through to an error message.

**Full implementation would:** Either always include the LLVM backend (making inkwell a required dependency) or provide a fallback native codegen path.

---

## 12. LLVM Backend — Complex Address Resolution Unsupported

**File:** `crates/fuc/src/codegen/llvm_backend.rs` L323–337

```rust
/// Resolve an Address to an LLVM PointerValue.
fn resolve_address(&self, addr: &Address) -> Result<PointerValue<'ctx>, CodegenError> {
    match addr {
        Address::Variable { name, .. } => {
            self.value_map.get(name)
                .and_then(|v| if let BasicValueEnum::PointerValue(p) = v { Some(*p) } else { None })
                .ok_or_else(|| CodegenError::LlvmError(format!("Undefined address: {}", name)))
        }
        Address::Pointer { val, .. } => {
            let ptr_val = self.resolve_value(val)?;
            Ok(ptr_val.into_pointer_value())
        }
        _ => Err(CodegenError::Unsupported("Complex address resolution".to_string())),
    }
}
```

Only `Variable` and `Pointer` are resolved. `Element` and `Field` fall through to the wildcard `_ => Err(...)`.

**Full implementation would:** Implement GEP chains for element and field addresses.

---

## 13. LLVM Backend — Phi Nodes Have No Incoming Values

Phi nodes are created but their incoming value/block pairs are not populated. The incoming edges must be added after all predecessor blocks are compiled.

**Full implementation would:** Defer phi incoming population until all predecessor blocks are emitted, then call `phi.add_incoming(...)`.

---

## 14. LLVM Backend — Closure / MakeClosure Stores Raw Function Pointer

`MakeClosure` stores the raw function pointer; captured variables are not packed into an environment struct. No fat pointer is created.

**Full implementation would:** Allocate a heap object containing the function pointer and all captured values, then return a fat pointer.

---

## 15. Optimizer — New Instructions Pass Through Unchanged

**File:** `crates/fuc/src/optimizer.rs` L83–199

The `constant_fold_block` function matches these instructions explicitly:
- `BinaryOperation` (L92–141) — constant-folds int/bool arithmetic
- `Call` (L142–153) — resolves arguments through constants
- `Load` (L154–161) — resolves address through constants
- `Store` (L162–169) — resolves address and value through constants
- `GetElementPtr` (L171–181) — resolves base and index through constants
- `GetFieldPtr` (L182–198) — resolves base through constants

The wildcard arm at L199:
```rust
other => new_instrs.push(other),
```

All other instructions (`Alloca`, `Copy`, `UnaryNot`, `Phi`, `MakeClosure`, `Comment`, `GetAddress`) pass through unchanged.

**Full implementation would:** Add constant-folding rules for `UnaryNot` (e.g., `!true` → `false`), dead `Alloca` elimination, `Copy` propagation, and `Phi` simplification.

---

## 16. Optimizer CFG — Dead Block Elimination Is Simplified

**File:** `crates/fuc/src/optimizer_cfg.rs` L73

```rust
let original_len = func.blocks.len();
func.blocks.retain(|_b| reachable.contains(&0)); // Simplified for bootstrap
func.blocks.len() != original_len
```

The `retain` closure ignores the iterated block (`_b`) and always checks block 0. This means all blocks are treated as reachable.

**Full implementation would:** Use `func.blocks.retain(|i, _b| reachable.contains(&i))` to properly filter unreachable blocks.

---

## 17. SSA Construction — Phi Placement Is Placeholder

**File:** `crates/fuc/src/ssa.rs` L82–103

```rust
// Step 3: Insert Phi Nodes (Placeholder for Cytron's dominance frontier algorithm)
// In a full implementation, we calculate Dominator Trees and Dominance Frontiers here.
self.insert_phi_nodes(&mut ssa_blocks);

// Step 4: Rename variables to use subscripts (x_1, x_2)
self.rename_variables(&mut ssa_blocks, func.entry_block);
```

Both methods are empty stubs:
```rust
fn insert_phi_nodes(&mut self, _blocks: &mut FMap<ir::BlockId, SsaBlock>) {
    // SSA construction requires placing phi nodes at merge points.
    // This is structurally prepared for the dominance frontier pass.
}

fn rename_variables(&mut self, _blocks: &mut FMap<ir::BlockId, SsaBlock>, _entry: ir::BlockId) {
    // Pre-order traversal of dominator tree to assign versions to Variable uses/defs.
}
```

**Full implementation would:** Compute the dominance frontier for each basic block and insert phi nodes at iterated dominance frontiers, then rename variables with version subscripts.

---

## 18. Package System — forge_pkg Stubs

**File:** `crates/fuc/src/forge_pkg.rs` L45–75

```rust
/// Initializes the Forge build system for a given project directory.
pub fn new(project_root: &str) -> Result<Self, FString> {
    // Stub: In a full implementation, this parses fusion.toml.
    // Defaulting to Supernova as the flagship modern runtime.
    let manifest = ProjectManifest {
        name: "fusion_project".to_string(),
        version: "0.1.0".to_string(),
        authors: vec![],
        runtime: RuntimeTarget::Supernova,
        dependencies: vec![],
        build_cache_enabled: true,
    };

    Ok(Self {
        manifest,
        cache_dir: format!("{}/.fusion_cache", project_root),
    })
}

/// Resolves the dependency graph. (Addresses "No dependency graph")
pub fn resolve_dependencies(&self) -> Result<FVec<FString>, FString> {
    let mut resolved_paths = Vec::new();
    for dep in &self.manifest.dependencies {
        // Placeholder for actual registry/git fetching
        let dep_path = dep.path.clone().unwrap_or_else(|| {
            format!("{}/{}-{}", self.cache_dir, dep.name, dep.version)
        });
        resolved_paths.push(dep_path);
    }
    Ok(resolved_paths)
}
```

**Full implementation would:** Parse `fusion.toml` for dependencies, fetch packages from a registry or git, resolve versions, and build a dependency graph.

---

## 19. LSP — Dummy URI and Text

**File:** `crates/fuc/src/lsp.rs` L39–88

```rust
fn read_message(&mut self) -> FString {
    // Read "Content-Length: X\r\n\r\n{...}"
    // Stub for compilation
    "".to_string()
}

fn handle_did_open(&mut self, _msg: FString) {
    // Extract URI and text...
    let uri = "dummy_uri"; // Stub
    let text = "dummy_text"; // Stub
    self.documents.insert(uri.to_string(), text.to_string());
    self.publish_diagnostics(uri.to_string(), text.to_string());
}

fn handle_did_change(&mut self, _msg: FString) {
    let uri = "dummy_uri"; // Stub
    let text = "dummy_text"; // Stub
    self.documents.insert(uri.to_string(), text.to_string());
    self.publish_diagnostics(uri.to_string(), text.to_string());
}
```

**Full implementation would:** Receive real LSP messages over stdin/stdout JSON-RPC, maintain a document store, and respond with real diagnostics.

---

## 20. Runtime / Stdlib — Stubs

### fs.rs L13–15 — `read_to_string` returns placeholder
```rust
/// Reads the entire contents of a file as a string.
pub fn read_to_string(_path: &str) -> Result<FString, FString> {
    // Native bindings would invoke platform-specific read syscalls
    Ok("file_content_placeholder".to_string())
}
```

### reactor.rs L15, L80 — Simplified result type and printf hook
```rust
pub enum PollState {
    /// The task is waiting on external I/O or a timer.
    Pending,
    /// The task has completed and yielded a value.
    Ready(FString), // Simplified for bootstrap: returns a stringified value
}
```
```rust
pub fn run_until_idle(&mut self) {
    // Native printf hook placeholder for debugging
    // printf("HyperRing: Starting event loop with %d tasks.\n", self.tasks.len());
```

### test_framework.rs L55 — Test runner always Ok
```rust
for test in &self.tests {
    if test.is_benchmark { continue; }
    // In native code, we invoke the function pointer
    // let result = (test.func)();
    let result: Result<(), FString> = Ok(()); // Stubbed for bootstrap
```

### rtti.rs L54 — No alignment computation
```rust
// Stub: Alignments should be factored in here
current_offset += size;
```

### fmt.rs L91–93 — Block statement formatting is stubbed
```rust
// Stub: Format block statements
self.push_indent();
self.output.push_str("// Statements go here\n");
```

### macros.rs L48–52 — Macro expansion is identity
```rust
fn expand_statement(&self, stmt: Statement) -> Statement {
    // Stub: recursively expand statements. If a macro like `vec![]` is found
    // in an assignment or let binding, it transforms it into raw AST
    // ArrayLiteral or sequence of push commands.
    stmt
}
```

**Full implementation would:** Wire each of these to real OS primitives, runtime reflection, and formatting logic.

---

## 21. Lexer — Self-Described Stub

**File:** `crates/fuc/src/lexer.rs` L3

```rust
//! This Rust stub provides the type definitions and a minimal tokenizer.
```

The comment is slightly outdated — the Rust lexer is actually fairly complete (50+ tokens, keyword matching, string/int/float literals). The `.fu` lexer in `lexer.fu` is the canonical one.

---

## Summary Grid

| # | Area | Severity | Impact |
|---|---|---|---|
| 1 | Aspirational constructs skipped | Medium | enum/impl/trait/const/static not usable |
| 2 | AddressOf/Dereference identity | Low | Pointer ops not lowered in parser |
| 3 | For-loop bounds check missing | Medium | Infinite loops at runtime |
| 4 | MemberAccess field index = 0 | **High** | Wrong field accessed for non-first fields |
| 5 | Match always takes first arm | **High** | Match is effectively non-functional |
| 6 | WASM for-loop (single iteration) | Medium | For-loops in WASM don't iterate |
| 7 | WASM member access (no offset) | **High** | Struct field access broken in WASM |
| 8 | WASM match (first arm only) | **High** | Match broken in WASM |
| 9 | WASM struct/array literals (no alloc) | Medium | Composite values not materialized |
| 10 | WASM closures (inlined) | Medium | No true closure support in WASM |
| 11 | LLVM feature-gated | Low | Requires `--features llvm` |
| 12 | LLVM complex addresses | Medium | Nested GEP unsupported |
| 13 | LLVM phi nodes (no incoming) | Medium | Merge-point values incorrect |
| 14 | LLVM closures (no capture packing) | Medium | Captured variables lost |
| 15 | Optimizer new-instruction pass-through | Low | Missed optimization opportunities |
| 16 | Optimizer CFG dead-block filter | Low | Dead code not eliminated |
| 17 | SSA phi placement | Medium | SSA form incomplete |
| 18 | Package system stubs | Medium | No external dependency support |
| 19 | LSP dummy data | Low | LSP not functional |
| 20 | Runtime/stdlib stubs | Low | fs, reactor, test, rtti, fmt, macros stubbed |
| 21 | Lexer self-description | Cosmetic | Comment outdated |

**High-severity items (4, 5, 7, 8):** These produce incorrect output at runtime. They are the top candidates for the next hardening pass.

**Medium-severity items (1, 3, 6, 9, 10, 12, 13, 14, 17, 18):** These limit functionality or produce degraded results but don't silently corrupt output.

**Low-severity items (2, 11, 15, 16, 19, 20):** These are missing optimizations, feature gates, or non-critical stubs.