//! Fusion type aliases for the Rust-to-Fusion bridge.
//! These types mirror the Fusion standard library types used in aspirational code.

use std::collections::{HashMap, HashSet};

pub type FString = String;
pub type FBool = bool;
pub type FSize = usize;
pub type FI64 = i64;
pub type FVec<T> = Vec<T>;
pub type FMap<K, V> = HashMap<K, V>;
pub type FSet<T> = HashSet<T>;