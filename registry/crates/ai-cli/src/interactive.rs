// __FU_COMPAT_START__
#![allow(missing_docs)]
use std::io;
use std::io::Write;
#[allow(missing_docs, dead_code)] type FBool = bool;
// __FU_COMPAT_END__
use anyhow::Result;
pub fn start_session(offline: FBool) -> Result<()> {
    println!("Starting interactive session...");
    println!("Type 'exit' to quit, 'help' for commands\n");
    loop {
        print!("fusion-ai> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        match input {
            "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }
            "help" => {
                print_help();
            }
            "" => continue,
            _ => {
                crate::commands::handle_prompt(input, offline)?;
            }
        }
    }
    Ok(())
}
pub fn print_help() {
    println!("Available commands:");
    println!("  help    - Show this help");
    println!("  exit    - Exit the session");
    println!("  <text>  - Send prompt to AI");
}
