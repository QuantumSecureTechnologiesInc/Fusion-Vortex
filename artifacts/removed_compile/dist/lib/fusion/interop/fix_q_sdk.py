import os
import re

root = 'registry/crates'

def fix_file(path):
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original = content
    
    # Fix: add package="fusion_q_sim" to fusion_quantum_sdk dependency pointing to q-sim
    content = re.sub(
        r'fusion_quantum_sdk\s*=\s*{\s*path\s*=\s*"\.\./q-sim"\s*}',
        r'fusion_quantum_sdk = { path = "../q-sim", package = "fusion_q_sim" }',
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
