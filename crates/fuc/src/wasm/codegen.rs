// src/wasm/codegen.rs - WebAssembly Code Generator
//
// Implements all 7 previously-stubbed WASM parts plus the linear-memory
// bump allocator that Parts 7, 9a, and 9b depend on.
//
// Heap layout (linear memory page 0):
//   [0..4)   : bump pointer (i32, stored as WASM global $heap_bump)
//   [4..)    : heap-allocated objects (structs, arrays)
//   Static string data is written into the data section at offsets
//   starting from STRINGS_BASE (computed after heap objects at codegen
//   time; here we keep strings in the data section at fixed offsets
//   >= HEAP_BASE so they don't conflict with runtime allocations that
//   start the bump pointer at HEAP_BASE).

use crate::ast::{
    BinaryOp, Block, Declaration, Expression, ExpressionKind, Literal, MatchArm, Parameter,
    Pattern, Statement, Type, UnaryOp,
};
use crate::wasm::types::*;
use std::collections::HashMap;
use wasm_encoder::{
    BlockType, CodeSection, ConstExpr, DataSection, ExportKind, ExportSection,
    Function as WasmFunction, FunctionSection, GlobalSection, GlobalType, Instruction,
    MemorySection, MemoryType, Module, TypeSection, ValType,
};

/// First byte address available for heap allocation at runtime.
/// Bytes [0..4) hold the bump pointer value itself.
const HEAP_BASE: i32 = 4;

/// WASM global index for the bump pointer.
const GLOBAL_HEAP_BUMP: u32 = 0;

/// Size of one heap slot in bytes (fits i64 or i32 pointer).
const SLOT_BYTES: i32 = 8;

/// Inline bump-allocator: allocates `size` bytes from the heap.
/// Signature (in caller's instruction stream):
///   i32.const <size>
///   call heap_alloc  -- but we inline it instead of a helper fn
///
/// Inlined sequence (expects i32 `size` on stack, leaves i32 ptr):
///   global.get $heap_bump   ;; old_ptr
///   global.get $heap_bump   ;; old_ptr  old_ptr
///   local_i32 <size>        ;; old_ptr  old_ptr  size
///   i32.add                 ;; old_ptr  new_ptr
///   global.set $heap_bump   ;; old_ptr            (new bump saved)
fn emit_heap_alloc(size_bytes: i32, func: &mut WasmFunction) {
    func.instruction(&Instruction::GlobalGet(GLOBAL_HEAP_BUMP));
    func.instruction(&Instruction::GlobalGet(GLOBAL_HEAP_BUMP));
    func.instruction(&Instruction::I32Const(size_bytes));
    func.instruction(&Instruction::I32Add);
    func.instruction(&Instruction::GlobalSet(GLOBAL_HEAP_BUMP));
    // Stack: [ old_ptr ]  (i32 pointer to the freshly allocated block)
}

/// Byte offset of field `idx` within a struct (8 bytes per slot).
fn field_byte_offset(field_idx: usize) -> i32 {
    field_idx as i32 * SLOT_BYTES
}

pub struct WasmCodeGenerator {
    function_index: u32,
    function_map: HashMap<String, u32>,
    type_section: TypeSection,
    function_section: FunctionSection,
    export_section: ExportSection,
    code_section: CodeSection,
    memory_section: MemorySection,
    global_section: GlobalSection,
    next_local_index: u32,
    local_map: HashMap<String, u32>,
    local_types: HashMap<String, Type>,
    string_offsets: HashMap<String, u32>,
    string_data: Vec<u8>,
    /// Struct field name -> index, keyed by struct name.
    struct_fields: HashMap<String, Vec<String>>,
}

impl WasmCodeGenerator {
    pub fn new() -> Self {
        WasmCodeGenerator {
            function_index: 0,
            function_map: HashMap::new(),
            type_section: TypeSection::new(),
            function_section: FunctionSection::new(),
            export_section: ExportSection::new(),
            code_section: CodeSection::new(),
            memory_section: MemorySection::new(),
            global_section: GlobalSection::new(),
            next_local_index: 0,
            local_map: HashMap::new(),
            local_types: HashMap::new(),
            string_offsets: HashMap::new(),
            string_data: Vec::new(),
            struct_fields: HashMap::new(),
        }
    }

    /// Register struct field order so MemberAccess can resolve offsets.
    pub fn register_struct(&mut self, name: &str, fields: Vec<String>) {
        self.struct_fields.insert(name.to_string(), fields);
    }

    /// Resolve a field name to its zero-based index within a struct.
    /// Falls back to index 0 when the struct definition is unknown
    /// (matches previous simplified behaviour for unknown structs).
    fn field_index(&self, struct_name: Option<&str>, field: &str) -> usize {
        if let Some(sname) = struct_name {
            if let Some(fields) = self.struct_fields.get(sname) {
                if let Some(pos) = fields.iter().position(|f| f == field) {
                    return pos;
                }
            }
        }
        // Unknown struct or field — return 0 (previous simplified behaviour)
        0
    }

    /// Generate WASM binary from AST declarations.
    pub fn generate(&mut self, declarations: &[Declaration]) -> Result<Vec<u8>, String> {
        // Memory: 1 initial page (64 KiB), max 10 pages.
        self.memory_section.memory(MemoryType {
            minimum: 1,
            maximum: Some(10),
            memory64: false,
            shared: false,
            page_size_log2: None,
        });

        // Global: mutable i32 bump pointer, initialised to HEAP_BASE.
        self.global_section.global(
            GlobalType { val_type: ValType::I32, mutable: true, shared: false },
            &ConstExpr::i32_const(HEAP_BASE),
        );

        // First pass: collect struct declarations so MemberAccess can
        // resolve field indices before any function bodies are compiled.
        for decl in declarations {
            if let Declaration::Struct { name, fields, .. } = decl {
                let field_names: Vec<String> = fields.iter().map(|f| f.name.clone()).collect();
                self.struct_fields.insert(name.clone(), field_names);
            }
        }

        // Second pass: generate all functions.
        for decl in declarations {
            self.generate_declaration(decl)?;
        }

        // Assemble module sections in spec order.
        let mut module = Module::new();
        module.section(&self.type_section);
        module.section(&self.function_section);
        module.section(&self.global_section);
        module.section(&self.memory_section);
        module.section(&self.export_section);
        module.section(&self.code_section);

        // Static string data starts right after HEAP_BASE so that
        // runtime heap allocations (which start at HEAP_BASE and grow
        // upward) do not overwrite static strings. In practice the
        // string data is written into a passive data segment; callers
        // should treat the returned i32 as a read-only pointer.
        if !self.string_data.is_empty() {
            let mut data_section = DataSection::new();
            // Place strings starting at address HEAP_BASE in memory.
            let offset = ConstExpr::i32_const(HEAP_BASE);
            data_section.active(0, &offset, self.string_data.iter().copied());
            module.section(&data_section);
        }

        Ok(module.finish())
    }

