#!/usr/bin/env python3
from __future__ import annotations

import shutil
import subprocess
from pathlib import Path
import os
import re

ROOT = Path(__file__).resolve().parents[1]
FUC_DIR = ROOT / "crates" / "fuc"
DEFAULT_STD = ROOT / "registry" / "crates" / "std"
STD_RUST_DIR = Path(os.environ.get("FUSION_STD_PATH", "")).expanduser() if os.environ.get("FUSION_STD_PATH") else DEFAULT_STD
OUT_DIR = ROOT / "target" / "release"
BUILD_TARGET_DIR = Path(os.environ.get("FUC_BUILD_TARGET", str(ROOT / "target_fuc"))).expanduser()

ALIAS_DEFS = [
    ("FBool", "type FBool = bool;"),
    ("FChar", "type FChar = char;"),
    ("FInt", "type FInt = i32;"),
    ("FI64", "type FI64 = i64;"),
    ("FString", "type FString = String;"),
    ("FU32", "type FU32 = u32;"),
    ("FU64", "type FU64 = u64;"),
    ("FSize", "type FSize = usize;"),
    ("FVec", "type FVec<T> = Vec<T>;"),
    ("FMap", "type FMap<K, V> = HashMap<K, V>;"),
    ("FBTreeMap", "type FBTreeMap<K, V> = BTreeMap<K, V>;"),
    ("FSet", "type FSet<T> = HashSet<T>;"),
    ("FBTreeSet", "type FBTreeSet<T> = BTreeSet<T>;"),
]


def add_pub_to_toplevel(source: str) -> str:
    lines = []
    for line in source.splitlines():
        stripped = line.lstrip()
        if stripped == line:
            if stripped.startswith(("struct ", "enum ", "type ", "fn ", "trait ", "const ", "static ", "mod ")):
                if not stripped.startswith("pub "):
                    line = "pub " + line
        lines.append(line)
    return "\n".join(lines) + "\n"


def build_compat(source: str) -> str:
    uses = []
    if re.search(r"\bfmt::", source) and "std::fmt::" not in source and "core::fmt::" not in source:
        uses.append("use std::fmt;")
    if re.search(r"\bfs::", source) and "std::fs::" not in source:
        uses.append("use std::fs;")
    if re.search(r"\bCommand\b", source):
        uses.append("use std::process::Command;")
    if re.search(r"\bRange\s*(<|::)", source) and "std::ops::Range" not in source:
        uses.append("use std::ops::Range;")
    if re.search(r"\bPath\b", source) and "std::path::Path" not in source:
        uses.append("use std::path::Path;")
    if re.search(r"\bPathBuf\b", source) and "std::path::PathBuf" not in source:
        uses.append("use std::path::PathBuf;")

    needs = []
    if re.search(r"\b(HashMap|FMap)(<|\b)", source) and "std::collections::HashMap" not in source:
        needs.append("HashMap")
    if re.search(r"\b(BTreeMap|FBTreeMap)(<|\b)", source) and "std::collections::BTreeMap" not in source:
        needs.append("BTreeMap")
    if re.search(r"\b(HashSet|FSet)(<|\b)", source) and "std::collections::HashSet" not in source:
        needs.append("HashSet")
    if re.search(r"\b(BTreeSet|FBTreeSet)(<|\b)", source) and "std::collections::BTreeSet" not in source:
        needs.append("BTreeSet")
    if needs:
        if len(needs) == 1:
            uses.insert(0, f"use std::collections::{needs[0]};")
        else:
            uses.insert(0, f"use std::collections::{{{', '.join(needs)}}};")

    alias_lines = []
    for name, line in ALIAS_DEFS:
        if re.search(rf"\b{name}(<|\b)", source):
            alias_lines.append(f"#[allow(missing_docs, dead_code)] {line}")

    if not uses and not alias_lines:
        return ""

    parts = ["// __FU_COMPAT_START__", "#![allow(missing_docs)]"]
    parts.extend(uses)
    parts.extend(alias_lines)
    parts.append("// __FU_COMPAT_END__")
    return "\n".join(parts)


def fix_fstring_variants(source: str) -> str:
    return source.replace("Type::FString", "Type::String").replace("ast::Type::FString", "ast::Type::String")


