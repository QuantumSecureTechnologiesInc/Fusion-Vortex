#!/usr/bin/env python3
from __future__ import annotations

import shutil
import subprocess
from pathlib import Path
import tomllib

ROOT = Path(__file__).resolve().parents[1]
CRATE_DIR = ROOT / "ecosystem" / "core_runtime_evolution"
OUT_DIR = ROOT / "dist" / "core_runtime_evolution"
BUILD_TARGET_DIR = Path("/mnt/c/Projects/core_runtime_evolution_build")


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


def materialize_cargo_from_fusion(crate_dir: Path) -> None:
    fusion_toml = crate_dir / "Fusion.toml"
    if not fusion_toml.exists():
        raise SystemExit(f"Missing {fusion_toml}")
    data = parse_toml(fusion_toml)

    cargo_doc: dict = {}
    cargo_doc["package"] = data.get("package", {})

    for section in ("dependencies", "dev-dependencies", "build-dependencies"):
        deps = data.get(section, {})
        if not deps:
            continue
        resolved = {}
        for name, entry in deps.items():
            if isinstance(entry, str):
                resolved[name] = {"version": entry}
            elif isinstance(entry, dict):
                entry = dict(entry)
                if "path" in entry:
                    entry["path"] = str((crate_dir / entry["path"]).resolve())
                resolved[name] = entry
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


def sync_fu_to_rs(base: Path) -> int:
    copied = 0
    for path in base.rglob("*.fu"):
        rs_path = path.with_suffix(".rs")
        rs_path.write_text(path.read_text(encoding="utf-8"), encoding="utf-8")
        copied += 1
    return copied


def main() -> None:
    if not CRATE_DIR.exists():
        raise SystemExit(f"Missing {CRATE_DIR}")

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    BUILD_TARGET_DIR.mkdir(parents=True, exist_ok=True)

    materialize_cargo_from_fusion(CRATE_DIR)
    copied = sync_fu_to_rs(CRATE_DIR)
    print(f"synced_fu_to_rs={copied}")

    cargo_toml = CRATE_DIR / "Cargo.toml"
    cmd = [
        "cargo",
        "build",
        "--release",
        "--manifest-path",
        str(cargo_toml),
        "--target-dir",
        str(BUILD_TARGET_DIR),
    ]
    subprocess.check_call(cmd)

    release_dir = BUILD_TARGET_DIR / "release"
    for path in release_dir.glob("libfusion_runtime_core_evolution*.rlib"):
        shutil.copy2(path, OUT_DIR / path.name)


if __name__ == "__main__":
    main()
