#!/usr/bin/env python3
from __future__ import annotations

import os
import re
import shutil
import subprocess
from pathlib import Path
import tomllib

ROOT = Path(__file__).resolve().parents[1]
FUSION_DIR = ROOT / "cmd" / "fusion"
OUT_DIR = ROOT / "target" / "release"
BUILD_TARGET_DIR = Path("/mnt/c/Projects/fusion_cli_build")
STD_RUST_DIR = Path("/mnt/c/Projects/fusion_v0_1_all_features/registry/crates/std")
CORE_RUST_DIR = Path("/mnt/c/Projects/fusion_v0_1_all_features/registry/crates/fusion-core")

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


def build_compat(source: str, *, allow_aliases: bool) -> str:
    uses = []
    if (
        re.search(r"\bfmt::", source)
        and "core::fmt::" not in source
        and "std::fmt::" not in source
        and not re.search(r"\bcommands::fmt::", source)
    ):
        uses.append("use std::fmt;")
    if "SystemTime::now" in source or "UNIX_EPOCH" in source:
        uses.append("use std::time::{SystemTime, UNIX_EPOCH};")
    if re.search(r"\bimpl\s+(Add|Sub|Mul|Div)\b", source):
        uses.append("use std::ops::{Add, Sub, Mul, Div};")
    if (
        re.search(r"\bfs::", source)
        and "tokio::fs" not in source
        and "std::fs::" not in source
        and "use std::fs" not in source
    ):
        uses.append("use std::fs;")
    if (
        re.search(r"\bCommand::new\b", source)
        and "tokio::process" not in source
        and "use std::process::Command" not in source
        and "use clap::Command" not in source
        and "clap::Command" not in source
    ):
        uses.append("use std::process::Command;")
    if re.search(r"\bRange\s*(<|::)", source) and "std::ops::Range" not in source:
        uses.append("use std::ops::Range;")
    if (
        re.search(r"(?<!std::path::)\bPath::", source)
        or re.search(r"(?<!std::path::)\bPath\b(?=[\s:<>,)])", source)
    ) and "use std::path::Path" not in source and not re.search(r"use\s+std::path::\{[^}]*\bPath\b", source):
        uses.append("use std::path::Path;")
    if (
        re.search(r"(?<!std::path::)\bPathBuf\b(?=[\s:<>,)])", source)
        and "use std::path::PathBuf" not in source
        and not re.search(r"use\s+std::path::\{[^}]*\bPathBuf\b", source)
    ):
        uses.append("use std::path::PathBuf;")
    if re.search(r"\bArc<|\bArc::", source):
        uses.append("use std::sync::Arc;")
    if re.search(r"\bFuture\b", source) and "std::future::Future" not in source:
        uses.append("use std::future::Future;")
    if re.search(r"\bPin<", source) and "std::pin::Pin" not in source:
        uses.append("use std::pin::Pin;")
    if "Context<" in source or "Poll<" in source or "Poll::" in source:
        uses.append("use std::task::{Context, Poll};")
    if "VecDeque" in source:
        uses.append("use std::collections::VecDeque;")
    if "AtomicU64" in source or "Ordering::" in source:
        uses.append("use std::sync::atomic::{AtomicU64, Ordering};")
    if "SocketAddr" in source:
        uses.append("use std::net::SocketAddr;")
    needs_duration = (
        "Duration" in source
        and "std::time::Duration" not in source
        and "chrono::Duration" not in source
        and "tokio::time::Duration" not in source
    )
    needs_instant = "Instant" in source and "std::time::Instant" not in source
    if needs_duration and needs_instant:
        uses.append("use std::time::{Duration, Instant};")
    elif needs_duration:
        uses.append("use std::time::Duration;")
    elif needs_instant:
        uses.append("use std::time::Instant;")
    if "UnixStream" in source:
        uses.append("use std::os::unix::net::UnixStream;")
    if "Stdio::" in source and "std::process::Stdio" not in source:
        uses.append("use std::process::Stdio;")
    if (
        re.search(r"\bChild\b", source)
        and "tokio::process" not in source
        and "use std::process::Child" not in source
    ):
        uses.append("use std::process::Child;")
    if (
        "BufReader::" in source
        and "std::io::BufReader" not in source
        and "use std::io::BufReader" not in source
        and "tokio::io::BufReader" not in source
        and not re.search(r"tokio::io::\{[^}]*\bBufReader\b", source)
    ):
        uses.append("use std::io::BufReader;")
    if re.search(r"\bio::", source) and "std::io::" not in source and "tokio::io::" not in source and "use std::io" not in source:
        uses.append("use std::io;")
    if re.search(r"\benv::", source) and "std::env::" not in source and "use std::env" not in source:
        uses.append("use std::env;")
    needs_write = (
        ".write_all(" in source or ".flush(" in source
    ) and "std::io::Write" not in source and "tokio::io::" not in source
    needs_read = (
        ".read_exact(" in source
        or ".read_to_end(" in source
        or (
            ".read_to_string(" in source
            and "fs::read_to_string" not in source
            and "std::fs::read_to_string" not in source
        )
    ) and "std::io::Read" not in source and "tokio::io::" not in source
    if needs_read and needs_write:
        uses.append("use std::io::{Read, Write};")
    elif needs_read:
        uses.append("use std::io::Read;")
    elif needs_write:
        uses.append("use std::io::Write;")
    if (
        ".read_line(" in source
        and "BufReader" in source
        and "tokio::io::" not in source
        and "std::io::BufRead" not in source
        and "use std::io::BufRead" not in source
    ):
        uses.append("use std::io::BufRead;")
    if (
        re.search(r"\bOrdering\b", source)
        and "Ordering::" not in source
        and "std::sync::atomic::Ordering" not in source
        and "std::cmp::Ordering" not in source
    ):
        uses.append("use std::cmp::Ordering;")

    needs = []
    if (
        re.search(r"\bFMap(<|\b)", source)
        or re.search(r"(?<!std::collections::)\bHashMap\b", source)
    ) and "use std::collections::HashMap" not in source and not re.search(r"use\s+std::collections::\{[^}]*\bHashMap\b", source):
        needs.append("HashMap")
    if (
        re.search(r"\bFBTreeMap(<|\b)", source)
        or re.search(r"(?<!std::collections::)\bBTreeMap\b", source)
    ) and "use std::collections::BTreeMap" not in source and not re.search(r"use\s+std::collections::\{[^}]*\bBTreeMap\b", source):
        needs.append("BTreeMap")
    if (
        re.search(r"\bFSet(<|\b)", source)
        or re.search(r"(?<!std::collections::)\bHashSet\b", source)
    ) and "use std::collections::HashSet" not in source and not re.search(r"use\s+std::collections::\{[^}]*\bHashSet\b", source):
        needs.append("HashSet")
    if (
        re.search(r"\bFBTreeSet(<|\b)", source)
        or re.search(r"(?<!std::collections::)\bBTreeSet\b", source)
    ) and "use std::collections::BTreeSet" not in source and not re.search(r"use\s+std::collections::\{[^}]*\bBTreeSet\b", source):
        needs.append("BTreeSet")
    if (
        re.search(r"(?<!std::collections::)\bBinaryHeap\b", source)
    ) and "use std::collections::BinaryHeap" not in source and not re.search(r"use\s+std::collections::\{[^}]*\bBinaryHeap\b", source):
        needs.append("BinaryHeap")
    if needs:
        if len(needs) == 1:
            uses.insert(0, f"use std::collections::{needs[0]};")
        else:
            uses.insert(0, f"use std::collections::{{{', '.join(needs)}}};")

    alias_lines = []
    if allow_aliases:
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


