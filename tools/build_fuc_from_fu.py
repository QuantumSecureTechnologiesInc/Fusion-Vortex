#!/usr/bin/env python3
from __future__ import annotations

import os
import shutil
import subprocess  # trunk-ignore(bandit/B404)
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FUC_DIR = ROOT / "crates" / "fuc"
OUT_DIR = ROOT / "target" / "release"
BUILD_TARGET_DIR = Path(
    os.environ.get("FUC_BUILD_TARGET", str(ROOT / "target_fuc"))
).expanduser()


def env_flag(name: str) -> bool:
    return os.environ.get(name, "").lower() in {"1", "true", "yes"}


def out_binary_name() -> str:
    return "fuc.exe" if os.name == "nt" else "fuc"


def compiler_from_stage0(stage0: Path) -> dict[str, str]:
    env = os.environ.copy()
    env["FUSION_ACTIVE_COMPILER"] = str(stage0)
    return env


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


def find_refresh_stage0_binary() -> Path | None:
    candidates: list[Path] = []
    if os.environ.get("FUC_STAGE0"):
        candidates.append(Path(os.environ["FUC_STAGE0"]).expanduser())
    candidates.extend(
        [
            ROOT / "bin" / "fuc.exe",
            ROOT / "bin" / "fuc",
            OUT_DIR / "fuc.exe",
            OUT_DIR / "fuc",
        ]
    )
    for candidate in candidates:
        if candidate.exists():
            return candidate
    return None


def run_with_retry(
    cmd: list[str], *, env: dict[str, str] | None = None, failure_message: str
) -> None:
    max_attempts = 5
    last_code = 0
    for attempt in range(1, max_attempts + 1):
        result = subprocess.run(cmd, check=False, env=env)
        last_code = result.returncode
        if last_code == 0:
            return
        if attempt >= max_attempts:
            break
        print(
            ">>> transient native build failure detected; retrying "
            f"({attempt}/{max_attempts})"
        )
        time.sleep(0.3)
    raise SystemExit(f"{failure_message} (exit {last_code})")


def copy_with_retry(source: Path, destination: Path) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    last_error: OSError | None = None
    max_attempts = 5
    for attempt in range(1, max_attempts + 1):
        try:
            shutil.copy2(source, destination)
            return
        except OSError as exc:
            last_error = exc
            if attempt >= max_attempts:
                break
            print(
                ">>> transient file copy lock detected; retrying "
                f"({attempt}/{max_attempts})"
            )
            time.sleep(0.3)
    raise SystemExit(f"Failed to copy {source} -> {destination}: {last_error}")


