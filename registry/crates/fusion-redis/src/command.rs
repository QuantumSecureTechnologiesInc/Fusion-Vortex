use crate::resp::Value;
use crate::store::Store;
use std::sync::Arc;

pub struct CommandHandler {
    store: Arc<Store>,
}

impl CommandHandler {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }

    pub fn handle(&self, value: Value) -> Value {
        let args = match value {
            Value::Array(Some(v)) => v,
            _ => return Value::Error("ERR request must be an array".to_string()),
        };

        if args.is_empty() {
            return Value::Error("ERR empty command".to_string());
        }

        let cmd_name = match &args[0] {
            Value::BulkString(Some(b)) => String::from_utf8_lossy(b).to_string().to_uppercase(),
            _ => return Value::Error("ERR command name must be a bulk string".to_string()),
        };

        match cmd_name.as_str() {
            "PING" => Value::SimpleString("PONG".to_string()),
            "ECHO" => {
                if args.len() < 2 {
                    Value::Error("ERR wrong number of arguments for 'echo' command".to_string())
                } else {
                    args[1].clone()
                }
            }
            "SET" => self.handle_set(&args),
            "GET" => self.handle_get(&args),
            "DEL" => self.handle_del(&args),
            "EXISTS" => self.handle_exists(&args),
            _ => Value::Error(format!("ERR unknown command '{}'", cmd_name)),
        }
    }

    fn handle_set(&self, args: &[Value]) -> Value {
        if args.len() < 3 {
            return Value::Error("ERR wrong number of arguments for 'set' command".to_string());
        }

        let key = match &args[1] {
            Value::BulkString(Some(b)) => String::from_utf8_lossy(b).to_string(),
            _ => return Value::Error("ERR key must be a bulk string".to_string()),
        };

        let value = match &args[2] {
            Value::BulkString(Some(b)) => b.clone(),
            _ => return Value::Error("ERR value must be a bulk string".to_string()),
        };

        self.store.set(key, value, None);
        Value::SimpleString("OK".to_string())
    }

    fn handle_get(&self, args: &[Value]) -> Value {
        if args.len() != 2 {
            return Value::Error("ERR wrong number of arguments for 'get' command".to_string());
        }

        let key = match &args[1] {
            Value::BulkString(Some(b)) => String::from_utf8_lossy(b).to_string(),
            _ => return Value::Error("ERR key must be a bulk string".to_string()),
        };

        match self.store.get(&key) {
            Some(v) => Value::BulkString(Some(v)),
            None => Value::BulkString(None),
        }
    }

    fn handle_del(&self, args: &[Value]) -> Value {
        if args.len() < 2 {
            return Value::Error("ERR wrong number of arguments for 'del' command".to_string());
        }

        let mut count = 0;
        for arg in &args[1..] {
            let key = match arg {
                Value::BulkString(Some(b)) => String::from_utf8_lossy(b).to_string(),
                _ => continue,
            };
            if self.store.del(&key) {
                count += 1;
            }
        }
        Value::Integer(count)
    }

    fn handle_exists(&self, args: &[Value]) -> Value {
        if args.len() < 2 {
            return Value::Error("ERR wrong number of arguments for 'exists' command".to_string());
        }

        let mut count = 0;
        for arg in &args[1..] {
            let key = match arg {
                Value::BulkString(Some(b)) => String::from_utf8_lossy(b).to_string(),
                _ => continue,
            };
            if self.store.exists(&key) {
                count += 1;
            }
        }
        Value::Integer(count)
    }
}
