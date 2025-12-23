use crate::compiler::ast::*;
use crate::compiler::chunk::OpCode;
use crate::compiler::function::Function;
use crate::compiler::value::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
    #[error("Undefined function: {0}")]
    UndefinedFunction(String),
    #[error("No main function found")]
    NoMainFunction,
    #[error("Main is not a function")]
    MainNotFunction,
}

pub type CompilerResult<T> = Result<T, CompilerError>;

pub struct Compiler {
    // Current function being compiled
    function: Option<Function>,
    // All compiled functions
    #[allow(dead_code)]
    functions: Vec<Function>,

    // Locals for current function
    locals: HashMap<String, u16>,
    local_count: u16,
    // Globals (Functions) mapping name -> index in `functions` (or we stick to putting them in constants?)
    // In our VM model, functions are values. So we emit a Constant that is the Function.

    // Struct Layouts: StructName -> { FieldName -> Index }
    struct_layouts: HashMap<String, HashMap<String, u8>>,
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
        // Find main to start, but we compile all
        // We treating declarations as just defining functions.
        // We will return `main` as the entry point function.
        // But main might call others.
        // Problem: If `main` calls `add`, `add` must be available.
        // In this VM, we don't have a "global" table yet easily accessible from inside a frame unless passed or stored.
        // Solution: Use `Constant` pool.
        // When we compile `main`, if it encounters `add`, we resolve it.
        // But `add` might not be compiled yet. (Forward ref).
        // Two pass:
        // 1. Scan all functions, create "Prototypes" or register names.
        // 2. Compile bodies.

        // For simplicity:
        // Return a top-level "Script" function that defines all other functions (as closures/constants) and then calls main?
        // Or just rely on the fact we only have `main` and helpers, and we handle logical linking?

        // Let's do:
        // Compile all functions.
        // But how does `main` know about `add`?
        // It needs to load `add` from... somewhere.
        //
        // Option A: Strings. `Call("add")` -> VM looks up "add". Dynamic dispatch. (Slow but easy).
        // Option B: Constants. `main` has a constant pool. Entry `0` is `Function add`.
        // This requires `main`'s chunk to contain the `Function` object of `add`.

        // Let's go with Option B (constants).
        // We need a map of global function names to their compiled `Function` objects.

        let mut globals: HashMap<String, Value> = HashMap::new();

