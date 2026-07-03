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
#[cfg(test)]
mod tests {
    use crate::{lex, parse_program, type_check_program};

    #[test]
    fn typecheck_simple_arith() {
        let src = r#"
            fn main() -> int {
                let x = 1 + 2
                return x
            }
        "#;
        let toks = lex(src);
        let prog = parse_program(&toks).unwrap();
        let info = type_check_program(&prog).unwrap();
        assert!(info.functions.contains_key("main"));
    }

    #[test]
    fn parse_use_and_mod() {
        let src = r#"
            use foo::bar;
            mod baz;
            fn main() -> int { return 0 }
        "#;
        let toks = lex(src);
        let prog = parse_program(&toks).unwrap();
        assert!(prog.items.len() >= 3);
    }
}
