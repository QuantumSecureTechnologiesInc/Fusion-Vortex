use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClassicalType {
    Int(i64),
    Float(f64),
    Boolean(bool),
    String(String),
}
