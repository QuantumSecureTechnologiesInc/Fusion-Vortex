// src/wasm/codegen.rs - WebAssembly Code Generator

use crate::ast::{
    BinaryOp, Block, Declaration, Expression, ExpressionKind, Literal, Parameter, Statement,
    Type, UnaryOp,
};
use crate::wasm::types::*;
use std::collections::HashMap;
use wasm_encoder::{
    BlockType, CodeSection, ConstExpr, DataSection, ExportKind, ExportSection,
    Function as WasmFunction, FunctionSection, Instruction, MemorySection, MemoryType, Module,
    TypeSection, ValType,
};

pub struct WasmCodeGenerator {
    function_index: u32,
    function_map: HashMap<String, u32>, // Function name -> index
    type_section: TypeSection,
    function_section: FunctionSection,
    export_section: ExportSection,
    code_section: CodeSection,
    memory_section: MemorySection,
    next_local_index: u32,
    local_map: HashMap<String, u32>, // Variable name -> local index
    string_offsets: HashMap<String, u32>, // String literal -> memory offset
    string_data: Vec<u8>,                  // Accumulated string bytes
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
            next_local_index: 0,
            local_map: HashMap::new(),
            string_offsets: HashMap::new(),
            string_data: Vec::new(),
        }
    }

    /// Generate WASM binary from AST
    pub fn generate(&mut self, declarations: &[Declaration]) -> Result<Vec<u8>, String> {
        // Add memory section (1 page = 64KB)
        self.memory_section.memory(MemoryType {
            minimum: 1,
            maximum: Some(10),
            memory64: false,
            shared: false,
            page_size_log2: None,
        });

        // Process all declarations
        for decl in declarations {
            self.generate_declaration(decl)?;
        }

        // Build final module
        let mut module = Module::new();
        module.section(&self.type_section);
        module.section(&self.function_section);
        module.section(&self.memory_section);
        module.section(&self.export_section);
        module.section(&self.code_section);

        // Emit data section with string literals
        if !self.string_data.is_empty() {
            let mut data_section = DataSection::new();
            let offset = ConstExpr::i32_const(0);
            data_section.active(0, &offset, self.string_data.iter().copied());
            module.section(&data_section);
        }

        Ok(module.finish())
    }

    fn generate_declaration(&mut self, decl: &Declaration) -> Result<(), String> {
        match decl {
            Declaration::Function {
                name,
                params,
                return_type,
                body,
                ..
            } => {
                self.generate_function(name, params, return_type, body)?;
            }
            Declaration::ModuleDecl { .. } | Declaration::UseDecl { .. } | Declaration::ImportDecl { .. } => {
                // Skip module system declarations in WASM generation
            }
            _ => {
                // Skip other declarations for now
            }
        }
        Ok(())
    }

    /// Count additional local variables needed beyond function parameters.
    fn count_locals(block: &Block) -> u32 {
        let mut count = 0u32;
        for stmt in &block.statements {
            match stmt {
                Statement::Let { .. } | Statement::VariableDeclaration { .. } => {
                    count += 1;
                }
                Statement::If { then_block, else_block, .. } => {
                    count += Self::count_locals(then_block);
                    if let Some(else_blk) = else_block {
                        count += Self::count_locals(else_blk);
                    }
                }
                Statement::While { body, .. } => {
                    count += Self::count_locals(body);
                }
                _ => {}
            }
        }
        count
    }

    fn generate_function(
        &mut self,
        name: &str,
        params: &[Parameter],
        return_type: &Type,
        body: &Block,
    ) -> Result<(), String> {
        // Build function type
        let param_types: Vec<ValType> = params
            .iter()
            .filter_map(|p| fusion_to_wasm_type(&p.param_type))
            .collect();

        let result_types: Vec<ValType> = fusion_to_wasm_type(return_type).into_iter().collect();

        // Add to type section
        let type_idx = self.type_section.len();
        self.type_section
            .ty()
            .function(param_types.clone(), result_types.clone());

        // Add to function section
        self.function_section.function(type_idx);

        // Map function name to index
        let func_idx = self.function_index;
        self.function_map.insert(name.to_string(), func_idx);
        self.function_index += 1;

        // Export the function
        self.export_section.export(name, ExportKind::Func, func_idx);

        // Generate function body
        // Count additional locals beyond params
        let extra_locals = Self::count_locals(body);
        let mut func_body = if extra_locals > 0 {
            WasmFunction::new(vec![(extra_locals, ValType::I64)])
        } else {
            WasmFunction::new(vec![])
        };

        // Reset local tracking for this function
        self.next_local_index = params.len() as u32;
        self.local_map.clear();

        // Map parameters to locals
        for (i, param) in params.iter().enumerate() {
            self.local_map.insert(param.name.clone(), i as u32);
        }

        // Generate statements
        for stmt in &body.statements {
            self.generate_statement(stmt, &mut func_body)?;
        }

        func_body.instruction(&Instruction::End);

        // Add function to code section
        self.code_section.function(&func_body);

        Ok(())
    }

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
            Statement::VariableDeclaration {
                name, initializer, ..
            } => {
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
            Statement::If {
                cond,
                then_block,
                else_block,
            } => {
                // Generate condition on stack (as i64), then convert to i32 for WASM if
                self.generate_expression(cond, func)?;
                func.instruction(&Instruction::I32WrapI64);
                // WASM if: takes i32 condition from stack
                func.instruction(&Instruction::If(BlockType::Empty));

                // Then block
                for s in &then_block.statements {
                    self.generate_statement(s, func)?;
                }

                if let Some(else_blk) = else_block {
                    func.instruction(&Instruction::Else);
                    for s in &else_blk.statements {
                        self.generate_statement(s, func)?;
                    }
                }

                func.instruction(&Instruction::End);
                // If both branches return, we need unreachable to satisfy the type checker
                func.instruction(&Instruction::Unreachable);
            }
            Statement::While { cond, body } => {
                // WASM while pattern: block (outer) + loop (inner)
                //   block       ;; break target
                //     loop      ;; continue target
                //       ;; condition -> i32
                //       i32.eqz
                //       br_if 1  ;; break out of block if condition false
                //       ;; body
                //       br 0     ;; continue loop
                //     end
                //   end
                func.instruction(&Instruction::Block(BlockType::Empty));
                func.instruction(&Instruction::Loop(BlockType::Empty));

                // Generate condition
                self.generate_expression(cond, func)?;
                func.instruction(&Instruction::I32WrapI64);
                func.instruction(&Instruction::I32Eqz);
                func.instruction(&Instruction::BrIf(1));

                // Body
                for s in &body.statements {
                    self.generate_statement(s, func)?;
                }

                // Continue
                func.instruction(&Instruction::Br(0));
                func.instruction(&Instruction::End); // loop
                func.instruction(&Instruction::End); // block
            }
            Statement::Expression(expr) => {
                self.generate_expression(expr, func)?;
                func.instruction(&Instruction::Drop);
            }
            Statement::Assignment { target, value } => {
                // Generate the value, then store to the target variable
                self.generate_expression(value, func)?;
                if let ExpressionKind::Variable(name) = &target.kind {
                    let local_idx = self
                        .local_map
                        .get(name)
                        .ok_or(format!("Unknown variable for assignment: {}", name))?;
                    func.instruction(&Instruction::LocalSet(*local_idx));
                } else {
                    return Err("Assignment target must be a variable".to_string());
                }
            }
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
        }
        Ok(())
    }

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
                    // Collect string into memory segment, return offset
                    let offset = if let Some(&existing) = self.string_offsets.get(s) {
                        existing
                    } else {
                        let off = self.string_data.len() as u32;
                        self.string_data.extend_from_slice(s.as_bytes());
                        self.string_data.push(0); // null terminator
                        self.string_offsets.insert(s.clone(), off);
                        off
                    };
                    func.instruction(&Instruction::I32Const(offset as i32));
                }
            },
            ExpressionKind::Variable(name) => {
                let local_idx = self
                    .local_map
                    .get(name)
                    .ok_or(format!("Unknown variable: {}", name))?;
                func.instruction(&Instruction::LocalGet(*local_idx));
            }
            ExpressionKind::BinaryOp { left, op, right } => {
                self.generate_expression(left, func)?;
                self.generate_expression(right, func)?;

                match op {
                    BinaryOp::Add => { func.instruction(&Instruction::I64Add); }
                    BinaryOp::Sub => { func.instruction(&Instruction::I64Sub); }
                    BinaryOp::Mul => { func.instruction(&Instruction::I64Mul); }
                    BinaryOp::Div => { func.instruction(&Instruction::I64DivS); }
                    BinaryOp::Mod => { func.instruction(&Instruction::I64RemS); }
                    BinaryOp::Eq => { func.instruction(&Instruction::I64Eq); func.instruction(&Instruction::I64ExtendI32S); }
                    BinaryOp::Neq => { func.instruction(&Instruction::I64Ne); func.instruction(&Instruction::I64ExtendI32S); }
                    BinaryOp::Lt => { func.instruction(&Instruction::I64LtS); func.instruction(&Instruction::I64ExtendI32S); }
                    BinaryOp::Gt => { func.instruction(&Instruction::I64GtS); func.instruction(&Instruction::I64ExtendI32S); }
                    BinaryOp::Le => { func.instruction(&Instruction::I64LeS); func.instruction(&Instruction::I64ExtendI32S); }
                    BinaryOp::Ge => { func.instruction(&Instruction::I64GeS); func.instruction(&Instruction::I64ExtendI32S); }
                    BinaryOp::And => { func.instruction(&Instruction::I64And); }
                    BinaryOp::Or => { func.instruction(&Instruction::I64Or); }
                };
            }
            ExpressionKind::FunctionCall { name, args, .. } => {
                // Generate arguments
                for arg in args {
                    self.generate_expression(arg, func)?;
                }

                // Call function
                let func_idx = self
                    .function_map
                    .get(name)
                    .ok_or(format!("Unknown function: {}", name))?;
                func.instruction(&Instruction::Call(*func_idx));
            }
            ExpressionKind::UnaryOp { op, expr } => {
                self.generate_expression(expr, func)?;
                match op {
                    UnaryOp::Neg => {
                        // i64 negation: expr * -1
                        func.instruction(&Instruction::I64Const(-1));
                        func.instruction(&Instruction::I64Mul);
                    }
                    UnaryOp::Not => {
                        // Logical not: i64.eqz → i32, then extend to i64
                        func.instruction(&Instruction::I64Eqz);
                        func.instruction(&Instruction::I64ExtendI32S);
                    }
                }
            }
            ExpressionKind::MemberAccess { base, field: _field } => {
                // For now, treat member access as returning the base pointer
                // TODO: proper GEP-like offset calculation for struct fields
                self.generate_expression(base, func)?;
            }
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
            ExpressionKind::Closure { params: _, body } => {
                // Simplified: generate closure body
                self.generate_expression(body, func)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_expr(kind: ExpressionKind) -> Expression {
        Expression { kind, ty: None }
    }

    fn make_var(name: &str) -> Expression {
        make_expr(ExpressionKind::Variable(name.to_string()))
    }

    fn make_int(n: i64) -> Expression {
        make_expr(ExpressionKind::Literal(Literal::Integer(n)))
    }

    #[allow(dead_code)]
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

    #[test]
    fn test_simple_function() {
        let mut generator = WasmCodeGenerator::new();

        // fn add(a: int, b: int) -> int { return a + b; }
        let decl = Declaration::Function {
            name: "add".to_string(),
            params: vec![
                Parameter { name: "a".to_string(), param_type: Type::Int },
                Parameter { name: "b".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_binary(
                    make_var("a"),
                    BinaryOp::Add,
                    make_var("b"),
                )))],
            },
            where_bounds: vec![],
        };

        let result = generator.generate(&[decl]);
        let wasm_bytes = result.expect("WASM generation failed");
        assert!(!wasm_bytes.is_empty());
        let validation = wasmparser::validate(&wasm_bytes);
        assert!(validation.is_ok(), "WASM validation failed: {:?}", validation.err());
    }

    #[test]
    fn test_all_binary_ops() {
        let mut generator = WasmCodeGenerator::new();

        let ops = [
            (BinaryOp::Add, "add_op"),
            (BinaryOp::Sub, "sub_op"),
            (BinaryOp::Mul, "mul_op"),
            (BinaryOp::Div, "div_op"),
            (BinaryOp::Mod, "mod_op"),
            (BinaryOp::Eq, "eq_op"),
            (BinaryOp::Neq, "neq_op"),
            (BinaryOp::Lt, "lt_op"),
            (BinaryOp::Gt, "gt_op"),
            (BinaryOp::Le, "le_op"),
            (BinaryOp::Ge, "ge_op"),
            (BinaryOp::And, "and_op"),
            (BinaryOp::Or, "or_op"),
        ];

        let mut decls = Vec::new();
        for (op, name) in &ops {
            decls.push(Declaration::Function {
                name: name.to_string(),
                params: vec![
                    Parameter { name: "x".to_string(), param_type: Type::Int },
                    Parameter { name: "y".to_string(), param_type: Type::Int },
                ],
                return_type: Type::Int,
                body: Block {
                    statements: vec![Statement::Return(Some(make_binary(
                        make_var("x"),
                        *op,
                        make_var("y"),
                    )))],
                },
                where_bounds: vec![],
            });
        }

        let result = generator.generate(&decls);
        let wasm_bytes = result.expect("WASM generation failed");
        assert!(!wasm_bytes.is_empty());
        let validation = wasmparser::validate(&wasm_bytes);
        assert!(validation.is_ok(), "WASM validation failed: {:?}", validation.err());
    }

    #[test]
    fn test_if_else() {
        let mut generator = WasmCodeGenerator::new();

        // fn test_if(x: int) -> int {
        //     if x > 0 { return 1; } else { return 0; }
        // }
        let decl = Declaration::Function {
            name: "test_if".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
            ],
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

        let result = generator.generate(&[decl]);
        let wasm_bytes = result.expect("WASM generation failed");
        assert!(!wasm_bytes.is_empty());
        let validation = wasmparser::validate(&wasm_bytes);
        assert!(validation.is_ok(), "WASM validation failed: {:?}", validation.err());
    }

    #[test]
    fn test_multiple_functions() {
        let mut generator = WasmCodeGenerator::new();

        let decls = vec![
            Declaration::Function {
                name: "square".to_string(),
                params: vec![
                    Parameter { name: "n".to_string(), param_type: Type::Int },
                ],
                return_type: Type::Int,
                body: Block {
                    statements: vec![Statement::Return(Some(make_binary(
                        make_var("n"),
                        BinaryOp::Mul,
                        make_var("n"),
                    )))],
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

        let result = generator.generate(&decls);
        let wasm_bytes = result.expect("WASM generation failed");
        assert!(!wasm_bytes.is_empty());
        let validation = wasmparser::validate(&wasm_bytes);
        assert!(validation.is_ok(), "WASM validation failed: {:?}", validation.err());
    }

    #[test]
    fn test_while_loop() {
        let mut generator = WasmCodeGenerator::new();

        // fn countdown(n: int) -> int {
        //     let mut x: int = n;
        //     while x > 0 {
        //         x = x - 1;
        //     }
        //     return 0;
        // }
        let decl = Declaration::Function {
            name: "countdown".to_string(),
            params: vec![
                Parameter { name: "n".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![
                    Statement::Let {
                        name: "x".to_string(),
                        value: make_var("n"),
                        ty: Type::Int,
                    },
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

        let result = generator.generate(&[decl]);
        let wasm_bytes = result.expect("WASM generation failed");
        assert!(!wasm_bytes.is_empty());
        let validation = wasmparser::validate(&wasm_bytes);
        assert!(validation.is_ok(), "WASM validation failed: {:?}", validation.err());
    }

    #[test]
    fn test_unary_ops() {
        let mut generator = WasmCodeGenerator::new();

        // fn negate(x: int) -> int { return -x; }
        let decl_neg = Declaration::Function {
            name: "negate".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_expr(
                    ExpressionKind::UnaryOp {
                        op: UnaryOp::Neg,
                        expr: Box::new(make_var("x")),
                    },
                )))],
            },
            where_bounds: vec![],
        };

        // fn is_zero(x: int) -> int { return !x; }
        let decl_not = Declaration::Function {
            name: "is_zero".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_expr(
                    ExpressionKind::UnaryOp {
                        op: UnaryOp::Not,
                        expr: Box::new(make_var("x")),
                    },
                )))],
            },
            where_bounds: vec![],
        };

        let result = generator.generate(&[decl_neg, decl_not]);
        let wasm_bytes = result.expect("WASM generation failed");
        assert!(!wasm_bytes.is_empty());
        let validation = wasmparser::validate(&wasm_bytes);
        assert!(validation.is_ok(), "WASM validation failed: {:?}", validation.err());
    }

    #[test]
    fn test_struct_access() {
        let mut generator = WasmCodeGenerator::new();

        // fn get_x(p: int) -> int { return p; }
        // (MemberAccess is a stub that returns the base pointer)
        let decl = Declaration::Function {
            name: "get_x".to_string(),
            params: vec![
                Parameter { name: "p".to_string(), param_type: Type::Int },
            ],
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

        let result = generator.generate(&[decl]);
        let wasm_bytes = result.expect("WASM generation failed");
        assert!(!wasm_bytes.is_empty());
        let validation = wasmparser::validate(&wasm_bytes);
        assert!(validation.is_ok(), "WASM validation failed: {:?}", validation.err());
    }

    #[test]
    fn test_nested_control_flow() {
        let mut generator = WasmCodeGenerator::new();

        // fn nested(x: int) -> int {
        //     let mut result: int = 0;
        //     while x > 0 {
        //         if x > 10 { result = 1; } else { result = 2; }
        //         x = x - 1;
        //     }
        //     return result;
        // }
        let decl = Declaration::Function {
            name: "nested".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![
                    Statement::Let {
                        name: "result".to_string(),
                        value: make_int(0),
                        ty: Type::Int,
                    },
                    Statement::While {
                        cond: make_binary(make_var("x"), BinaryOp::Gt, make_int(0)),
                        body: Box::new(Block {
                            statements: vec![
                                Statement::If {
                                    cond: make_binary(make_var("x"), BinaryOp::Gt, make_int(10)),
                                    then_block: Box::new(Block {
                                        statements: vec![Statement::Assignment {
                                            target: make_var("result"),
                                            value: make_int(1),
                                        }],
                                    }),
                                    else_block: Some(Box::new(Block {
                                        statements: vec![Statement::Assignment {
                                            target: make_var("result"),
                                            value: make_int(2),
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

        let result = generator.generate(&[decl]);
        let wasm_bytes = result.expect("WASM generation failed");
        assert!(!wasm_bytes.is_empty());
        let validation = wasmparser::validate(&wasm_bytes);
        assert!(validation.is_ok(), "WASM validation failed: {:?}", validation.err());
    }

    #[test]
    fn test_string_literals() {
        let mut generator = WasmCodeGenerator::new();

        // fn hello() -> string { return "Hello, World!"; }
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

        let result = generator.generate(&[decl]);
        let wasm_bytes = result.expect("WASM generation failed");
        assert!(!wasm_bytes.is_empty());

        // Validate the WASM module
        let validation = wasmparser::validate(&wasm_bytes);
        assert!(validation.is_ok(), "WASM validation failed: {:?}", validation.err());

        // Verify the data section contains the string
        let mut found_string = false;
        for payload in wasmparser::Parser::new(0).parse_all(&wasm_bytes) {
            if let Ok(wasmparser::Payload::DataSection(reader)) = payload {
                for seg in reader {
                    if let Ok(seg) = seg {
                        let data = seg.data.iter().map(|b| *b as u8).collect::<Vec<u8>>();
                        if data.windows(13).any(|w| w == b"Hello, World!") {
                            found_string = true;
                        }
                    }
                }
            }
        }
        assert!(found_string, "String 'Hello, World!' not found in data section");
    }

    #[test]
    fn test_multi_string_dedup() {
        let mut generator = WasmCodeGenerator::new();

        // Two functions that both reference the same string literal "hi"
        // fn first() -> string { return "hi"; }
        // fn second() -> string { return "hi"; }
        let decls = vec![
            Declaration::Function {
                name: "first".to_string(),
                params: vec![],
                return_type: Type::String,
                body: Block {
                    statements: vec![Statement::Return(Some(make_expr(
                        ExpressionKind::Literal(Literal::String("hi".to_string())),
                    )))],
                },
                where_bounds: vec![],
            },
            Declaration::Function {
                name: "second".to_string(),
                params: vec![],
                return_type: Type::String,
                body: Block {
                    statements: vec![Statement::Return(Some(make_expr(
                        ExpressionKind::Literal(Literal::String("hi".to_string())),
                    )))],
                },
                where_bounds: vec![],
            },
        ];

        let result = generator.generate(&decls);
        let wasm_bytes = result.expect("WASM generation failed");
        let validation = wasmparser::validate(&wasm_bytes);
        assert!(validation.is_ok(), "WASM validation failed: {:?}", validation.err());

        // Count occurrences of "hi" in the data section
        let mut hi_count = 0usize;
        for payload in wasmparser::Parser::new(0).parse_all(&wasm_bytes) {
            if let Ok(wasmparser::Payload::DataSection(reader)) = payload {
                for seg in reader {
                    if let Ok(seg) = seg {
                        let data = seg.data.iter().map(|b| *b as u8).collect::<Vec<u8>>();
                        // Count "hi" as a null-terminated string
                        if data.windows(3).any(|w| w == b"hi\0") {
                            hi_count += 1;
                        }
                    }
                }
            }
        }
        // "hi" should only appear once (deduplication)
        assert_eq!(hi_count, 1, "String 'hi' should be deduplicated, found {} occurrences", hi_count);
    }
}