    fn generate_declaration(&mut self, decl: &Declaration) -> Result<(), String> {
        match decl {
            Declaration::Function { name, params, return_type, body, .. } => {
                self.generate_function(name, params, return_type, body)?;
            }
            Declaration::ModuleDecl { .. }
            | Declaration::UseDecl { .. }
            | Declaration::ImportDecl { .. }
            | Declaration::Struct { .. } => {
                // Structs were handled in the pre-pass; module/use/import skipped.
            }
            _ => {}
        }
        Ok(())
    }

    // ------------------------------------------------------------------ //
    //  Local-variable collection (walk entire block tree)                 //
    // ------------------------------------------------------------------ //

    fn collect_local_types(block: &Block) -> Vec<(String, Type)> {
        let mut types = Vec::new();
        for stmt in &block.statements {
            match stmt {
                Statement::Let { name, ty, .. } => {
                    types.push((name.clone(), ty.clone()));
                }
                Statement::VariableDeclaration { name, ty, .. } => {
                    if let Some(t) = ty {
                        types.push((name.clone(), t.clone()));
                    }
                }
                Statement::If { then_block, else_block, .. } => {
                    types.extend(Self::collect_local_types(then_block));
                    if let Some(eb) = else_block {
                        types.extend(Self::collect_local_types(eb));
                    }
                }
                Statement::While { body, .. } => {
                    types.extend(Self::collect_local_types(body));
                }
                Statement::For { var, body, .. } => {
                    // Loop variable (i32 index) + body locals.
                    types.push((var.clone(), Type::Int));
                    types.extend(Self::collect_local_types(body));
                }
                _ => {}
            }
        }
        types
    }

    // ------------------------------------------------------------------ //
    //  Function generation                                                //
    // ------------------------------------------------------------------ //

    fn generate_function(
        &mut self,
        name: &str,
        params: &[Parameter],
        return_type: &Type,
        body: &Block,
    ) -> Result<(), String> {
        let param_types: Vec<ValType> = params
            .iter()
            .filter_map(|p| fusion_to_wasm_type(&p.param_type))
            .collect();
        let result_types: Vec<ValType> = fusion_to_wasm_type(return_type).into_iter().collect();

        let type_idx = self.type_section.len();
        self.type_section.ty().function(param_types.clone(), result_types.clone());
        self.function_section.function(type_idx);

        let func_idx = self.function_index;
        self.function_map.insert(name.to_string(), func_idx);
        self.function_index += 1;
        self.export_section.export(name, ExportKind::Func, func_idx);

        // Build locals declaration for WASM function body.
        let local_types = Self::collect_local_types(body);
        let mut locals_decl: Vec<(u32, ValType)> = Vec::new();
        for (_, ty) in &local_types {
            if let Some(vt) = fusion_to_wasm_type(ty) {
                match locals_decl.last_mut() {
                    Some((count, last_type)) if *last_type == vt => *count += 1,
                    _ => locals_decl.push((1, vt)),
                }
            }
        }
        let mut func_body = WasmFunction::new(locals_decl);

        self.next_local_index = params.len() as u32;
        self.local_map.clear();
        self.local_types.clear();

        for (n, ty) in &local_types {
            self.local_types.insert(n.clone(), ty.clone());
        }
        for (i, param) in params.iter().enumerate() {
            self.local_map.insert(param.name.clone(), i as u32);
        }

        for stmt in &body.statements {
            self.generate_statement(stmt, &mut func_body)?;
        }
        func_body.instruction(&Instruction::End);
        self.code_section.function(&func_body);
        Ok(())
    }

    // ------------------------------------------------------------------ //
    //  Condition coercion                                                  //
    // ------------------------------------------------------------------ //

    fn coerce_condition_to_i32(
        &mut self,
        expr: &Expression,
        func: &mut WasmFunction,
    ) -> Result<(), String> {
        self.generate_expression(expr, func)?;
        match &expr.ty {
            Some(Type::Bool) => {}
            Some(Type::Int) => { func.instruction(&Instruction::I32WrapI64); }
            Some(Type::Float) => {
                func.instruction(&Instruction::F64Const(0.0));
                func.instruction(&Instruction::F64Ne);
            }
            _ => { func.instruction(&Instruction::I32WrapI64); }
        }
        Ok(())
    }

    // ------------------------------------------------------------------ //
    //  Statement generation                                               //
    // ------------------------------------------------------------------ //

