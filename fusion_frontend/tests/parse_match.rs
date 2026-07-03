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
use fusion_frontend::{lex, parse_program, Item, Stmt};
#[test]
fn parses_match_statement() {
    let src = r#"
        fn main() -> int {
            let x = 2
            match x { 0 => println("zero"), _ => println("other") }
            return 0
        }
    "#;
    let toks = lex(src);
    let prog = parse_program(&toks).unwrap();
    let Item::Function(f) = &prog.items[0] else {
        panic!("expected function");
    };
    let mut found = false;
    for s in &f.body.stmts {
        if matches!(s, Stmt::Match { .. }) {
            found = true;
        }
    }
    assert!(found, "match stmt not found");
}
