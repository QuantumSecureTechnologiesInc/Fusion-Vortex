param(
    [string]$OutputObject = "artifacts/fusion-iot/fusion_iot.o"
)

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$sourceRel = "registry/crates/fusion-iot/src/lib.fu"
$sourceWin = Join-Path $repoRoot $sourceRel

if (-not (Get-Command wsl -ErrorAction SilentlyContinue)) {
    throw "WSL is required for stable Fusion code generation on this workspace."
}

if (-not (Test-Path $sourceWin)) {
    throw "Missing IoT source file: $sourceWin"
}

$repoWsl = (wsl wslpath -a "$repoRoot").Trim()
$outputWin = Join-Path $repoRoot $OutputObject
$outputDir = Split-Path -Parent $outputWin
New-Item -ItemType Directory -Force -Path $outputDir | Out-Null
$outputWsl = (wsl wslpath -a "$outputWin").Trim()

$tmpObj = "/tmp/fusion_iot.o"

$cmds = @(
    "cd '$repoWsl'",
    "./bin/fuc '$sourceRel' --parse-only",
    "./bin/fuc '$sourceRel' --sema-only",
    "./bin/fuc '$sourceRel' --lib -o '$tmpObj'",
    "cp '$tmpObj' '$outputWsl'",
    "ls -l '$outputWsl'"
) -join " && "

wsl bash -lc "$cmds"
if ($LASTEXITCODE -ne 0) {
    throw "IoT build failed with exit code $LASTEXITCODE"
}

Write-Host "Built IoT object at: $outputWin"