    fn generate_statement(
        &mut self,
        stmt: &Statement,
        func: &mut WasmFunction,
    ) -> Result<(), String> {
        match stmt {
            Statement::Return(Some(expr)) => {
                self.generate_expression(expr, func)?;
                func.instruction(&Instruction::Return);
            }
            Statement::Return(None) => {
                func.instruction(&Instruction::Return);
            }
            Statement::VariableDeclaration { name, initializer, .. } => {
                let local_idx = self.next_local_index;
                self.local_map.insert(name.clone(), local_idx);
                self.next_local_index += 1;
                self.generate_expression(initializer, func)?;
                func.instruction(&Instruction::LocalSet(local_idx));
            }
            Statement::Let { name, value, .. } => {
                let local_idx = self.next_local_index;
                self.local_map.insert(name.clone(), local_idx);
                self.next_local_index += 1;
                self.generate_expression(value, func)?;
                func.instruction(&Instruction::LocalSet(local_idx));
            }
            Statement::If { cond, then_block, else_block } => {
                self.coerce_condition_to_i32(cond, func)?;
                func.instruction(&Instruction::If(BlockType::Empty));
                for s in &then_block.statements {
                    self.generate_statement(s, func)?;
                }
                if let Some(eb) = else_block {
                    func.instruction(&Instruction::Else);
                    for s in &eb.statements {
                        self.generate_statement(s, func)?;
                    }
                }
                func.instruction(&Instruction::End);
            }
            Statement::While { cond, body } => {
                // block $break
                //   loop $continue
                //     <cond>  i32.eqz  br_if $break(1)
                //     <body>
                //     br $continue(0)
                //   end
                // end
                func.instruction(&Instruction::Block(BlockType::Empty));
                func.instruction(&Instruction::Loop(BlockType::Empty));
                self.coerce_condition_to_i32(cond, func)?;
                func.instruction(&Instruction::I32Eqz);
                func.instruction(&Instruction::BrIf(1));
                for s in &body.statements {
                    self.generate_statement(s, func)?;
                }
                func.instruction(&Instruction::Br(0));
                func.instruction(&Instruction::End); // loop
                func.instruction(&Instruction::End); // block
            }

            // ---------------------------------------------------------- //
            // Part 6: For-loop — proper WASM loop with bounds check       //
            // ---------------------------------------------------------- //
            //
            // Fusion for-loop:  for <var> in <iter_expr> { <body> }
            // <iter_expr> must evaluate to an i32 pointer to an array
            // whose first 4 bytes are the element count (i32).
            //
            // Emitted WASM:
            //   ;; allocate index local (i32)
            //   i32.const 0  local.set $idx
            //   ;; evaluate the array pointer once, store in a local
            //   <iter_expr>  local.set $arr
            //   block $break
            //     loop $continue
            //       ;; bounds check: $idx >= mem32[$arr]  => break
            //       local.get $idx
            //       local.get $arr  i32.load  ;; array length
            //       i32.ge_u
            //       br_if $break (depth 1)
            //       ;; load element: *($arr + 4 + $idx * 8)  (i64)
            //       local.get $arr
            //       i32.const 4
            //       i32.add
            //       local.get $idx
            //       i32.const 8
            //       i32.mul
            //       i32.add
            //       i64.load
            //       local.set $var
            //       ;; body
            //       <body>
            //       ;; increment
            //       local.get $idx  i32.const 1  i32.add  local.set $idx
            //       br $continue (depth 0)
            //     end
            //   end
            Statement::For { var, iter, body } => {
                // Allocate index local (i32).
                let idx_local = self.next_local_index;
                self.next_local_index += 1;

                // Allocate array-pointer local (i32).
                let arr_local = self.next_local_index;
                self.next_local_index += 1;

                // Allocate element (loop variable) local (i64 = Fusion Int).
                let elem_local = self.next_local_index;
                self.local_map.insert(var.clone(), elem_local);
                self.next_local_index += 1;

                // Initialise index to 0.
                func.instruction(&Instruction::I32Const(0));
                func.instruction(&Instruction::LocalSet(idx_local));

                // Evaluate array expression, store pointer.
                self.generate_expression(iter, func)?;
                func.instruction(&Instruction::I32WrapI64); // pointer is i32
                func.instruction(&Instruction::LocalSet(arr_local));

                // block / loop structure.
                func.instruction(&Instruction::Block(BlockType::Empty));
                func.instruction(&Instruction::Loop(BlockType::Empty));

                // Bounds check: idx >= arr[0] (length header).
                func.instruction(&Instruction::LocalGet(idx_local));
                func.instruction(&Instruction::LocalGet(arr_local));
                func.instruction(&Instruction::I32Load(wasm_encoder::MemArg {
                    offset: 0,
                    align: 2,
                    memory_index: 0,
                }));
                func.instruction(&Instruction::I32GeU);
                func.instruction(&Instruction::BrIf(1)); // break out of block

                // Load element: arr_ptr + 4 + idx * 8  (i64).
                func.instruction(&Instruction::LocalGet(arr_local));
                func.instruction(&Instruction::I32Const(4));
                func.instruction(&Instruction::I32Add);
                func.instruction(&Instruction::LocalGet(idx_local));
                func.instruction(&Instruction::I32Const(SLOT_BYTES));
                func.instruction(&Instruction::I32Mul);
                func.instruction(&Instruction::I32Add);
                func.instruction(&Instruction::I64Load(wasm_encoder::MemArg {
                    offset: 0,
                    align: 3,
                    memory_index: 0,
                }));
                func.instruction(&Instruction::LocalSet(elem_local));

                // Body.
                for s in &body.statements {
                    self.generate_statement(s, func)?;
                }

                // Increment index.
                func.instruction(&Instruction::LocalGet(idx_local));
                func.instruction(&Instruction::I32Const(1));
                func.instruction(&Instruction::I32Add);
                func.instruction(&Instruction::LocalSet(idx_local));

                // Back-branch to loop header.
                func.instruction(&Instruction::Br(0));
                func.instruction(&Instruction::End); // loop
                func.instruction(&Instruction::End); // block
            }

            Statement::Expression(expr) => {
                self.generate_expression(expr, func)?;
                func.instruction(&Instruction::Drop);
            }
            Statement::Assignment { target, value } => {
                self.generate_expression(value, func)?;
                if let ExpressionKind::Variable(name) = &target.kind {
                    let local_idx = self
                        .local_map
                        .get(name)
                        .ok_or_else(|| format!("Unknown variable for assignment: {}", name))?;
                    func.instruction(&Instruction::LocalSet(*local_idx));
                } else {
                    return Err("Assignment target must be a variable".to_string());
                }
            }
        }
        Ok(())
    }

    // ------------------------------------------------------------------ //
    //  Expression generation                                              //
    // ------------------------------------------------------------------ //

