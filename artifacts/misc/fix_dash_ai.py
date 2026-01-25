import os
import re

root = 'registry/crates'

def fix_file(path):
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original = content
    
    # Fix: revert fusion_ai_core -> fusion-ai-core IF path is ../ai-core/
    # matches fusion_ai_core = { path = "../ai-core/" ... }
    # capturing the rest of the line to preserve version etc.
    content = re.sub(
        r'fusion_ai_core\s*=\s*{\s*path\s*=\s*"\.\./ai-core/?"',
        r'fusion-ai-core = { path = "../ai-core/"',
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
