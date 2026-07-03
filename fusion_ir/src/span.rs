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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Span {
    pub start: FSize,
    pub end: FSize,
    pub line: FU32,
    pub col: FU32,
}
impl Span {
    pub fn new(start: FSize, end: FSize, line: FU32, col: FU32) -> Self {
        Self { start, end, line, col }
    }
}
