#!/usr/bin/env python3
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parent
DEPS = [
    ROOT,
    ROOT.parent / "fusion_frontend",
    ROOT.parent / "fusion_ir",
    ROOT.parent / "fusion_project",
    ROOT.parent / "fusion_vm",
]

FU_COMPAT = """\
// __FU_COMPAT_START__
use std::fs;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

type FBool = bool;
type FChar = char;
type FInt = i32;
type FI64 = i64;
type FString = String;
type FU32 = u32;
type FU64 = u64;
type FSize = usize;
type FVec<T> = Vec<T>;
type FMap<K, V> = HashMap<K, V>;
type FBTreeMap<K, V> = BTreeMap<K, V>;
type FSet<T> = HashSet<T>;
type FBTreeSet<T> = BTreeSet<T>;
// __FU_COMPAT_END__
"""

def inject_compat(source: str) -> str:
    if "__FU_COMPAT_START__" in source:
        return source
    lines = source.splitlines()
    insert_at = 0
    while insert_at < len(lines) and lines[insert_at].startswith("#!"):
        insert_at += 1
    return "\n".join(lines[:insert_at] + [FU_COMPAT.rstrip()] + lines[insert_at:]) + "\n"

copied = 0
for base in DEPS:
    if not base.exists():
        continue
    for path in base.rglob("*.fu"):
        rs_path = path.with_suffix(".rs")
        src = path.read_text(encoding="utf-8")
        rs_path.write_text(inject_compat(src), encoding="utf-8")
        copied += 1

print(f"synced_fu_to_rs={copied}")

# build fusionc using synced .rs
cmd = ["cargo", "build", "--release"]
proc = subprocess.run(cmd, cwd=str(ROOT))
if proc.returncode != 0:
    sys.exit(proc.returncode)
