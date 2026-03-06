#![allow(missing_docs)]
#[allow(missing_docs, dead_code)]
type FI64 = FI64;
#[allow(missing_docs, dead_code)]
type FString = FString;
#[allow(missing_docs, dead_code)]
type FVec<T> = FVec<T>;
#[allow(missing_docs, dead_code)]
type FMap<K, V> = FMap<K, V>;
use crate::compiler::ast::*;
use crate::compiler::chunk::OpCode;
use crate::compiler::function::Function;
use crate::compiler::value::Value;
use thiserror::Error;
#[derive(Error, Debug)]
enum CompilerError {
    #[error("Undefined variable: {0}")]
    UndefinedVariable(FString),
    #[error("Undefined function: {0}")]
    UndefinedFunction(FString),
    #[error("No main function found")]
    NoMainFunction,
    #[error("Main is not a function")]
    MainNotFunction,
}
type CompilerResult<T> = Result<T, CompilerError>;
struct Compiler {
    function: Option<Function>,
    #[allow(dead_code)]
    functions: FVec<Function>,
    locals: FMap<FString, u16>,
    local_count: u16,
    struct_layouts: FMap<FString, FMap<FString, u8>>,
}
impl Compiler {
    pub fn new() -> Self {
        Compiler {
            function: None,
            functions: Vec::new(),
            locals: HashMap::new(),
            local_count: 0,
            struct_layouts: HashMap::new(),
        }
    }
    pub fn compile(mut self, program: Program) -> CompilerResult<Function> {
        let mut globals: FMap<FString, Value> = HashMap::new();
        globals
            .insert(
                "print".to_string(),
                Value::Native(crate::compiler::value::NativeFunction {
                    name: "print".to_string(),
                    func: |args| {
                        for arg in args {
                            println!("{}", arg);
                        }
                        Value::Void
                    },
                }),
            );
        globals
            .insert(
                "clock".to_string(),
                Value::Native(crate::compiler::value::NativeFunction {
                    name: "clock".to_string(),
                    func: |_| {
                        let start = SystemTime::now();
                        let since_the_epoch = start
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards");
                        Value::Int(since_the_epoch.as_secs() as FI64)
                    },
                }),
            );
        for decl in &program.declarations {
            if let Declaration::Struct(s) = decl {
                let mut layout = HashMap::new();
                for (i, (name, _)) in s.fields.iter().enumerate() {
                    layout.insert(name.clone(), i as u8);
                }
                self.struct_layouts.insert(s.name.clone(), layout);
            }
        }
        for decl in &program.declarations {
            if let Declaration::Function(f) = decl {
                let compiled = self.compile_fn(f.clone(), &globals);
                globals.insert(f.name.clone(), Value::Function(Box::new(compiled)));
            }
        }
        for decl in program.declarations {
            if let Declaration::Function(f) = decl {
                let compiled = self.compile_fn(f.clone(), &globals);
                globals.insert(f.name.clone(), Value::Function(Box::new(compiled)));
            }
        }
        if let Some(main_val) = globals.get("main") {
            if let Value::Function(f) = main_val {
                Ok(*f.clone())
            } else {
                Err(CompilerError::MainNotFunction)
            }
        } else {
            Err(CompilerError::NoMainFunction)
        }
    }
    fn compile_fn(
        &mut self,
        ast_func: FunctionDecl,
        globals: &FMap<FString, Value>,
    ) -> Function {
        let func = Function::new(ast_func.name.clone(), ast_func.params.len());
        self.locals.clear();
        self.local_count = 0;
        for (param_name, _) in ast_func.params {
            self.locals.insert(param_name, self.local_count);
            self.local_count += 1;
        }
        self.function = Some(func);
        for stmt in ast_func.body {
            self.compile_statement(stmt, globals);
        }
        self.function.as_mut().unwrap().chunk.write(OpCode::Return);
        self.function.take().unwrap()
    }
    fn compile_statement(&mut self, stmt: Statement, globals: &FMap<FString, Value>) {
        match stmt {
            Statement::Let(name, _, expr) => {
                self.compile_expression(expr, globals);
                let slot = self.local_count;
                self.locals.insert(name, slot);
                self.local_count += 1;
            }
            Statement::Return(Some(expr)) => {
                self.compile_expression(expr, globals);
                self.function.as_mut().unwrap().chunk.write(OpCode::Return);
            }
            Statement::Return(None) => {
                self.function.as_mut().unwrap().chunk.write(OpCode::Return);
            }
            Statement::Expression(expr) => {
                self.compile_expression(expr, globals);
                self.function.as_mut().unwrap().chunk.write(OpCode::Pop);
            }
            Statement::If(cond, then_block, else_block_opt) => {
                self.compile_expression(cond, globals);
                let jump_if_false_idx = self.function.as_ref().unwrap().chunk.code.len();
                self.function.as_mut().unwrap().chunk.write(OpCode::JumpIfFalse(0xFFFF));
                self.function.as_mut().unwrap().chunk.write(OpCode::Pop);
                for s in then_block {
                    self.compile_statement(s, globals);
                }
                let jump_to_end_idx = self.function.as_ref().unwrap().chunk.code.len();
                self.function.as_mut().unwrap().chunk.write(OpCode::Jump(0xFFFF));
                let else_idx = self.function.as_ref().unwrap().chunk.code.len();
                let _offset = (else_idx - jump_if_false_idx - 1) as u16;
                self.function.as_mut().unwrap().chunk.code[jump_if_false_idx] = OpCode::JumpIfFalse(
                    _offset,
                );
                self.function.as_mut().unwrap().chunk.write(OpCode::Pop);
                if let Some(else_block) = else_block_opt {
                    for s in else_block {
                        self.compile_statement(s, globals);
                    }
                }
                let end_idx = self.function.as_ref().unwrap().chunk.code.len();
                let offset = (end_idx - jump_to_end_idx - 1) as u16;
                self.function.as_mut().unwrap().chunk.code[jump_to_end_idx] = OpCode::Jump(
                    offset,
                );
            }
            Statement::While(cond, body) => {
                let loop_start = self.function.as_ref().unwrap().chunk.code.len();
                self.compile_expression(cond, globals);
                let exit_jump = self.function.as_ref().unwrap().chunk.code.len();
                self.function.as_mut().unwrap().chunk.write(OpCode::JumpIfFalse(0xFFFF));
                self.function.as_mut().unwrap().chunk.write(OpCode::Pop);
                for s in body {
                    self.compile_statement(s, globals);
                }
                let loop_end = self.function.as_ref().unwrap().chunk.code.len();
                let loop_offset = (loop_end + 1 - loop_start) as u16;
                self.function.as_mut().unwrap().chunk.write(OpCode::Loop(loop_offset));
                let end = self.function.as_ref().unwrap().chunk.code.len();
                let exit_offset = (end - exit_jump - 1) as u16;
                self.function.as_mut().unwrap().chunk.code[exit_jump] = OpCode::JumpIfFalse(
                    exit_offset,
                );
                self.function.as_mut().unwrap().chunk.write(OpCode::Pop);
            }
            Statement::Block(stmts) => {
                for s in stmts {
                    self.compile_statement(s, globals);
                }
            }
        }
    }
    fn compile_expression(&mut self, expr: Expression, globals: &FMap<FString, Value>) {
        match expr {
            Expression::StructInit(_name, fields) => {
                let field_count = fields.len();
                for (field_name, expr) in fields {
                    let name_idx = self
                        .function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .add_constant(Value::FString(field_name.clone()));
                    self.function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .write(OpCode::Constant(name_idx));
                    self.compile_expression(expr.clone(), globals);
                }
                self.function
                    .as_mut()
                    .unwrap()
                    .chunk
                    .write(OpCode::MakeStruct(field_count as u8));
            }
            Expression::Get(obj, field) => {
                self.compile_expression(obj.as_ref().clone(), globals);
                let name_idx = self
                    .function
                    .as_mut()
                    .unwrap()
                    .chunk
                    .add_constant(Value::FString(field.clone()));
                self.function
                    .as_mut()
                    .unwrap()
                    .chunk
                    .write(OpCode::GetProp(name_idx as u8));
            }
            Expression::Set(obj, field, val) => {
                self.compile_expression(obj.as_ref().clone(), globals);
                self.compile_expression(val.as_ref().clone(), globals);
                let name_idx = self
                    .function
                    .as_mut()
                    .unwrap()
                    .chunk
                    .add_constant(Value::FString(field.clone()));
                self.function
                    .as_mut()
                    .unwrap()
                    .chunk
                    .write(OpCode::SetProp(name_idx as u8));
            }
            Expression::Literal(lit) => {
                match lit {
                    Literal::Integer(i) => {
                        let idx = self
                            .function
                            .as_mut()
                            .unwrap()
                            .chunk
                            .add_constant(Value::Int(i));
                        self.function
                            .as_mut()
                            .unwrap()
                            .chunk
                            .write(OpCode::Constant(idx));
                    }
                    Literal::Bool(b) => {
                        let idx = self
                            .function
                            .as_mut()
                            .unwrap()
                            .chunk
                            .add_constant(Value::Bool(b));
                        self.function
                            .as_mut()
                            .unwrap()
                            .chunk
                            .write(OpCode::Constant(idx));
                    }
                    Literal::FString(s) => {
                        let idx = self
                            .function
                            .as_mut()
                            .unwrap()
                            .chunk
                            .add_constant(Value::FString(s));
                        self.function
                            .as_mut()
                            .unwrap()
                            .chunk
                            .write(OpCode::Constant(idx));
                    }
                }
            }
            Expression::Binary(left, op, right) => {
                self.compile_expression(*left, globals);
                self.compile_expression(*right, globals);
                let op_code = match op {
                    BinaryOp::Add => OpCode::Add,
                    BinaryOp::Sub => OpCode::Sub,
                    BinaryOp::Mul => OpCode::Mul,
                    BinaryOp::Div => OpCode::Div,
                    BinaryOp::Equal => OpCode::Equal,
                    BinaryOp::NotEqual => OpCode::NotEqual,
                    BinaryOp::LessThan => OpCode::LessThan,
                    BinaryOp::GreaterThan => OpCode::GreaterThan,
                };
                self.function.as_mut().unwrap().chunk.write(op_code);
            }
            Expression::Identifier(name) => {
                if let Some(&slot) = self.locals.get(&name) {
                    self.function.as_mut().unwrap().chunk.write(OpCode::GetLocal(slot));
                } else if let Some(val) = globals.get(&name) {
                    let idx = self
                        .function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .add_constant(val.clone());
                    self.function.as_mut().unwrap().chunk.write(OpCode::Constant(idx));
                } else {
                    eprintln!(
                        "Warning: Undefined variable '{}' - will fail at runtime", name
                    );
                    let idx = self
                        .function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .add_constant(Value::Void);
                    self.function.as_mut().unwrap().chunk.write(OpCode::Constant(idx));
                }
            }
            Expression::Call(name, args) => {
                for arg in &args {
                    self.compile_expression(arg.clone(), globals);
                }
                if let Some(&slot) = self.locals.get(&name) {
                    self.function.as_mut().unwrap().chunk.write(OpCode::GetLocal(slot));
                } else if let Some(val) = globals.get(&name) {
                    let idx = self
                        .function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .add_constant(val.clone());
                    self.function.as_mut().unwrap().chunk.write(OpCode::Constant(idx));
                } else {
                    eprintln!(
                        "Warning: Undefined function '{}' - will fail at runtime", name
                    );
                    let idx = self
                        .function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .add_constant(Value::Void);
                    self.function.as_mut().unwrap().chunk.write(OpCode::Constant(idx));
                }
                for arg in args.clone() {
                    self.compile_expression(arg, globals);
                }
                self.function
                    .as_mut()
                    .unwrap()
                    .chunk
                    .write(OpCode::Call(args.len() as u8));
            }
            Expression::Assign(name, val) => {
                self.compile_expression(*val, globals);
                if let Some(&slot) = self.locals.get(&name) {
                    self.function.as_mut().unwrap().chunk.write(OpCode::SetLocal(slot));
                } else {
                    eprintln!(
                        "Warning: Undefined variable assignment '{}' - will fail at runtime",
                        name
                    );
                    self.function.as_mut().unwrap().chunk.write(OpCode::Pop);
                }
            }
        }
    }
}
