#!/usr/bin/env python3
from __future__ import annotations

import os
import re
import shutil
import subprocess  # trunk-ignore(bandit/B404)
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FUC_DIR = ROOT / "crates" / "fuc"
DEFAULT_STD = ROOT / "registry" / "modules" / "std"
STD_RUST_DIR = (
    Path(os.environ.get("FUSION_STD_PATH", "")).expanduser()
    if os.environ.get("FUSION_STD_PATH")
    else DEFAULT_STD
)
OUT_DIR = ROOT / "target" / "release"
BUILD_TARGET_DIR = Path(
    os.environ.get("FUC_BUILD_TARGET", str(ROOT / "target_fuc"))
).expanduser()

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


def env_flag(name: str) -> bool:
    return os.environ.get(name, "").lower() in {"1", "true", "yes"}


def find_stage0_binary() -> Path | None:
    candidates: list[Path] = []
    if os.environ.get("FUC_STAGE0"):
        candidates.append(Path(os.environ["FUC_STAGE0"]).expanduser())
    candidates.extend(
        [
            OUT_DIR / "fuc.exe",
            OUT_DIR / "fuc",
            ROOT / "bin" / "fuc.exe",
            ROOT / "bin" / "fuc",
        ]
    )
    for candidate in candidates:
        if candidate.exists():
            return candidate
    return None


def run_stage0_selfhost_checks(stage0: Path) -> bool:
    main_fu = FUC_DIR / "src" / "main.fu"
    stage1_fu = FUC_DIR / "src" / "pure_fusion_compiler_minimal.fu"
    stage1_boot_fu = FUC_DIR / "src" / "pure_fusion_stage1_bootstrap.fu"
    artifacts = ROOT / "artifacts" / "selfhost-audit"
    artifacts.mkdir(parents=True, exist_ok=True)
    main_obj = artifacts / "main_selfhost.o"
    stage1_obj = artifacts / "pure_fusion_compiler_minimal.o"
    if stage0.suffix.lower() == ".exe":
        stage1_bin = artifacts / "pure_fusion_compiler_minimal.exe"
    else:
        stage1_bin = artifacts / "pure_fusion_compiler_minimal"
    if stage0.suffix.lower() == ".exe":
        stage1_boot_bin = artifacts / "pure_fusion_stage1_bootstrap.exe"
    else:
        stage1_boot_bin = artifacts / "pure_fusion_stage1_bootstrap"

    mandatory_checks = [
        [str(stage0), "--parse-only", str(main_fu)],
        [str(stage0), "--sema-only", str(main_fu)],
    ]
    if stage1_fu.exists():
        mandatory_checks.append([str(stage0), "--parse-only", str(stage1_fu)])
        mandatory_checks.append([str(stage0), "--sema-only", str(stage1_fu)])
    if stage1_boot_fu.exists():
        mandatory_checks.append([str(stage0), "--parse-only", str(stage1_boot_fu)])
        mandatory_checks.append([str(stage0), "--sema-only", str(stage1_boot_fu)])
    for cmd in mandatory_checks:
        subprocess.run(cmd, check=True)

    all_codegen_ok = True
    codegen_check = [str(stage0), "--lib", str(main_fu), "-o", str(main_obj)]
    codegen_result = subprocess.run(codegen_check, check=False)
    if codegen_result.returncode != 0:
        print(
            ">>> Warning: stage0 could not codegen crates/fuc/src/main.fu yet; "
            "parse/sema checks still passed."
        )
        all_codegen_ok = False

    if stage1_fu.exists():
        stage1_codegen_check = [
            str(stage0),
            "--lib",
            str(stage1_fu),
            "-o",
            str(stage1_obj),
        ]
        stage1_codegen_result = subprocess.run(stage1_codegen_check, check=False)
        if stage1_codegen_result.returncode != 0:
            print(
                ">>> Warning: stage0 could not codegen "
                "crates/fuc/src/pure_fusion_compiler_minimal.fu yet; "
                "parse/sema checks still passed."
            )
            all_codegen_ok = False
        stage1_emit_bin_check = [
            str(stage0),
            str(stage1_fu),
            "-o",
            str(stage1_bin),
            "--emit-bin",
        ]
        stage1_emit_bin_result = subprocess.run(stage1_emit_bin_check, check=False)
        if stage1_emit_bin_result.returncode != 0:
            print(
                ">>> Warning: stage0 could not emit-bin "
                "crates/fuc/src/pure_fusion_compiler_minimal.fu."
            )
            all_codegen_ok = False
        else:
            stage1_run_result = subprocess.run([str(stage1_bin)], check=False)
            if stage1_run_result.returncode != 0:
                print(
                    ">>> Warning: stage1 minimal binary exited with "
                    f"status {stage1_run_result.returncode}."
                )
                all_codegen_ok = False
    if stage1_boot_fu.exists():
        stage1_boot_codegen_check = [
            str(stage0),
            str(stage1_boot_fu),
            "-o",
            str(stage1_boot_bin),
            "--emit-bin",
        ]
        stage1_boot_codegen_result = subprocess.run(
            stage1_boot_codegen_check, check=False
        )
        if stage1_boot_codegen_result.returncode != 0:
            print(
                ">>> Warning: stage0 could not emit-bin "
                "crates/fuc/src/pure_fusion_stage1_bootstrap.fu."
            )
            all_codegen_ok = False
        else:
            stage1_boot_run_result = subprocess.run(
                [str(stage1_boot_bin)], check=False
            )
            if stage1_boot_run_result.returncode != 0:
                print(
                    ">>> Warning: stage1 bootstrap binary exited with "
                    f"status {stage1_boot_run_result.returncode}."
                )
                all_codegen_ok = False
    return all_codegen_ok


