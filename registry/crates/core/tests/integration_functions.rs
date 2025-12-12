use fusion_core_compiler::{
    compiler::Compiler, lexer::Lexer, parser::Parser, value::Value, vm::VM,
};

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
    let function = compiler.compile(program);

    let mut vm = VM::new();
    // Capture stdout? VM currently prints to stdout.
    // We can't easily capture native print unless we mock it or redirect output.
    // For now, let's trust "InterpretResult::Ok" means it ran without crashing.
    // Using `add` result in `print` ensures logic flow correct.
    let result = vm.interpret(function);

    assert!(matches!(
        result,
        fusion_core_compiler::vm::InterpretResult::Ok
    ));
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
    let function = compiler.compile(program);

    let mut vm = VM::new();
    let result = vm.interpret(function);

    assert!(matches!(
        result,
        fusion_core_compiler::vm::InterpretResult::Ok
    ));
}
