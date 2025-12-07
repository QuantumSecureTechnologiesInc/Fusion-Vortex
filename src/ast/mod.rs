// src/ast/mod.rs - Abstract Syntax Tree Definitions (Updated for Generics and Traits)

#[allow(dead_code)] // Suppresses warnings for fields/variants not yet used in the compiler
#[derive(Debug, Clone)]
pub enum Declaration {
    Function {
        name: String,
        attributes: Vec<Attribute>,
        generic_params: Vec<String>, // New: Generic Type Parameters (e.g., T, U)
        where_bounds: Vec<TraitBound>, // New: Trait Constraints
        params: Vec<Parameter>,
        return_type: Type,
        body: Block,
    },
    Class {
        name: String,
        generic_params: Vec<String>, // New: Generic Type Parameters for the class itself
        implements: Vec<String>,
        fields: Vec<Field>,
        methods: Vec<Declaration>,
    },
    Trait {
        name: String,
        methods: Vec<MethodSignature>,
    },
    GlobalVariable {
        name: String,
        var_type: Option<Type>,
        initializer: Expression,
    },
    ExternFunction {
        name: String,
        params: Vec<Parameter>,
        return_type: Type,
    },
    Module {
        name: String,
        declarations: Vec<Declaration>,
    },
    ModuleDecl {
        name: String,
        is_public: bool,
    },
    UseDecl {
        path: Vec<String>,     // e.g., ["lib", "utils"]
        alias: Option<String>, // Optional rename
        import_all: bool,      // true for "use mod::*"
    },
}

#[derive(Debug, Clone)]
pub struct TraitBound {
    pub type_name: String,  // The generic type being constrained (e.g., "T")
    pub trait_name: String, // The required trait (e.g., "Serializable")
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: Type,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        mutable: bool, // New: tracks if variable is mutable
        var_type: Option<Type>,
        initializer: Expression,
    },
    Assignment {
        target: Expression,
        value: Expression,
    },
    If {
        condition: Expression,
        then_block: Block,
        else_block: Option<Block>,
    },
    While {
        condition: Expression,
        body: Block,
    },
    For {
        variable: String,
        iterator: Expression,
        body: Block,
    },
    Return(Option<Expression>),
    Expression(Expression),
    Break,
    Continue,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expression>,
    },
    FunctionCall {
        name: String,
        generic_args: Vec<Type>, // New: Explicit generic arguments, e.g. foo<int>(...)
        args: Vec<Expression>,
    },
    MethodCall {
        object: Box<Expression>,
        method: String,
        args: Vec<Expression>,
    },
    FieldAccess {
        object: Box<Expression>,
        field: String,
    },
    StructInit {
        name: String,
        generic_args: Vec<Type>, // New
        fields: Vec<(String, Expression)>,
    },
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    Array(Vec<Expression>),
    Map(Vec<(Expression, Expression)>),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    LogicalAnd,
    LogicalOr,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unknown,
    Void,
    Integer,
    Float,
    String,
    Boolean,
    Custom(String),        // Named types (e.g., DataProcessor)
    TypeParameter(String), // New: Represents an unresolved generic type (e.g., 'T')
    Array(Box<Type>),
    Optional(Box<Type>),
    Union(Vec<Type>),
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    // New: A concrete type instantiated with generic arguments (e.g., List<int>)
    GenericInstance {
        base_name: String, // "List"
        args: Vec<Type>,   // [Integer]
    },
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Attribute {
    HardwareAccelerated(String),
    // Add other attributes as needed
}