def inject_compat(source: str, *, allow_aliases: bool) -> str:
    if "__FU_COMPAT_START__" in source:
        return source
    source = source.replace("std::collections::FMap", "HashMap")
    source = source.replace("std::collections::FSet", "HashSet")
    source = source.replace("Type::FString", "Type::String")
    source = source.replace("ast::Type::FString", "ast::Type::String")
    source = source.replace("Literal::FString", "Literal::String")
    source = source.replace("Value::FString", "Value::String")
    compat = build_compat(source, allow_aliases=allow_aliases)
    if not compat:
        return add_pub_to_toplevel(source)
    lines = source.splitlines()
    insert_at = 0
    while insert_at < len(lines) and (lines[insert_at].startswith("#!") or lines[insert_at].startswith("//!")):
        insert_at += 1
    merged = "\n".join(lines[:insert_at] + [compat.rstrip()] + lines[insert_at:]) + "\n"
    return add_pub_to_toplevel(merged)


def sync_fu_to_rs(base: Path) -> int:
    copied = 0
    for path in base.rglob("*.fu"):
        allow_aliases = True
        if "registry/crates/std" in str(path):
            allow_aliases = False
        rs_path = path.with_suffix(".rs")
        rs_path.write_text(
            inject_compat(path.read_text(encoding="utf-8"), allow_aliases=allow_aliases),
            encoding="utf-8",
        )
        copied += 1
    return copied


