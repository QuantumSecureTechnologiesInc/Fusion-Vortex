import os

root_dir = "registry/crates/fusion-core/src"

def fix_double_imports(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original = content
    
    # We accidentally created use crate::compiler::compiler::MOD
    # We want use crate::compiler::MOD
    
    # We simply replace `crate::compiler::compiler::` with `crate::compiler::`
    # BUT we must be careful.
    # If the user intended to access the compiler module inside compiler, that would be `crate::compiler::compiler`.
    # But `ast`, `chunk`, etc are siblings to `compiler.rs`.
    
    # The erroneous pattern I introduced was likely replacing `crate::compiler` prefix which affected `crate::compiler::ast`.
    
    # So I will replace `crate::compiler::compiler::` with `crate::compiler::`
    
    content = content.replace("crate::compiler::compiler::", "crate::compiler::")
    
    if content != original:
        print(f"Fixing double imports in {file_path}")
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)

# Walk through the directory
for subdir, dirs, files in os.walk(root_dir):
    for file in files:
        if file.endswith(".rs"):
            fix_double_imports(os.path.join(subdir, file))
