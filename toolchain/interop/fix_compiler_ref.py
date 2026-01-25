import os
import re

root = 'registry/crates'

def fix_file(path):
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original = content
    
    # Replace usage of fusion_core_compiler with fusion_core in code
    # This matches `use fusion_core_compiler::...`
    content = re.sub(
        r'use fusion_core_compiler',
        r'use fusion_core',
        content
    )
    
     # Replace fully qualified paths if any
    content = re.sub(
        r'fusion_core_compiler::',
        r'fusion_core::',
        content
    )
    
    # Also need to fix Cargo.toml dependencies if they point to compiler but we want core
    # But previous scripts largely handled dependencies. This script focuses on CODE.
    # However, some Cargo.toml might still have `name = "fusion_core_compiler"` or depend on it.
    
    if content != original:
        print(f"Updating code in {path}")
        with open(path, 'w', encoding='utf-8') as f:
            f.write(content)

def fix_toml(path):
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
    original = content
    
    # If a crate depends on fusion_core_compiler, change it to fusion_core
    # But only if it's NOT the core crate itself (which defines it? no core defines fusion_core_compiler name? confusing)
    # The 'core' crate has `name = "fusion_core_compiler"`.
    # Dependent crates should depend on `fusion_core` (the runtime/types one).
    
    if 'fusion_core_compiler' in content and 'name = "fusion_core_compiler"' not in content:
        content = re.sub(r'fusion_core_compiler\s*=\s*\{', r'fusion_core = {', content)
        # Also fix simpler deps
        # content = re.sub(r'fusion_core_compiler\s*=', r'fusion_core =', content) # risky if version
        
    if content != original:
        print(f"Updating TOML in {path}")
        with open(path, 'w', encoding='utf-8') as f:
             f.write(content)

for subdir, dirs, files in os.walk(root):
    for file in files:
        path = os.path.join(subdir, file)
        if file.endswith('.rs'):
            fix_file(path)
        elif file == 'Cargo.toml':
            fix_toml(path)
