import os

root_dir = "registry/crates/fusion-core/src"
# Exclude 'compiler' from this list to avoid ambiguity/directory conflict
modules = [
    "ast", "chunk", "error", "function", "lexer", 
    "parser", "semantic", "token", "type_checker", "typechecker", "value"
]

def fix_inline_imports(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original = content
    
    for mod in modules:
        search = f"crate::{mod}"
        replace = f"crate::compiler::{mod}"
        # This is safe because crate::compiler::mod does not contain "crate::mod" as a substring
        if search in content:
            content = content.replace(search, replace)
            
    if content != original:
        print(f"Fixing inline imports in {file_path}")
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)

for subdir, dirs, files in os.walk(root_dir):
    for file in files:
        if file.endswith(".rs"):
            fix_inline_imports(os.path.join(subdir, file))
