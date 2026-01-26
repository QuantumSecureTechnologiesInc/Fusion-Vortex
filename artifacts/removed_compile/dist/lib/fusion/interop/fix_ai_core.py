import os
import re

root = 'registry/crates'

def fix_file(path):
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original = content
    
    # regex 1: replace fusion-ai-core with fusion_ai_core (dependency key)
    # usually: fusion-ai-core = {
    content = re.sub(
        r'fusion-ai-core\s*=',
        r'fusion_ai_core =',
        content
    )

    # regex 2: update version of fusion_ai_core to 0.2.0
    # matches fusion_ai_core = { ... version = "0.1.0"
    content = re.sub(
        r'(fusion_ai_core\s*=\s*{[^}]*version\s*=\s*)["\']0\.1\.0["\']',
        r'\g<1>"0.2.0"',
        content
    )

    if content != original:
        print(f"Updating {path}")
        with open(path, 'w', encoding='utf-8') as f:
            f.write(content)

for subdir, dirs, files in os.walk(root):
    for file in files:
        if file == 'Cargo.toml':
            path = os.path.join(subdir, file)
            try:
                fix_file(path)
            except Exception as e:
                print(f"Error processing {path}: {e}")
