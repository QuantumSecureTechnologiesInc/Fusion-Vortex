use fusion_core::compiler::Compiler;
use fusion_core::lexer::Lexer;
use fusion_core::parser::Parser;
use fusion_core::type_checker::TypeChecker;
use fusion_core::vm::VM;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let source = if args.len() > 1 {
        std::fs::read_to_string(&args[1])
            .map_err(|e| e.to_string())
            .unwrap()
    } else {
        // Internal test source
        r#"
        struct Point {
            x: Int,
            y: Int,
        }

        fn main() : Int {
            let p: Point = Point { x: 10, y: 20 };
            print("Initial x:");
            print(p.x);
            
            p.x = 30;
            print("New x:");
            print(p.x);
            
            let sum: Int = 0;
            for (let j: Int = 0; j < 3; j = j + 1) {
                sum = sum + p.y;
            }
            print("Sum loops:");
            print(sum);

            return sum;
        }
        "#
        .to_string()
    };

    println!("Source: {}", source);

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);

    match parser.parse_program() {
        Ok(program) => {
            println!("Parsed AST: {:?}", program);

            let mut checker = TypeChecker::new();
            checker.init_stdlib();
            match checker.check_program(&program) {
                Ok(_) => {
                    // println!("Type Checking Passed!");

                    let compiler = Compiler::new();
                    let func = compiler.compile(program);

                    println!("Executing VM...");
                    let mut vm = VM::new();
                    let result = vm.interpret(func);

                    match result {
                        fusion_core::vm::InterpretResult::Ok => {
                            if let Some(val) = vm.last_popped() {
                                println!("Result: {}", val);
                            }
                        }
                        fusion_core::vm::InterpretResult::CompileError => {
                            println!("VM Compile Error")
                        }
                        fusion_core::vm::InterpretResult::RuntimeError(e) => {
                            println!("Runtime Error: {}", e)
                        }
                    }
                }
                Err(e) => println!("Type Error: {}", e),
            }
        }
        Err(e) => println!("Parse Error: {}", e),
    }
}
