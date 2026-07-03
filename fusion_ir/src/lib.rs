// __FU_COMPAT_START__
use std::fs;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

type FBool = bool;
type FChar = char;
type FInt = i32;
type FI64 = i64;
type FString = String;
type FU32 = u32;
type FU64 = u64;
type FSize = usize;
type FVec<T> = Vec<T>;
type FMap<K, V> = HashMap<K, V>;
type FBTreeMap<K, V> = BTreeMap<K, V>;
type FSet<T> = HashSet<T>;
type FBTreeSet<T> = BTreeSet<T>;
// __FU_COMPAT_END__
use serde::{Deserialize, Serialize};
mod bytecode;
mod span;
pub use bytecode::*;
pub use span::*;
const BYTECODE_MAGIC: &[u8; 8] = b"FUSIONBC";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub functions: FVec<Function>,
    pub entry: Option<FunctionId>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionId(pub FU32);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub id: FunctionId,
    pub name: FString,
    pub arity: u8,
    pub code: FVec<Op>,
    pub consts: FVec<ValueConst>,
    pub span: Span,
    pub effects: Effects,
}
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Effects {
    pub borrowed: FBool,
    pub constant_time: FBool,
    pub gpu_accelerated: FBool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueConst {
    Int(FI64),
    Float(f64),
    Bool(FBool),
    Str(FString),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytecodeFile {
    pub version: u16,
    pub module: Module,
}
pub fn encode(file: &BytecodeFile) -> Result<FVec<u8>, bincode::Error> {
    let mut out = Vec::new();
    out.extend_from_slice(BYTECODE_MAGIC);
    out.extend(bincode::serialize(file)?);
    Ok(out)
}
pub fn decode(bytes: &[u8]) -> Result<BytecodeFile, DecodeError> {
    if bytes.len() < BYTECODE_MAGIC.len()
        || &bytes[..BYTECODE_MAGIC.len()] != BYTECODE_MAGIC
    {
        return Err(DecodeError::BadMagic);
    }
    let payload = &bytes[BYTECODE_MAGIC.len()..];
    let file: BytecodeFile = bincode::deserialize(payload)
        .map_err(DecodeError::Bincode)?;
    Ok(file)
}
#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("invalid bytecode magic header")]
    BadMagic,
    #[error("decode error: {0}")]
    Bincode(#[from] bincode::Error),
}