def wrap_unsafe_gep(source: str) -> str:
    lines = []
    for line in source.splitlines():
        if "builder.build_gep(" in line and "unsafe" not in line:
            stripped = line.strip()
            trailing_comma = stripped.endswith(",")
            if trailing_comma:
                stripped = stripped[:-1].rstrip()
                lines.append(f"unsafe {{ {stripped} }},")
            else:
                lines.append(f"unsafe {{ {stripped} }}")
        else:
            lines.append(line)
    return "\n".join(lines) + "\n"


def inject_compat(source: str) -> str:
    if "__FU_COMPAT_START__" in source:
        return source
    compat = build_compat(source)
    if not compat:
        return add_pub_to_toplevel(source)
    lines = source.splitlines()
    insert_at = 0
    while insert_at < len(lines) and (lines[insert_at].startswith("#!") or lines[insert_at].startswith("//!")):
        insert_at += 1
    merged = "\n".join(lines[:insert_at] + [compat.rstrip()] + lines[insert_at:]) + "\n"
    merged = fix_fstring_variants(merged)
    merged = wrap_unsafe_gep(merged)
    return add_pub_to_toplevel(merged)


def sync_fu_to_rs(base: Path) -> int:
    copied = 0
    for path in base.rglob("*.fu"):
        rs_path = path.with_suffix(".rs")
        rs_path.write_text(inject_compat(path.read_text(encoding="utf-8")), encoding="utf-8")
        copied += 1
    return copied


def write_cargo_from_fusion(fusion_toml: Path, cargo_toml: Path, *, stdlib_path: Path | None) -> None:
    content = fusion_toml.read_text(encoding="utf-8")
    if stdlib_path is not None:
        content = content.replace('fusion_std = "1.0.0"', f'fusion_std = {{ path = "{stdlib_path.as_posix()}" }}')
    else:
        content = content.replace('fusion_std = "1.0.0"\n', "")
    cargo_toml.write_text(content, encoding="utf-8")


def build() -> None:
    if not FUC_DIR.exists():
        raise SystemExit("Missing crates/fuc")
    if not STD_RUST_DIR.exists():
        raise SystemExit(f"Missing rust stdlib at {STD_RUST_DIR}")

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    BUILD_TARGET_DIR.mkdir(parents=True, exist_ok=True)
    build_env = os.environ.copy()
    build_env["CARGO_TARGET_DIR"] = str(BUILD_TARGET_DIR)

    # Generate Cargo.toml for fuc from Fusion.toml, pointing to rust stdlib.
    fuc_cargo = FUC_DIR / "Cargo.toml"
    write_cargo_from_fusion(FUC_DIR / "Fusion.toml", fuc_cargo, stdlib_path=STD_RUST_DIR)

    # Sync .fu -> .rs for compiler only.
    copied = sync_fu_to_rs(FUC_DIR)
    print(f"synced_fu_to_rs={copied}")

    # Build rust stdlib first.
    subprocess.run(
        ["cargo", "build", "--release", "--manifest-path", str(STD_RUST_DIR / "Cargo.toml")],
        check=True,
        env=build_env,
    )

    # Build compiler.
    subprocess.run(
        ["cargo", "build", "--release", "--manifest-path", str(fuc_cargo)],
        check=True,
        env=build_env,
    )

    # Copy outputs to root target/release
    stdlib_out = BUILD_TARGET_DIR / "release" / "libfusion_std.a"
    fuc_out = BUILD_TARGET_DIR / "release" / "fuc"
    if stdlib_out.exists():
        shutil.copy2(stdlib_out, OUT_DIR / "libfusion_std.a")
    else:
        deps_dir = BUILD_TARGET_DIR / "release" / "deps"
        if deps_dir.exists():
            rlibs = sorted(deps_dir.glob("libfusion_std-*.rlib"), key=lambda p: p.stat().st_mtime, reverse=True)
            if rlibs:
                shutil.copy2(rlibs[0], OUT_DIR / "libfusion_std.rlib")
    if fuc_out.exists():
        shutil.copy2(fuc_out, OUT_DIR / "fuc")


def cleanup() -> None:
    for path in FUC_DIR.rglob("*.rs"):
        path.unlink()
    for name in ("Cargo.toml", "Cargo.lock"):
        p = FUC_DIR / name
        if p.exists():
            p.unlink()


if __name__ == "__main__":
    try:
        build()
    finally:
        cleanup()