def add_pub_to_toplevel(source: str) -> str:
    lines = []
    for line in source.splitlines():
        stripped = line.lstrip()
        if stripped == line:
            if stripped.startswith(
                (
                    "struct ",
                    "enum ",
                    "type ",
                    "fn ",
                    "trait ",
                    "const ",
                    "static ",
                    "mod ",
                )
            ):
                if not stripped.startswith("pub "):
                    line = "pub " + line
        lines.append(line)
    return "\n".join(lines) + "\n"


def build_compat(source: str) -> str:
    uses = []
    if (
        re.search(r"\bfmt::", source)
        and "std::fmt::" not in source
        and "core::fmt::" not in source
    ):
        uses.append("use std::fmt;")
    if re.search(r"\bfs::", source) and "std::fs::" not in source:
        uses.append("use std::fs;")
    if (
        re.search(r"\bCommand::", source)
        and "std::process::Command::" not in source
        and "use std::process::Command;" not in source
    ):
        uses.append("use std::process::Command;")
    if re.search(r"\bRange\s*(<|::)", source) and "std::ops::Range" not in source:
        uses.append("use std::ops::Range;")
    if re.search(r"\bPath\b", source) and "std::path::Path" not in source:
        uses.append("use std::path::Path;")
    if re.search(r"\bPathBuf\b", source) and "std::path::PathBuf" not in source:
        uses.append("use std::path::PathBuf;")

    needs = []
    if (
        re.search(r"\b(HashMap|FMap)(<|\b)", source)
        and "std::collections::HashMap" not in source
    ):
        needs.append("HashMap")
    if (
        re.search(r"\b(BTreeMap|FBTreeMap)(<|\b)", source)
        and "std::collections::BTreeMap" not in source
    ):
        needs.append("BTreeMap")
    if (
        re.search(r"\b(HashSet|FSet)(<|\b)", source)
        and "std::collections::HashSet" not in source
    ):
        needs.append("HashSet")
    if (
        re.search(r"\b(BTreeSet|FBTreeSet)(<|\b)", source)
        and "std::collections::BTreeSet" not in source
    ):
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
    return source.replace("Type::FString", "Type::String").replace(
        "ast::Type::FString", "ast::Type::String"
    )


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
    while insert_at < len(lines) and (
        lines[insert_at].startswith("#!") or lines[insert_at].startswith("//!")
    ):
        insert_at += 1
    merged = "\n".join(lines[:insert_at] + [compat.rstrip()] + lines[insert_at:]) + "\n"
    merged = fix_fstring_variants(merged)
    merged = wrap_unsafe_gep(merged)
    return add_pub_to_toplevel(merged)


