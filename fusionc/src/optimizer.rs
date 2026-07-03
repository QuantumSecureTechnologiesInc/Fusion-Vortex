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
use fusion_ir::{Op, ValueConst};

fn optimize(code: &mut FVec<Op>, consts: &mut FVec<ValueConst>) {
    peephole_const_folding(code, consts);
    dead_code_after_ret(code);
}

fn dead_code_after_ret(code: &mut FVec<Op>) {
    // remove any ops after the first Ret in each straight-line segment.
    // With our simple backend, code is single function and retains structured jumps,
    // so we only remove trailing ops after a Ret at the end.
    if let Some(pos) = code.iter().rposition(|op| matches!(op, Op::Ret)) {
        code.truncate(pos + 1);
    }
}

fn peephole_const_folding(code: &mut FVec<Op>, consts: &mut FVec<ValueConst>) {
    // pattern: Const(a), Const(b), Add/Sub/Mul/Div => Const(result)
    let mut i = 0usize;
    while i + 2 < code.len() {
        let (a_ix, b_ix) = match (code[i], code[i + 1]) {
            (Op::Const(a), Op::Const(b)) => (a as usize, b as usize),
            _ => { i += 1; continue; }
        };
        let op = code[i + 2];
        let res = match (consts.get(a_ix), consts.get(b_ix), op) {
            (Some(ValueConst::Int(a)), Some(ValueConst::Int(b)), Op::Add) => Some(ValueConst::Int(a + b)),
            (Some(ValueConst::Int(a)), Some(ValueConst::Int(b)), Op::Sub) => Some(ValueConst::Int(a - b)),
            (Some(ValueConst::Int(a)), Some(ValueConst::Int(b)), Op::Mul) => Some(ValueConst::Int(a * b)),
            (Some(ValueConst::Int(a)), Some(ValueConst::Int(b)), Op::Div) if *b != 0 => Some(ValueConst::Int(a / b)),
            (Some(ValueConst::Float(a)), Some(ValueConst::Float(b)), Op::Add) => Some(ValueConst::Float(a + b)),
            (Some(ValueConst::Float(a)), Some(ValueConst::Float(b)), Op::Sub) => Some(ValueConst::Float(a - b)),
            (Some(ValueConst::Float(a)), Some(ValueConst::Float(b)), Op::Mul) => Some(ValueConst::Float(a * b)),
            (Some(ValueConst::Float(a)), Some(ValueConst::Float(b)), Op::Div) if *b != 0.0 => Some(ValueConst::Float(a / b)),
            _ => None,
        };
        if let Some(c) = res {
            let new_ix = intern_const(consts, c) as u32;
            code.splice(i..i+3, [Op::Const(new_ix)]);
            // don't advance; allow chaining
        } else {
            i += 1;
        }
    }
}

fn intern_const(consts: &mut FVec<ValueConst>, c: ValueConst) -> usize {
    if let Some(pos) = consts.iter().position(|x| const_eq(x, &c)) {
        pos
    } else {
        consts.push(c);
        consts.len() - 1
    }
}

fn const_eq(a: &ValueConst, b: &ValueConst) -> FBool {
    match (a, b) {
        (ValueConst::Int(x), ValueConst::Int(y)) => x == y,
        (ValueConst::Float(x), ValueConst::Float(y)) => x.to_bits() == y.to_bits(),
        (ValueConst::Bool(x), ValueConst::Bool(y)) => x == y,
        (ValueConst::Str(x), ValueConst::Str(y)) => x == y,
        _ => false,
    }
}
