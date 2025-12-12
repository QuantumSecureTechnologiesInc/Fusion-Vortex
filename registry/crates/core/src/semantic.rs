use crate::ast::Type;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub typ: Type,
    // potentially definition span, mutable, etc.
}

pub struct Scope {
    pub symbols: HashMap<String, SymbolInfo>,
    pub parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn new(parent: Option<Box<Scope>>) -> Self {
        Scope {
            symbols: HashMap::new(),
            parent,
        }
    }

    pub fn insert(&mut self, name: String, typ: Type) -> Result<(), String> {
        if self.symbols.contains_key(&name) {
            return Err(format!("Symbol {} already exists in scope", name));
        }
        self.symbols.insert(name.clone(), SymbolInfo { name, typ });
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<SymbolInfo> {
        if let Some(info) = self.symbols.get(name) {
            Some(info.clone())
        } else if let Some(parent) = &self.parent {
            parent.lookup(name)
        } else {
            None
        }
    }
}
