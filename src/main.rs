use ast::{Block, Declaration, Expression, Literal, Statement, Type};
use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;

mod ast;
mod borrow_checker;
mod codegen;
mod crypto;
mod lexer;
mod lsp;
mod module_resolver;
mod package_manager;
mod parser;
mod semantic_analyzer;
mod stdlib;
mod wasm;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to compile
    #[arg(short, long)]
    input: Option<String>,

    /// Start in Language Server Protocol mode
    #[arg(long)]
    lsp: bool,

    /// Enable multi-file compilation mode
    #[arg(long)]
    multi_file: bool,

    /// Target compilation backend (llvm or wasm)
    #[arg(long, default_value = "llvm")]
    target: String,

    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Launch LSP server if --lsp flag is provided
    if args.lsp {
        println!("Starting Fusion Language Server...");
        tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime")
            .block_on(async {
                lsp::server::run_server().await;
            });
        return;
    }

    if let Some(input_file) = args.input {
        // Check target
        match args.target.as_str() {
            "wasm" => {
                // WebAssembly compilation
                compile_to_wasm(&input_file, args.output.as_deref());
            }
            "llvm" => {
                // LLVM IR compilation (default)
                if args.multi_file {
                    compile_multi_file(&input_file);
                } else {
                    compile_single_file(&input_file);
                }
            }
            _ => {
                eprintln!(
                    "Error: Unknown target '{}'. Use 'llvm' or 'wasm'.",
                    args.target
                );
                std::process::exit(1);
            }
        }
    } else {
        println!("Fusion Compiler v0.1.0 - Hello World Demo");

        // 1. Manually construct AST for:
        // fn main() {
        //     return 0;
        // }
        let ast = vec![Declaration::Function {
            name: "main".to_string(),
            attributes: vec![],
            generic_params: vec![],
            where_bounds: vec![],
            params: vec![],
            return_type: Type::Void,
            body: Block {
                statements: vec![Statement::Return(Some(Expression::Literal(
                    Literal::Integer(0),
                )))],
            },
        }];

        println!("\n[1] AST Constructed");

        // 2. Semantic Analysis
        let mut analyzer = semantic_analyzer::SemanticAnalyzer::new();
        match analyzer.analyze(ast) {
            Ok(checked_ast) => {
                println!("[2] Semantic Analysis Passed");

                // 3. Code Generation
                let mut codegen = codegen::CodeGenerator::new();
                match codegen.generate(&checked_ast) {
                    Ok(ir) => {
                        println!("[3] LLVM IR Generated:\n");
                        println!("{}", ir);
                    }
                    Err(e) => eprintln!("Codegen Error: {}", e),
                }
            }
            Err(e) => {
                eprintln!("Semantic Error: {:?}", e);
            }
        }
    }
}

