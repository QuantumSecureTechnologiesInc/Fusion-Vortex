use crate::compiler::chunk::Chunk;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub arity: usize,
    pub chunk: Chunk,
    pub name: String,
}

impl Function {
    pub fn new(name: String, arity: usize) -> Self {
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
