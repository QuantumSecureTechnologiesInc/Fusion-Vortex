//! Vortex Safety Engine
//! Implements entropic flow analysis: permission state tracking, loan tracking,
//! and collision detection for the Fusion borrow checker.
//! The full Vortex implementation lives in vortex.fu (self-hosted Fusion source).

use crate::sema::{TypedProgram, TypedFunction, TypedStatement, TypedExpression, TypedExpressionKind};
use crate::ast::Span;
use std::collections::HashMap;

// ---- Permission States ----

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionState {
    /// Resource is alive and fully accessible.
    Intact,
    /// Resource is exclusively borrowed by a mutable reference.
    ExclusiveBorrowed,
    /// Resource is shared by N immutable readers.
    SharedBorrowed(usize),
    /// Resource has been consumed / moved.
    Dissipated,
}

// ---- Loan Tracking ----

#[derive(Debug, Clone)]
pub struct Loan {
    pub var_name: String,
    pub is_mutable: bool,
    pub span: Span,
}

// ---- Collision Events ----

#[derive(Debug, Clone)]
pub struct EventCollision {
    pub var_name: String,
    pub existing_state: PermissionState,
    pub collision_span: Span,
}

// ---- Vortex Context ----

pub struct VortexContext {
    /// Permission state for each variable.
    permissions: HashMap<String, PermissionState>,
    /// Active loans on variables.
    loans: HashMap<String, Vec<Loan>>,
    /// Collision events detected during verification.
    pub collisions: Vec<EventCollision>,
    /// Whether verification passed.
    pub verified: bool,
}

impl VortexContext {
    pub fn new() -> Self {
        Self {
            permissions: HashMap::new(),
            loans: HashMap::new(),
            collisions: Vec::new(),
            verified: true,
        }
    }

    /// Verify an entire typed program.
    pub fn verify_program(&mut self, prog: &TypedProgram) -> bool {
        self.verified = true;
        self.collisions.clear();

        for func in &prog.functions {
            self.verify_function(func);
        }

        self.verified
    }

    /// Verify a single typed function.
    pub fn verify_function(&mut self, func: &TypedFunction) {
        self.permissions.clear();
        self.loans.clear();

        // Initialize parameters as Intact
        for (name, _) in &func.params {
            self.permissions.insert(name.clone(), PermissionState::Intact);
        }

        self.verify_block(&func.body);
    }

    /// Verify a block of statements.
    fn verify_block(&mut self, stmts: &[TypedStatement]) {
        for stmt in stmts {
            self.verify_statement(stmt);
        }
    }

    /// Verify a single statement for permission violations.
    pub fn verify_statement(&mut self, stmt: &TypedStatement) {
        match stmt {
            TypedStatement::Let { name, value, .. } => {
                // Evaluate the initializer
                self.verify_expression(value, false);
                // Register the new binding
                self.permissions.insert(name.clone(), PermissionState::Intact);
            }
            TypedStatement::Assignment { target, value } => {
                self.verify_expression(value, false);
                // Target must be writable
                if let TypedExpressionKind::Variable(name) = &target.node {
                    self.check_mutable_access(name, &target.span);
                    self.permissions.insert(name.clone(), PermissionState::Intact);
                } else {
                    self.verify_expression(target, true);
                }
            }
            TypedStatement::Expression(expr) => {
                self.verify_expression(expr, false);
            }
            TypedStatement::Return(Some(expr)) => {
                self.verify_expression(expr, false);
                // All local variables are dissipated on return
                for (name, _) in self.permissions.clone() {
                    self.permissions.insert(name, PermissionState::Dissipated);
                }
            }
            TypedStatement::Return(None) => {
                for (name, _) in self.permissions.clone() {
                    self.permissions.insert(name, PermissionState::Dissipated);
                }
            }
            TypedStatement::If { cond, then_block, else_block } => {
                self.verify_expression(cond, false);

                let saved_perms = self.permissions.clone();
                let saved_loans = self.loans.clone();

                self.verify_block(then_block);
                let then_perms = self.permissions.clone();

                self.permissions = saved_perms;
                self.loans = saved_loans;

                if let Some(else_b) = else_block {
                    self.verify_block(else_b);
                }

                // Intersect: if dissipated in either branch, it's dissipated overall
                for (name, state) in &then_perms {
                    if *state == PermissionState::Dissipated {
                        if let Some(cur) = self.permissions.get(name) {
                            if *cur != PermissionState::Dissipated {
                                self.permissions.insert(name.clone(), PermissionState::Dissipated);
                            }
                        }
                    }
                }
            }
            TypedStatement::While { cond, body } => {
                self.verify_expression(cond, false);
                self.verify_block(body);
                // Re-verify condition (loop may re-execute)
                self.verify_expression(cond, false);
            }
            TypedStatement::For { var, iter, body } => {
                self.verify_expression(iter, false);
                // Register loop variable
                self.permissions.insert(var.clone(), PermissionState::Intact);
                self.verify_block(body);
            }
        }
    }