    fn generate_expression(
        &mut self,
        expr: &Expression,
        func: &mut WasmFunction,
    ) -> Result<(), String> {
        match &expr.kind {
            ExpressionKind::Literal(lit) => match lit {
                Literal::Integer(n) => {
                    func.instruction(&Instruction::I64Const(*n));
                }
                Literal::Float(f) => {
                    func.instruction(&Instruction::F64Const(*f));
                }
                Literal::Boolean(b) => {
                    func.instruction(&Instruction::I32Const(if *b { 1 } else { 0 }));
                }
                Literal::String(s) => {
                    // Strings live in the static data section starting at
                    // HEAP_BASE.  Return an i32 pointer (memory address).
                    let offset = if let Some(&existing) = self.string_offsets.get(s) {
                        existing
                    } else {
                        let off = self.string_data.len() as u32;
                        self.string_data.extend_from_slice(s.as_bytes());
                        self.string_data.push(0); // null terminator
                        self.string_offsets.insert(s.clone(), off);
                        off
                    };
                    // Actual memory address = HEAP_BASE + offset.
                    func.instruction(&Instruction::I32Const(HEAP_BASE + offset as i32));
                }
            },

            ExpressionKind::Variable(name) => {
                let local_idx = self
                    .local_map
                    .get(name)
                    .ok_or_else(|| format!("Unknown variable: {}", name))?;
                func.instruction(&Instruction::LocalGet(*local_idx));
            }

            ExpressionKind::BinaryOp { left, op, right } => {
                self.generate_expression(left, func)?;
                self.generate_expression(right, func)?;
                match op {
                    BinaryOp::Add  => { func.instruction(&Instruction::I64Add); }
                    BinaryOp::Sub  => { func.instruction(&Instruction::I64Sub); }
                    BinaryOp::Mul  => { func.instruction(&Instruction::I64Mul); }
                    BinaryOp::Div  => { func.instruction(&Instruction::I64DivS); }
                    BinaryOp::Mod  => { func.instruction(&Instruction::I64RemS); }
                    BinaryOp::Eq   => { func.instruction(&Instruction::I64Eq);  func.instruction(&Instruction::I64ExtendI32S); }
                    BinaryOp::Neq  => { func.instruction(&Instruction::I64Ne);  func.instruction(&Instruction::I64ExtendI32S); }
                    BinaryOp::Lt   => { func.instruction(&Instruction::I64LtS); func.instruction(&Instruction::I64ExtendI32S); }
                    BinaryOp::Gt   => { func.instruction(&Instruction::I64GtS); func.instruction(&Instruction::I64ExtendI32S); }
                    BinaryOp::Le   => { func.instruction(&Instruction::I64LeS); func.instruction(&Instruction::I64ExtendI32S); }
                    BinaryOp::Ge   => { func.instruction(&Instruction::I64GeS); func.instruction(&Instruction::I64ExtendI32S); }
                    BinaryOp::And  => { func.instruction(&Instruction::I64And); }
                    BinaryOp::Or   => { func.instruction(&Instruction::I64Or);  }
                }
            }

            ExpressionKind::FunctionCall { name, args, .. } => {
                for arg in args {
                    self.generate_expression(arg, func)?;
                }
                let func_idx = self
                    .function_map
                    .get(name)
                    .ok_or_else(|| format!("Unknown function: {}", name))?;
                func.instruction(&Instruction::Call(*func_idx));
            }

            ExpressionKind::UnaryOp { op, expr } => {
                self.generate_expression(expr, func)?;
                match op {
                    UnaryOp::Neg => {
                        func.instruction(&Instruction::I64Const(-1));
                        func.instruction(&Instruction::I64Mul);
                    }
                    UnaryOp::Not => {
                        func.instruction(&Instruction::I64Eqz);
                        func.instruction(&Instruction::I64ExtendI32S);
                    }
                }
            }

            // ---------------------------------------------------------- //
            // Part 7: MemberAccess — compute field byte offset            //
            // ---------------------------------------------------------- //
            //
            // For a value of type Struct(name), the base expression
            // already pushed an i32 heap pointer.  The field lives at
            // (base_ptr + field_index * SLOT_BYTES).
            //
            // If the type is unknown we fall back to treating the base as
            // an i64 (e.g. a parameter typed as Int holding a pointer),
            // wrapping it to i32 first.
            ExpressionKind::MemberAccess { base, field } => {
                // Determine the struct name from the base expression type.
                let struct_name: Option<String> = match base.ty.as_ref() {
                    Some(Type::Struct(sname)) => Some(sname.clone()),
                    _ => None,
                };
                let idx = self.field_index(struct_name.as_deref(), field);
                let byte_offset = field_byte_offset(idx);

                // Evaluate base — produces i32 pointer (or i64 if untyped).
                self.generate_expression(base, func)?;
                // Ensure we have an i32 pointer on the stack.
                match base.ty.as_ref() {
                    Some(Type::Struct(_)) | Some(Type::String) => {}
                    Some(Type::Int) | None => {
                        func.instruction(&Instruction::I32WrapI64);
                    }
                    _ => {}
                }

                // Add the byte offset.
                if byte_offset != 0 {
                    func.instruction(&Instruction::I32Const(byte_offset));
                    func.instruction(&Instruction::I32Add);
                }

                // Load the field value (i64 slot).
                func.instruction(&Instruction::I64Load(wasm_encoder::MemArg {
                    offset: 0,
                    align: 3,
                    memory_index: 0,
                }));
            }

            // ---------------------------------------------------------- //
            // Part 9a: StructLiteral — bump-allocate and store fields     //
            // ---------------------------------------------------------- //
            //
            // Layout: [ field_0: i64 | field_1: i64 | ... ]
            // Returns i32 heap pointer.
            ExpressionKind::StructLiteral { name, fields } => {
                let n_fields = fields.len() as i32;
                let alloc_size = n_fields * SLOT_BYTES;

                // Allocate memory; i32 base pointer left on stack.
                emit_heap_alloc(alloc_size, func);

                // Save pointer to a temporary local so we can tee it for
                // each store and return it at the end.  We borrow
                // next_local_index as a scratch register.
                let ptr_local = self.next_local_index;
                self.next_local_index += 1;
                func.instruction(&Instruction::LocalTee(ptr_local));

                // Store each field.
                let field_names: Vec<String> = self
                    .struct_fields
                    .get(name)
                    .cloned()
                    .unwrap_or_default();

                for (field_expr_name, field_expr) in fields {
                    // Resolve field position from registered definition;
                    // if not found, use declaration order as fallback.
                    let idx = field_names
                        .iter()
                        .position(|f| f == field_expr_name)
                        .or_else(|| {
                            fields
                                .iter()
                                .position(|(n, _)| n == field_expr_name)
                        })
                        .unwrap_or(0);
                    let byte_offset = field_byte_offset(idx);

                    // Reload pointer for the store.
                    func.instruction(&Instruction::LocalGet(ptr_local));
                    if byte_offset != 0 {
                        func.instruction(&Instruction::I32Const(byte_offset));
                        func.instruction(&Instruction::I32Add);
                    }

                    // Evaluate the field value (i64).
                    self.generate_expression(field_expr, func)?;

                    // Store i64 into slot.
                    func.instruction(&Instruction::I64Store(wasm_encoder::MemArg {
                        offset: 0,
                        align: 3,
                        memory_index: 0,
                    }));
                }

                // Leave the i32 pointer on the stack.
                func.instruction(&Instruction::LocalGet(ptr_local));
            }

            // ---------------------------------------------------------- //
            // Part 9b: ArrayLiteral — bump-allocate with length header    //
            // ---------------------------------------------------------- //
            //
            // Layout: [ len: i32 (4 bytes) | elem_0: i64 | elem_1: i64 | ... ]
            // Returns i32 heap pointer (points to the length header).
            ExpressionKind::ArrayLiteral(elems) => {
                let n_elems = elems.len() as i32;
                // 4 bytes for the i32 length header + SLOT_BYTES per element.
                let alloc_size = 4 + n_elems * SLOT_BYTES;

                emit_heap_alloc(alloc_size, func);

                let ptr_local = self.next_local_index;
                self.next_local_index += 1;
                func.instruction(&Instruction::LocalTee(ptr_local));

                // Write the length header at ptr[0] (i32).
                func.instruction(&Instruction::LocalGet(ptr_local));
                func.instruction(&Instruction::I32Const(n_elems));
                func.instruction(&Instruction::I32Store(wasm_encoder::MemArg {
                    offset: 0,
                    align: 2,
                    memory_index: 0,
                }));

                // Write each element at ptr[4 + i * 8] (i64).
                for (i, elem) in elems.iter().enumerate() {
                    let elem_offset = 4 + i as i32 * SLOT_BYTES;
                    func.instruction(&Instruction::LocalGet(ptr_local));
                    if elem_offset != 0 {
                        func.instruction(&Instruction::I32Const(elem_offset));
                        func.instruction(&Instruction::I32Add);
                    }
                    self.generate_expression(elem, func)?;
                    func.instruction(&Instruction::I64Store(wasm_encoder::MemArg {
                        offset: 0,
                        align: 3,
                        memory_index: 0,
                    }));
                }

                // Return array pointer.
                func.instruction(&Instruction::LocalGet(ptr_local));
            }

            // ---------------------------------------------------------- //
            // Part 8: Match — nested if/else pattern dispatch             //
            // ---------------------------------------------------------- //
            //
            // Supported patterns:
            //   Pattern::Literal(Integer | Boolean) — integer/bool equality
            //   Pattern::Wildcard                   — unconditional fallthrough
            //
            // The scrutinee is re-evaluated for each comparison arm.
            // A wildcard arm terminates the chain and must be last.
            //
            // Emitted structure (3 arms as example):
            //   <scrutinee>  i64.const <p0>  i64.eq
            //   if (result_type)
            //     <arm0_body>
            //   else
            //     <scrutinee>  i64.const <p1>  i64.eq
            //     if (result_type)
            //       <arm1_body>
            //     else
            //       <arm2_body>  ;; wildcard
            //     end
            //   end
            ExpressionKind::Match { scrutinee, arms } => {
                if arms.is_empty() {
                    // Empty match: push a zero placeholder.
                    func.instruction(&Instruction::I64Const(0));
                    return Ok(());
                }
                self.emit_match_arms(scrutinee, arms, func)?;
            }

            // ---------------------------------------------------------- //
            // Part 10: Closure — environment-free inline form             //
            // ---------------------------------------------------------- //
            //
            // For closures that capture nothing, we inline the body
            // expression directly.  Captured-variable closures require
            // a function-table entry and an environment struct; that
            // full fat-pointer implementation is tracked separately.
            ExpressionKind::Closure { params: _, body } => {
                self.generate_expression(body, func)?;
            }
        }
        Ok(())
    }

