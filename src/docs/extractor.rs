// src/docs/extractor.rs - Extract Documentation from AST (Simplified)
#![allow(dead_code)]
// Parses doc comments and extracts API documentation

use super::{DocItem, DocItemType};
use crate::ast::{Attribute, Declaration, Parameter, Type};

/// Documentation extractor
pub struct DocExtractor {
    /// Current module path
    #[allow(dead_code)]
    current_module: Vec<String>,
}

impl DocExtractor {
    /// Create a new documentation extractor
    pub fn new() -> Self {
        Self {
            current_module: Vec::new(),
        }
    }

    /// Extract documentation from declarations
    pub fn extract(&mut self, declarations: &[Declaration]) -> Vec<DocItem> {
        let mut items = Vec::new();

        for decl in declarations {
            if let Some(item) = self.extract_from_declaration(decl) {
                items.push(item);
            }
        }

        items
    }

    /// Extract documentation from a single declaration
    fn extract_from_declaration(&mut self, decl: &Declaration) -> Option<DocItem> {
        match decl {
            Declaration::Function {
                name,
                params,
                return_type,
                attributes,
                ..
            } => Some(self.extract_function_docs(name, params, return_type, attributes)),
            Declaration::Class { name, .. } => Some(DocItem::new(
                DocItemType::Class,
                name,
                format!("class {}", name),
            )),
            Declaration::Trait { name, .. } => Some(DocItem::new(
                DocItemType::Trait,
                name,
                format!("trait {}", name),
            )),
            _ => None,
        }
    }

    /// Extract function documentation
    fn extract_function_docs(
        &self,
        name: &str,
        params: &[Parameter],
        return_type: &Type,
        attributes: &[Attribute],
    ) -> DocItem {
        let signature = self.build_function_signature(name, params, return_type);
        let docs = self.extract_doc_comments(attributes);

        DocItem::new(DocItemType::Function, name, signature).with_docs(docs)
    }

    /// Build function signature string
    fn build_function_signature(
        &self,
        name: &str,
        params: &[Parameter],
        return_type: &Type,
    ) -> String {
        let params_str: Vec<String> = params
            .iter()
            .map(|p| format!("{}: {}", p.name, self.type_to_string(&p.param_type)))
            .collect();

        let return_str = match return_type {
            Type::Void => String::new(),
            _ => format!(" -> {}", self.type_to_string(return_type)),
        };

        format!("fn {}({}){}", name, params_str.join(", "), return_str)
    }

    /// Convert type to string
    fn type_to_string(&self, t: &Type) -> String {
        match t {
            Type::Integer => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::Boolean => "bool".to_string(),
            Type::String => "string".to_string(),
            Type::Void => "void".to_string(),
            Type::Custom(name) => name.clone(),
            Type::TypeParameter(name) => name.clone(),
            Type::Array(inner) => format!("[{}]", self.type_to_string(inner)),
            Type::Optional(inner) => format!("?{}", self.type_to_string(inner)),
            Type::GenericInstance { base_name, args } => {
                let args_str: Vec<String> = args.iter().map(|t| self.type_to_string(t)).collect();
                format!("{}<{}>", base_name, args_str.join(", "))
            }
            _ => "unknown".to_string(),
        }
    }

    /// Extract documentation comments from attributes
    fn extract_doc_comments(&self, _attributes: &[Attribute]) -> String {
        // Simplified - would extract from doc comment attributes
        String::new()
    }
}

impl Default for DocExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_extractor_creation() {
        let extractor = DocExtractor::new();
        assert!(extractor.current_module.is_empty());
    }

    #[test]
    fn test_type_to_string() {
        let extractor = DocExtractor::new();

        assert_eq!(extractor.type_to_string(&Type::Integer), "int");
        assert_eq!(extractor.type_to_string(&Type::String), "string");
    }
}