        // Define Natives
        globals.insert(
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

        globals.insert(
            "clock".to_string(),
            Value::Native(crate::compiler::value::NativeFunction {
                name: "clock".to_string(),
                func: |_| {
                    use std::time::{SystemTime, UNIX_EPOCH};
                    let start = SystemTime::now();
                    let since_the_epoch = start
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");
                    Value::Int(since_the_epoch.as_secs() as i64)
                },
            }),
        );

        // 1. Scan for Structs to build layouts.
        for decl in &program.declarations {
            if let Declaration::Struct(s) = decl {
                let mut layout = HashMap::new();
                for (i, (name, _)) in s.fields.iter().enumerate() {
                    layout.insert(name.clone(), i as u8);
                }
                self.struct_layouts.insert(s.name.clone(), layout);
            }
        }

        // 2. Pre-declare all functions (so we can recursive call or forward call)
        // Just names for now.

        // 2. Compile each function
        for decl in &program.declarations {
            if let Declaration::Function(f) = decl {
                let compiled = self.compile_fn(f.clone(), &globals); // Pass others?
                globals.insert(f.name.clone(), Value::Function(Box::new(compiled)));
            }
        }

        // Re-compile or patch?
        // Single pass with fixup is standard.
        // Or lazy compilation.

        // Hack for Phase 4 MVP:
        // Just compile strictly bottom-up? Order matters?
        // Or support recursion?

        // Let's implement `compile_single_function` which compiles a `FunctionDecl`.
        // Inside `compile_expression` for `Call`, if name is not local, we assume Global.
        // We assume Global functions are available to be put in Constant pool.
        // But we don't have them yet.

        // We can do this:
        // 1. Create `Function` objects with empty chunks.
        // 2. Put them in map.
        // 3. Compile bodies.

        // But Chunk owns code. `Function` owns `Chunk`.
        // We can't mutate `Function` inside map easily if we clone it into Constants.

        // Refined Plan:
        // Compile everything. `Call` emits a placeholder or works if we order `add` before `main` in source.
        // Let's relax and require definition before use for this step?
        // Or simpler:
        // Use Global Registry in VM.
        // `VM` has `globals: HashMap<String, Value>`.
        // `OpCode::GetGlobal(name_idx)`.
        // Load function from global. Invoke.

        // This requires `GetGlobal` opcode.
        // Let's add `GetGlobal` to VM?
        // Or `DefineGlobal`.

        // Let's stick to what we have:
        // If we compile `add`, we get a `Function` value.
        // When compiling `main`, can we just insert that `Value`?
        // Yes, if we have it.
        // So we just iterate declarations. If we see `Function`, we compile it.
        // Then we store it in `completed_functions` map.
        // When compiling `main`, if it calls `add`, we look in map. If found, add to constants.
        // This works for ordered declarations.

        for decl in program.declarations {
            if let Declaration::Function(f) = decl {
                // We create a FRESH compiler for each function context to reuse logic
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

    fn compile_fn(&mut self, ast_func: FunctionDecl, globals: &HashMap<String, Value>) -> Function {
        let func = Function::new(ast_func.name.clone(), ast_func.params.len());

        // Setup locals
        self.locals.clear();
        self.local_count = 0;

        // Params are locals 0..N
        for (param_name, _) in ast_func.params {
            self.locals.insert(param_name, self.local_count);
            self.local_count += 1;
        }

        // Current Chunk swap
        self.function = Some(func);
        // We need to access `globals` in compile_expression.
        // Passing it down or storing in struct?
        // Storing in struct is cleaner but ownership issues if we hold ref.
        // I will clone needed globals or just lookup.
        // Let's pass `globals` to `compile_statement`... recursive.

        for stmt in ast_func.body {
            self.compile_statement(stmt, globals);
        }

        // Implicit return
        self.function.as_mut().unwrap().chunk.write(OpCode::Return);

        self.function.take().unwrap()
    }

    fn compile_statement(&mut self, stmt: Statement, globals: &HashMap<String, Value>) {
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
                self.function
                    .as_mut()
                    .unwrap()
                    .chunk
                    .write(OpCode::JumpIfFalse(0xFFFF));
                self.function.as_mut().unwrap().chunk.write(OpCode::Pop);

                for s in then_block {
                    self.compile_statement(s, globals);
                }

                let jump_to_end_idx = self.function.as_ref().unwrap().chunk.code.len();
                self.function
                    .as_mut()
                    .unwrap()
                    .chunk
                    .write(OpCode::Jump(0xFFFF));

                let else_idx = self.function.as_ref().unwrap().chunk.code.len();
                let _offset = (else_idx - jump_if_false_idx - 1) as u16;
                self.function.as_mut().unwrap().chunk.code[jump_if_false_idx] =
                    OpCode::JumpIfFalse(_offset);

                self.function.as_mut().unwrap().chunk.write(OpCode::Pop);

                if let Some(else_block) = else_block_opt {
                    for s in else_block {
                        self.compile_statement(s, globals);
                    }
                }

                let end_idx = self.function.as_ref().unwrap().chunk.code.len();
                let offset = (end_idx - jump_to_end_idx - 1) as u16;
                self.function.as_mut().unwrap().chunk.code[jump_to_end_idx] = OpCode::Jump(offset);
            }
            Statement::While(cond, body) => {
                let loop_start = self.function.as_ref().unwrap().chunk.code.len();
                self.compile_expression(cond, globals);

                let exit_jump = self.function.as_ref().unwrap().chunk.code.len();
                self.function
                    .as_mut()
                    .unwrap()
                    .chunk
                    .write(OpCode::JumpIfFalse(0xFFFF));
                self.function.as_mut().unwrap().chunk.write(OpCode::Pop);

                for s in body {
                    self.compile_statement(s, globals);
                }

                let loop_end = self.function.as_ref().unwrap().chunk.code.len();
                let loop_offset = (loop_end + 1 - loop_start) as u16;
                self.function
                    .as_mut()
                    .unwrap()
                    .chunk
                    .write(OpCode::Loop(loop_offset));

                let end = self.function.as_ref().unwrap().chunk.code.len();
                let exit_offset = (end - exit_jump - 1) as u16;
                self.function.as_mut().unwrap().chunk.code[exit_jump] =
                    OpCode::JumpIfFalse(exit_offset);

                self.function.as_mut().unwrap().chunk.write(OpCode::Pop);
            }
            Statement::Block(stmts) => {
                for s in stmts {
                    self.compile_statement(s, globals);
                }
            }
        }
    }

    fn compile_expression(&mut self, expr: Expression, globals: &HashMap<String, Value>) {
        match expr {
            Expression::StructInit(_name, fields) => {
                let field_count = fields.len();
                for (field_name, expr) in fields {
                    // Key
                    let name_idx = self
                        .function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .add_constant(Value::String(field_name.clone()));
                    self.function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .write(OpCode::Constant(name_idx));

                    // Value
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
                    .add_constant(Value::String(field.clone()));
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
                    .add_constant(Value::String(field.clone()));
                self.function
                    .as_mut()
                    .unwrap()
                    .chunk
                    .write(OpCode::SetProp(name_idx as u8));
            }
            Expression::Literal(lit) => match lit {
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
                Literal::String(s) => {
                    let idx = self
                        .function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .add_constant(Value::String(s));
                    self.function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .write(OpCode::Constant(idx));
                }
            },
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
                    self.function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .write(OpCode::GetLocal(slot));
                } else if let Some(val) = globals.get(&name) {
                    // Global Function found
                    let idx = self
                        .function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .add_constant(val.clone());
                    self.function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .write(OpCode::Constant(idx));
                } else {
                    // Store error for later reporting
                    eprintln!(
                        "Warning: Undefined variable '{}' - will fail at runtime",
                        name
                    );
                    // Emit a constant null/void value as placeholder
                    let idx = self
                        .function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .add_constant(Value::Void);
                    self.function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .write(OpCode::Constant(idx));
                }
            }
            Expression::Call(name, args) => {
                // 1. Push arguments
                for arg in &args {
                    self.compile_expression(arg.clone(), globals);
                }

                // 2. Resolve function (push it on stack)
                // We reuse Identifier logic implicitly or duplicates it?
                // Identifier puts it on stack.
                // But Call format is: [Func] [Arg0]...
                // Wait.
                // Stack convention:
                // [Func] [Arg0] [Arg1] ...
                // Then `Call(2)` -> Peeks `2` deep to find Func.

                // So we must push Function FIRST.
                // But the Arguments are expressions.
                // If I push function first, then eval args, stack is:
                // [Func] [Arg0value] ...
                // Yes.

                // So:
                // Find Function
                if let Some(&slot) = self.locals.get(&name) {
                    self.function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .write(OpCode::GetLocal(slot));
                } else if let Some(val) = globals.get(&name) {
                    let idx = self
                        .function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .add_constant(val.clone());
                    self.function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .write(OpCode::Constant(idx));
                } else {
                    // Store error for later reporting
                    eprintln!(
                        "Warning: Undefined function '{}' - will fail at runtime",
                        name
                    );
                    // Emit a constant null/void value as placeholder
                    let idx = self
                        .function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .add_constant(Value::Void);
                    self.function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .write(OpCode::Constant(idx));
                }

                // Push Args
                for arg in args.clone() {
                    self.compile_expression(arg, globals);
                }

                // OpCode Call
                self.function
                    .as_mut()
                    .unwrap()
                    .chunk
                    .write(OpCode::Call(args.len() as u8));
            }
            Expression::Assign(name, val) => {
                self.compile_expression(*val, globals);
                if let Some(&slot) = self.locals.get(&name) {
                    self.function
                        .as_mut()
                        .unwrap()
                        .chunk
                        .write(OpCode::SetLocal(slot));
                } else {
                    // Store error for later reporting
                    eprintln!(
                        "Warning: Undefined variable assignment '{}' - will fail at runtime",
                        name
                    );
                    // Emit a pop to maintain stack balance
                    self.function.as_mut().unwrap().chunk.write(OpCode::Pop);
                }
            }
        }
    }
}