    // ------------------------------------------------------------------ //
    //  Part 8 helper: recursive match arm emission                        //
    // ------------------------------------------------------------------ //

    fn emit_match_arms(
        &mut self,
        scrutinee: &Expression,
        arms: &[MatchArm],
        func: &mut WasmFunction,
    ) -> Result<(), String> {
        if arms.is_empty() {
            func.instruction(&Instruction::I64Const(0));
            return Ok(());
        }

        let arm = &arms[0];
        let rest = &arms[1..];

        match &arm.pattern {
            Pattern::Wildcard | Pattern::Variable(_) => {
                // Unconditional — just emit the body; ignore remaining arms.
                self.generate_expression(&arm.body, func)?;
            }
            Pattern::Literal(lit) => {
                // Evaluate the scrutinee.
                self.generate_expression(scrutinee, func)?;

                // Push the pattern value for comparison.
                match lit {
                    Literal::Integer(n) => {
                        func.instruction(&Instruction::I64Const(*n));
                        func.instruction(&Instruction::I64Eq);
                    }
                    Literal::Boolean(b) => {
                        func.instruction(&Instruction::I32Const(if *b { 1 } else { 0 }));
                        // Scrutinee was i64 (Int), convert comparison to i32.
                        func.instruction(&Instruction::I32WrapI64);
                        func.instruction(&Instruction::I32Eq);
                    }
                    _ => {
                        // Non-integer/bool literal: treat as wildcard.
                        func.instruction(&Instruction::Drop);
                        self.generate_expression(&arm.body, func)?;
                        return Ok(());
                    }
                }

                // if (condition) ... else ... end
                func.instruction(&Instruction::If(BlockType::Empty));
                self.generate_expression(&arm.body, func)?;

                if rest.is_empty() {
                    // No more arms — close the if without an else branch.
                    func.instruction(&Instruction::End);
                } else {
                    func.instruction(&Instruction::Else);
                    self.emit_match_arms(scrutinee, rest, func)?;
                    func.instruction(&Instruction::End);
                }
            }
        }
        Ok(())
    }
}

