use crate::ast::Program;
use crate::error::CompilerError;

pub fn check(_program: &Program) -> Result<(), CompilerError> {
    // Placeholder type checker
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Program, Span};

    #[test]
    fn test_check_empty_program() {
        let program = Program {
            declarations: vec![],
        };
        assert!(check(&program).is_ok());
    }
}
