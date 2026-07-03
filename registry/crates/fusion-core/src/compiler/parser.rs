use crate::compiler::ast::*;
use crate::compiler::lexer::Lexer;
use crate::compiler::token::{SpannedToken, Token};

pub struct Parser {
    lexer: Lexer,
    current_token: SpannedToken,
    peek_token: SpannedToken,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            peek_token,
        }
    }

    fn advance_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    #[allow(dead_code)]
    fn expect_peek(&mut self, token: Token) -> bool {
        if std::mem::discriminant(&self.peek_token.token) == std::mem::discriminant(&token) {
            self.advance_token();
            true
        } else {
            false
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut declarations = Vec::new();
        while self.current_token.token != Token::EOF {
            match self.current_token.token {
                Token::Fn => declarations.push(Declaration::Function(self.parse_function()?)),
                Token::Extern => declarations.push(Declaration::Extern(self.parse_extern()?)),
                Token::Struct => declarations.push(Declaration::Struct(self.parse_struct()?)),
                _ => {
                    return Err(format!(
                        "Unexpected token {:?} at start of declaration",
                        self.current_token.token
                    ));
                }
            }
            // Consume semantic tokens if necessary or just loop
            // Note: parse_function etc should consume the last token of the Decl
            self.advance_token(); // Move past the last token of the previous declaration
        }
        Ok(Program { declarations })
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        match &self.current_token.token {
            Token::Identifier(s) => match s.as_str() {
                "Int" => Ok(Type::Int),
                "Bool" => Ok(Type::Bool),
                "String" => Ok(Type::String),
                "Void" => Ok(Type::Void),
                _ => Ok(Type::Custom(s.clone())),
            },
            _ => Err(format!("Expected type, got {:?}", self.current_token.token)),
        }
    }

    fn parse_function(&mut self) -> Result<FunctionDecl, String> {
        // Current is Fn
        self.advance_token(); // Eat Fn

        let name = match &self.current_token.token {
            Token::Identifier(s) => s.clone(),
            _ => return Err("Expected function name".into()),
        };
        self.advance_token();

        if self.current_token.token != Token::LPren {
            return Err("Expected ( after function name".into());
        }

        let params = self.parse_params()?;

        // Return type annotation -> fn foo() : Int { ... }
        let return_type = if self.current_token.token == Token::Colon {
            self.advance_token();
            let t = self.parse_type()?;
            self.advance_token();
            t
        } else {
            Type::Void
        };

        if self.current_token.token != Token::LBrace {
            return Err(format!(
                "Expected {{ to start function body, got {:?}",
                self.current_token.token
            ));
        }

        let body = self.parse_block()?;

        Ok(FunctionDecl {
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_extern(&mut self) -> Result<ExternDecl, String> {
        // Current is Extern
        self.advance_token();
        if self.current_token.token != Token::Fn {
            return Err("Expected fn after extern".into());
        }
        self.advance_token();

        let name = match &self.current_token.token {
            Token::Identifier(s) => s.clone(),
            _ => return Err("Expected extern function name".into()),
        };
        self.advance_token();

        let params = self.parse_params()?;

        let return_type = if self.current_token.token == Token::Colon {
            self.advance_token();
            let t = self.parse_type()?;
            self.advance_token();
            t
        } else {
            Type::Void
        };

        if self.current_token.token != Token::SemiColon {
            return Err("Expected ; after extern declaration".into());
        }

        Ok(ExternDecl {
            name,
            params,
            return_type,
        })
    }

    fn parse_struct(&mut self) -> Result<StructDecl, String> {
        self.advance_token(); // Eat Struct
        let name = match &self.current_token.token {
            Token::Identifier(s) => s.clone(),
            _ => return Err("Expected struct name".into()),
        };
        self.advance_token();

        if self.current_token.token != Token::LBrace {
            return Err("Expected { after struct name".into());
        }
        self.advance_token();

        let mut fields = Vec::new();
        while self.current_token.token != Token::RBrace {
            let field_name = match &self.current_token.token {
                Token::Identifier(s) => s.clone(),
                _ => return Err("Expected field name".into()),
            };
            self.advance_token();

            if self.current_token.token != Token::Colon {
                return Err("Expected : after field name".into());
            }
            self.advance_token();

            let field_type = self.parse_type()?;
            self.advance_token();

            fields.push((field_name, field_type));

            if self.current_token.token == Token::Comma {
                self.advance_token();
            }
        }

        Ok(StructDecl { name, fields })
    }

    fn parse_params(&mut self) -> Result<Vec<(String, Type)>, String> {
        self.advance_token(); // Eat (
        let mut params = Vec::new();
        while self.current_token.token != Token::RPren {
            let name = match &self.current_token.token {
                Token::Identifier(s) => s.clone(),
                _ => return Err("Expected param name".into()),
            };
            self.advance_token();

            if self.current_token.token != Token::Colon {
                return Err("Expected : after param name".into());
            }
            self.advance_token();

            let param_type = self.parse_type()?;
            self.advance_token();

            params.push((name, param_type));

            if self.current_token.token == Token::Comma {
                self.advance_token();
            }
        }
        self.advance_token(); // Eat )
        Ok(params)
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        self.advance_token(); // Eat {
        let mut statements = Vec::new();
        while self.current_token.token != Token::RBrace && self.current_token.token != Token::EOF {
            statements.push(self.parse_statement()?);
            // check for semicolon?
            if self.current_token.token == Token::SemiColon {
                self.advance_token();
            }
        }
        // self.advance_token(); // Do not eat } here, let wrapper handle it or eat it?
        // Wait, parse_function calls parse_block then immediately ends. Correct context:
        // parse_block is called at start of {
        // should return when }
        // The loop condition handles }
        // But we need to consume } *after* the loop?
        // Let's verify loop exit.
        // if Token::RBrace -> loop breaks.
        // We do typically consume it here?
        // Yes.
        // Exception: If we just return statements, caller might expect to consume RBrace.
        // But standard logic usually consumes delimiters of the block unless it's a list.
        // I will consume it.
        /*
          But wait looking at parse_function:
          let body = self.parse_block()?;
          Ok(...)

          It ends there. Next iteration of loop in parse_program expects to satisfy declarations.
          Loop says `declarations.push(...)`. Then `self.advance_token()`.

          If parse_block does NOT consume '}', then `current_token` is '}'.
          parse_function returns.
          parse_program loop: `self.advance_token()` consumes '}'.
          Then next token allows checking for next Fn/Extern.

          However, `Statements` like `If` also have blocks.

          Let's decide: `parse_block` consumes `{` (already done by caller? No, caller checks it).
          Wait, in `parse_function`:
             if != LBrace return error
             body = self.parse_block()

          I implemented `self.advance_token()` inside `parse_block` as first step?
          Yes: `self.advance_token(); // Eat {`

          Then loop. Upon hitting `}`, loop breaks.
          `current_token` is `}`.
          Do we consume it?
          If I don't, `parse_function` returns with `current_token` as `}`.
          Then `parse_program` advances, consuming `}`. This works.

          What about `If` statement?
             parse_if calls parse_block.
             It returns with `}`.
             Then `else` check might look at `peek_token`?
             If I don't consume `}`, `peek` is whatever is after.
             So `current` is `}`.

          It is safer if `parse_block` leaves `current_token` AS `}`.
        */
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token.token {
            Token::Let => {
                self.advance_token();
                let name = match &self.current_token.token {
                    Token::Identifier(s) => s.clone(),
                    _ => return Err("Expected identifier in let".into()),
                };
                self.advance_token();
                let ty = if self.current_token.token == Token::Colon {
                    self.advance_token();
                    let t = self.parse_type()?;
                    self.advance_token();
                    Some(t)
                } else {
                    None
                };

                if self.current_token.token != Token::Assign {
                    return Err("Expected = in let".into());
                }
                self.advance_token();

                let expr = self.parse_expression()?; // Simple expression
                Ok(Statement::Let(name, ty, expr))
            }
            Token::Return => {
                self.advance_token();
                if self.current_token.token == Token::SemiColon {
                    Ok(Statement::Return(None))
                } else {
                    let expr = self.parse_expression()?;
                    Ok(Statement::Return(Some(expr)))
                }
            }
            Token::If => {
                self.advance_token();
                let condition = self.parse_expression()?;
                if self.current_token.token != Token::LBrace {
                    return Err("Expected { after if condition".into());
                }
                let outcome = self.parse_block()?;
                // parse_block leaves current as }
                // consume }
                self.advance_token();

                let alt = if self.current_token.token == Token::Else {
                    self.advance_token();
                    if self.current_token.token != Token::LBrace {
                        return Err("Expected { after else".into());
                    }
                    let e = self.parse_block()?;
                    self.advance_token(); // consume }
                    Some(e)
                } else {
                    None
                };

                Ok(Statement::If(condition, outcome, alt))
            }
            Token::While => {
                self.advance_token();
                let condition = self.parse_expression()?;
                if self.current_token.token != Token::LBrace {
                    return Err("Expected { after while condition".into());
                }
                let body = self.parse_block()?;
                self.advance_token(); // consume }
                Ok(Statement::While(condition, body))
            }
            Token::For => {
                // for (init; cond; inc) { body }
                self.advance_token();
                if self.current_token.token != Token::LPren {
                    return Err("Expected ( after for".into());
                }
                self.advance_token();

                // 1. Initializer
                let init = if self.current_token.token == Token::SemiColon {
                    None
                } else {
                    Some(Box::new(self.parse_statement()?))
                };
                // If parse_statement consumed semicolon? No, usually let statement does not consume semicolon.
                // But my `Let` parser does not expect semicolon?
                // Wait. In `parse_block`: `statements.push(self.parse_statement()?); if SemiColon ...`
                // `parse_statement` returns naked Statement.
                // So here I must check/consume semicolon.

                // Correction: `Let` does not consume semicolon.
                if self.current_token.token == Token::SemiColon {
                    self.advance_token();
                } else {
                    return Err("Expected ; after for initializer".into());
                }

                // 2. Condition
                let cond = if self.current_token.token == Token::SemiColon {
                    Expression::Literal(Literal::Bool(true))
                } else {
                    self.parse_expression()?
                };
                if self.current_token.token != Token::SemiColon {
                    return Err("Expected ; after for condition".into());
                }
                self.advance_token();

                // 3. Increment
                let inc = if self.current_token.token == Token::RPren {
                    None
                } else {
                    Some(self.parse_expression()?)
                };
                if self.current_token.token != Token::RPren {
                    return Err("Expected ) after for clauses".into());
                }
                self.advance_token();

                if self.current_token.token != Token::LBrace {
                    return Err("Expected { after for clauses".into());
                }
                let mut body = self.parse_block()?;
                self.advance_token(); // consume }

                // Desugar to:
                // {
                //    init;
                //    while (cond) {
                //       body;
                //       inc;
                //    }
                // }

                if let Some(increment_expr) = inc {
                    body.push(Statement::Expression(increment_expr));
                }

                let while_stmt = Statement::While(cond, body);

                let mut block_body = Vec::new();
                if let Some(init_stmt) = init {
                    block_body.push(*init_stmt);
                }
                block_body.push(while_stmt);

                Ok(Statement::Block(block_body))
            }
            Token::LBrace => {
                let blk = self.parse_block()?;
                self.advance_token(); // consume }
                Ok(Statement::Block(blk))
            }
            _ => {
                let expr = self.parse_expression()?;
                Ok(Statement::Expression(expr))
            }
        }
    }

    // Very simple expression parser (no precedence for brevity, user can ask to improve)
    // Actually, I should do at least basic binary ops.
    fn parse_expression(&mut self) -> Result<Expression, String> {
        let expr = self.parse_binary(0)?;

        if self.current_token.token == Token::Assign {
            self.advance_token();
            let value = self.parse_expression()?;

            match expr {
                Expression::Identifier(name) => {
                    return Ok(Expression::Assign(name, Box::new(value)));
                }
                Expression::Get(obj, field) => {
                    return Ok(Expression::Set(obj, field, Box::new(value)));
                }
                _ => return Err("Invalid assignment target".into()),
            }
        }

        Ok(expr)
    }

    fn parse_binary(&mut self, min_precedence: u8) -> Result<Expression, String> {
        let mut left = self.parse_primary()?;

        loop {
            // Field Access Precedence should be very high (like Call).
            // Handled here? Or inside parse_primary via loop?
            // "Pratt Parser" usually handles suffix ops (call, field) with high precedence.
            // But here I'm doing recursive descent + operator precedence for binary.

            // Let's check for Dot here?
            // `obj.field`
            if self.current_token.token == Token::Dot {
                self.advance_token();
                match &self.current_token.token {
                    Token::Identifier(field) => {
                        let field_name = field.clone();
                        self.advance_token();
                        left = Expression::Get(Box::new(left), field_name);
                        continue;
                    }
                    _ => return Err("Expected field identifier after .".into()),
                }
            } else if let Some(op) = self.get_binary_op(&self.current_token.token) {
                let precedence = self.get_precedence(&op);
                if precedence < min_precedence {
                    break;
                }
                self.advance_token();
                let right = self.parse_binary(precedence + 1)?;
                left = Expression::Binary(Box::new(left), op, Box::new(right));
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn get_binary_op(&self, token: &Token) -> Option<BinaryOp> {
        match token {
            Token::Plus => Some(BinaryOp::Add),
            Token::Minus => Some(BinaryOp::Sub),
            Token::Asterisk => Some(BinaryOp::Mul),
            Token::Slash => Some(BinaryOp::Div),
            Token::Equals => Some(BinaryOp::Equal),
            Token::NotEquals => Some(BinaryOp::NotEqual),
            Token::LessThan => Some(BinaryOp::LessThan),
            Token::GreaterThan => Some(BinaryOp::GreaterThan),
            _ => None,
        }
    }

    fn get_precedence(&self, op: &BinaryOp) -> u8 {
        match op {
            BinaryOp::Equal | BinaryOp::NotEqual | BinaryOp::LessThan | BinaryOp::GreaterThan => 1,
            BinaryOp::Add | BinaryOp::Sub => 2,
            BinaryOp::Mul | BinaryOp::Div => 3,
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match &self.current_token.token {
            Token::Identifier(s) => {
                let name = s.clone();
                self.advance_token();
                if self.current_token.token == Token::LPren {
                    // Function call
                    self.advance_token();
                    let mut args = Vec::new();
                    while self.current_token.token != Token::RPren {
                        args.push(self.parse_expression()?);
                        if self.current_token.token == Token::Comma {
                            self.advance_token();
                        }
                    }
                    self.advance_token(); // eat )
                    Ok(Expression::Call(name, args))
                } else if self.current_token.token == Token::LBrace {
                    // Struct Init: Name { field: expr, ... }
                    self.advance_token(); // eat {
                    let mut fields = Vec::new();
                    while self.current_token.token != Token::RBrace {
                        match &self.current_token.token {
                            Token::Identifier(field_name) => {
                                let f_name = field_name.clone();
                                self.advance_token();
                                if self.current_token.token != Token::Colon {
                                    return Err("Expected : in struct init".into());
                                }
                                self.advance_token();
                                let expr = self.parse_expression()?;
                                fields.push((f_name, expr));

                                if self.current_token.token == Token::Comma {
                                    self.advance_token();
                                }
                            }
                            _ => return Err("Expected field name in struct init".into()),
                        }
                    }
                    self.advance_token(); // eat }
                    Ok(Expression::StructInit(name, fields))
                } else {
                    Ok(Expression::Identifier(name))
                }
            }
            Token::Integer(i) => {
                let val = *i;
                self.advance_token();
                Ok(Expression::Literal(Literal::Integer(val)))
            }
            Token::StringLiteral(s) => {
                let val = s.clone();
                self.advance_token();
                Ok(Expression::Literal(Literal::String(val)))
            }
            Token::True => {
                self.advance_token();
                Ok(Expression::Literal(Literal::Bool(true)))
            }
            Token::False => {
                self.advance_token();
                Ok(Expression::Literal(Literal::Bool(false)))
            }
            Token::LPren => {
                self.advance_token();
                let expr = self.parse_expression()?;
                if self.current_token.token != Token::RPren {
                    return Err("Expected )".into());
                }
                self.advance_token();
                Ok(expr)
            }
            _ => Err(format!(
                "Unexpected token in expression: {:?}",
                self.current_token.token
            )),
        }
    }
}
