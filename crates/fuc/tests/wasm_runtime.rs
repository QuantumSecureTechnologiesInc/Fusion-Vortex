//! WASM runtime execution tests using wasmtime.
//! Compiles .fu files to WASM and executes them to verify runtime behavior.
#![cfg(feature = "wasm")]

use fuc::ast::*;
use fuc::wasm::WasmCodeGenerator;
use wasmtime::*;

/// Helper: create a typed expression
fn make_expr(kind: ExpressionKind) -> Expression {
    Expression { kind, ty: None }
}

/// Helper: create a variable reference
fn make_var(name: &str) -> Expression {
    make_expr(ExpressionKind::Variable(name.to_string()))
}

/// Helper: create an integer literal
fn make_int(n: i64) -> Expression {
    make_expr(ExpressionKind::Literal(Literal::Integer(n)))
}

/// Helper: create a binary operation
fn make_binop(left: Expression, op: BinaryOp, right: Expression) -> Expression {
    make_expr(ExpressionKind::BinaryOp {
        left: Box::new(left),
        op,
        right: Box::new(right),
    })
}

/// Compile declarations to WASM, then execute a named function with 0 args.
fn run_wasm_func0(declarations: &[Declaration], func_name: &str) -> Result<i64, String> {
    let mut codegen = WasmCodeGenerator::new();
    let wasm_bytes = codegen.generate(declarations)
        .map_err(|e| format!("Codegen error: {}", e))?;

    let engine = Engine::default();
    let module = Module::new(&engine, &wasm_bytes)
        .map_err(|e| format!("Module error: {}", e))?;

    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);
    let instance = linker.instantiate(&mut store, &module)
        .map_err(|e| format!("Instantiation error: {}", e))?;

    let func = instance.get_typed_func::<(), i64>(&mut store, func_name)
        .map_err(|e| format!("Function lookup error for '{}': {}", func_name, e))?;

    func.call(&mut store, ())
        .map_err(|e| format!("Call error: {}", e))
}

/// Compile declarations to WASM, then execute a named function with 1 i64 arg.
fn run_wasm_func1(declarations: &[Declaration], func_name: &str, a: i64) -> Result<i64, String> {
    let mut codegen = WasmCodeGenerator::new();
    let wasm_bytes = codegen.generate(declarations)
        .map_err(|e| format!("Codegen error: {}", e))?;

    let engine = Engine::default();
    let module = Module::new(&engine, &wasm_bytes)
        .map_err(|e| format!("Module error: {}", e))?;

    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);
    let instance = linker.instantiate(&mut store, &module)
        .map_err(|e| format!("Instantiation error: {}", e))?;

    let func = instance.get_typed_func::<(i64,), i64>(&mut store, func_name)
        .map_err(|e| format!("Function lookup error for '{}': {}", func_name, e))?;

    func.call(&mut store, (a,))
        .map_err(|e| format!("Call error: {}", e))
}

/// Compile declarations to WASM, then execute a named function with 2 i64 args.
fn run_wasm_func2(declarations: &[Declaration], func_name: &str, a: i64, b: i64) -> Result<i64, String> {
    let mut codegen = WasmCodeGenerator::new();
    let wasm_bytes = codegen.generate(declarations)
        .map_err(|e| format!("Codegen error: {}", e))?;

    let engine = Engine::default();
    let module = Module::new(&engine, &wasm_bytes)
        .map_err(|e| format!("Module error: {}", e))?;

    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);
    let instance = linker.instantiate(&mut store, &module)
        .map_err(|e| format!("Instantiation error: {}", e))?;

    let func = instance.get_typed_func::<(i64, i64), i64>(&mut store, func_name)
        .map_err(|e| format!("Function lookup error for '{}': {}", func_name, e))?;

    func.call(&mut store, (a, b))
        .map_err(|e| format!("Call error: {}", e))
}

#[test]
fn test_add_wasm_runtime() {
    let decls = vec![
        Declaration::Function {
            name: "add".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
                Parameter { name: "y".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_binop(
                    make_var("x"),
                    BinaryOp::Add,
                    make_var("y"),
                )))],
            },
            where_bounds: vec![],
        },
    ];

    let result = run_wasm_func2(&decls, "add", 2, 3).unwrap();
    assert_eq!(result, 5, "add(2, 3) should be 5");
}

