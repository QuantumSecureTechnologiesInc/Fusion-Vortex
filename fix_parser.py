import re

with open('crates/fuc/src/parser.fu', 'r') as f:
    content = f.read()

# Replace the function call mapping to handle macros using regex
pattern = r'\.map\(\|\(\(name, _\), args\)\| ast::Expression::FunctionCall \{\s*name,\s*args,\s*\}\);'
replacement = '''.map(|((name, is_macro), args)| {
                if is_macro.is_some() {
                    ast::Expression::MacroInvocation { name, args }
                } else {
                    ast::Expression::FunctionCall { name, args }
                }
            });'''

content = re.sub(pattern, replacement, content)

with open('crates/fuc/src/parser.fu', 'w') as f:
    f.write(content)
print('Updated parser.fu successfully!')
