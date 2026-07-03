use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StructInfo {
    pub fields: HashMap<String, (crate::ir::Type, usize)>,
    pub ordered_fields: Vec<(String, crate::ir::Type)>,
}