def parse_toml(path: Path) -> dict:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def toml_quote(value: str) -> str:
    escaped = value.replace("\\", "\\\\").replace('"', '\\"')
    return f"\"{escaped}\""


def toml_value(value):
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, int):
        return str(value)
    if isinstance(value, float):
        return str(value)
    if isinstance(value, str):
        return toml_quote(value)
    if isinstance(value, list):
        inner = ", ".join(toml_value(v) for v in value)
        return f"[{inner}]"
    if isinstance(value, dict):
        inner = ", ".join(f"{k} = {toml_value(v)}" for k, v in value.items())
        return f"{{ {inner} }}"
    raise TypeError(f"Unsupported toml value: {value!r}")


def write_toml(doc: dict, path: Path) -> None:
    lines = []
    for section, content in doc.items():
        if section.startswith("[["):
            continue
        if section == "bin" and isinstance(content, list):
            for item in content:
                lines.append("[[bin]]")
                for ik, iv in item.items():
                    lines.append(f"{ik} = {toml_value(iv)}")
                lines.append("")
            continue
        if section:
            lines.append(f"[{section}]")
        if isinstance(content, dict):
            for key, value in content.items():
                if isinstance(value, list) and value and isinstance(value[0], dict):
                    for item in value:
                        lines.append(f"[[{section}.{key}]]" if section else f"[[{key}]]")
                        for ik, iv in item.items():
                            lines.append(f"{ik} = {toml_value(iv)}")
                else:
                    lines.append(f"{key} = {toml_value(value)}")
        lines.append("")
    path.write_text("\n".join(lines).rstrip() + "\n", encoding="utf-8")


def resolve_workspace_entry(name: str, entry: dict, workspace: dict) -> dict:
    if entry.get("workspace") is True:
        ws_deps = workspace.get("dependencies", {})
        if name not in ws_deps:
            raise SystemExit(f"Workspace dependency '{name}' not found in root Fusion.toml")
        resolved = ws_deps[name]
        if isinstance(resolved, str):
            merged = {"version": resolved}
        elif isinstance(resolved, dict):
            merged = dict(resolved)
        else:
            raise SystemExit(f"Unsupported workspace dependency format for {name}")
        # Merge explicit fields from the crate's dependency entry (e.g., features)
        for key, value in entry.items():
            if key == "workspace":
                continue
            merged[key] = value
        if "path" in merged:
            merged["path"] = str((ROOT / merged["path"]).resolve())
        return merged
    return dict(entry)


def resolve_package_fields(pkg: dict, workspace_pkg: dict) -> dict:
    out = dict(pkg)
    for key in ("version", "edition", "authors", "license", "repository", "homepage", "rust-version"):
        if isinstance(out.get(key), dict) and out[key].get("workspace") is True:
            out.pop(key, None)
            if key in workspace_pkg:
                out[key] = workspace_pkg[key]
    return out


