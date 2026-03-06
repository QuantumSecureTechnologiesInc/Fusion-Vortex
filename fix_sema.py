import re

with open('crates/fuc/src/sema.fu', 'r') as f:
    content = f.read()

# Check if MacroInvocation is already handled
if 'ast::Expression::MacroInvocation' in content:
    print('MacroInvocation already handled')
    exit(0)

# Find the position of the last closing braces and add the macro code before them
# Look for the pattern at the end of analyze_expression
pattern = r'(\s+ast::Expression::Dereference\(inner\) => \{[^}]+\}\s+\}\s+\})(\s*;\s*TypedExpression \{\s*node,\s*ty,\s*span: expr\.span,\s*\}\s*\}\s*\})'

match = re.search(pattern, content, re.DOTALL)
if match:
    # Insert MacroInvocation case before the final closing
    insert_pos = match.end(1)
    
    macro_case = '''
            ast::Expression::MacroInvocation { name, args } => {
                self.expand_macro(&name, args, &expr.span)
            }'''
    
    content = content[:insert_pos] + macro_case + content[insert_pos:]
    print('Added MacroInvocation case')
else:
    print('Pattern not found, trying alternative approach')
    # Try simpler approach - find the last match arm before TypedExpression return
    old = '''            }
        };
        TypedExpression {
            node,
            ty,
            span: expr.span,
        }
    }
}'''
    new = '''            }
            ast::Expression::MacroInvocation { name, args } => {
                self.expand_macro(&name, args, &expr.span)
            }
        };
        TypedExpression {
            node,
            ty,
            span: expr.span,
        }
    }

    /// Expand built-in macros
    fn expand_macro(&mut self, name: &str, args: FVec<Spanned<ast::Expression>>, span: &Span) -> (TypedExpressionKind, ast::Type) {
        match name {
            "println" => {
                if args.is_empty() {
                    let fmt_str = TypedExpression {
                        node: TypedExpressionKind::StringLiteral("\\n".to_string()),
                        ty: ast::Type::String,
                        span: span.clone(),
                    };
                    return (
                        TypedExpressionKind::FunctionCall {
                            name: "printf".to_string(),
                            args: vec![fmt_str],
                        },
                        ast::Type::Int,
                    );
                }
                let mut typed_args: FVec<TypedExpression> = args
                    .into_iter()
                    .map(|arg| self.analyze_expression(arg))
                    .collect();
                if let TypedExpressionKind::StringLiteral(ref mut s) = typed_args[0].node {
                    s.push('\\n');
                }
                (
                    TypedExpressionKind::FunctionCall {
                        name: "printf".to_string(),
                        args: typed_args,
                    },
                    ast::Type::Int,
                )
            }
            "print" => {
                let typed_args: FVec<TypedExpression> = args
                    .into_iter()
                    .map(|arg| self.analyze_expression(arg))
                    .collect();
                (
                    TypedExpressionKind::FunctionCall {
                        name: "printf".to_string(),
                        args: typed_args,
                    },
                    ast::Type::Int,
                )
            }
            "vec" => {
                let typed_elements: FVec<_> = args
                    .into_iter()
                    .map(|e| self.analyze_expression(e))
                    .collect();
                if typed_elements.is_empty() {
                    self.report_error(span.clone(), "Empty vec![] is not yet supported".to_string());
                    return (TypedExpressionKind::ArrayLiteral(vec![]), ast::Type::Void);
                }
                let first_ty = typed_elements[0].ty.clone();
                let array_ty = ast::Type::Array(Box::new(first_ty), typed_elements.len());
                (TypedExpressionKind::ArrayLiteral(typed_elements), array_ty)
            }
            _ => {
                self.report_error(span.clone(), format!("Unknown macro: {}!", name));
                (TypedExpressionKind::IntLiteral(0), ast::Type::Void)
            }
        }
    }
}'''
    if old in content:
        content = content.replace(old, new)
        print('Added macro expansion using alternative approach')
    else:
        print('Alternative pattern also not found')
        # Debug: show last 200 chars
        print('Last 200 chars:', repr(content[-200:]))

with open('crates/fuc/src/sema.fu', 'w') as f:
    f.write(content)
print('Done')
