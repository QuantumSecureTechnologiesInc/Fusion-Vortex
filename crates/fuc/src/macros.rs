//! AST-to-AST Macro Expansion Pass
//! Addresses: No macro system, No preprocessor.
use crate::types::*;
use std::collections::HashMap;

use crate::ast::{Program, Statement};

/// Represents a simple `macro_rules!` style text substitution macro.
pub struct MacroDefinition {
    pub name: FString,
    pub pattern: FVec<FString>, // Token patterns
    pub template: FVec<FString>, // Output AST template tokens
}

pub struct MacroExpander {
    macros: FMap<FString, MacroDefinition>,
}

impl MacroExpander {
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
        }
    }

    /// Registers a new macro definition into the expander
    pub fn register_macro(&mut self, def: MacroDefinition) {
        self.macros.insert(def.name.clone(), def);
    }

    /// Performs a pre-order traversal of the AST, replacing macro invocations
    /// with their expanded AST sub-trees before Semantic Analysis occurs.
    pub fn expand_program(&mut self, mut prog: Program) -> Program {
        // Iterate through function bodies and look for Expression::FunctionCall
        // where the name ends in `!` (e.g., `println!`).
        
        for func in &mut prog.functions {
            let mut new_body = Vec::new();
            for stmt in &func.body.statements {
                new_body.push(self.expand_statement(stmt.clone()));
            }
            func.body.statements = new_body;
        }
        
        prog
    }

    fn expand_statement(&self, stmt: Statement) -> Statement {
        // Stub: recursively expand statements. If a macro like `vec![]` is found
        // in an assignment or let binding, it transforms it into raw AST 
        // ArrayLiteral or sequence of push commands.
        stmt
    }
}