    /// Verify an expression for permission violations.
    fn verify_expression(&mut self, expr: &TypedExpression, is_mutable_context: bool) {
        match &expr.node {
            TypedExpressionKind::Variable(name) => {
                if is_mutable_context {
                    self.check_mutable_access(name, &expr.span);
                } else {
                    self.check_immutable_access(name, &expr.span);
                }
            }
            TypedExpressionKind::FunctionCall { args, .. } => {
                for arg in args {
                    self.verify_expression(arg, false);
                }
            }
            TypedExpressionKind::BinaryOperation { left, right, .. } => {
                self.verify_expression(left, false);
                self.verify_expression(right, false);
            }
            TypedExpressionKind::UnaryOperation { expr: inner, .. } => {
                self.verify_expression(inner, false);
            }
            TypedExpressionKind::ArrayLiteral(elems) => {
                for e in elems {
                    self.verify_expression(e, false);
                }
            }
            TypedExpressionKind::StructLiteral { fields, .. } => {
                for (_, _, field_expr) in fields {
                    self.verify_expression(field_expr, false);
                }
            }
            TypedExpressionKind::MemberAccess { base, .. } => {
                self.verify_expression(base, is_mutable_context);
            }
            TypedExpressionKind::AddressOf(inner) => {
                // Taking address creates an immutable borrow
                if let TypedExpressionKind::Variable(name) = &inner.node {
                    self.add_loan(name, false, &expr.span);
                }
                self.verify_expression(inner, false);
            }
            TypedExpressionKind::Dereference(inner) => {
                self.verify_expression(inner, is_mutable_context);
            }
            TypedExpressionKind::Index { array, index } => {
                self.verify_expression(array, is_mutable_context);
                self.verify_expression(index, false);
            }
            TypedExpressionKind::Match { scrutinee, arms } => {
                self.verify_expression(scrutinee, false);
                for arm in arms {
                    let saved_perms = self.permissions.clone();
                    let saved_loans = self.loans.clone();

                    if let Some(ref guard) = arm.guard {
                        self.verify_expression(guard, false);
                    }
                    self.verify_expression(&arm.body, false);

                    self.permissions = saved_perms;
                    self.loans = saved_loans;
                }
            }
            TypedExpressionKind::Closure { params, body } => {
                // Closures capture their environment immutably
                let saved_perms = self.permissions.clone();
                let saved_loans = self.loans.clone();

                for (name, _) in params {
                    self.permissions.insert(name.clone(), PermissionState::Intact);
                }
                self.verify_expression(body, false);

                self.permissions = saved_perms;
                self.loans = saved_loans;
            }
            _ => {} // Primitives need no verification
        }
    }

