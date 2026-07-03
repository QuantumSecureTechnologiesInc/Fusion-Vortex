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
use crate::lexer::{Token, TokKind};
use fusion_ir::Span;
#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("unexpected end of input")]
    Eof,
    #[error("unexpected token at line {line} col {col}: {msg}")]
    Unexpected { line: FU32, col: FU32, msg: FString },
}
pub fn parse_program(tokens: &[Token]) -> Result<Program, ParseError> {
    let mut p = Parser { tokens, i: 0 };
    let mut items = Vec::new();
    while !p.is_eof() {
        if matches!(p.peek().map(| t | & t.kind), Some(TokKind::Use)) {
            items.push(Item::Use(p.parse_use_decl()?));
            continue;
        }
        if matches!(p.peek().map(| t | & t.kind), Some(TokKind::Mod)) {
            items.push(Item::Mod(p.parse_mod_decl()?));
            continue;
        }
        items.push(Item::Function(p.parse_function()?));
    }
    Ok(Program { items })
}
struct Parser<'a> {
    tokens: &'a [Token],
    i: FSize,
}
impl<'a> Parser<'a> {
    fn is_eof(&self) -> FBool {
        self.i >= self.tokens.len()
    }
    fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.i)
    }
    fn bump(&mut self) -> Option<&'a Token> {
        let t = self.tokens.get(self.i);
        if t.is_some() {
            self.i += 1;
        }
        t
    }
    fn expect(
        &mut self,
        kind: fn(&TokKind) -> FBool,
        what: &str,
    ) -> Result<Token, ParseError> {
        let t = self.bump().ok_or(ParseError::Eof)?;
        if kind(&t.kind) {
            Ok(t.clone())
        } else {
            Err(ParseError::Unexpected {
                line: t.span.line,
                col: t.span.col,
                msg: format!("expected {what}, got {:?}", t.kind),
            })
        }
    }
    fn parse_effects(&mut self) -> Result<EffectsAst, ParseError> {
        let mut e = EffectsAst::default();
        loop {
            let Some(t) = self.peek() else {
                break;
            };
            if !matches!(t.kind, TokKind::At) {
                break;
            }
            self.bump();
            let name = self.expect(|k| matches!(k, TokKind::Ident(_)), "effect name")?;
            let effect = match name.kind {
                TokKind::Ident(s) => s,
                _ => unreachable!(),
            };
            match effect.as_str() {
                "borrowed" => e.borrowed = true,
                "constant_time" => e.constant_time = true,
                "gpu_accelerated" => e.gpu_accelerated = true,
                other => {
                    return Err(ParseError::Unexpected {
                        line: name.span.line,
                        col: name.span.col,
                        msg: format!("unknown effect '@{other}'"),
                    });
                }
            }
        }
        Ok(e)
    }
    fn parse_use_decl(&mut self) -> Result<UseDecl, ParseError> {
        let use_tok = self.expect(|k| matches!(k, TokKind::Use), "`use`")?;
        let mut path = Vec::new();
        let first = self.expect(|k| matches!(k, TokKind::Ident(_)), "path segment")?;
        path.push(
            match first.kind {
                TokKind::Ident(s) => s,
                _ => unreachable!(),
            },
        );
        while matches!(self.peek().map(| t | & t.kind), Some(TokKind::ColonColon)) {
            self.bump();
            let seg = self.expect(|k| matches!(k, TokKind::Ident(_)), "path segment")?;
            path.push(
                match seg.kind {
                    TokKind::Ident(s) => s,
                    _ => unreachable!(),
                },
            );
        }
        self.consume_semi_opt();
        Ok(UseDecl {
            path,
            span: use_tok.span,
        })
    }
    fn parse_mod_decl(&mut self) -> Result<ModDecl, ParseError> {
        let mod_tok = self.expect(|k| matches!(k, TokKind::Mod), "`mod`")?;
        let name_tok = self.expect(|k| matches!(k, TokKind::Ident(_)), "module name")?;
        let name = match name_tok.kind {
            TokKind::Ident(s) => s,
            _ => unreachable!(),
        };
        self.consume_semi_opt();
        Ok(ModDecl {
            name,
            span: mod_tok.span,
        })
    }
    fn parse_function(&mut self) -> Result<Function, ParseError> {
        let effects = self.parse_effects()?;
        let fn_tok = self.expect(|k| matches!(k, TokKind::Fn), "`fn`")?;
        let name_tok = self.expect(|k| matches!(k, TokKind::Ident(_)), "function name")?;
        let name = match name_tok.kind {
            TokKind::Ident(s) => s,
            _ => unreachable!(),
        };
        self.expect(|k| matches!(k, TokKind::LParen), "`(`")?;
        let mut params = Vec::new();
        if !matches!(self.peek().map(| t | & t.kind), Some(TokKind::RParen)) {
            loop {
                let p_tok = self
                    .expect(|k| matches!(k, TokKind::Ident(_)), "param name")?;
                let p_name = match p_tok.kind {
                    TokKind::Ident(s) => s,
                    _ => unreachable!(),
                };
                params.push(p_name);
                if matches!(self.peek().map(| t | & t.kind), Some(TokKind::Comma)) {
                    self.bump();
                    continue;
                }
                break;
            }
        }
        self.expect(|k| matches!(k, TokKind::RParen), "`)`")?;
        if matches!(self.peek().map(| t | & t.kind), Some(TokKind::Arrow)) {
            self.bump();
            self.expect(|k| matches!(k, TokKind::Ident(_)), "type name")?;
        }
        let body = self.parse_block()?;
        let span = merge_span(fn_tok.span, body.span);
        Ok(Function {
            name,
            params,
            body,
            span,
            effects,
        })
    }
    fn parse_block(&mut self) -> Result<Block, ParseError> {
        let l = self.expect(|k| matches!(k, TokKind::LBrace), "`{`")?;
        let mut stmts = Vec::new();
        while !matches!(self.peek().map(| t | & t.kind), Some(TokKind::RBrace)) {
            stmts.push(self.parse_stmt()?);
        }
        let r = self.expect(|k| matches!(k, TokKind::RBrace), "`}`")?;
        Ok(Block {
            stmts,
            span: merge_span(l.span, r.span),
        })
    }
    fn consume_semi_opt(&mut self) {
        if matches!(self.peek().map(| t | & t.kind), Some(TokKind::Semi)) {
            self.bump();
        }
    }
    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        let Some(t) = self.peek() else {
            return Err(ParseError::Eof);
        };
        match &t.kind {
            TokKind::Let => {
                let let_tok = self.bump().unwrap().clone();
                let name_tok = self
                    .expect(|k| matches!(k, TokKind::Ident(_)), "identifier")?;
                let name = match name_tok.kind {
                    TokKind::Ident(s) => s,
                    _ => unreachable!(),
                };
                self.expect(|k| matches!(k, TokKind::EqAssign), "`=`")?;
                let expr = self.parse_expr()?;
                let span = merge_span(let_tok.span, expr.span());
                self.consume_semi_opt();
                Ok(Stmt::Let { name, expr, span })
            }
            TokKind::Return => {
                let ret_tok = self.bump().unwrap().clone();
                if matches!(
                    self.peek().map(| t | & t.kind), Some(TokKind::Semi) |
                    Some(TokKind::RBrace)
                ) {
                    self.consume_semi_opt();
                    return Ok(Stmt::Return {
                        expr: None,
                        span: ret_tok.span,
                    });
                }
                let expr = self.parse_expr()?;
                let span = merge_span(ret_tok.span, expr.span());
                self.consume_semi_opt();
                Ok(Stmt::Return {
                    expr: Some(expr),
                    span,
                })
            }
            TokKind::If => {
                let if_tok = self.bump().unwrap().clone();
                let cond = self.parse_expr()?;
                let then_b = self.parse_block()?;
                let else_b = if matches!(
                    self.peek().map(| t | & t.kind), Some(TokKind::Else)
                ) {
                    self.bump();
                    Some(self.parse_block()?)
                } else {
                    None
                };
                let end_span = else_b.as_ref().map(|b| b.span).unwrap_or(then_b.span);
                Ok(Stmt::If {
                    cond,
                    then_b,
                    else_b,
                    span: merge_span(if_tok.span, end_span),
                })
            }
            TokKind::Match => self.parse_match_stmt(),
            _ => {
                let expr = self.parse_expr()?;
                let span = expr.span();
                self.consume_semi_opt();
                Ok(Stmt::Expr { expr, span })
            }
        }
    }
    fn parse_match_stmt(&mut self) -> Result<Stmt, ParseError> {
        let m_tok = self.expect(|k| matches!(k, TokKind::Match), "`match`")?;
        let scrutinee = self.parse_expr()?;
        let l = self.expect(|k| matches!(k, TokKind::LBrace), "`{`")?;
        let mut arms = Vec::new();
        while !matches!(self.peek().map(| t | & t.kind), Some(TokKind::RBrace)) {
            let pat = self.parse_pattern()?;
            self.expect(|k| matches!(k, TokKind::FatArrow), "`=>`")?;
            let expr = self.parse_expr()?;
            let end_span = expr.span();
            self.consume_semi_opt();
            let arm_span = merge_span(match_pat_span(&pat), end_span);
            arms.push(MatchArm {
                pat,
                expr,
                span: arm_span,
            });
            if matches!(self.peek().map(| t | & t.kind), Some(TokKind::Comma)) {
                self.bump();
            }
        }
        let r = self.expect(|k| matches!(k, TokKind::RBrace), "`}`")?;
        Ok(Stmt::Match {
            scrutinee,
            arms,
            span: merge_span(m_tok.span, merge_span(l.span, r.span)),
        })
    }
    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        let t = self.bump().ok_or(ParseError::Eof)?.clone();
        Ok(
            match t.kind {
                TokKind::Underscore => Pattern::Wildcard(t.span),
                TokKind::Int(i) => Pattern::Int(i, t.span),
                TokKind::Ident(s) => Pattern::Ident(s, t.span),
                other => {
                    return Err(ParseError::Unexpected {
                        line: t.span.line,
                        col: t.span.col,
                        msg: format!("invalid pattern: {:?}", other),
                    });
                }
            },
        )
    }
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_bp(0)
    }
    fn parse_bp(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_primary()?;
        loop {
            let op = match self.peek().map(|t| &t.kind) {
                Some(TokKind::Plus) => BinOp::Add,
                Some(TokKind::Minus) => BinOp::Sub,
                Some(TokKind::Star) => BinOp::Mul,
                Some(TokKind::Slash) => BinOp::Div,
                Some(TokKind::EqEq) => BinOp::Eq,
                Some(TokKind::Lt) => BinOp::Lt,
                Some(TokKind::Gt) => BinOp::Gt,
                Some(TokKind::LParen) => {
                    lhs = self.finish_call(lhs)?;
                    continue;
                }
                _ => break,
            };
            let (l_bp, r_bp) = infix_binding_power(op);
            if l_bp < min_bp {
                break;
            }
            let op_tok = self.bump().unwrap().clone();
            let rhs = self.parse_bp(r_bp)?;
            let span = merge_span(lhs.span(), rhs.span());
            lhs = Expr::Binary {
                op,
                left: Box::new(lhs),
                right: Box::new(rhs),
                span: merge_span(op_tok.span, span),
            };
        }
        Ok(lhs)
    }
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let t = self.bump().ok_or(ParseError::Eof)?.clone();
        Ok(
            match t.kind {
                TokKind::Int(i) => Expr::Int(i, t.span),
                TokKind::Float(f) => Expr::Float(f, t.span),
                TokKind::True => Expr::Bool(true, t.span),
                TokKind::False => Expr::Bool(false, t.span),
                TokKind::Str(s) => Expr::Str(s, t.span),
                TokKind::Ident(s) => Expr::Ident(s, t.span),
                TokKind::LParen => {
                    let inner = self.parse_expr()?;
                    self.expect(|k| matches!(k, TokKind::RParen), "`)`")?;
                    inner
                }
                other => {
                    return Err(ParseError::Unexpected {
                        line: t.span.line,
                        col: t.span.col,
                        msg: format!("unexpected token: {:?}", other),
                    });
                }
            },
        )
    }
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let lpar = self.expect(|k| matches!(k, TokKind::LParen), "`(`")?;
        let mut args = Vec::new();
        if !matches!(self.peek().map(| t | & t.kind), Some(TokKind::RParen)) {
            loop {
                args.push(self.parse_expr()?);
                if matches!(self.peek().map(| t | & t.kind), Some(TokKind::Comma)) {
                    self.bump();
                    continue;
                }
                break;
            }
        }
        let rpar = self.expect(|k| matches!(k, TokKind::RParen), "`)`")?;
        let span = merge_span(callee.span(), rpar.span);
        Ok(Expr::Call {
            callee: Box::new(callee),
            args,
            span: merge_span(lpar.span, span),
        })
    }
}
fn infix_binding_power(op: BinOp) -> (u8, u8) {
    match op {
        BinOp::Mul | BinOp::Div => (7, 8),
        BinOp::Add | BinOp::Sub => (5, 6),
        BinOp::Eq | BinOp::Lt | BinOp::Gt => (3, 4),
    }
}
fn merge_span(a: Span, b: Span) -> Span {
    let start = a.start.min(b.start);
    let end = a.end.max(b.end);
    Span {
        start,
        end,
        line: a.line,
        col: a.col,
    }
}
fn match_pat_span(p: &Pattern) -> Span {
    match p {
        Pattern::Wildcard(s) => *s,
        Pattern::Int(_, s) => *s,
        Pattern::Ident(_, s) => *s,
    }
}