#[test]
fn test_factorial_wasm_runtime() {
    // factorial(5) = 120
    // Uses a recursive-ish approach: iterative loop
    let decls = vec![
        Declaration::Function {
            name: "factorial".to_string(),
            params: vec![
                Parameter { name: "n".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![
                    // let mut result: int = 1;
                    Statement::VariableDeclaration {
                        name: "result".to_string(),
                        initializer: make_int(1),
                        ty: Some(Type::Int),
                    },
                    // let mut i: int = n;
                    Statement::VariableDeclaration {
                        name: "i".to_string(),
                        initializer: make_var("n"),
                        ty: Some(Type::Int),
                    },
                    // while i > 1
                    Statement::While {
                        cond: make_binop(
                            make_var("i"),
                            BinaryOp::Gt,
                            make_int(1),
                        ),
                        body: Box::new(Block {
                            statements: vec![
                                // result = result * i;
                                Statement::Assignment {
                                    target: make_var("result"),
                                    value: make_binop(
                                        make_var("result"),
                                        BinaryOp::Mul,
                                        make_var("i"),
                                    ),
                                },
                                // i = i - 1;
                                Statement::Assignment {
                                    target: make_var("i"),
                                    value: make_binop(
                                        make_var("i"),
                                        BinaryOp::Sub,
                                        make_int(1),
                                    ),
                                },
                            ],
                        }),
                    },
                    // return result;
                    Statement::Return(Some(make_var("result"))),
                ],
            },
            where_bounds: vec![],
        },
    ];

    let result = run_wasm_func1(&decls, "factorial", 5).unwrap();
    assert_eq!(result, 120, "factorial(5) should be 120");
}

#[test]
fn test_subtract_wasm_runtime() {
    let decls = vec![
        Declaration::Function {
            name: "sub".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
                Parameter { name: "y".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_binop(
                    make_var("x"),
                    BinaryOp::Sub,
                    make_var("y"),
                )))],
            },
            where_bounds: vec![],
        },
    ];

    let result = run_wasm_func2(&decls, "sub", 10, 3).unwrap();
    assert_eq!(result, 7, "sub(10, 3) should be 7");
}

#[test]
fn test_multiply_wasm_runtime() {
    let decls = vec![
        Declaration::Function {
            name: "mul".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
                Parameter { name: "y".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_binop(
                    make_var("x"),
                    BinaryOp::Mul,
                    make_var("y"),
                )))],
            },
            where_bounds: vec![],
        },
    ];

    let result = run_wasm_func2(&decls, "mul", 6, 7).unwrap();
    assert_eq!(result, 42, "mul(6, 7) should be 42");
}

#[test]
fn test_division_wasm_runtime() {
    let decls = vec![
        Declaration::Function {
            name: "div".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
                Parameter { name: "y".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_binop(
                    make_var("x"),
                    BinaryOp::Div,
                    make_var("y"),
                )))],
            },
            where_bounds: vec![],
        },
    ];

    let result = run_wasm_func2(&decls, "div", 42, 6).unwrap();
    assert_eq!(result, 7, "div(42, 6) should be 7");
}

#[test]
fn test_modulus_wasm_runtime() {
    let decls = vec![
        Declaration::Function {
            name: "modulus".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
                Parameter { name: "y".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_binop(
                    make_var("x"),
                    BinaryOp::Mod,
                    make_var("y"),
                )))],
            },
            where_bounds: vec![],
        },
    ];

    let result = run_wasm_func2(&decls, "modulus", 17, 5).unwrap();
    assert_eq!(result, 2, "modulus(17, 5) should be 2");
}

#[test]
fn test_comparisons_wasm_runtime() {
    let decls = vec![
        // eq: returns 1 if x == y else 0
        Declaration::Function {
            name: "eq".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
                Parameter { name: "y".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_binop(
                    make_var("x"),
                    BinaryOp::Eq,
                    make_var("y"),
                )))],
            },
            where_bounds: vec![],
        },
        // lt: returns 1 if x < y else 0
        Declaration::Function {
            name: "lt".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
                Parameter { name: "y".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_binop(
                    make_var("x"),
                    BinaryOp::Lt,
                    make_var("y"),
                )))],
            },
            where_bounds: vec![],
        },
        // gt: returns 1 if x > y else 0
        Declaration::Function {
            name: "gt".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
                Parameter { name: "y".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_binop(
                    make_var("x"),
                    BinaryOp::Gt,
                    make_var("y"),
                )))],
            },
            where_bounds: vec![],
        },
    ];

    assert_eq!(run_wasm_func2(&decls, "eq", 5, 5).unwrap(), 1, "eq(5,5) should be 1");
    assert_eq!(run_wasm_func2(&decls, "eq", 5, 3).unwrap(), 0, "eq(5,3) should be 0");
    assert_eq!(run_wasm_func2(&decls, "lt", 3, 5).unwrap(), 1, "lt(3,5) should be 1");
    assert_eq!(run_wasm_func2(&decls, "lt", 5, 3).unwrap(), 0, "lt(5,3) should be 0");
    assert_eq!(run_wasm_func2(&decls, "gt", 5, 3).unwrap(), 1, "gt(5,3) should be 1");
    assert_eq!(run_wasm_func2(&decls, "gt", 3, 5).unwrap(), 0, "gt(3,5) should be 0");
}