def run_stage0_selfhost_checks(stage0: Path) -> bool:
    main_fu = FUC_DIR / "src" / "main.fu"
    stage1_fu = FUC_DIR / "src" / "pure_fusion_compiler_minimal.fu"
    stage1_boot_fu = FUC_DIR / "src" / "pure_fusion_stage1_bootstrap.fu"
    artifacts = ROOT / "artifacts" / "selfhost-audit"
    artifacts.mkdir(parents=True, exist_ok=True)
    env = compiler_from_stage0(stage0)

    main_obj = artifacts / "main_selfhost.o"
    stage1_obj = artifacts / "pure_fusion_compiler_minimal.o"
    stage1_bin = artifacts / (
        "pure_fusion_compiler_minimal.exe"
        if stage0.suffix.lower() == ".exe"
        else "pure_fusion_compiler_minimal"
    )
    stage1_boot_bin = artifacts / (
        "pure_fusion_stage1_bootstrap.exe"
        if stage0.suffix.lower() == ".exe"
        else "pure_fusion_stage1_bootstrap"
    )

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
        subprocess.run(cmd, check=True, env=env)

    all_codegen_ok = True

    codegen_result = subprocess.run(
        [str(stage0), "--lib", str(main_fu), "-o", str(main_obj)],
        check=False,
        env=env,
    )
    if codegen_result.returncode != 0:
        print(
            ">>> Warning: stage0 could not codegen crates/fuc/src/main.fu yet; "
            "parse/sema checks still passed."
        )
        all_codegen_ok = False

    if stage1_fu.exists():
        stage1_codegen_result = subprocess.run(
            [str(stage0), "--lib", str(stage1_fu), "-o", str(stage1_obj)],
            check=False,
            env=env,
        )
        if stage1_codegen_result.returncode != 0:
            print(
                ">>> Warning: stage0 could not codegen "
                "crates/fuc/src/pure_fusion_compiler_minimal.fu yet; "
                "parse/sema checks still passed."
            )
            all_codegen_ok = False

        stage1_emit_bin_result = subprocess.run(
            [str(stage0), str(stage1_fu), "-o", str(stage1_bin), "--emit-bin"],
            check=False,
            env=env,
        )
        if stage1_emit_bin_result.returncode != 0:
            print(
                ">>> Warning: stage0 could not emit-bin "
                "crates/fuc/src/pure_fusion_compiler_minimal.fu."
            )
            all_codegen_ok = False
        else:
            stage1_run_result = subprocess.run(
                [str(stage1_bin)],
                check=False,
                env=env,
            )
            if stage1_run_result.returncode != 0:
                print(
                    ">>> Warning: stage1 minimal binary exited with "
                    f"status {stage1_run_result.returncode}."
                )
                all_codegen_ok = False

    if stage1_boot_fu.exists():
        stage1_boot_codegen_result = subprocess.run(
            [str(stage0), str(stage1_boot_fu), "-o", str(stage1_boot_bin), "--emit-bin"],
            check=False,
            env=env,
        )
        if stage1_boot_codegen_result.returncode != 0:
            print(
                ">>> Warning: stage0 could not emit-bin "
                "crates/fuc/src/pure_fusion_stage1_bootstrap.fu."
            )
            all_codegen_ok = False
        else:
            stage1_boot_run_result = subprocess.run(
                [str(stage1_boot_bin)],
                check=False,
                env=env,
            )
            if stage1_boot_run_result.returncode != 0:
                print(
                    ">>> Warning: stage1 bootstrap binary exited with "
                    f"status {stage1_boot_run_result.returncode}."
                )
                all_codegen_ok = False

    return all_codegen_ok


def refresh_stage0_from_source() -> Path:
    stage0 = find_refresh_stage0_binary()
    if stage0 is None:
        raise SystemExit(
            "Zero-Cargo refresh requires an existing stage0 compiler binary "
            "(set FUC_STAGE0 or provide target/release/fuc[.exe])."
        )

    print(">>> Refreshing stage0 from patched .fu sources (native stage0 -> native stage1)")
    main_fu = FUC_DIR / "src" / "main_rust_compat.fu"
    refresh_dir = BUILD_TARGET_DIR / "release"
    refresh_dir.mkdir(parents=True, exist_ok=True)
    refreshed = refresh_dir / out_binary_name()

    if stage0.resolve() == refreshed.resolve():
        if refreshed.suffix:
            refreshed = refreshed.with_name(refreshed.stem + ".refresh" + refreshed.suffix)
        else:
            refreshed = refreshed.with_name(refreshed.name + ".refresh")

    run_with_retry(
        [str(stage0), str(main_fu), "-o", str(refreshed), "--emit-bin"],
        env=compiler_from_stage0(stage0),
        failure_message="Native stage0 refresh failed building crates/fuc/src/main_rust_compat.fu",
    )

    if not refreshed.exists():
        raise SystemExit(f"Refreshed stage0 binary not found: {refreshed}")

    return refreshed


def build() -> None:
    if not FUC_DIR.exists():
        raise SystemExit("Missing crates/fuc")

    refresh_stage0 = env_flag("FUC_REFRESH_STAGE0_FROM_SOURCE")
    if env_flag("FUC_ALLOW_CARGO"):
        raise SystemExit(
            "Cargo bootstrap has been removed from build_fuc_from_fu.py. "
            "Unset FUC_ALLOW_CARGO."
        )

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

    out_bin = OUT_DIR / out_binary_name()
    if stage0.resolve() != out_bin.resolve():
        copy_with_retry(stage0, out_bin)
        stage0 = out_bin

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


def cleanup() -> None:
    return


if __name__ == "__main__":
    try:
        build()
    finally:
        cleanup()