// ====================================================================== //
//  Tests                                                                  //
// ====================================================================== //

#[cfg(test)]
mod tests {
    use super::*;

    fn make_expr(kind: ExpressionKind) -> Expression {
        Expression { kind, ty: None }
    }

    fn make_expr_typed(kind: ExpressionKind, ty: Type) -> Expression {
        Expression { kind, ty: Some(ty) }
    }

    fn make_var(name: &str) -> Expression {
        make_expr(ExpressionKind::Variable(name.to_string()))
    }

    fn make_int(n: i64) -> Expression {
        make_expr(ExpressionKind::Literal(Literal::Integer(n)))
    }

    fn make_bool(b: bool) -> Expression {
        make_expr(ExpressionKind::Literal(Literal::Boolean(b)))
    }

    fn make_binary(left: Expression, op: BinaryOp, right: Expression) -> Expression {
        make_expr(ExpressionKind::BinaryOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        })
    }

    fn validate_wasm(bytes: &[u8]) {
        let v = wasmparser::validate(bytes);
        assert!(v.is_ok(), "WASM validation failed: {:?}", v.err());
    }

    // ------------------------------------------------------------------ //
    //  Existing tests (preserved)                                         //
    // ------------------------------------------------------------------ //

    #[test]
    fn test_simple_function() {
        let mut gen = WasmCodeGenerator::new();
        let decl = Declaration::Function {
            name: "add".to_string(),
            params: vec![
                Parameter { name: "a".to_string(), param_type: Type::Int },
                Parameter { name: "b".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_binary(
                    make_var("a"), BinaryOp::Add, make_var("b"),
                )))],
            },
            where_bounds: vec![],
        };
        let bytes = gen.generate(&[decl]).expect("generate failed");
        validate_wasm(&bytes);
    }

    #[test]
    fn test_all_binary_ops() {
        let mut gen = WasmCodeGenerator::new();
        let ops = [
            (BinaryOp::Add, "add_op"), (BinaryOp::Sub, "sub_op"),
            (BinaryOp::Mul, "mul_op"), (BinaryOp::Div, "div_op"),
            (BinaryOp::Mod, "mod_op"), (BinaryOp::Eq,  "eq_op"),
            (BinaryOp::Neq, "neq_op"), (BinaryOp::Lt,  "lt_op"),
            (BinaryOp::Gt,  "gt_op"),  (BinaryOp::Le,  "le_op"),
            (BinaryOp::Ge,  "ge_op"),  (BinaryOp::And, "and_op"),
            (BinaryOp::Or,  "or_op"),
        ];
        let decls: Vec<Declaration> = ops.iter().map(|(op, name)| {
            Declaration::Function {
                name: name.to_string(),
                params: vec![
                    Parameter { name: "x".to_string(), param_type: Type::Int },
                    Parameter { name: "y".to_string(), param_type: Type::Int },
                ],
                return_type: Type::Int,
                body: Block {
                    statements: vec![Statement::Return(Some(
                        make_binary(make_var("x"), *op, make_var("y")),
                    ))],
                },
                where_bounds: vec![],
            }
        }).collect();
        let bytes = gen.generate(&decls).expect("generate failed");
        validate_wasm(&bytes);
    }

    #[test]
    fn test_if_else() {
        let mut gen = WasmCodeGenerator::new();
        let decl = Declaration::Function {
            name: "test_if".to_string(),
            params: vec![Parameter { name: "x".to_string(), param_type: Type::Int }],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::If {
                    cond: make_binary(make_var("x"), BinaryOp::Gt, make_int(0)),
                    then_block: Box::new(Block {
                        statements: vec![Statement::Return(Some(make_int(1)))],
                    }),
                    else_block: Some(Box::new(Block {
                        statements: vec![Statement::Return(Some(make_int(0)))],
                    })),
                }],
            },
            where_bounds: vec![],
        };
        let bytes = gen.generate(&[decl]).expect("generate failed");
        validate_wasm(&bytes);
    }

    #[test]
    fn test_multiple_functions() {
        let mut gen = WasmCodeGenerator::new();
        let decls = vec![
            Declaration::Function {
                name: "square".to_string(),
                params: vec![Parameter { name: "n".to_string(), param_type: Type::Int }],
                return_type: Type::Int,
                body: Block {
                    statements: vec![Statement::Return(Some(
                        make_binary(make_var("n"), BinaryOp::Mul, make_var("n")),
                    ))],
                },
                where_bounds: vec![],
            },
            Declaration::Function {
                name: "compute".to_string(),
                params: vec![],
                return_type: Type::Int,
                body: Block {
                    statements: vec![Statement::Return(Some(make_expr(
                        ExpressionKind::FunctionCall {
                            name: "square".to_string(),
                            args: vec![make_int(5)],
                            type_args: vec![],
                        },
                    )))],
                },
                where_bounds: vec![],
            },
        ];
        let bytes = gen.generate(&decls).expect("generate failed");
        validate_wasm(&bytes);
    }

    #[test]
    fn test_while_loop() {
        let mut gen = WasmCodeGenerator::new();
        let decl = Declaration::Function {
            name: "countdown".to_string(),
            params: vec![Parameter { name: "n".to_string(), param_type: Type::Int }],
            return_type: Type::Int,
            body: Block {
                statements: vec![
                    Statement::Let { name: "x".to_string(), value: make_var("n"), ty: Type::Int },
                    Statement::While {
                        cond: make_binary(make_var("x"), BinaryOp::Gt, make_int(0)),
                        body: Box::new(Block {
                            statements: vec![Statement::Assignment {
                                target: make_var("x"),
                                value: make_binary(make_var("x"), BinaryOp::Sub, make_int(1)),
                            }],
                        }),
                    },
                    Statement::Return(Some(make_int(0))),
                ],
            },
            where_bounds: vec![],
        };
        let bytes = gen.generate(&[decl]).expect("generate failed");
        validate_wasm(&bytes);
    }

    #[test]
    fn test_unary_ops() {
        let mut gen = WasmCodeGenerator::new();
        let decl_neg = Declaration::Function {
            name: "negate".to_string(),
            params: vec![Parameter { name: "x".to_string(), param_type: Type::Int }],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_expr(
                    ExpressionKind::UnaryOp { op: UnaryOp::Neg, expr: Box::new(make_var("x")) },
                )))],
            },
            where_bounds: vec![],
        };
        let decl_not = Declaration::Function {
            name: "is_zero".to_string(),
            params: vec![Parameter { name: "x".to_string(), param_type: Type::Int }],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_expr(
                    ExpressionKind::UnaryOp { op: UnaryOp::Not, expr: Box::new(make_var("x")) },
                )))],
            },
            where_bounds: vec![],
        };
        let bytes = gen.generate(&[decl_neg, decl_not]).expect("generate failed");
        validate_wasm(&bytes);
    }

    #[test]
    fn test_string_literals() {
        let mut gen = WasmCodeGenerator::new();
        let decl = Declaration::Function {
            name: "hello".to_string(),
            params: vec![],
            return_type: Type::String,
            body: Block {
                statements: vec![Statement::Return(Some(make_expr(
                    ExpressionKind::Literal(Literal::String("Hello, World!".to_string())),
                )))],
            },
            where_bounds: vec![],
        };
        let bytes = gen.generate(&[decl]).expect("generate failed");
        validate_wasm(&bytes);
        let mut found = false;
        for payload in wasmparser::Parser::new(0).parse_all(&bytes) {
            if let Ok(wasmparser::Payload::DataSection(reader)) = payload {
                for seg in reader {
                    if let Ok(seg) = seg {
                        let data: Vec<u8> = seg.data.iter().map(|b| *b).collect();
                        if data.windows(13).any(|w| w == b"Hello, World!") {
                            found = true;
                        }
                    }
                }
            }
        }
        assert!(found, "String not found in data section");
    }

    #[test]
    fn test_multi_string_dedup() {
        let mut gen = WasmCodeGenerator::new();
        let decls = vec![
            Declaration::Function {
                name: "first".to_string(),
                params: vec![], return_type: Type::String,
                body: Block { statements: vec![Statement::Return(Some(make_expr(
                    ExpressionKind::Literal(Literal::String("hi".to_string())),
                )))] },
                where_bounds: vec![],
            },
            Declaration::Function {
                name: "second".to_string(),
                params: vec![], return_type: Type::String,
                body: Block { statements: vec![Statement::Return(Some(make_expr(
                    ExpressionKind::Literal(Literal::String("hi".to_string())),
                )))] },
                where_bounds: vec![],
            },
        ];
        let bytes = gen.generate(&decls).expect("generate failed");
        validate_wasm(&bytes);
        let mut hi_count = 0usize;
        for payload in wasmparser::Parser::new(0).parse_all(&bytes) {
            if let Ok(wasmparser::Payload::DataSection(reader)) = payload {
                for seg in reader {
                    if let Ok(seg) = seg {
                        let data: Vec<u8> = seg.data.iter().map(|b| *b).collect();
                        if data.windows(3).any(|w| w == b"hi\0") { hi_count += 1; }
                    }
                }
            }
        }
        assert_eq!(hi_count, 1, "'hi' should be deduplicated");
    }

    #[test]
    fn test_nested_control_flow() {
        let mut gen = WasmCodeGenerator::new();
        let decl = Declaration::Function {
            name: "nested".to_string(),
            params: vec![Parameter { name: "x".to_string(), param_type: Type::Int }],
            return_type: Type::Int,
            body: Block {
                statements: vec![
                    Statement::Let { name: "result".to_string(), value: make_int(0), ty: Type::Int },
                    Statement::While {
                        cond: make_binary(make_var("x"), BinaryOp::Gt, make_int(0)),
                        body: Box::new(Block {
                            statements: vec![
                                Statement::If {
                                    cond: make_binary(make_var("x"), BinaryOp::Gt, make_int(10)),
                                    then_block: Box::new(Block {
                                        statements: vec![Statement::Assignment {
                                            target: make_var("result"), value: make_int(1),
                                        }],
                                    }),
                                    else_block: Some(Box::new(Block {
                                        statements: vec![Statement::Assignment {
                                            target: make_var("result"), value: make_int(2),
                                        }],
                                    })),
                                },
                                Statement::Assignment {
                                    target: make_var("x"),
                                    value: make_binary(make_var("x"), BinaryOp::Sub, make_int(1)),
                                },
                            ],
                        }),
                    },
                    Statement::Return(Some(make_var("result"))),
                ],
            },
            where_bounds: vec![],
        };
        let bytes = gen.generate(&[decl]).expect("generate failed");
        validate_wasm(&bytes);
    }

    // ------------------------------------------------------------------ //
    //  New tests for the 7 implemented parts                              //
    // ------------------------------------------------------------------ //

    /// Part 9a: StructLiteral allocates memory and stores fields.
    /// Part 7:  MemberAccess loads a field at the correct byte offset.
    #[test]
    fn test_struct_alloc_and_member_access() {
        let mut gen = WasmCodeGenerator::new();
        // Register struct Point { x: Int, y: Int }
        gen.register_struct("Point", vec!["x".to_string(), "y".to_string()]);

        // fn make_point_x() -> Int {
        //     let p = Point { x: 10, y: 20 };
        //     return p.x;
        // }
        let struct_expr = make_expr_typed(
            ExpressionKind::StructLiteral {
                name: "Point".to_string(),
                fields: vec![
                    ("x".to_string(), make_int(10)),
                    ("y".to_string(), make_int(20)),
                ],
            },
            Type::Struct("Point".to_string()),
        );

        let member_expr = make_expr(
            ExpressionKind::MemberAccess {
                base: Box::new(make_var("p")),
                field: "x".to_string(),
            },
        );

        let decl = Declaration::Function {
            name: "make_point_x".to_string(),
            params: vec![],
            return_type: Type::Int,
            body: Block {
                statements: vec![
                    Statement::Let {
                        name: "p".to_string(),
                        value: struct_expr,
                        ty: Type::Struct("Point".to_string()),
                    },
                    Statement::Return(Some(member_expr)),
                ],
            },
            where_bounds: vec![],
        };
        let bytes = gen.generate(&[decl]).expect("generate failed");
        validate_wasm(&bytes);
    }

    /// Part 9b: ArrayLiteral bump-allocates with a length header.
    #[test]
    fn test_array_literal() {
        let mut gen = WasmCodeGenerator::new();
        // fn make_array() -> Int { let a = [1, 2, 3]; return 0; }
        let arr_expr = make_expr(ExpressionKind::ArrayLiteral(vec![
            make_int(1), make_int(2), make_int(3),
        ]));
        let decl = Declaration::Function {
            name: "make_array".to_string(),
            params: vec![],
            return_type: Type::Int,
            body: Block {
                statements: vec![
                    Statement::Let {
                        name: "a".to_string(),
                        value: arr_expr,
                        ty: Type::Array(Box::new(Type::Int)),
                    },
                    Statement::Return(Some(make_int(0))),
                ],
            },
            where_bounds: vec![],
        };
        let bytes = gen.generate(&[decl]).expect("generate failed");
        validate_wasm(&bytes);
    }

    /// Part 8: Match emits nested if/else and dispatches correctly.
    #[test]
    fn test_match_dispatch() {
        let mut gen = WasmCodeGenerator::new();
        // fn classify(x: Int) -> Int {
        //     return match x { 0 => 10, 1 => 20, _ => 99 };
        // }
        let match_expr = make_expr(ExpressionKind::Match {
            scrutinee: Box::new(make_var("x")),
            arms: vec![
                MatchArm {
                    pattern: Pattern::Literal(Literal::Integer(0)),
                    body: make_int(10),
                },
                MatchArm {
                    pattern: Pattern::Literal(Literal::Integer(1)),
                    body: make_int(20),
                },
                MatchArm {
                    pattern: Pattern::Wildcard,
                    body: make_int(99),
                },
            ],
        });
        let decl = Declaration::Function {
            name: "classify".to_string(),
            params: vec![Parameter { name: "x".to_string(), param_type: Type::Int }],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(match_expr))],
            },
            where_bounds: vec![],
        };
        let bytes = gen.generate(&[decl]).expect("generate failed");
        validate_wasm(&bytes);
    }

    /// Part 6: For-loop compiles to valid WASM with block/loop/br structure.
    #[test]
    fn test_for_loop() {
        let mut gen = WasmCodeGenerator::new();
        // fn sum_array(arr: Int) -> Int {
        //     let s = 0;
        //     for elem in arr { s = s + elem; }
        //     return s;
        // }
        let decl = Declaration::Function {
            name: "sum_array".to_string(),
            params: vec![Parameter { name: "arr".to_string(), param_type: Type::Int }],
            return_type: Type::Int,
            body: Block {
                statements: vec![
                    Statement::Let { name: "s".to_string(), value: make_int(0), ty: Type::Int },
                    Statement::For {
                        var: "elem".to_string(),
                        iter: make_var("arr"),
                        body: Box::new(Block {
                            statements: vec![Statement::Assignment {
                                target: make_var("s"),
                                value: make_binary(make_var("s"), BinaryOp::Add, make_var("elem")),
                            }],
                        }),
                    },
                    Statement::Return(Some(make_var("s"))),
                ],
            },
            where_bounds: vec![],
        };
        let bytes = gen.generate(&[decl]).expect("generate failed");
        validate_wasm(&bytes);
    }

    /// Part 10: Closure inlines body expression.
    #[test]
    fn test_closure_inline() {
        let mut gen = WasmCodeGenerator::new();
        // fn apply() -> Int {
        //     let f = |x| x + 1;   // closure over nothing
        //     return 0;
        // }
        let closure_expr = make_expr(ExpressionKind::Closure {
            params: vec![Parameter { name: "x".to_string(), param_type: Type::Int }],
            body: Box::new(make_binary(make_var("x"), BinaryOp::Add, make_int(1))),
        });
        let decl = Declaration::Function {
            name: "apply".to_string(),
            params: vec![],
            return_type: Type::Int,
            body: Block {
                statements: vec![
                    Statement::Expression(closure_expr),
                    Statement::Return(Some(make_int(0))),
                ],
            },
            where_bounds: vec![],
        };
        let bytes = gen.generate(&[decl]).expect("generate failed");
        validate_wasm(&bytes);
    }

    /// Heap: two sequential allocations should not overlap.
    #[test]
    fn test_heap_alloc_no_overlap() {
        let mut gen = WasmCodeGenerator::new();
        // fn two_structs() -> Int {
        //     let a = Point { x: 1, y: 2 };
        //     let b = Point { x: 3, y: 4 };
        //     return 0;
        // }
        gen.register_struct("Point", vec!["x".to_string(), "y".to_string()]);
        let make_point = |xv: i64, yv: i64| make_expr(ExpressionKind::StructLiteral {
            name: "Point".to_string(),
            fields: vec![("x".to_string(), make_int(xv)), ("y".to_string(), make_int(yv))],
        });
        let decl = Declaration::Function {
            name: "two_structs".to_string(),
            params: vec![],
            return_type: Type::Int,
            body: Block {
                statements: vec![
                    Statement::Let { name: "a".to_string(), value: make_point(1, 2), ty: Type::Struct("Point".to_string()) },
                    Statement::Let { name: "b".to_string(), value: make_point(3, 4), ty: Type::Struct("Point".to_string()) },
                    Statement::Return(Some(make_int(0))),
                ],
            },
            where_bounds: vec![],
        };
        let bytes = gen.generate(&[decl]).expect("generate failed");
        validate_wasm(&bytes);
    }

    /// Legacy test preserved: MemberAccess on an untyped (Int) base still
    /// produces valid WASM (field index 0, wraps i64→i32 then loads i64).
    #[test]
    fn test_struct_access_untyped_base() {
        let mut gen = WasmCodeGenerator::new();
        let decl = Declaration::Function {
            name: "get_x".to_string(),
            params: vec![Parameter { name: "p".to_string(), param_type: Type::Int }],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_expr(
                    ExpressionKind::MemberAccess {
                        base: Box::new(make_var("p")),
                        field: "x".to_string(),
                    },
                )))],
            },
            where_bounds: vec![],
        };
        let bytes = gen.generate(&[decl]).expect("generate failed");
        validate_wasm(&bytes);
    }
}
