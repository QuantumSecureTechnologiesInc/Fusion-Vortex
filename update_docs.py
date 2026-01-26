
import os
import re

TARGET_DIR = "docs"
ROOT_README = "README.md"

REPLACEMENTS = [
    (r"# Fusion Toolchain", "# Fusion v2.0 Vortex Toolchain"),
    (r"# Fusion Programming Language", "# Fusion v2.0 Vortex Programming Language"),
    (r"Fusion Programming Language", "Fusion v2.0 Vortex Programming Language"),
    (r"The Fusion Language", "The Fusion v2.0 Vortex Language"),
    (r"Fusion VSC CLI", "Fusion v2.0 Vortex VSC CLI"),
    (r"Fusion ecosystem", "Fusion v2.0 Vortex ecosystem"),
    (r"Fusion language", "Fusion v2.0 Vortex language"),
]

def update_file(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply specific replacements first
        for pattern, replacement in REPLACEMENTS:
            content = re.sub(pattern, replacement, content)
            
        # Generalized replacement for headers: "# Fusion" -> "# Fusion v2.0 Vortex"
        # Avoid double replacing if it's already Fusion v2.0 Vortex
        content = re.sub(r"^# Fusion(?! v2\.0 Vortex)", r"# Fusion v2.0 Vortex", content, flags=re.MULTILINE)
        
        # Replace "Fusion v2.0" (older version ref) with "Fusion v2.0 Vortex" if strictly needed, 
        # but user said "Fusion v2.0 Vortex".
        
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Updated: {filepath}")
        else:
            print(f"No changes: {filepath}")

    except Exception as e:
        print(f"Error processing {filepath}: {e}")

def main():
    # Update Root README
    if os.path.exists(ROOT_README):
        update_file(ROOT_README)

    # Walk docs dir
    for root, dirs, files in os.walk(TARGET_DIR):
        for file in files:
            if file.endswith(".md"):
                update_file(os.path.join(root, file))

if __name__ == "__main__":
    main()
