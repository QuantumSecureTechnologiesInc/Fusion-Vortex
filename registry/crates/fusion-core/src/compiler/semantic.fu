#![allow(missing_docs)]
#[allow(missing_docs, dead_code)]
type FString = FString;
#[allow(missing_docs, dead_code)]
type FMap<K, V> = FMap<K, V>;
use crate::compiler::ast::Type;
#[derive(Debug, Clone)]
struct SymbolInfo {
    pub name: FString,
    pub typ: Type,
}
struct Scope {
    pub symbols: FMap<FString, SymbolInfo>,
    pub parent: Option<Box<Scope>>,
}
impl Scope {
    pub fn new(parent: Option<Box<Scope>>) -> Self {
        Scope {
            symbols: HashMap::new(),
            parent,
        }
    }
    pub fn insert(&mut self, name: FString, typ: Type) -> Result<(), FString> {
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
