//! Fusion Code Formatter (fuc fmt)
//! Addresses: No formatter, Developer Experience gaps.
use crate::types::*;

use crate::ast::*;

pub struct Formatter {
    indent_level: usize,
    indent_string: FString,
    output: FString,
}

impl Formatter {
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            indent_string: "    ".to_string(), // 4 spaces default
            output: String::new(),
        }
    }

    fn push_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.output.push_str(&self.indent_string);
        }
    }

    fn newline(&mut self) {
        self.output.push('\n');
    }

    pub fn format_program(&mut self, prog: &Program) -> FString {
        for s in &prog.structs {
            self.format_struct(s);
            self.newline();
        }
        for f in &prog.functions {
            self.format_function(f);
            self.newline();
        }
        std::mem::replace(&mut self.output, String::new())
    }

    fn format_type(&mut self, ty: &Type) {
        let ty_str = match ty {
            Type::Int => "int".to_string(),
            Type::Bool => "bool".to_string(),
            Type::String => "string".to_string(),
            Type::Void => "void".to_string(),
            Type::Struct(name) => name.clone(),
            Type::Pointer(_inner) => "*".to_string(), // Needs recursive formatting
            _ => "unknown".to_string(),
        };
        self.output.push_str(&ty_str);
    }

    fn format_struct(&mut self, def: &StructDefinition) {
        self.push_indent();
        self.output.push_str(&format!("struct {} {{\n", def.name));
        self.indent_level += 1;
        
        for (field_name, field_ty) in &def.fields {
            self.push_indent();
            self.output.push_str(&format!("{}: ", field_name));
            self.format_type(field_ty);
            self.output.push_str(",\n");
        }
        
        self.indent_level -= 1;
        self.push_indent();
        self.output.push_str("}\n");
    }

    fn format_function(&mut self, func: &Function) {
        self.push_indent();
        self.output.push_str(&format!("fn {}(", func.name));
        
        for (i, param) in func.params.iter().enumerate() {
            self.output.push_str(&format!("{}: ", param.name));
            self.format_type(&param.param_type);
            if i < func.params.len() - 1 {
                self.output.push_str(", ");
            }
        }
        
        self.output.push_str(") -> ");
        self.format_type(&func.return_type);
        self.output.push_str(" {\n");
        
        self.indent_level += 1;
        // Stub: Format block statements
        self.push_indent();
        self.output.push_str("// Statements go here\n");
        self.indent_level -= 1;
        
        self.push_indent();
        self.output.push_str("}\n");
    }
}