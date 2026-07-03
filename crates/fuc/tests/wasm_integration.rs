//! WASM integration tests — exercises the full WASM codegen pipeline.
//! Validates WASM binary output with wasmparser.

#![cfg(feature = "wasm")]

use fuc::ast::{
    BinaryOp, Block, Declaration, Expression, ExpressionKind, Literal, Parameter, Statement, Type,
};
use fuc::wasm::WasmCodeGenerator;

fn make_expr(kind: ExpressionKind) -> Expression {
    Expression { kind, ty: None }
}

fn make_var(name: &str) -> Expression {
    make_expr(ExpressionKind::Variable(name.to_string()))
}

fn make_int(n: i64) -> Expression {
    make_expr(ExpressionKind::Literal(Literal::Integer(n)))
}

fn make_binary(left: Expression, op: BinaryOp, right: Expression) -> Expression {
    make_expr(ExpressionKind::BinaryOp {
        left: Box::new(left),
        op,
        right: Box::new(right),
    })
}

fn make_str(s: &str) -> Expression {
    make_expr(ExpressionKind::Literal(Literal::String(s.to_string())))
}

fn validate_wasm(bytes: &[u8]) {
    let validation = wasmparser::validate(bytes);
    assert!(
        validation.is_ok(),
        "WASM validation failed: {:?}",
        validation.err()
    );
}

/// Count exported functions in a WASM module
fn count_exports(bytes: &[u8]) -> usize {
    let mut count = 0;
    for payload in wasmparser::Parser::new(0).parse_all(bytes) {
        if let Ok(wasmparser::Payload::ExportSection(reader)) = payload {
            for export in reader {
                if export.is_ok() {
                    count += 1;
                }
            }
        }
    }
    count
}

/// Check if a WASM module exports a specific function name
fn has_export(bytes: &[u8], expected_name: &str) -> bool {
    for payload in wasmparser::Parser::new(0).parse_all(bytes) {
        if let Ok(wasmparser::Payload::ExportSection(reader)) = payload {
            for export in reader {
                if let Ok(export) = export {
                    if export.name == expected_name {
                        return true;
                    }
                }
            }
        }
    }
    false
}

#[test]
fn test_basic_math_wasm() {
    let mut generator = WasmCodeGenerator::new();

    let ops = [
        (BinaryOp::Add, "add"),
        (BinaryOp::Sub, "sub"),
        (BinaryOp::Mul, "mul"),
        (BinaryOp::Div, "div"),
        (BinaryOp::Mod, "mod"),
        (BinaryOp::Eq, "eq"),
        (BinaryOp::Neq, "neq"),
        (BinaryOp::Lt, "lt"),
        (BinaryOp::Gt, "gt"),
        (BinaryOp::Le, "le"),
        (BinaryOp::Ge, "ge"),
        (BinaryOp::And, "and"),
        (BinaryOp::Or, "or"),
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
                    make_var("x"), *op, make_var("y"),
                )))],
            },
            where_bounds: vec![],
        });
    }

    let wasm_bytes = generator.generate(&decls).expect("WASM generation failed");
    validate_wasm(&wasm_bytes);
    assert!(wasm_bytes.len() > 100, "WASM binary too small: {} bytes", wasm_bytes.len());

    // Verify all 13 functions are exported
    assert_eq!(count_exports(&wasm_bytes), 13);
    for (_, name) in &ops {
        assert!(has_export(&wasm_bytes, name), "Missing export: {}", name);
    }
}

