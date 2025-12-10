use anyhow::Result;
use fusion_core::compiler::Compiler;
use fusion_core::lexer::Lexer;
use fusion_core::parser::Parser;
use fusion_core::vm::VM;
use std::fs;
use tracing::info;

pub fn run(release: bool, args: &[String]) -> Result<()> {
    if args.is_empty() {
        anyhow::bail!("No input file specified");
    }

    let filename = &args[0];
    info!("Running {} (release: {})", filename, release);

    let source = fs::read_to_string(filename)?;

    let mut lexer = Lexer::new(&source);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program().map_err(|e| anyhow::anyhow!(e))?;

    // TODO: Type check

    let compiler = Compiler::new();
    let function = compiler.compile(program);

    let mut vm = VM::new();
    vm.interpret(function);

    Ok(())
}
