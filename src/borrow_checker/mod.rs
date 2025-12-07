use crate::ast::*;
use std::collections::HashMap;

#[allow(dead_code)] // Borrowed/MutBorrowed states reserved for future borrow checking features
#[derive(Debug, Clone, PartialEq)]
enum OwnershipState {
    Owned,
    Moved,
    Borrowed,    // Immutable borrow coverage
    MutBorrowed, // Mutable borrow coverage
}

struct VariableState {
    state: OwnershipState,
    is_copy: bool,    // Primitives are Copy, Classes are not (by default)
    is_mutable: bool, // Track if variable can be reassigned
    var_type: Type,
}

pub struct BorrowChecker {
    // Scopes mapping variable name to its state
    scopes: Vec<HashMap<String, VariableState>>,
    errors: Vec<String>,
    function_signatures: HashMap<String, Type>,
}

impl BorrowChecker {
    pub fn new() -> Self {
        BorrowChecker {
            scopes: vec![HashMap::new()],
            errors: Vec::new(),
            function_signatures: HashMap::new(),
        }
    }

    pub fn check(&mut self, program: &Vec<Declaration>) -> Result<(), Vec<String>> {
        // First pass: Collect function signatures
        for decl in program {
            match decl {
                Declaration::Function {
                    name, return_type, ..
                } => {
                    self.function_signatures
                        .insert(name.clone(), return_type.clone());
                }
                Declaration::ExternFunction {
                    name, return_type, ..
                } => {
                    self.function_signatures
                        .insert(name.clone(), return_type.clone());
                }
                _ => {}
            }
        }

        // Second pass: Check bodies
        for decl in program {
            self.check_declaration(decl);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_variable(&mut self, name: &str, var_type: Type, is_mutable: bool) {
        let is_copy = self.is_type_copy(&var_type);
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(
                name.to_string(),
                VariableState {
                    state: OwnershipState::Owned,
                    is_copy,
                    is_mutable,
                    var_type,
                },
            );
        }
    }

    fn check_variable_access(&mut self, name: &str) -> Option<VariableState> {
        for scope in self.scopes.iter().rev() {
            if let Some(state) = scope.get(name) {
                if state.state == OwnershipState::Moved {
                    self.errors.push(format!("Use of moved value: {}", name));
                }
                return Some(VariableState {
                    state: state.state.clone(),
                    is_copy: state.is_copy,
                    is_mutable: state.is_mutable,
                    var_type: state.var_type.clone(),
                });
            }
        }
        None
    }

    fn move_variable(&mut self, name: &str) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(state) = scope.get_mut(name) {
                if !state.is_copy {
                    state.state = OwnershipState::Moved;
                }
                return;
            }
        }
    }

    fn check_declaration(&mut self, decl: &Declaration) {
        match decl {
            Declaration::Function { body, params, .. } => {
                self.enter_scope();
                for param in params {
                    self.declare_variable(&param.name, param.param_type.clone(), false);
                }
                self.check_block(body);
                self.exit_scope();
            }
            Declaration::Class { methods, .. } => {
                for method in methods {
                    self.check_declaration(method);
                }
            }
            _ => {}
        }
    }

    fn check_block(&mut self, block: &Block) {
        self.enter_scope();
        for stmt in &block.statements {
            self.check_statement(stmt);
        }
        self.exit_scope();
    }

    fn check_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VariableDeclaration {
                name,
                mutable,
                initializer,
                var_type,
            } => {
                let expr_type = self.check_expression(initializer);
                // If var_type is explicit, use it. Otherwise use inferred expr_type.
                let final_type = if let Some(t) = var_type {
                    t.clone() // Assuming strict type checking happened in semantic analysis
                } else {
                    expr_type
                };
                self.declare_variable(name, final_type, *mutable);
            }
            Statement::Assignment { target, value } => {
                self.check_expression(value);
                // Check if target is a valid mutable variable
                if let Expression::Variable(name) = target {
                    if let Some(state) = self.check_variable_access(name) {
                        if !state.is_mutable {
                            self.errors.push(format!(
                                "Cannot assign to immutable variable '{}'. Use 'let mut' to make it mutable.",
                                name
                            ));
                        }
                    }
                }
            }
            Statement::Expression(expr) => {
                self.check_expression(expr);
            }
            Statement::If {
                condition,
                then_block,
                else_block,
            } => {
                self.check_expression(condition);
                self.check_block(then_block);
                if let Some(blk) = else_block {
                    self.check_block(blk);
                }
            }
            Statement::While { condition, body } => {
                self.check_expression(condition);
                self.check_block(body);
            }
            Statement::Return(Some(expr)) => {
                self.check_expression(expr);
            }
            _ => {}
        }
    }

    fn check_expression(&mut self, expr: &Expression) -> Type {
        match expr {
            Expression::Variable(name) => {
                if let Some(state) = self.check_variable_access(name) {
                    self.move_variable(name);
                    return state.var_type;
                }
                Type::Unknown
            }
            Expression::BinaryOp { left, right, .. } => {
                let l_ty = self.check_expression(left);
                self.check_expression(right);
                l_ty // Simplified: return left type
            }
            Expression::FunctionCall { name, args, .. } => {
                for arg in args {
                    self.check_expression(arg);
                }
                self.function_signatures
                    .get(name)
                    .cloned()
                    .unwrap_or(Type::Unknown)
            }
            Expression::StructInit { fields, .. } => {
                for (_, expr) in fields {
                    self.check_expression(expr);
                }
                Type::Custom("Struct".to_string())
            }
            Expression::MethodCall { object, args, .. } => {
                self.check_expression(object);
                for arg in args {
                    self.check_expression(arg);
                }
                Type::Unknown // Method return type tracking not implemented yet
            }
            Expression::Literal(lit) => match lit {
                Literal::Integer(_) => Type::Integer,
                Literal::Boolean(_) => Type::Boolean,
                Literal::String(_) => Type::String,
                _ => Type::Unknown,
            },
            _ => Type::Unknown,
        }
    }

    fn is_type_copy(&self, t: &Type) -> bool {
        match t {
            Type::Integer | Type::Float | Type::Boolean | Type::String => true,
            _ => false, // Structs, Arrays are Move by default
        }
    }
}