#[test]
fn test_if_else_wasm_runtime() {
    let decls = vec![
        Declaration::Function {
            name: "max".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
                Parameter { name: "y".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::If {
                    cond: make_binop(make_var("x"), BinaryOp::Gt, make_var("y")),
                    then_block: Box::new(Block {
                        statements: vec![Statement::Return(Some(make_var("x")))],
                    }),
                    else_block: Some(Box::new(Block {
                        statements: vec![Statement::Return(Some(make_var("y")))],
                    })),
                }],
            },
            where_bounds: vec![],
        },
    ];

    assert_eq!(run_wasm_func2(&decls, "max", 5, 3).unwrap(), 5);
    assert_eq!(run_wasm_func2(&decls, "max", 3, 5).unwrap(), 5);
    assert_eq!(run_wasm_func2(&decls, "max", 7, 7).unwrap(), 7);
}

#[test]
fn test_negate_wasm_runtime() {
    let decls = vec![
        Declaration::Function {
            name: "neg".to_string(),
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
        },
    ];

    assert_eq!(run_wasm_func1(&decls, "neg", 42).unwrap(), -42);
    assert_eq!(run_wasm_func1(&decls, "neg", -10).unwrap(), 10);
    assert_eq!(run_wasm_func1(&decls, "neg", 0).unwrap(), 0);
}

#[test]
fn test_not_wasm_runtime() {
    let decls = vec![
        Declaration::Function {
            name: "not".to_string(),
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
        },
    ];

    // WASM eqz on non-zero → 0, on zero → 1
    assert_eq!(run_wasm_func1(&decls, "not", 0).unwrap(), 1);
    assert_eq!(run_wasm_func1(&decls, "not", 1).unwrap(), 0);
    assert_eq!(run_wasm_func1(&decls, "not", 42).unwrap(), 0);
}

#[test]
fn test_while_sum_wasm_runtime() {
    // sum(n) = sum of 1..n
    let decls = vec![
        Declaration::Function {
            name: "sum_to".to_string(),
            params: vec![
                Parameter { name: "n".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![
                    Statement::VariableDeclaration {
                        name: "total".to_string(),
                        initializer: make_int(0),
                        ty: Some(Type::Int),
                    },
                    Statement::VariableDeclaration {
                        name: "i".to_string(),
                        initializer: make_int(1),
                        ty: Some(Type::Int),
                    },
                    Statement::While {
                        cond: make_binop(
                            make_var("i"),
                            BinaryOp::Le,
                            make_var("n"),
                        ),
                        body: Box::new(Block {
                            statements: vec![
                                Statement::Assignment {
                                    target: make_var("total"),
                                    value: make_binop(
                                        make_var("total"),
                                        BinaryOp::Add,
                                        make_var("i"),
                                    ),
                                },
                                Statement::Assignment {
                                    target: make_var("i"),
                                    value: make_binop(
                                        make_var("i"),
                                        BinaryOp::Add,
                                        make_int(1),
                                    ),
                                },
                            ],
                        }),
                    },
                    Statement::Return(Some(make_var("total"))),
                ],
            },
            where_bounds: vec![],
        },
    ];

    assert_eq!(run_wasm_func1(&decls, "sum_to", 5).unwrap(), 15, "sum_to(5) = 1+2+3+4+5 = 15");
    assert_eq!(run_wasm_func1(&decls, "sum_to", 10).unwrap(), 55, "sum_to(10) = 55");
    assert_eq!(run_wasm_func1(&decls, "sum_to", 1).unwrap(), 1, "sum_to(1) = 1");
}