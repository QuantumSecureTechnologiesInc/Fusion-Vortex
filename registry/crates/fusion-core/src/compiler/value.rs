#![allow(missing_docs)]
#[allow(missing_docs, dead_code)]
type FBool = FBool;
#[allow(missing_docs, dead_code)]
type FI64 = FI64;
#[allow(missing_docs, dead_code)]
type FString = FString;
use core::fmt;
#[derive(Clone)]
struct NativeFunction {
    pub name: FString,
    pub func: fn(&[Value]) -> Value,
}
impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> FBool {
        self.name == other.name
    }
}
impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn {}>", self.name)
    }
}
#[derive(Debug, Clone, PartialEq)]
enum Value {
    Int(FI64),
    Bool(FBool),
    String(FString),
    Function(Box<crate::compiler::function::Function>),
    Native(NativeFunction),
    Struct(std::rc::Rc<std::cell::RefCell<FMap<FString, Value>>>),
    Void,
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{}", v),
            Value::Bool(v) => write!(f, "{}", v),
            Value::FString(s) => write!(f, "{}", s),
            Value::Function(fun) => write!(f, "{}", fun),
            Value::Native(nat) => write!(f, "<native fn {}>", nat.name),
            Value::Struct(instance) => {
                let borrowed = instance.borrow();
                write!(f, "StructInstance({:?})", borrowed)
            }
            Value::Void => write!(f, "void"),
        }
    }
}
