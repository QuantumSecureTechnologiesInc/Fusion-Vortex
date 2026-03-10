#![allow(missing_docs)]
#[allow(missing_docs, dead_code)]
type FBool = FBool;
#[allow(missing_docs, dead_code)]
type FI64 = FI64;
#[allow(missing_docs, dead_code)]
type FString = FString;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum ClassicalType {
    Int(FI64),
    Float(f64),
    Boolean(FBool),
    String(FString),
}
