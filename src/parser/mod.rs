// src/parser/mod.rs - Recursive Descent Parser

use crate::ast::*;
use crate::lexer::Token;
use logos::{Lexer, Logos};

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Token::lexer(input);
        let current_token = lexer.next().and_then(|r| r.ok());
        Parser {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next().and_then(|r| r.ok());
    }

    fn check(&self, token: &Token) -> bool {
        match (&self.current_token, token) {
            (Some(t1), t2) => std::mem::discriminant(t1) == std::mem::discriminant(t2),
            _ => false,
        }
    }

    fn consume(&mut self, token: Token, error_msg: &str) -> Result<(), String> {
        if self.check(&token) {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, found {:?}. Context: {}",
                token, self.current_token, error_msg
            ))
        }
    }

    fn parse_generic_params(&mut self) -> Result<Vec<String>, String> {
        let mut params = Vec::new();
        if self.check(&Token::Lt) {
            self.advance();
            loop {
                if let Some(Token::Identifier(name)) = self.current_token.take() {
                    self.advance();
                    params.push(name);
                } else {
                    return Err("Expected generic type name".to_string());
                }

                if self.check(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            self.consume(Token::Gt, "Generic params end")?;
        }
        Ok(params)
    }

    // --- Grammar Rules ---

    pub fn parse_program(&mut self) -> Result<Vec<Declaration>, String> {
        let mut decls = Vec::new();
        while self.current_token.is_some() {
            decls.push(self.parse_declaration()?);
        }
        Ok(decls)
    }

    fn parse_declaration(&mut self) -> Result<Declaration, String> {
        // Check for pub modifier
        let is_public = if self.check(&Token::Pub) {
            self.advance();
            true
        } else {
            false
        };

        match &self.current_token {
            Some(Token::Mod) => self.parse_mod_declaration(is_public),
            Some(Token::Use) => self.parse_use_declaration(),
            Some(Token::Fn) => self.parse_function(),
            Some(Token::Class) => self.parse_class(),
            Some(Token::Extern) => self.parse_extern_function(),
            _ => Err(format!(
                "Unexpected token at top level: {:?}",
                self.current_token
            )),
        }
    }

    fn parse_mod_declaration(&mut self, is_public: bool) -> Result<Declaration, String> {
        self.consume(Token::Mod, "Module declaration")?;

        let name = match self.current_token.take() {
            Some(Token::Identifier(s)) => {
                self.advance();
                s
            }
            t => return Err(format!("Expected module name, found {:?}", t)),
        };

        self.consume(Token::Semicolon, "Module declaration end")?;

        Ok(Declaration::ModuleDecl { name, is_public })
    }

    fn parse_use_declaration(&mut self) -> Result<Declaration, String> {
        self.consume(Token::Use, "Use declaration")?;

        // Parse path: lib::utils or lib::utils::*
        let mut path = Vec::new();
        let mut import_all = false;

        loop {
            match self.current_token.take() {
                Some(Token::Identifier(s)) => {
                    self.advance();
                    path.push(s);
                }
                Some(Token::Star) => {
                    self.advance();
                    import_all = true;
                    break;
                }
                t => return Err(format!("Expected identifier or *, found {:?}", t)),
            }

            if self.check(&Token::DoubleColon) {
                self.advance();
            } else {
                break;
            }
        }

        if path.is_empty() {
            return Err("Use declaration must have at least one path component".to_string());
        }

        // Optional alias: use lib::utils as my_utils
        let alias = if self.check(&Token::Identifier("as".to_string())) {
            self.advance();
            match self.current_token.take() {
                Some(Token::Identifier(s)) => {
                    self.advance();
                    Some(s)
                }
                t => return Err(format!("Expected alias name after 'as', found {:?}", t)),
            }
        } else {
            None
        };

        self.consume(Token::Semicolon, "Use declaration end")?;

        Ok(Declaration::UseDecl {
            path,
            alias,
            import_all,
        })
    }

    fn parse_class(&mut self) -> Result<Declaration, String> {
        self.consume(Token::Class, "Class declaration")?;

        let name = match self.current_token.take() {
            Some(Token::Identifier(s)) => {
                self.advance();
                s
            }
            t => return Err(format!("Expected class name, found {:?}", t)),
        };

        let generic_params = self.parse_generic_params()?;

        self.consume(Token::LBrace, "Class body start")?;

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while !self.check(&Token::RBrace) && self.current_token.is_some() {
            // Check for method (fn) or field (name: type) or public modifier
            let _is_pub = if self.check(&Token::Pub) {
                self.advance();
                true
            } else {
                false
            };

            if self.check(&Token::Fn) {
                methods.push(self.parse_function()?);
            } else if let Some(Token::Identifier(field_name)) = &self.current_token {
                // Field declaration: name: Type;
                let field_name = field_name.clone();
                self.advance();
                self.consume(Token::Colon, "Field type separator")?;
                let field_type = self.parse_type()?;
                self.consume(Token::Semicolon, "Field declaration end")?;

                fields.push(Field {
                    name: field_name,
                    field_type,
                });
            } else {
                return Err(format!(
                    "Unexpected token in class body: {:?}",
                    self.current_token
                ));
            }
        }

        self.consume(Token::RBrace, "Class body end")?;

        Ok(Declaration::Class {
            name,
            generic_params,
            implements: vec![],
            fields,
            methods,
        })
    }

    fn parse_function(&mut self) -> Result<Declaration, String> {
        self.consume(Token::Fn, "Function declaration")?;

        let name = match self.current_token.take() {
            Some(Token::Identifier(s)) => {
                self.advance();
                s
            }
            t => return Err(format!("Expected identifier, found {:?}", t)),
        };

        let generic_params = self.parse_generic_params()?;

        self.consume(Token::LParen, "Function params start")?;

        // Parse params
        let mut params = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                // name: Type
                let param_name = match self.current_token.take() {
                    Some(Token::Identifier(s)) => {
                        self.advance();
                        s
                    }
                    t => return Err(format!("Expected parameter name, found {:?}", t)),
                };

                self.consume(Token::Colon, "Parameter type separator")?;
                let param_type = self.parse_type()?;

                params.push(Parameter {
                    name: param_name,
                    param_type,
                });

                if self.check(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.consume(Token::RParen, "Function params end")?;

        let return_type = if self.check(&Token::Arrow) {
            self.advance();
            self.parse_type()?
        } else {
            Type::Void
        };

        let body = self.parse_block()?;

        Ok(Declaration::Function {
            name,
            attributes: vec![],
            generic_params,
            where_bounds: vec![],
            params,
            return_type,
            body,
        })
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        match self.current_token.take() {
            Some(Token::Identifier(s)) if s == "int" => {
                self.advance();
                Ok(Type::Integer)
            }
            Some(Token::Identifier(s)) if s == "void" => {
                self.advance();
                Ok(Type::Void)
            }
            Some(Token::Identifier(s)) if s == "string" => {
                self.advance();
                Ok(Type::String) // Primitive string
            }
            Some(Token::Identifier(s)) => {
                self.advance();
                if self.check(&Token::Lt) {
                    self.advance();
                    let mut args = Vec::new();
                    loop {
                        args.push(self.parse_type()?);
                        if self.check(&Token::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    self.consume(Token::Gt, "Generic args end")?;
                    Ok(Type::GenericInstance { base_name: s, args })
                } else {
                    Ok(Type::Custom(s))
                }
            }
            t => Err(format!("Expected type, found {:?}", t)),
        }
    }

    fn parse_extern_function(&mut self) -> Result<Declaration, String> {
        self.consume(Token::Extern, "Extern declaration")?;
        self.consume(Token::Fn, "Extern function declaration")?;

        let name = match self.current_token.take() {
            Some(Token::Identifier(s)) => {
                self.advance();
                s
            }
            t => return Err(format!("Expected identifier, found {:?}", t)),
        };

        self.consume(Token::LParen, "Extern params start")?;

        let mut params = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                let param_name = match self.current_token.take() {
                    Some(Token::Identifier(s)) => {
                        self.advance();
                        s
                    }
                    t => return Err(format!("Expected parameter name, found {:?}", t)),
                };

                self.consume(Token::Colon, "Parameter type separator")?;
                let param_type = self.parse_type()?;

                params.push(Parameter {
                    name: param_name,
                    param_type,
                });

                if self.check(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.consume(Token::RParen, "Extern params end")?;

        let return_type = if self.check(&Token::Arrow) {
            self.advance();
            self.parse_type()?
        } else {
            Type::Void
        };

        self.consume(Token::Semicolon, "Extern declaration end")?;

        Ok(Declaration::ExternFunction {
            name,
            params,
            return_type,
        })
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        self.consume(Token::LBrace, "Block start")?;
        let mut statements = Vec::new();
        while !self.check(&Token::RBrace) && self.current_token.is_some() {
            statements.push(self.parse_statement()?);
        }
        self.consume(Token::RBrace, "Block end")?;
        Ok(Block { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        if self.check(&Token::Let) {
            self.advance();
            self.parse_variable_declaration()
        } else if self.check(&Token::If) {
            self.advance();
            self.parse_if_statement()
        } else if self.check(&Token::While) {
            self.advance();
            self.parse_while_loop()
        } else if self.check(&Token::Return) {
            self.advance();
            let expr = if !self.check(&Token::Semicolon) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            self.consume(Token::Semicolon, "Return statement")?;
            Ok(Statement::Return(expr))
        } else {
            // Expression or Assignment
            let expr = self.parse_expression()?;

            if self.check(&Token::Assign) {
                self.advance();
                let value = self.parse_expression()?;
                self.consume(Token::Semicolon, "Assignment end")?;

                // Validate target is lvalue
                match &expr {
                    Expression::Variable(_) | Expression::FieldAccess { .. } => {}
                    _ => return Err("Invalid assignment target".to_string()),
                }

                Ok(Statement::Assignment {
                    target: expr,
                    value,
                })
            } else {
                self.consume(Token::Semicolon, "End of expression statement")?;
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, String> {
        // Check for optional 'mut' keyword
        let mutable = if self.check(&Token::Mut) {
            self.advance();
            true
        } else {
            false
        };

        let name = match self.current_token.take() {
            Some(Token::Identifier(s)) => {
                self.advance();
                s
            }
            t => return Err(format!("Expected identifier after 'let', found {:?}", t)),
        };

        let var_type = if self.check(&Token::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume(Token::Assign, "Variable declaration assignment")?;
        let initializer = self.parse_expression()?;
        self.consume(Token::Semicolon, "Variable declaration end")?;

        Ok(Statement::VariableDeclaration {
            name,
            mutable,
            var_type,
            initializer,
        })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.consume(Token::LParen, "If condition start")?;
        let condition = self.parse_expression()?;
        self.consume(Token::RParen, "If condition end")?;

        let then_block = self.parse_block()?;
        let else_block = if self.check(&Token::Else) {
            self.advance();
            if self.check(&Token::If) {
                self.advance();
                let if_stmt = self.parse_if_statement()?;
                Some(Block {
                    statements: vec![if_stmt],
                })
            } else {
                Some(self.parse_block()?)
            }
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_block,
            else_block,
        })
    }

    fn parse_while_loop(&mut self) -> Result<Statement, String> {
        self.consume(Token::LParen, "While condition start")?;
        let condition = self.parse_expression()?;
        self.consume(Token::RParen, "While condition end")?;

        let body = self.parse_block()?;

        Ok(Statement::While { condition, body })
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_logical_and()?;
        while self.check(&Token::LogicalOr) {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op: BinaryOp::LogicalOr,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_equality()?;
        while self.check(&Token::LogicalAnd) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op: BinaryOp::LogicalAnd,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_comparison()?;
        while let Some(op) = self.match_equality_op() {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn match_equality_op(&self) -> Option<BinaryOp> {
        match self.current_token {
            Some(Token::Eq) => Some(BinaryOp::Equal),
            Some(Token::Neq) => Some(BinaryOp::NotEqual),
            _ => None,
        }
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_term()?;
        while let Some(op) = self.match_comparison_op() {
            self.advance();
            let right = self.parse_term()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn match_comparison_op(&self) -> Option<BinaryOp> {
        match self.current_token {
            Some(Token::Lt) => Some(BinaryOp::LessThan),
            Some(Token::Gt) => Some(BinaryOp::GreaterThan),
            _ => None,
        }
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_factor()?;
        while let Some(op) = self.match_term_op() {
            self.advance();
            let right = self.parse_factor()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn match_term_op(&self) -> Option<BinaryOp> {
        match self.current_token {
            Some(Token::Plus) => Some(BinaryOp::Add),
            Some(Token::Minus) => Some(BinaryOp::Subtract),
            _ => None,
        }
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_postfix()?;
        while let Some(op) = self.match_factor_op() {
            self.advance();
            let right = self.parse_postfix()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn match_factor_op(&self) -> Option<BinaryOp> {
        match self.current_token {
            Some(Token::Star) => Some(BinaryOp::Multiply),
            Some(Token::Slash) => Some(BinaryOp::Divide),
            _ => None,
        }
    }

    fn parse_postfix(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.check(&Token::Dot) {
                self.advance();
                let name = match self.current_token.take() {
                    Some(Token::Identifier(s)) => {
                        self.advance();
                        s
                    }
                    t => return Err(format!("Expected field name, found {:?}", t)),
                };

                if self.check(&Token::LParen) {
                    let args = self.parse_args()?;
                    expr = Expression::MethodCall {
                        object: Box::new(expr),
                        method: name,
                        args,
                    };
                } else {
                    expr = Expression::FieldAccess {
                        object: Box::new(expr),
                        field: name,
                    };
                }
            } else if self.check(&Token::DoubleColon) {
                if let Expression::Variable(name) = expr {
                    self.advance();

                    if self.check(&Token::Lt) {
                        // Generic call or struct init: Foo::<T>
                        self.advance();
                        let mut generic_args = Vec::new();
                        loop {
                            generic_args.push(self.parse_type()?);
                            if self.check(&Token::Comma) {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        self.consume(Token::Gt, "End of generic args")?;

                        if self.check(&Token::LParen) {
                            let args = self.parse_args()?;
                            expr = Expression::FunctionCall {
                                name,
                                generic_args,
                                args,
                            };
                        } else if self.check(&Token::LBrace) {
                            let fields = self.parse_struct_fields()?;
                            expr = Expression::StructInit {
                                name,
                                generic_args,
                                fields,
                            };
                        } else {
                            return Err("Expected function call or struct init after generic args"
                                .to_string());
                        }
                    } else if let Some(Token::Identifier(sub_name)) = self.current_token.take() {
                        // Static Method / Path: Name::SubName
                        self.advance();
                        let full_name = format!("{}_{}", name, sub_name);

                        if self.check(&Token::LParen) {
                            let args = self.parse_args()?;
                            expr = Expression::FunctionCall {
                                name: full_name,
                                generic_args: vec![],
                                args,
                            };
                        } else {
                            expr = Expression::Variable(full_name);
                        }
                    } else {
                        return Err("Expected < or identifier after ::".to_string());
                    }
                } else {
                    return Err("Generics/Path on non-variable".to_string());
                }
            } else if self.check(&Token::LParen) {
                if let Expression::Variable(name) = expr {
                    let args = self.parse_args()?;
                    expr = Expression::FunctionCall {
                        name,
                        generic_args: vec![],
                        args,
                    };
                } else {
                    return Err("Function call on non-variable not yet supported".to_string());
                }
            } else if self.check(&Token::LBrace) {
                if let Expression::Variable(name) = expr {
                    let fields = self.parse_struct_fields()?;
                    expr = Expression::StructInit {
                        name,
                        generic_args: vec![],
                        fields,
                    };
                } else {
                    return Err("Struct init on non-identifier not supported".to_string());
                }
            } else if self.check(&Token::LBracket) {
                self.advance();
                let index = self.parse_expression()?;
                self.consume(Token::RBracket, "Index end")?;
                expr = Expression::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_struct_fields(&mut self) -> Result<Vec<(String, Expression)>, String> {
        self.consume(Token::LBrace, "Struct init start")?;
        let mut fields = Vec::new();
        if !self.check(&Token::RBrace) {
            loop {
                let name = match self.current_token.take() {
                    Some(Token::Identifier(s)) => {
                        self.advance();
                        s
                    }
                    t => return Err(format!("Expected field name, found {:?}", t)),
                };
                self.consume(Token::Colon, "Field separator")?;
                let val = self.parse_expression()?;
                fields.push((name, val));

                if self.check(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.consume(Token::RBrace, "Struct init end")?;
        Ok(fields)
    }

    fn parse_args(&mut self) -> Result<Vec<Expression>, String> {
        self.consume(Token::LParen, "Call start")?;
        let mut args = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                args.push(self.parse_expression()?);
                if self.check(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.consume(Token::RParen, "Call end")?;
        Ok(args)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        let token = self.current_token.take().ok_or("Unexpected end of input")?;

        match token {
            Token::Integer(i) => {
                self.advance();
                Ok(Expression::Literal(Literal::Integer(i)))
            }
            Token::StringLit(s) => {
                self.advance();
                Ok(Expression::Literal(Literal::String(s)))
            }
            Token::True => {
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(true)))
            }
            Token::False => {
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(false)))
            }
            Token::Identifier(name) => {
                self.advance();
                Ok(Expression::Variable(name))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(Token::RParen, "Mismatched parenthesis")?;
                Ok(expr)
            }
            Token::Minus => {
                // Unary minus for negative literals
                self.advance();
                let expr = self.parse_primary()?;
                Ok(Expression::UnaryOp {
                    op: UnaryOp::Negate,
                    operand: Box::new(expr),
                })
            }
            t => Err(format!("Expression not implemented for token {:?}", t)),
        }
    }
}