/// Single-file compilation (legacy mode)
fn compile_single_file(input_file: &str) {
    println!("Compiling {}...", input_file);

    let mut content = std::fs::read_to_string(&input_file).expect("Failed to read input file");

    // Prepend Core Library Declarations
    content = format!("{}{}", stdlib::CORE_LIBS, content);

    let mut parser = parser::Parser::new(&content);
    match parser.parse_program() {
        Ok(ast) => {
            println!("Parsed AST successfully.");
            // 2. Semantic Analysis
            let mut analyzer = semantic_analyzer::SemanticAnalyzer::new();
            match analyzer.analyze(ast) {
                Ok(checked_ast) => {
                    println!("Semantic Analysis Passed");

                    // 3. Borrow Checker
                    let mut borrow_checker = borrow_checker::BorrowChecker::new();
                    match borrow_checker.check(&checked_ast) {
                        Ok(_) => {
                            println!("Borrow Checker Passed");

                            // 4. Code Generation
                            let mut codegen = codegen::CodeGenerator::new();
                            match codegen.generate(&checked_ast) {
                                Ok(ir) => {
                                    println!("LLVM IR Generated:\n{}", ir);
                                }
                                Err(e) => eprintln!("Codegen Error: {}", e),
                            }
                        }
                        Err(errors) => {
                            eprintln!("Borrow/Ownership Errors:");
                            for err in errors {
                                eprintln!(" - {}", err);
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Semantic Error: {:?}", e),
            }
        }
        Err(e) => eprintln!("Parse Error: {}", e),
    }
}

/// Multi-file compilation mode
fn compile_multi_file(entry_file: &str) {
    println!("Multi-file compilation starting from {}...", entry_file);

    // 1. Resolve module dependencies
    let entry_path = PathBuf::from(entry_file);
    let mut resolver = module_resolver::ModuleResolver::new(entry_path.clone());

    let compilation_order = match resolver.resolve() {
        Ok(order) => {
            println!("Module resolution successful.");
            println!("Compilation order:");
            for (i, module_name) in order.iter().enumerate() {
                println!("  {}. {}", i + 1, module_name);
            }
            order
        }
        Err(e) => {
            eprintln!("Module resolution error: {}", e);
            return;
        }
    };

    // 2. Compile each module in dependency order
    let mut module_irs: HashMap<String, String> = HashMap::new();

    for module_name in &compilation_order {
        let module = match resolver.get_module(module_name) {
            Some(m) => m,
            None => {
                eprintln!(
                    "Internal error: Module '{}' not found in resolver",
                    module_name
                );
                return;
            }
        };

        println!("\nCompiling module '{}'...", module_name);

        // Read module source
        let mut content = match std::fs::read_to_string(&module.path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read module '{}': {}", module_name, e);
                return;
            }
        };

        // Prepend stdlib for each module
        content = format!("{}{}", stdlib::CORE_LIBS, content);

        // Parse
        let mut parser = parser::Parser::new(&content);
        let ast = match parser.parse_program() {
            Ok(ast) => {
                println!("  Parsed successfully.");
                ast
            }
            Err(e) => {
                eprintln!("  Parse error in '{}': {}", module_name, e);
                return;
            }
        };

        // Semantic analysis
        let mut analyzer = semantic_analyzer::SemanticAnalyzer::new();
        let checked_ast = match analyzer.analyze(ast) {
            Ok(ast) => {
                println!("  Semantic analysis passed.");
                ast
            }
            Err(e) => {
                eprintln!("  Semantic error in '{}': {:?}", module_name, e);
                return;
            }
        };

        // Borrow checking
        let mut borrow_checker = borrow_checker::BorrowChecker::new();
        match borrow_checker.check(&checked_ast) {
            Ok(_) => println!("  Borrow checker passed."),
            Err(errors) => {
                eprintln!("  Borrow checker errors in '{}':", module_name);
                for err in errors {
                    eprintln!("    - {}", err);
                }
                return;
            }
        }

        // Code generation
        let mut codegen = codegen::CodeGenerator::new();
        let ir = match codegen.generate(&checked_ast) {
            Ok(ir) => {
                println!("  LLVM IR generated successfully.");
                ir
            }
            Err(e) => {
                eprintln!("  Codegen error in '{}': {}", module_name, e);
                return;
            }
        };

        module_irs.insert(module_name.clone(), ir);
    }

    // 3. Link all module IRs
    println!("\n=== Linking {} modules ===", module_irs.len());

    for module_name in &compilation_order {
        if let Some(ir) = module_irs.get(module_name) {
            println!("\n--- Module '{}' IR ---", module_name);
            println!("{}", ir);
        }
    }

    println!("\n✅ Multi-file compilation successful!");
    println!("Compiled {} modules in total.", compilation_order.len());
}

/// WebAssembly compilation mode
fn compile_to_wasm(input_file: &str, output_file: Option<&str>) {
    use wasm::WasmCodeGenerator;

    println!("Compiling {} to WebAssembly...", input_file);

    // Read source file
    let content = match std::fs::read_to_string(input_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read input file: {}", e);
            return;
        }
    };

    // Parse
    let mut parser = parser::Parser::new(&content);
    let ast = match parser.parse_program() {
        Ok(ast) => {
            println!("  ✅ Parsed successfully.");
            ast
        }
        Err(e) => {
            eprintln!("  ❌ Parse error: {}", e);
            return;
        }
    };

    // Semantic analysis
    let mut analyzer = semantic_analyzer::SemanticAnalyzer::new();
    let checked_ast = match analyzer.analyze(ast) {
        Ok(ast) => {
            println!("  ✅ Semantic analysis passed.");
            ast
        }
        Err(e) => {
            eprintln!("  ❌ Semantic error: {:?}", e);
            return;
        }
    };

    // Generate WASM
    let mut wasm_generator = WasmCodeGenerator::new();
    let wasm_bytes = match wasm_generator.generate(&checked_ast) {
        Ok(bytes) => {
            println!("  ✅ WebAssembly generated ({} bytes).", bytes.len());
            bytes
        }
        Err(e) => {
            eprintln!("  ❌ WASM generation error: {}", e);
            return;
        }
    };

    // Validate WASM
    match wasmparser::validate(&wasm_bytes) {
        Ok(_) => println!("  ✅ WASM validation passed."),
        Err(e) => {
            eprintln!("  ⚠️  WASM validation failed: {}", e);
            eprintln!("  (Continuing anyway...)");
        }
    }

    // Write output file
    let output_path = output_file.unwrap_or("output.wasm");
    match std::fs::write(output_path, &wasm_bytes) {
        Ok(_) => {
            println!("\n✅ WebAssembly compilation successful!");
            println!("Output written to: {}", output_path);
            println!("Size: {} bytes", wasm_bytes.len());
        }
        Err(e) => {
            eprintln!("Failed to write output file: {}", e);
        }
    }
}
