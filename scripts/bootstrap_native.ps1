param(
    [switch]$SkipRegression,
    [switch]$SkipPackage
)

$ErrorActionPreference = "Stop"

function Require-Command {
    param([string]$Name)
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "Missing required command: $Name"
    }
}

function Write-Utf8NoBom {
    param(
        [string]$Path,
        [string]$Content
    )
    $encoding = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllText($Path, $Content, $encoding)
}

Write-Host ">>> Fusion Native Bootstrap" -ForegroundColor Cyan

$Root = (Get-Item $PSScriptRoot).Parent.FullName
$TargetDir = Join-Path $Root "target_fuc_native"
$ReleaseDir = Join-Path $Root "target\release"
$BinDir = Join-Path $Root "bin"
$ArtifactsDir = Join-Path $Root "artifacts\native-bootstrap"
$SmokeSrc = Join-Path $ArtifactsDir "smoke_main.fu"
$SmokeExe = Join-Path $ArtifactsDir "smoke_main.exe"
$SmokeObj = Join-Path $ArtifactsDir "smoke_main.o"
$Stage1Src = Join-Path $Root "crates\fuc\src\pure_fusion_compiler_minimal.fu"
$Stage1Obj = Join-Path $ArtifactsDir "pure_fusion_compiler_minimal.o"
$Stage1BootSrc = Join-Path $Root "crates\fuc\src\pure_fusion_stage1_bootstrap.fu"
$Stage1BootExe = Join-Path $ArtifactsDir "pure_fusion_stage1_bootstrap.exe"

Require-Command "llc"
Require-Command "clang"

New-Item -ItemType Directory -Force -Path $ArtifactsDir | Out-Null
New-Item -ItemType Directory -Force -Path $BinDir | Out-Null

$CompilerExe = Join-Path $ReleaseDir "fuc.exe"
if (-not (Test-Path $CompilerExe)) {
    throw "Native compiler not found: $CompilerExe. Strict no-Cargo mode requires an existing native compiler binary."
}
Write-Host ">>> Reusing existing native compiler: $CompilerExe" -ForegroundColor Yellow

Copy-Item -Force $CompilerExe (Join-Path $BinDir "fuc.exe")

Write-Utf8NoBom -Path $SmokeSrc -Content @'
pub fn main() -> int {
    return 0;
}
'@

Write-Host ">>> Running smoke build with native fuc.exe..." -ForegroundColor Yellow
& $CompilerExe $SmokeSrc -o $SmokeExe --emit-bin
if ($LASTEXITCODE -ne 0) {
    throw "Smoke emit-bin compilation failed"
}

& $SmokeExe
if ($LASTEXITCODE -ne 0) {
    throw "Smoke executable failed"
}

& $CompilerExe --lib (Join-Path $Root "registry\crates\fusion-iot\src\lib.fu") -o (Join-Path $ArtifactsDir "fusion_iot_native.o")
if ($LASTEXITCODE -ne 0) {
    throw "IoT library native object build failed"
}

if (Test-Path $Stage1Src) {
    Write-Host ">>> Validating stage1 compiler source as native object..." -ForegroundColor Yellow
    & $CompilerExe --lib $Stage1Src -o $Stage1Obj
    if ($LASTEXITCODE -ne 0) {
        throw "Stage1 compiler source object build failed"
    }
}

if (Test-Path $Stage1BootSrc) {
    Write-Host ">>> Validating stage1 bootstrap source as native executable..." -ForegroundColor Yellow
    & $CompilerExe $Stage1BootSrc -o $Stage1BootExe --emit-bin
    if ($LASTEXITCODE -ne 0) {
        throw "Stage1 bootstrap executable build failed"
    }
    & $Stage1BootExe
    if ($LASTEXITCODE -ne 0) {
        throw "Stage1 bootstrap executable failed"
    }
}

if (-not $SkipRegression) {
    Write-Host ">>> Running native regression fixtures..." -ForegroundColor Yellow
    & powershell -ExecutionPolicy Bypass -File (Join-Path $Root "scripts\run_native_regression.ps1")
    if ($LASTEXITCODE -ne 0) {
        throw "Regression run failed"
    }
}

if (-not $SkipPackage) {
    Write-Host ">>> Building native package..." -ForegroundColor Yellow
    & powershell -ExecutionPolicy Bypass -File (Join-Path $Root "scripts\package_native.ps1")
    if ($LASTEXITCODE -ne 0) {
        throw "Packaging failed"
    }
}

Write-Host ">>> Native bootstrap complete." -ForegroundColor Green
Write-Host "    Compiler: $CompilerExe" -ForegroundColor Green
Write-Host "    Bin copy: $(Join-Path $BinDir 'fuc.exe')" -ForegroundColor Green