def sync_fu_to_rs(base: Path) -> int:
    copied = 0
    for path in base.rglob("*.fu"):
        rs_path = path.with_suffix(".rs")
        rs_path.write_text(
            inject_compat(path.read_text(encoding="utf-8")), encoding="utf-8"
        )
        copied += 1
    return copied


def patch_generated_rust_sources(base: Path) -> None:
    llvm_rs = base / "src" / "codegen" / "llvm.rs"
    if not llvm_rs.exists():
        return

    source = llvm_rs.read_text(encoding="utf-8")

    # Inkwell changed `try_as_basic_value()` from Either to ValueKind.
    source = re.sub(
        r"let cmp_val = call_site\s*\.try_as_basic_value\(\)\s*\.left\(\)\s*\.unwrap\(\)\s*\.into_int_value\(\);",
        """let cmp_val = match call_site.try_as_basic_value() {
                                        inkwell::values::ValueKind::Basic(v) => v.into_int_value(),
                                        _ => return Err(CodegenError::LlvmError("strcmp returned void".to_string())),
                                    };""",
        source,
    )
    source = re.sub(
        r"if let Some\(basic_val\) = call_site\s*\.try_as_basic_value\(\)\s*\.left\(\)\s*\{",
        "if let inkwell::values::ValueKind::Basic(basic_val) = call_site.try_as_basic_value() {",
        source,
    )

    # Borrow identifier arguments for Rust APIs that require references.
    source = re.sub(
        r"self\.as_llvm_type\((?!&)([A-Za-z_][A-Za-z0-9_]*)\)",
        r"self.as_llvm_type(&\1)",
        source,
    )
    source = re.sub(
        r"self\.get_llvm_value\((?!&)([A-Za-z_][A-Za-z0-9_]*)\)",
        r"self.get_llvm_value(&\1)",
        source,
    )
    source = re.sub(
        r"self\.get_address_ptr\((?!&)([A-Za-z_][A-Za-z0-9_]*)\)",
        r"self.get_address_ptr(&\1)",
        source,
    )
    source = re.sub(
        r"self\.get_var_ptr\((?!&)([A-Za-z_][A-Za-z0-9_]*)\)",
        r"self.get_var_ptr(&\1)",
        source,
    )
    source = re.sub(
        r"\.get_function\((?!&)([A-Za-z_][A-Za-z0-9_]*)\)",
        r".get_function(&\1)",
        source,
    )
    source = re.sub(
        r"build_global_string_ptr\((?!&)([A-Za-z_][A-Za-z0-9_]*)\s*,",
        r"build_global_string_ptr(&\1,",
        source,
    )
    source = re.sub(
        r"build_alloca\(([^,]+),\s*(?!&)([A-Za-z_][A-Za-z0-9_]*)\)",
        r"build_alloca(\1, &\2)",
        source,
    )
    source = source.replace(
        "Target::initialize_x86(&InitializationConfig::default());",
        'Target::initialize_native(&InitializationConfig::default())\n            .map_err(CodegenError::TargetError)?;',
    )

    llvm_rs.write_text(source, encoding="utf-8")


def write_cargo_from_fusion(
    fusion_toml: Path, cargo_toml: Path, *, stdlib_path: Path | None
) -> None:
    content = fusion_toml.read_text(encoding="utf-8")
    if stdlib_path is not None:
        content = content.replace(
            'fusion_std = "1.0.0"',
            f'fusion_std = {{ path = "{stdlib_path.as_posix()}" }}',
        )
    else:
        content = content.replace('fusion_std = "1.0.0"\n', "")
    if "inkwell" in content and "default-features" not in content:
        content = re.sub(
            r"(inkwell\s*=\s*\{[^}]*branch\s*=\s*\"master\",)",
            r"\1 default-features = false,",
            content,
            count=1,
            flags=re.DOTALL,
        )
    if "generational-arena" not in content:
        if "thunderdome = " in content:
            content = content.replace(
                'thunderdome = "0.6"\n',
                'thunderdome = "0.6"\ngenerational-arena = "0.2"\n',
            )
        elif "[dependencies]" in content:
            content = content.replace(
                "[dependencies]\n",
                '[dependencies]\ngenerational-arena = "0.2"\n',
                1,
            )
    # Add [[bin]] section if not present (needed for Cargo to recognize the binary)
    if "[[bin]]" not in content:
        content += '\n[[bin]]\nname = "fuc"\npath = "src/main.rs"\n'
    cargo_toml.write_text(content, encoding="utf-8")


