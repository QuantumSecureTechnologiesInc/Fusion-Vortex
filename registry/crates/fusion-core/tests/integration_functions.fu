use fusion_core::compiler::{Compiler, Lexer, Parser};
use fusion_core::vm::VM;
#[test]
fn test_function_definition_and_call() {
    let source = r#"
    fn add(a: Int, b: Int) : Int {
        return a + b;
    }

    fn main() {
        let result = add(10, 20);
        print(result);
    }
    "#;
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().expect("Failed to parse");
    let compiler = Compiler::new();
    let function = compiler.compile(program).expect("Compilation failed");
    let mut vm = VM::new();
    let result = vm.interpret(function);
    assert!(matches!(result, fusion_core::vm::InterpretResult::Ok));
}
#[test]
fn test_recursive_function() {
    let source = r#"
    fn fib(n: Int) : Int {
        if n < 2 {
            return n;
        }
        return fib(n - 1) + fib(n - 2);
    }

    fn main() {
        let x = fib(6); // 8
        print(x);
        // assert(x == 8); // We don't have assert yet
    }
    "#;
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().expect("Failed to parse");
    let compiler = Compiler::new();
    let function = compiler.compile(program).expect("Compilation failed");
    let mut vm = VM::new();
    let result = vm.interpret(function);
    assert!(matches!(result, fusion_core::vm::InterpretResult::Ok));
}
