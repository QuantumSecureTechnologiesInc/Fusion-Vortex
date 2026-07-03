//! Borrow Checking and Ownership Analysis (Vortex Base)
//! 
//! This module implements affine type tracking (move semantics).
//! It ensures that variables are not used after they have been moved.
use crate::types::*;
use std::collections::HashMap;

use crate::sema::{TypedProgram, TypedFunction, TypedStatement, TypedExpressionKind};
use crate::ast::{Span, Type};

#[derive(Clone, Debug, PartialEq)]
enum ValState {
    Uninitialized,
    Active,
    Moved(Span), // Tracks where it was moved for diagnostics
}

pub struct BorrowDiagnostic {
    pub span: Span,
    pub message: FString,
}

pub struct BorrowChecker {
    errors: FVec<BorrowDiagnostic>,
    env: FMap<FString, ValState>,
}

impl BorrowChecker {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            env: HashMap::new(),
        }
    }

    pub fn check_program(&mut self, prog: &TypedProgram) -> FVec<BorrowDiagnostic> {
        for func in &prog.functions {
            self.check_function(func);
        }
        std::mem::replace(&mut self.errors, Vec::new())
    }

    fn check_function(&mut self, func: &TypedFunction) {
        self.env.clear();
        
        // Initialize parameters as Active
        for (name, _) in &func.params {
            self.env.insert(name.clone(), ValState::Active);
        }

        self.check_block(&func.body);
    }

    fn check_block(&mut self, block: &FVec<TypedStatement>) {
        for stmt in block {
            self.check_statement(stmt);
        }
    }

    fn check_statement(&mut self, stmt: &TypedStatement) {
        match stmt {
            TypedStatement::Let { name, value, .. } => {
                // Mark variable as uninitialized before checking the initializer
                self.env.insert(name.clone(), ValState::Uninitialized);
                self.check_expression(value);
                self.env.insert(name.clone(), ValState::Active);
            }
            TypedStatement::Assignment { target, value } => {
                self.check_expression(value);
                // If target is a variable, mark it active again (re-initialization)
                if let TypedExpressionKind::Variable(name) = &target.node {
                    self.env.insert(name.clone(), ValState::Active);
                } else {
                    self.check_expression(target);
                }
            }
            TypedStatement::Expression(expr) | TypedStatement::Return(Some(expr)) => {
                self.check_expression(expr);
            }
            TypedStatement::Return(None) => {}
            TypedStatement::If { cond, then_block, else_block } => {
                self.check_expression(cond);
                
                // Clone environment for branch analysis
                let env_before = self.env.clone();
                
                self.check_block(then_block);
                let env_then = self.env.clone();
                
                self.env = env_before;
                if let Some(else_b) = else_block {
                    self.check_block(else_b);
                }
                
                // Intersect environments (if moved in either branch, it's moved overall)
                for (name, state) in &env_then {
                    if let ValState::Moved(span) = state {
                        self.env.insert(name.clone(), ValState::Moved(span.clone()));
                    }
                }
            }
            TypedStatement::While { cond, body } => {
                self.check_expression(cond);
                self.check_block(body);
            }
            TypedStatement::For { var: _, iter, body } => {
                self.check_expression(iter);
                self.check_block(body);
            }
        }
    }

    fn check_expression(&mut self, expr: &crate::sema::TypedExpression) {
        match &expr.node {
            TypedExpressionKind::Variable(name) => {
                if let Some(state) = self.env.get(name) {
                    if let ValState::Moved(moved_span) = state {
                        self.errors.push(BorrowDiagnostic {
                            span: expr.span.clone(),
                            message: format!("Use of moved value '{}'. It was previously moved at span {:?}", name, moved_span),
                        });
                    } else if !self.is_copy_type(&expr.ty) {
                        // Mark as moved if it's not a primitive/copy type
                        self.env.insert(name.clone(), ValState::Moved(expr.span.clone()));
                    }
                }
            }
            TypedExpressionKind::FunctionCall { args, .. } => {
                for arg in args {
                    self.check_expression(arg);
                }
            }
            TypedExpressionKind::BinaryOperation { left, right, .. } => {
                self.check_expression(left);
                self.check_expression(right);
            }
            TypedExpressionKind::ArrayLiteral(elems) => {
                for e in elems {
                    self.check_expression(e);
                }
            }
            TypedExpressionKind::StructLiteral { fields, .. } => {
                for (_, _, field_expr) in fields {
                    self.check_expression(field_expr);
                }
            }
            TypedExpressionKind::MemberAccess { base, .. } |
            TypedExpressionKind::AddressOf(base) |
            TypedExpressionKind::Dereference(base) => {
                self.check_expression(base);
            }
            TypedExpressionKind::Index { array, index } => {
                self.check_expression(array);
                self.check_expression(index);
            }
            TypedExpressionKind::Match { scrutinee, arms } => {
                self.check_expression(scrutinee);
                for arm in arms {
                    if let Some(ref g) = arm.guard {
                        self.check_expression(g);
                    }
                    self.check_expression(&arm.body);
                }
            }
            _ => {} // Primitives and constants don't trigger moves
        }
    }

    /// Determines if a type implements copy semantics vs move semantics
    fn is_copy_type(&self, ty: &Type) -> FBool {
        match ty {
            Type::Int | Type::Bool | Type::Pointer(_) | Type::GenericParam(_) => true,
            // Structs, Arrays, and Strings are moved by default in Fusion
            Type::Struct(_) | Type::Array(_, _) | Type::String | Type::Slice(_) | Type::Closure(_, _) => false,
            _ => true,
        }
    }
}