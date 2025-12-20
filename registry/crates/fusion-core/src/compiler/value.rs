use core::fmt;

#[derive(Clone)]
pub struct NativeFunction {
    pub name: String,
    pub func: fn(&[Value]) -> Value,
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name // Close enough for now, or check func ptr
    }
}

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn {}>", self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Bool(bool),
    String(String),
    Function(Box<crate::compiler::function::Function>),
    Native(NativeFunction),
    Struct(std::rc::Rc<std::cell::RefCell<std::collections::HashMap<String, Value>>>),
    Void,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{}", v),
            Value::Bool(v) => write!(f, "{}", v),
            Value::String(s) => write!(f, "{}", s),
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