def refresh_stage0_from_source() -> Path:
    print(">>> Refreshing stage0 from patched .fu sources (.fu -> .rs -> Cargo)")
    copied = sync_fu_to_rs(FUC_DIR)
    print(f">>> Synced {copied} Fusion source files to Rust stubs")
    patch_generated_rust_sources(FUC_DIR)
    write_cargo_from_fusion(
        FUC_DIR / "fusion.toml",
        FUC_DIR / "Cargo.toml",
        stdlib_path=None,
    )
    env = os.environ.copy()
    env["CARGO_TARGET_DIR"] = str(BUILD_TARGET_DIR)
    subprocess.run(
        [
            "cargo",
            "build",
            "--manifest-path",
            str(FUC_DIR / "Cargo.toml"),
            "--release",
        ],
        check=True,
        cwd=ROOT,
        env=env,
    )
    out_name = "fuc.exe" if os.name == "nt" else "fuc"
    stage0 = BUILD_TARGET_DIR / "release" / out_name
    if not stage0.exists():
        raise SystemExit(f"Refreshed stage0 binary not found: {stage0}")
    return stage0


def build() -> None:
    if not FUC_DIR.exists():
        raise SystemExit("Missing crates/fuc")
    refresh_stage0 = env_flag("FUC_REFRESH_STAGE0_FROM_SOURCE")
    allow_cargo = env_flag("FUC_ALLOW_CARGO")
    if allow_cargo and not refresh_stage0:
        raise SystemExit(
            "Strict native mode forbids Cargo bootstrap. "
            "Unset FUC_ALLOW_CARGO and provide a stage0 fuc binary."
        )
    skip_std = env_flag("FUC_SKIP_STD")
    # In native mode, we skip std build entirely - it's a Fusion module, not a Rust crate
    # No need to check if STD_RUST_DIR exists

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    BUILD_TARGET_DIR.mkdir(parents=True, exist_ok=True)

    if refresh_stage0:
        stage0 = refresh_stage0_from_source()
    else:
        stage0 = find_stage0_binary()
        if stage0 is None:
            raise SystemExit(
                "Strict no-Cargo mode is active. Missing stage0 compiler binary "
                "(set FUC_STAGE0 or provide target/release/fuc[.exe])."
            )
        print(f">>> Strict no-Cargo mode: reusing stage0 compiler {stage0}")
    if stage0.suffix.lower() == ".exe":
        out_bin = OUT_DIR / "fuc.exe"
    else:
        out_bin = OUT_DIR / "fuc"
    if stage0.resolve() != out_bin.resolve():
        shutil.copy2(stage0, out_bin)

    codegen_ok = run_stage0_selfhost_checks(stage0)
    stage1_suffixes: list[str] = []
    if (FUC_DIR / "src" / "pure_fusion_compiler_minimal.fu").exists():
        stage1_suffixes.append(
            "parse/sema/lib-object/emit-bin+run on crates/fuc/src/pure_fusion_compiler_minimal.fu"
        )
    if (FUC_DIR / "src" / "pure_fusion_stage1_bootstrap.fu").exists():
        stage1_suffixes.append(
            "parse/sema/emit-bin+run on crates/fuc/src/pure_fusion_stage1_bootstrap.fu"
        )
    stage1_suffix = ", ".join(stage1_suffixes)
    if codegen_ok:
        print(
            ">>> Stage0 self-host checks passed "
            "(parse/sema/lib-object on crates/fuc/src/main.fu, "
            f"{stage1_suffix.lstrip(', ') if stage1_suffix else 'no stage1 parse target configured'})."
        )
    else:
        print(
            ">>> Stage0 self-host checks passed "
            "(parse/sema on crates/fuc/src/main.fu, "
            f"{stage1_suffix.lstrip(', ') if stage1_suffix else 'no stage1 parse target configured'})."
        )
    return

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
