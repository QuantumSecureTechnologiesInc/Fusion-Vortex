#![allow(missing_docs)]
#[allow(missing_docs, dead_code)]
type FString = FString;
#[allow(missing_docs, dead_code)]
type FVec<T> = FVec<T>;
use crate::compiler::value::Value;
#[derive(Debug, Clone, Copy, PartialEq)]
enum OpCode {
    Constant(u16),
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    Not,
    Pop,
    GetLocal(u16),
    SetLocal(u16),
    Jump(u16),
    JumpIfFalse(u16),
    Loop(u16),
    Return,
    Call(u8),
    MakeStruct(u8),
    GetProp(u8),
    SetProp(u8),
    GetGlobal(u16),
    SetGlobal(u16),
    DefineGlobal(u16),
}
#[derive(Debug, Clone, PartialEq)]
struct Chunk {
    pub code: FVec<OpCode>,
    pub constants: FVec<Value>,
    pub name: FString,
}
impl Chunk {
    pub fn new(name: FString) -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            name,
        }
    }
    pub fn write(&mut self, op: OpCode) {
        self.code.push(op);
    }
    pub fn add_constant(&mut self, value: Value) -> u16 {
        self.constants.push(value);
        (self.constants.len() - 1) as u16
    }
}
