use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HybridValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<HybridValue>),
    Object(std::collections::HashMap<String, HybridValue>),
}

impl Default for HybridValue {
    fn default() -> Self {
        Self::Null
    }
}
