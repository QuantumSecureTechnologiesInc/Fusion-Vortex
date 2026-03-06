#![allow(missing_docs)]
#[allow(missing_docs, dead_code)]
type FBool = FBool;
#[allow(missing_docs, dead_code)]
type FI64 = FI64;
#[allow(missing_docs, dead_code)]
type FString = FString;
#[allow(missing_docs, dead_code)]
type FVec<T> = FVec<T>;
#[derive(Debug, PartialEq, Clone)]
enum Type {
    Int,
    Bool,
    String,
    Void,
    Custom(FString),
}
#[derive(Debug, PartialEq, Clone)]
struct Program {
    pub declarations: FVec<Declaration>,
}
#[derive(Debug, PartialEq, Clone)]
enum Declaration {
    Function(FunctionDecl),
    Extern(ExternDecl),
    Struct(StructDecl),
}
#[derive(Debug, PartialEq, Clone)]
struct FunctionDecl {
    pub name: FString,
    pub params: FVec<(FString, Type)>,
    pub return_type: Type,
    pub body: FVec<Statement>,
}
#[derive(Debug, PartialEq, Clone)]
struct ExternDecl {
    pub name: FString,
    pub params: FVec<(FString, Type)>,
    pub return_type: Type,
}
#[derive(Debug, PartialEq, Clone)]
struct StructDecl {
    pub name: FString,
    pub fields: FVec<(FString, Type)>,
}
#[derive(Debug, PartialEq, Clone)]
enum Statement {
    Let(FString, Option<Type>, Expression),
    Return(Option<Expression>),
    If(Expression, FVec<Statement>, Option<FVec<Statement>>),
    While(Expression, FVec<Statement>),
    Expression(Expression),
    Block(FVec<Statement>),
}
#[derive(Debug, PartialEq, Clone)]
enum Expression {
    Binary(Box<Expression>, BinaryOp, Box<Expression>),
    Call(FString, FVec<Expression>),
    Literal(Literal),
    Identifier(FString),
    Assign(FString, Box<Expression>),
    StructInit(FString, FVec<(FString, Expression)>),
    Get(Box<Expression>, FString),
    Set(Box<Expression>, FString, Box<Expression>),
}
#[derive(Debug, PartialEq, Clone)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
}
#[derive(Debug, PartialEq, Clone)]
enum Literal {
    Integer(FI64),
    String(FString),
    Bool(FBool),
}