def materialize_cargo_from_fusion(crate_dir: Path, workspace: dict) -> None:
    fusion_toml = crate_dir / "Fusion.toml"
    if not fusion_toml.exists():
        return
    data = parse_toml(fusion_toml)
    workspace_pkg = workspace.get("package", {})

    cargo_doc: dict = {}
    cargo_doc["package"] = resolve_package_fields(data.get("package", {}), workspace_pkg)

    for section in ("dependencies", "dev-dependencies", "build-dependencies"):
        deps = data.get(section, {})
        if not deps:
            continue
        resolved = {}
        ws_deps = workspace.get("dependencies", {})
        drop_for_cli = set()
        if crate_dir.resolve() == FUSION_DIR.resolve():
            drop_for_cli = {"fusion_std", "fusion_core", "fusion-core", "fusion-agentic-core"}
        drop_for_crate = set()
        if crate_dir.name in {"fusion-core", "std"}:
            drop_for_crate.add("fusion_std")
        for name, entry in deps.items():
            if name in drop_for_cli or name in drop_for_crate:
                continue
            if isinstance(entry, str):
                if name in {"fusion_core", "fusion-core"} and CORE_RUST_DIR.exists():
                    resolved[name] = {"path": str(CORE_RUST_DIR.resolve()), "package": "fusion-core"}
                elif name == "fusion_std" and STD_RUST_DIR.exists():
                    resolved[name] = {"path": str(STD_RUST_DIR.resolve())}
                elif name in ws_deps:
                    ws_entry = ws_deps[name]
                    if isinstance(ws_entry, str):
                        resolved[name] = {"version": ws_entry}
                    elif isinstance(ws_entry, dict):
                        ws_entry = dict(ws_entry)
                        if "path" in ws_entry:
                            ws_entry["path"] = str((ROOT / ws_entry["path"]).resolve())
                        resolved[name] = ws_entry
                    else:
                        raise SystemExit(f"Unsupported workspace dependency format for {name}")
                else:
                    resolved[name] = {"version": entry}
            elif isinstance(entry, dict):
                if name in {"fusion_core", "fusion-core"} and CORE_RUST_DIR.exists():
                    resolved[name] = {"path": str(CORE_RUST_DIR.resolve()), "package": "fusion-core"}
                elif name == "fusion_std" and STD_RUST_DIR.exists():
                    resolved[name] = {"path": str(STD_RUST_DIR.resolve())}
                else:
                    resolved[name] = resolve_workspace_entry(name, entry, workspace)
            else:
                raise SystemExit(f"Unsupported dependency spec for {name} in {fusion_toml}")
        cargo_doc[section] = resolved

    if "bin" in data:
        cargo_doc["bin"] = data["bin"]
    if "features" in data:
        cargo_doc["features"] = data["features"]
    if "lib" in data:
        cargo_doc["lib"] = data["lib"]

    cargo_toml = crate_dir / "Cargo.toml"
    write_toml(cargo_doc, cargo_toml)


def gather_path_deps(crate_dir: Path, workspace: dict) -> list[Path]:
    fusion_toml = crate_dir / "Fusion.toml"
    if not fusion_toml.exists():
        return []
    data = parse_toml(fusion_toml)
    deps = data.get("dependencies", {})
    paths: list[Path] = []
    for name, entry in deps.items():
        if isinstance(entry, dict):
            resolved = resolve_workspace_entry(name, entry, workspace)
            path = resolved.get("path")
            if path:
                dep_path = (crate_dir / path).resolve()
                if STD_RUST_DIR.exists() and dep_path == STD_RUST_DIR.resolve():
                    continue
                if CORE_RUST_DIR.exists() and dep_path == CORE_RUST_DIR.resolve():
                    continue
                paths.append(dep_path)
    return paths


def materialize_tree(root: Path, workspace: dict, seen: set[Path]) -> None:
    if root in seen:
        return
    seen.add(root)
    materialize_cargo_from_fusion(root, workspace)
    sync_fu_to_rs(root)
    for dep in gather_path_deps(root, workspace):
        materialize_tree(dep, workspace, seen)


def build() -> None:
    if not FUSION_DIR.exists():
        raise SystemExit("Missing cmd/fusion")
    workspace = parse_toml(ROOT / "Fusion.toml").get("workspace", {})

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    BUILD_TARGET_DIR.mkdir(parents=True, exist_ok=True)
    build_env = os.environ.copy()
    build_env["CARGO_TARGET_DIR"] = str(BUILD_TARGET_DIR)

    seen: set[Path] = set()
    materialize_tree(FUSION_DIR, workspace, seen)

    fusion_cargo = FUSION_DIR / "Cargo.toml"
    subprocess.run(
        ["cargo", "build", "--release", "--manifest-path", str(fusion_cargo)],
        check=True,
        env=build_env,
    )

    fusion_out = BUILD_TARGET_DIR / "release" / "fusion"
    if fusion_out.exists():
        shutil.copy2(fusion_out, OUT_DIR / "fusion")


def cleanup() -> None:
    for path in FUSION_DIR.rglob("*.rs"):
        path.unlink()
    for path in FUSION_DIR.rglob("Cargo.toml"):
        path.unlink()
    for path in FUSION_DIR.rglob("Cargo.lock"):
        path.unlink()


if __name__ == "__main__":
    try:
        build()
    finally:
        cleanup()
