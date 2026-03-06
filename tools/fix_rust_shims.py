import re
from pathlib import Path

STD_DIR = Path("registry/crates/std/src")


def fix_content(content: str) -> str:
    # 1. Fix Types
    content = re.sub(r"\bint\b", "i64", content)
    content = re.sub(r"\bstring\b", "String", content)
    content = re.sub(r"\bvoid\b", "()", content)

    # First pass: collect extern names
    extern_names = set()
    lines = content.splitlines()
    for line in lines:
        stripped = line.strip()
        # Robustly find extern functions
        if "extern" in stripped and "fn " in stripped:
            # Matches `extern fn foo` OR `extern "C" { fn foo`
            m = re.search(r"fn\s+(\w+)", stripped)
            if m:
                extern_names.add(m.group(1))

    # Second pass: apply fixes
    new_lines = []
    for line in lines:
        stripped = line.strip()

        # FIX EXTERN DECLARATIONS
        if stripped.startswith("extern fn") and ";" in line:
            stmt = stripped.replace("extern fn", "fn")
            new_lines.append(f'extern "C" {{ {stmt} }}')
            continue

        # FIX UNSAFE CALLS
        # For each extern name, regex replace `name(...)` with `unsafe { name(...) }`
        # But optimize: checks if line contains name
        # Also ensure we don't wrap if already wrapped or if it's a definition `fn name`

        current_line = line
        if "extern" not in current_line and "fn " not in current_line:
            for name in extern_names:
                # check if name exists in line as a word
                # logic: `\bname\(`
                pattern = rf"\b{name}\("
                if re.search(pattern, current_line):
                    if "unsafe {" not in current_line:
                        # Wrap the call: name(...) -> unsafe { name(...) }
                        # We use re.sub for this specific name
                        # Be careful with `unsafe { unsafe { ... } }` if multiple matches
                        # Use a lambda to avoid double wrapping?
                        # Simple regex: replace `name(args)` with `unsafe { name(args) }`
                        # Note: `[^;]*` is risky for nested parens.
                        # But for this simple shim file, it's likely single calls.
                        current_line = re.sub(
                            rf"(\b{name}\([^;]*\))", r"unsafe { \1 }", current_line
                        )

        new_lines.append(current_line)

    content = "\n".join(new_lines)
    return content


def main():
    if not STD_DIR.exists():
        print(f"Directory {STD_DIR} not found.")
        return

    for file_path in STD_DIR.glob("*.rs"):
        print(f"Fixing {file_path}...")
        original = file_path.read_text(encoding="utf-8")
        fixed = fix_content(original)
        file_path.write_text(fixed, encoding="utf-8")


if __name__ == "__main__":
    main()
