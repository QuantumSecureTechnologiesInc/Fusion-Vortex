#![allow(missing_docs)]
#[allow(missing_docs, dead_code)]
type FString = FString;
#[allow(missing_docs, dead_code)]
type FSize = FSize;
use crate::compiler::chunk::Chunk;
#[derive(Debug, Clone, PartialEq)]
struct Function {
    pub arity: FSize,
    pub chunk: Chunk,
    pub name: FString,
}
impl Function {
    pub fn new(name: FString, arity: FSize) -> Self {
        Function {
            arity,
            chunk: Chunk::new(name.clone()),
            name,
        }
    }
}
impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}
