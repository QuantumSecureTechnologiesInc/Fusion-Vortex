use crate::compiler::chunk::OpCode;
use crate::compiler::function::Function;
use crate::compiler::value::Value;
use std::collections::HashMap;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError(String),
}

#[derive(Clone)]
struct CallFrame {
    function: Function,
    ip: usize,
    base_pointer: usize,
}

pub struct VM {
    frames: Vec<CallFrame>,
    stack: Vec<Value>,
    pub globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        let mut globals = HashMap::new();

        // Define Natives
        globals.insert(
            "print".to_string(),
            Value::Native(crate::compiler::value::NativeFunction {
                name: "print".to_string(),
                func: |args| {
                    for arg in args {
                        println!("{}", arg);
                    }
                    Value::Void
                },
            }),
        );

        globals.insert(
            "clock".to_string(),
            Value::Native(crate::compiler::value::NativeFunction {
                name: "clock".to_string(),
                func: |_| {
                    use std::time::{SystemTime, UNIX_EPOCH};
                    let start = SystemTime::now();
                    let since_the_epoch = start
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");
                    Value::Int(since_the_epoch.as_secs() as i64)
                },
            }),
        );

        VM {
            frames: Vec::new(),
            stack: Vec::new(),
            globals,
        }
    }

    pub fn interpret(&mut self, script_function: Function) -> InterpretResult {
        self.stack.clear();
        self.frames.clear();

        // Push script function
        self.push(Value::Function(Box::new(script_function.clone())));
        // Call it (Script has arity 0)
        self.call(script_function, 0);

        self.run()
    }

    fn call(&mut self, function: Function, arg_count: usize) -> bool {
        if arg_count != function.arity {
            return false;
        }

        let frame = CallFrame {
            function,
            ip: 0,
            base_pointer: self.stack.len() - arg_count - 1,
        };
        self.frames.push(frame);
        true
    }

    fn run(&mut self) -> InterpretResult {
        let mut frame = self.frames.pop().unwrap();

        loop {
            if frame.ip >= frame.function.chunk.code.len() {
                if let Some(prev) = self.frames.pop() {
                    frame = prev;
                    continue;
                } else {
                    return InterpretResult::Ok;
                }
            }

            let instruction = frame.function.chunk.code[frame.ip];
            // println!("IP: {}, Inst: {:?}, Globals: {:?}", frame.ip, instruction, self.globals.keys());
            frame.ip += 1;

            match instruction {
                OpCode::Constant(idx) => {
                    let constant = frame.function.chunk.constants[idx as usize].clone();
                    self.push(constant);
                }
                OpCode::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    match (a, b) {
                        (Value::Int(v1), Value::Int(v2)) => self.push(Value::Int(v1 + v2)),
                        (Value::String(s1), Value::String(s2)) => {
                            self.push(Value::String(s1 + &s2))
                        }
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Operands must be numbers or strings".to_string(),
                            );
                        }
                    }
                }
                OpCode::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    match (a, b) {
                        (Value::Int(v1), Value::Int(v2)) => self.push(Value::Int(v1 - v2)),
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Operands must be numbers".to_string(),
                            );
                        }
                    }
                }
                OpCode::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    match (a, b) {
                        (Value::Int(v1), Value::Int(v2)) => self.push(Value::Int(v1 * v2)),
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Operands must be numbers".to_string(),
                            );
                        }
                    }
                }
                OpCode::Div => {
                    let b = self.pop();
                    let a = self.pop();
                    match (a, b) {
                        (Value::Int(v1), Value::Int(v2)) => {
                            if v2 == 0 {
                                return InterpretResult::RuntimeError(
                                    "Division by zero".to_string(),
                                );
                            }
                            self.push(Value::Int(v1 / v2));
                        }
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Operands must be numbers".to_string(),
                            );
                        }
                    }
                }
                OpCode::GreaterThan => {
                    let b = self.pop();
                    let a = self.pop();
                    match (a, b) {
                        (Value::Int(v1), Value::Int(v2)) => self.push(Value::Bool(v1 > v2)),
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Operands must be numbers".to_string(),
                            );
                        }
                    }
                }
                OpCode::LessThan => {
                    let b = self.pop();
                    let a = self.pop();
                    match (a, b) {
                        (Value::Int(v1), Value::Int(v2)) => self.push(Value::Bool(v1 < v2)),
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Operands must be numbers".to_string(),
                            );
                        }
                    }
                }
                OpCode::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Bool(a == b));
                }
                OpCode::NotEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Bool(a != b));
                }
                OpCode::Not => {
                    let a = self.pop();
                    match a {
                        Value::Bool(b) => self.push(Value::Bool(!b)),
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Operand must be boolean".to_string(),
                            );
                        }
                    }
                }
                OpCode::Pop => {
                    self.pop();
                }
                OpCode::GetLocal(slot) => {
                    let effective_idx = frame.base_pointer + 1 + slot as usize;
                    let val = self.stack[effective_idx].clone();
                    self.push(val);
                }
                OpCode::SetLocal(slot) => {
                    let val = self.peek(0).clone();
                    let effective_idx = frame.base_pointer + 1 + slot as usize;
                    self.stack[effective_idx] = val;
                }
                OpCode::GetGlobal(name_idx) => {
                    let name_val = frame.function.chunk.constants[name_idx as usize].clone();
                    let name = match name_val {
                        Value::String(s) => s,
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Global name must be string".into(),
                            )
                        }
                    };

                    if let Some(val) = self.globals.get(&name) {
                        self.push(val.clone());
                    } else {
                        return InterpretResult::RuntimeError(format!(
                            "Undefined global '{}'",
                            name
                        ));
                    }
                }
                OpCode::DefineGlobal(name_idx) => {
                    let name_val = frame.function.chunk.constants[name_idx as usize].clone();
                    let name = match name_val {
                        Value::String(s) => s,
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Global name must be string".into(),
                            )
                        }
                    };
                    let val = self.pop(); // Pop value to store
                    self.globals.insert(name, val);
                }
                OpCode::SetGlobal(name_idx) => {
                    let name_val = frame.function.chunk.constants[name_idx as usize].clone();
                    let name = match name_val {
                        Value::String(s) => s,
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Global name must be string".into(),
                            )
                        }
                    };

                    let val = self.peek(0).clone();
                    if self.globals.contains_key(&name) {
                        self.globals.insert(name, val);
                    } else {
                        return InterpretResult::RuntimeError(format!(
                            "Undefined global '{}'",
                            name
                        ));
                    }
                }
                OpCode::JumpIfFalse(offset) => {
                    let condition = self.peek(0);
                    match condition {
                        Value::Bool(b) => {
                            if !b {
                                frame.ip += offset as usize;
                            }
                        }
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Condition must be boolean".to_string(),
                            );
                        }
                    }
                }
                OpCode::Jump(offset) => {
                    frame.ip += offset as usize;
                }
                OpCode::Loop(offset) => {
                    frame.ip -= offset as usize;
                }
                OpCode::MakeStruct(count) => {
                    let mut fields = HashMap::new();
                    // Pops N * 2 items.
                    for _ in 0..count {
                        let val = self.pop();
                        let key = self.pop();
                        match key {
                            Value::String(k) => {
                                fields.insert(k, val);
                            }
                            _ => {
                                return InterpretResult::RuntimeError(
                                    "Struct keys must be strings".to_string(),
                                );
                            }
                        }
                    }
                    let instance = std::rc::Rc::new(std::cell::RefCell::new(fields));
                    self.push(Value::Struct(instance));
                }
                OpCode::GetProp(idx) => {
                    let name_val = frame.function.chunk.constants[idx as usize].clone();
                    let name = match name_val {
                        Value::String(s) => s,
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Property name must be string".to_string(),
                            );
                        }
                    };

                    let obj = self.pop();
                    match obj {
                        Value::Struct(instance) => {
                            let fields = instance.borrow();
                            if let Some(val) = fields.get(&name) {
                                self.push(val.clone());
                            } else {
                                return InterpretResult::RuntimeError(format!(
                                    "Property '{}' not found",
                                    name
                                ));
                            }
                        }
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Get property on non-struct".to_string(),
                            );
                        }
                    }
                }
                OpCode::SetProp(idx) => {
                    let name_val = frame.function.chunk.constants[idx as usize].clone();
                    let name = match name_val {
                        Value::String(s) => s,
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Property name must be string".to_string(),
                            );
                        }
                    };

                    let val = self.pop();
                    let obj = self.pop();

                    match obj {
                        Value::Struct(instance) => {
                            let mut fields = instance.borrow_mut();
                            fields.insert(name, val.clone());
                            self.push(val);
                        }
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Set property on non-struct".to_string(),
                            );
                        }
                    }
                }

                OpCode::Call(arg_count) => {
                    let callee = self.peek(arg_count as usize).clone();
                    match callee {
                        Value::Function(fun) => {
                            self.frames.push(frame.clone());

                            let ok = self.call(*fun, arg_count as usize);
                            if !ok {
                                return InterpretResult::RuntimeError("Arity mismatch".to_string());
                            }

                            frame = self.frames.pop().unwrap();
                        }
                        Value::Native(nat) => {
                            // Collect args
                            let arg_start = self.stack.len() - arg_count as usize;
                            let args = self.stack.drain(arg_start..).collect::<Vec<_>>();
                            self.pop(); // Pop the function

                            let result = (nat.func)(&args);
                            self.push(result);
                        }
                        _ => {
                            return InterpretResult::RuntimeError(
                                "Can only call functions".to_string(),
                            );
                        }
                    }
                }
                OpCode::Return => {
                    let result = self.pop();

                    let slots_to_pop = self.stack.len() - frame.base_pointer;
                    for _ in 0..slots_to_pop {
                        self.pop();
                    }

                    self.push(result);

                    if let Some(prev) = self.frames.pop() {
                        frame = prev;
                    } else {
                        return InterpretResult::Ok;
                    }
                }
            }
        }
    }

    fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack underflow")
    }

    fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.stack.len() - 1 - distance]
    }

    pub fn last_popped(&self) -> Option<Value> {
        if self.stack.len() > 0 {
            Some(self.stack.last().unwrap().clone())
        } else {
            None
        }
    }
}
