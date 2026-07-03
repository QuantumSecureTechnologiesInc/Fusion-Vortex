//! Fusion Intermediate Representation (IR)
//! Unified IR types for the optimizer, codegen, and borrow checker.


// ---- Loan analysis (borrow checker) ----

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoanKind {
    Immutable,
    Mutable,
}

// ---- Core IR types ----

pub type BlockId = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    IntConst(i64),
    BoolConst(bool),
    StringConst(String),
    FloatConst(f64),
    Variable(String),
    Temp(usize),
}

impl Eq for Value {}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Value::IntConst(v) => v.hash(state),
            Value::BoolConst(v) => v.hash(state),
            Value::StringConst(v) => v.hash(state),
            Value::FloatConst(v) => v.to_bits().hash(state),
            Value::Variable(v) => v.hash(state),
            Value::Temp(v) => v.hash(state),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypedValue {
    pub val: Value,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum Address {
    Variable { name: String, ty: Type },
    Pointer { val: TypedValue, pointed_to_ty: Type },
    Element { base: Box<Address>, index: TypedValue, element_ty: Type },
    Field { base: Box<Address>, field_index: usize, field_ty: Type, struct_name: String },
}

#[derive(Debug, Clone)]
pub enum Instruction {
    BinaryOperation {
        dest: TypedValue,
        op: BinaryOp,
        op1: TypedValue,
        op2: TypedValue,
    },
    Call {
        dest: Option<TypedValue>,
        func_name: String,
        args: Vec<TypedValue>,
    },
    Load {
        dest: TypedValue,
        src: Address,
    },
    Store {
        dest: Address,
        val: TypedValue,
    },
    GetElementPtr {
        dest: TypedValue,
        base_ptr: TypedValue,
        index: TypedValue,
        element_ty: Type,
    },
    GetFieldPtr {
        dest: TypedValue,
        base_ptr: TypedValue,
        field_index: usize,
        field_ty: Type,
        struct_name: String,
    },
    // Loan-related instructions (from middle-end IR)
    GetAddress {
        dest: String,
        var_name: String,
        is_mutable: bool,
    },
    // Stack allocation
    Alloca {
        dest: TypedValue,
        ty: Type,
    },
    // Value copy (for move/copy semantics)
    Copy {
        dest: TypedValue,
        src: TypedValue,
    },
    // Unary logical not
    UnaryNot {
        dest: TypedValue,
        operand: TypedValue,
    },
    // Phi node (SSA merge point)
    Phi {
        dest: TypedValue,
        incoming: Vec<(TypedValue, usize)>,
    },
    // Closure creation
    MakeClosure {
        dest: TypedValue,
        func_name: String,
        captured: Vec<TypedValue>,
    },
    // Debug/comment (no-op)
    Comment(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Neq, Lt, Gt, Le, Ge,
    And, Or,
}

// Terminator tag constants
pub const TERM_JUMP: u8 = 1;
pub const TERM_COND_JUMP: u8 = 2;
pub const TERM_RETURN: u8 = 3;
pub const TERM_UNREACHABLE: u8 = 4;

#[derive(Debug, Clone)]
pub enum Terminator {
    Jump(usize),
    ConditionalJump {
        cond: TypedValue,
        then_block: usize,
        else_block: usize,
    },
    Return(Option<TypedValue>),
    Unreachable,
}

impl Terminator {
    pub fn tag(&self) -> u8 {
        match self {
            Terminator::Jump(_) => TERM_JUMP,
            Terminator::ConditionalJump { .. } => TERM_COND_JUMP,
            Terminator::Return(_) => TERM_RETURN,
            Terminator::Unreachable => TERM_UNREACHABLE,
        }
    }
    pub fn jump(target: usize) -> Self {
        Terminator::Jump(target)
    }
    pub fn conditional(cond: TypedValue, then_block: usize, else_block: usize) -> Self {
        Terminator::ConditionalJump { cond, then_block, else_block }
    }
    pub fn return_(val: Option<TypedValue>) -> Self {
        Terminator::Return(val)
    }
    pub fn unreachable() -> Self {
        Terminator::Unreachable
    }
}

pub fn return_terminator(val: Option<TypedValue>) -> Terminator {
    Terminator::return_(val)
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub label: String,
    pub instrs: Vec<Instruction>,
    pub terminator: Terminator,
}

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub blocks: Vec<BasicBlock>,
    pub entry_block: usize,
}

// ---- Module-level definitions ----

#[derive(Debug, Clone)]
pub struct IrStructDef {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

#[derive(Debug, Clone)]
pub struct IrEnumDef {
    pub name: String,
    pub variants: Vec<(String, Option<Type>)>,
}

#[derive(Debug, Clone)]
pub struct IrGlobalString {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct IrExtern {
    pub name: String,
    pub params: Vec<Type>,
    pub return_type: Type,
}

#[derive(Debug, Clone)]
pub struct IrModule {
    pub functions: Vec<IrFunction>,
    pub externs: Vec<IrExtern>,
    pub structs: Vec<IrStructDef>,
    pub enums: Vec<IrEnumDef>,
    pub global_strings: Vec<IrGlobalString>,
}

impl IrModule {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            externs: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            global_strings: Vec::new(),
        }
    }
}

// ---- Type definitions (delegated to ast) ----

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Bool,
    String,
    Void,
    Float,
    Struct(String),
    Pointer(Box<Type>),
    Array(Box<Type>, usize),
    Slice(Box<Type>),
    GenericParam(String),
    Closure(Vec<Type>, Box<Type>),
    Optional(Box<Type>),
    Union(Vec<Type>),
    GenericInstance(String, Vec<Type>),
    Unknown,
}