use crate::value::Value;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    Constant(u16), // Index into constant pool

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // Comparison
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,

    // Logic
    Not,

    // Stack / Variables
    Pop,
    GetLocal(u16), // Stack slot index
    SetLocal(u16),

    // Control Flow
    Jump(u16),        // Unconditional jump forward
    JumpIfFalse(u16), // Jump forward if stack top is false
    Loop(u16),        // Unconditional jump backward

    // Functions
    Return,
    Call(u8), // Arg count

    // Structs
    MakeStruct(u8), // Pops N * 2 items (Key, Value pairs)
    GetProp(u8),    // Operand is Constant Index (Name). Pops Struct.
    SetProp(u8),    // Operand is Constant Index (Name). Bottom-Up: [Struct, Value].
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub name: String,
}

impl Chunk {
    pub fn new(name: String) -> Self {
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
