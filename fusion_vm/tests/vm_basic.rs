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
use fusion_ir::{
    BytecodeFile, Function, FunctionId, Module, Op, ValueConst, Effects, Span,
};
use fusion_vm::Vm;
#[test]
fn vm_runs_simple_add() {
    let f = Function {
        id: FunctionId(0),
        name: "main".into(),
        arity: 0,
        consts: vec![ValueConst::Int(40), ValueConst::Int(2)],
        code: vec![Op::Const(0), Op::Const(1), Op::Add, Op::Ret],
        span: Span::default(),
        effects: Effects::default(),
    };
    let file = BytecodeFile {
        version: 1,
        module: Module {
            functions: vec![f],
            entry: Some(FunctionId(0)),
        },
    };
    let mut vm = Vm::new(file);
    let out = vm.run().unwrap();
    assert_eq!(out, 42);
}