#[test]
fn test_control_flow_wasm() {
    let mut generator = WasmCodeGenerator::new();

    let decls = vec![
        // fn max(x: int, y: int) -> int { if x > y { return x; } else { return y; } }
        Declaration::Function {
            name: "max".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
                Parameter { name: "y".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::If {
                    cond: make_binary(make_var("x"), BinaryOp::Gt, make_var("y")),
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
        // fn countdown(n: int) -> int { let mut x = n; while x > 0 { x = x - 1; } return 0; }
        Declaration::Function {
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
        },
    ];

    let wasm_bytes = generator.generate(&decls).expect("WASM generation failed");
    validate_wasm(&wasm_bytes);
    assert!(has_export(&wasm_bytes, "max"));
    assert!(has_export(&wasm_bytes, "countdown"));
}

#[test]
fn test_strings_wasm() {
    let mut generator = WasmCodeGenerator::new();

    let decls = vec![
        Declaration::Function {
            name: "hello".to_string(),
            params: vec![],
            return_type: Type::String,
            body: Block {
                statements: vec![Statement::Return(Some(make_str("Hello, World!")))],
            },
            where_bounds: vec![],
        },
        Declaration::Function {
            name: "greeting".to_string(),
            params: vec![],
            return_type: Type::String,
            body: Block {
                statements: vec![Statement::Return(Some(make_str("Hello from Fusion WASM!")))],
            },
            where_bounds: vec![],
        },
        Declaration::Function {
            name: "empty".to_string(),
            params: vec![],
            return_type: Type::String,
            body: Block {
                statements: vec![Statement::Return(Some(make_str("")))],
            },
            where_bounds: vec![],
        },
    ];

    let wasm_bytes = generator.generate(&decls).expect("WASM generation failed");
    validate_wasm(&wasm_bytes);

    assert!(has_export(&wasm_bytes, "hello"));
    assert!(has_export(&wasm_bytes, "greeting"));
    assert!(has_export(&wasm_bytes, "empty"));

    // Verify data section contains the strings
    let mut found_hello = false;
    let mut found_greeting = false;
    for payload in wasmparser::Parser::new(0).parse_all(&wasm_bytes) {
        if let Ok(wasmparser::Payload::DataSection(reader)) = payload {
            for seg in reader {
                if let Ok(seg) = seg {
                    let data: Vec<u8> = seg.data.iter().map(|b| *b as u8).collect();
                    if data.windows(13).any(|w| w == b"Hello, World!") {
                        found_hello = true;
                    }
                    if data.windows(23).any(|w| w == b"Hello from Fusion WASM!") {
                        found_greeting = true;
                    }
                }
            }
        }
    }
    assert!(found_hello, "String 'Hello, World!' not found in data section");
    assert!(found_greeting, "String 'Hello from Fusion WASM!' not found in data section");
}

#[test]
fn test_structs_wasm() {
    let mut generator = WasmCodeGenerator::new();

    let decls = vec![
        // MemberAccess is a stub that passes through the base pointer.
        // Use Type::Int for params since the stub returns i64 (pass-through).
        Declaration::Function {
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
        },
        Declaration::Function {
            name: "get_y".to_string(),
            params: vec![
                Parameter { name: "p".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_expr(
                    ExpressionKind::MemberAccess {
                        base: Box::new(make_var("p")),
                        field: "y".to_string(),
                    },
                )))],
            },
            where_bounds: vec![],
        },
    ];

    let wasm_bytes = generator.generate(&decls).expect("WASM generation failed");
    validate_wasm(&wasm_bytes);
    assert!(has_export(&wasm_bytes, "get_x"));
    assert!(has_export(&wasm_bytes, "get_y"));
}

#[test]
fn test_full_pipeline_wasm() {
    // Comprehensive test covering all WASM features in one module
    let mut generator = WasmCodeGenerator::new();

    let decls = vec![
        // Arithmetic
        Declaration::Function {
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
        },
        // If/else
        Declaration::Function {
            name: "choose".to_string(),
            params: vec![
                Parameter { name: "cond".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::If {
                    cond: make_var("cond"),
                    then_block: Box::new(Block {
                        statements: vec![Statement::Return(Some(make_int(42)))],
                    }),
                    else_block: Some(Box::new(Block {
                        statements: vec![Statement::Return(Some(make_int(0)))],
                    })),
                }],
            },
            where_bounds: vec![],
        },
        // While loop (uses integer comparison, not boolean literal)
        Declaration::Function {
            name: "loop_forever".to_string(),
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
        },
        // String
        Declaration::Function {
            name: "message".to_string(),
            params: vec![],
            return_type: Type::String,
            body: Block {
                statements: vec![Statement::Return(Some(make_str("Integration test OK")))],
            },
            where_bounds: vec![],
        },
        // Unary op
        Declaration::Function {
            name: "negate".to_string(),
            params: vec![
                Parameter { name: "x".to_string(), param_type: Type::Int },
            ],
            return_type: Type::Int,
            body: Block {
                statements: vec![Statement::Return(Some(make_expr(
                    ExpressionKind::UnaryOp {
                        op: fuc::ast::UnaryOp::Neg,
                        expr: Box::new(make_var("x")),
                    },
                )))],
            },
            where_bounds: vec![],
        },
    ];

    let wasm_bytes = generator.generate(&decls).expect("WASM generation failed");
    validate_wasm(&wasm_bytes);

    let exports = count_exports(&wasm_bytes);
    assert_eq!(exports, 5, "Expected 5 exports, got {}", exports);
    assert!(has_export(&wasm_bytes, "add"));
    assert!(has_export(&wasm_bytes, "choose"));
    assert!(has_export(&wasm_bytes, "loop_forever"));
    assert!(has_export(&wasm_bytes, "message"));
    assert!(has_export(&wasm_bytes, "negate"));
}