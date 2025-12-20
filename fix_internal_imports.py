import os

root_dir = "registry/crates/fusion-core/src"
compiler_modules = [
    "ast", "chunk", "compiler", "error", "function", "lexer", 
    "parser", "semantic", "token", "type_checker", "typechecker", "value"
]

def fix_imports(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original = content
    
    # Replace `use crate::MODULE` with `use crate::compiler::MODULE`
    # only for the compiler modules moved
    for mod in compiler_modules:
        # We look for `use crate::MODULE` pattern
        # Simple replace might result in `crate::compiler::compiler::ast` if run twice or if overlaps
        # So we use targeted replacement
        search = f"use crate::{mod}"
        replace = f"use crate::compiler::{mod}"
        content = content.replace(search, replace)
        
    if content != original:
        print(f"Fixing imports in {file_path}")
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)

# Fix in src/compiler directory
compiler_dir = os.path.join(root_dir, "compiler")
for filename in os.listdir(compiler_dir):
    if filename.endswith(".rs"):
        fix_imports(os.path.join(compiler_dir, filename))

# Fix in src/vm.rs
vm_path = os.path.join(root_dir, "vm.rs")
if os.path.exists(vm_path):
    fix_imports(vm_path)
