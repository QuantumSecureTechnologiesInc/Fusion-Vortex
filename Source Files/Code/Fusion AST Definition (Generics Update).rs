// src/ast/mod.rs - Abstract Syntax Tree Definitions (Updated for Generics and Traits)

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
    // ... (other declarations remain the same)
}

#[derive(Debug, Clone)]
pub struct TraitBound {
    pub type_name: String,  // The generic type being constrained (e.g., "T")
    pub trait_name: String, // The required trait (e.g., "Serializable")
}

// ... (Parameter, Field, MethodSignature, Block, Statement, Expression, Literal, BinaryOp, UnaryOp enums remain the same) ...

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