    /// Check immutable access to a variable.
    fn check_immutable_access(&mut self, name: &str, span: &Span) {
        match self.permissions.get(name) {
            Some(PermissionState::Dissipated) => {
                self.verified = false;
                self.collisions.push(EventCollision {
                    var_name: name.to_string(),
                    existing_state: PermissionState::Dissipated,
                    collision_span: span.clone(),
                });
            }
            Some(PermissionState::ExclusiveBorrowed) => {
                self.verified = false;
                self.collisions.push(EventCollision {
                    var_name: name.to_string(),
                    existing_state: PermissionState::ExclusiveBorrowed,
                    collision_span: span.clone(),
                });
            }
            Some(PermissionState::SharedBorrowed(count)) => {
                // Multiple shared borrows are fine
                self.permissions.insert(name.to_string(), PermissionState::SharedBorrowed(count + 1));
            }
            Some(PermissionState::Intact) => {
                // Mark as shared-borrowed
                self.permissions.insert(name.to_string(), PermissionState::SharedBorrowed(1));
            }
            None => {} // Unknown variable, semantic analysis will catch
        }
    }

    /// Check mutable access to a variable.
    fn check_mutable_access(&mut self, name: &str, span: &Span) {
        match self.permissions.get(name) {
            Some(PermissionState::Dissipated) => {
                self.verified = false;
                self.collisions.push(EventCollision {
                    var_name: name.to_string(),
                    existing_state: PermissionState::Dissipated,
                    collision_span: span.clone(),
                });
            }
            Some(PermissionState::ExclusiveBorrowed) => {
                self.verified = false;
                self.collisions.push(EventCollision {
                    var_name: name.to_string(),
                    existing_state: PermissionState::ExclusiveBorrowed,
                    collision_span: span.clone(),
                });
            }
            Some(PermissionState::SharedBorrowed(_)) => {
                // Cannot mutate while shared borrows exist
                self.verified = false;
                self.collisions.push(EventCollision {
                    var_name: name.to_string(),
                    existing_state: PermissionState::SharedBorrowed(0),
                    collision_span: span.clone(),
                });
            }
            Some(PermissionState::Intact) => {
                // OK — mark as exclusively borrowed
                self.permissions.insert(name.to_string(), PermissionState::ExclusiveBorrowed);
            }
            None => {} // Unknown variable
        }
    }

    /// Add a loan on a variable.
    fn add_loan(&mut self, name: &str, is_mutable: bool, span: &Span) {
        let loan = Loan {
            var_name: name.to_string(),
            is_mutable,
            span: span.clone(),
        };
        self.loans.entry(name.to_string()).or_default().push(loan);
    }

    /// Release all loans on a variable (e.g., after borrow scope ends).
    pub fn release_loans(&mut self, name: &str) {
        self.loans.remove(name);
        if self.permissions.get(name) == Some(&PermissionState::SharedBorrowed(0))
            || self.permissions.get(name) == Some(&PermissionState::ExclusiveBorrowed)
        {
            self.permissions.insert(name.to_string(), PermissionState::Intact);
        }
    }
}

/// Verify a typed program using the Vortex engine.
/// Returns true if no collisions were detected.
pub fn verify_output(prog: &TypedProgram) -> bool {
    let mut ctx = VortexContext::new();
    ctx.verify_program(prog)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vortex_context_new() {
        let ctx = VortexContext::new();
        assert!(ctx.verified);
        assert!(ctx.collisions.is_empty());
    }

    #[test]
    fn test_permission_state_intact() {
        let mut ctx = VortexContext::new();
        ctx.permissions.insert("x".to_string(), PermissionState::Intact);
        let span = Span::default();
        ctx.check_immutable_access("x", &span);
        assert!(ctx.verified);
        assert_eq!(ctx.permissions.get("x"), Some(&PermissionState::SharedBorrowed(1)));
    }

    #[test]
    fn test_use_after_move_detected() {
        let mut ctx = VortexContext::new();
        ctx.permissions.insert("x".to_string(), PermissionState::Dissipated);
        let span = Span::default();
        ctx.check_immutable_access("x", &span);
        assert!(!ctx.verified);
        assert_eq!(ctx.collisions.len(), 1);
    }

    #[test]
    fn test_mutable_borrow_conflict() {
        let mut ctx = VortexContext::new();
        ctx.permissions.insert("x".to_string(), PermissionState::ExclusiveBorrowed);
        let span = Span::default();
        ctx.check_mutable_access("x", &span);
        assert!(!ctx.verified);
        assert_eq!(ctx.collisions.len(), 1);
    }
}