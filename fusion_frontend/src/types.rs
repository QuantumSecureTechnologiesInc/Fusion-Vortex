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
use crate::ast::*;
use fusion_ir::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    Unit,
    Unknown,
}

#[derive(thiserror::Error, Debug)]
pub enum TypeError {
    #[error("{span:?}: {msg}")]
    Msg { span: Span, msg: FString },
}

#[derive(Clone, Debug, Default)]
pub struct TypeEnv {
    vars: FMap<FString, Type>,
}

impl TypeEnv {
    fn new() -> Self { Self { vars: HashMap::new() } }
    fn set(&mut self, name: &str, ty: Type) { self.vars.insert(name.to_string(), ty); }
    fn get(&self, name: &str) -> Option<Type> { self.vars.get(name).cloned() }
}

#[derive(Debug)]
pub struct TypeInfo {
    pub functions: FMap<FString, (FVec<Type>, Type)>,
}

pub fn type_check_program(p: &Program) -> Result<TypeInfo, TypeError> {
    // First pass: collect function signatures (untyped params default to Unknown, return inferred)
    let mut info = TypeInfo { functions: HashMap::new() };
    for item in &p.items {
        if let Item::Function(f) = item {
            let params = vec![Type::Unknown; f.params.len()];
            info.functions.insert(f.name.clone(), (params, Type::Unknown));
        }
    }

    // Second pass: infer/validate each function body
    for item in &p.items {
        if let Item::Function(f) = item {
            let (param_tys, _) = info.functions.get(&f.name).cloned().unwrap();
            let mut env = TypeEnv::new();
            for (i, name) in f.params.iter().enumerate() {
                env.set(name, param_tys[i].clone());
            }
            let ret = infer_block(&f.body, &mut env, &info)?;
            // set return type (default unit->int? no, keep unit)
            if let Some((params, _)) = info.functions.get(&f.name).cloned() {
                info.functions.insert(f.name.clone(), (params, ret));
            }
        }
    }

    Ok(info)
}

fn infer_block(b: &Block, env: &mut TypeEnv, info: &TypeInfo) -> Result<Type, TypeError> {
    let mut last = Type::Unit;
    for s in &b.stmts {
        last = infer_stmt(s, env, info)?;
        if matches!(s, Stmt::Return{..}) {
            // return terminates; ignore remaining
            return Ok(last);
        }
    }
    Ok(last)
}

fn infer_stmt(s: &Stmt, env: &mut TypeEnv, info: &TypeInfo) -> Result<Type, TypeError> {
    match s {
        Stmt::Let { name, expr, .. } => {
            let ty = infer_expr(expr, env, info)?;
            env.set(name, ty.clone());
            Ok(Type::Unit)
        }
        Stmt::Return { expr, .. } => {
            if let Some(e) = expr {
                infer_expr(e, env, info)
            } else {
                Ok(Type::Unit)
            }
        }
        Stmt::If { cond, then_b, else_b, span } => {
            let cty = infer_expr(cond, env, info)?;
            if cty != Type::Bool && cty != Type::Unknown {
                return Err(TypeError::Msg { span: *span, msg: "if condition must be FBool".into() });
            }
            let t1 = infer_block(then_b, &mut env.clone(), info)?;
            let t2 = if let Some(e) = else_b { infer_block(e, &mut env.clone(), info)? } else { Type::Unit };
            Ok(unify(t1, t2))
        }
        Stmt::Match { scrutinee, arms, span } => {
            let _ = infer_expr(scrutinee, env, info)?;
            let mut arm_ty = Type::Unknown;
            for a in arms {
                // patterns don't introduce types in v0.1
                let t = infer_expr(&a.expr, &mut env.clone(), info)?;
                arm_ty = unify(arm_ty, t);
            }
            if arm_ty == Type::Unknown {
                // empty match shouldn't happen, but treat as unit
                arm_ty = Type::Unit;
            }
            Ok(arm_ty)
        }
        Stmt::Expr { expr, .. } => {
            let _ = infer_expr(expr, env, info)?;
            Ok(Type::Unit)
        }
    }
}

fn infer_expr(e: &Expr, env: &mut TypeEnv, info: &TypeInfo) -> Result<Type, TypeError> {
    match e {
        Expr::Int(_, _) => Ok(Type::Int),
        Expr::Float(_, _) => Ok(Type::Float),
        Expr::Bool(_, _) => Ok(Type::Bool),
        Expr::Str(_, _) => Ok(Type::Str),
        Expr::Ident(name, span) => env.get(name).ok_or(TypeError::Msg{ span:*span, msg: format!("unknown identifier '{name}'") }),
        Expr::Binary { op, left, right, span } => {
            let lt = infer_expr(left, env, info)?;
            let rt = infer_expr(right, env, info)?;
            match op {
                BinOp::Add|BinOp::Sub|BinOp::Mul|BinOp::Div => {
                    // numeric only
                    let t = unify_numeric(lt, rt).ok_or(TypeError::Msg{ span:*span, msg:"arithmetic requires numeric types".into() })?;
                    Ok(t)
                }
                BinOp::Eq|BinOp::Lt|BinOp::Gt => {
                    // comparisons
                    let _ = unify(lt, rt);
                    Ok(Type::Bool)
                }
            }
        }
        Expr::Call { callee, args, span } => {
            let name = match &**callee {
                Expr::Ident(s, _) => s.clone(),
                _ => return Err(TypeError::Msg{ span:*span, msg:"call target must be identifier".into() }),
            };
            if name == "println" {
                if args.len()!=1 { return Err(TypeError::Msg{ span:*span, msg:"println expects 1 arg".into() }); }
                let _ = infer_expr(&args[0], env, info)?;
                return Ok(Type::Unit);
            }
            let Some((params, ret)) = info.functions.get(&name).cloned() else {
                return Err(TypeError::Msg{ span:*span, msg: format!("unknown function '{name}'") });
            };
            if params.len()!=args.len() {
                return Err(TypeError::Msg{ span:*span, msg: format!("arity mismatch calling '{name}': expected {}, got {}", params.len(), args.len()) });
            }
            for (i,arg) in args.iter().enumerate() {
                let at = infer_expr(arg, env, info)?;
                let _ = unify(params[i].clone(), at);
            }
            Ok(ret)
        }
    }
}

fn unify(a: Type, b: Type) -> Type {
    if a == Type::Unknown { return b; }
    if b == Type::Unknown { return a; }
    if a == b { a } else { Type::Unknown }
}

fn unify_numeric(a: Type, b: Type) -> Option<Type> {
    match (a,b) {
        (Type::Int, Type::Int) => Some(Type::Int),
        (Type::Float, Type::Float) => Some(Type::Float),
        (Type::Int, Type::Float) | (Type::Float, Type::Int) => Some(Type::Float),
        (Type::Unknown, x) | (x, Type::Unknown) => Some(x),
        _ => None
    }
}


 
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{lex, parse_program};

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
