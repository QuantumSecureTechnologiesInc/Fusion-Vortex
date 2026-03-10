param(
    [switch]$SkipRegression,
    [switch]$SkipPackage
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $false

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

function Get-UnexpectedWarningLines {
    param(
        [string]$Path,
        [string[]]$AllowPatterns = @()
    )

    if (-not (Test-Path $Path)) {
        return @()
    }

    $warningRegex = '(?i)\bwarning:|>>> Warning:'
    $unexpected = New-Object System.Collections.Generic.List[string]

    foreach ($line in (Get-Content -Path $Path)) {
        if ($line -notmatch $warningRegex) {
            continue
        }

        $allowed = $false
        foreach ($pattern in $AllowPatterns) {
            if ([string]::IsNullOrWhiteSpace($pattern)) {
                continue
            }
            if ($line -match $pattern) {
                $allowed = $true
                break
            }
        }

        if (-not $allowed) {
            $unexpected.Add($line)
        }
    }

    return $unexpected
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
$Stage1Exe = Join-Path $ArtifactsDir "pure_fusion_compiler_minimal.exe"
$Stage1BootSrc = Join-Path $Root "crates\fuc\src\pure_fusion_stage1_bootstrap.fu"
$Stage1BootExe = Join-Path $ArtifactsDir "pure_fusion_stage1_bootstrap.exe"
$NativeMainSrc = Join-Path $Root "crates\fuc\src\main.fu"
$NativeMainExe = Join-Path $ArtifactsDir "main_native_bootstrap.exe"
$Stage1DeepProbeSrc = Join-Path $Root "crates\fuc\src\stage1_status_deep_probe.fu"
$Stage1DeepProbeExe = Join-Path $ArtifactsDir "stage1_status_deep_probe_bootstrap.exe"
$Stage1FullProbeSrc = Join-Path $Root "crates\fuc\src\stage1_full_status_probe.fu"
$Stage1FullProbeExe = Join-Path $ArtifactsDir "stage1_full_status_probe_bootstrap.exe"
$BootstrapLog = Join-Path $ArtifactsDir "bootstrap_native.log"
$AllowedWarningPatterns = @()
if (-not [string]::IsNullOrWhiteSpace($env:FUSION_ALLOWED_WARNING_REGEX)) {
    $AllowedWarningPatterns = $env:FUSION_ALLOWED_WARNING_REGEX -split ";"
}
$TranscriptStarted = $false

Require-Command "llc"
Require-Command "clang"

New-Item -ItemType Directory -Force -Path $ArtifactsDir | Out-Null
New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
if (Test-Path $BootstrapLog) {
    Remove-Item $BootstrapLog -Force
}

trap {
    if ($TranscriptStarted) {
        Stop-Transcript | Out-Null
        $TranscriptStarted = $false
    }
    throw
}

Start-Transcript -Path $BootstrapLog -Force | Out-Null
$TranscriptStarted = $true

$CompilerExe = Join-Path $ReleaseDir "fuc.exe"
if (-not (Test-Path $CompilerExe)) {
    throw "Native compiler not found: $CompilerExe. Strict no-Cargo mode requires an existing native compiler binary."
}
Write-Host ">>> Reusing existing native compiler: $CompilerExe" -ForegroundColor Yellow
$env:FUSION_ACTIVE_COMPILER = $CompilerExe
$env:FUSION_STRICT_UNRESOLVED_CALLS = "1"

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
    Write-Host ">>> Validating stage1 compiler source as native executable..." -ForegroundColor Yellow
    & $CompilerExe $Stage1Src -o $Stage1Exe --emit-bin
    if ($LASTEXITCODE -ne 0) {
        throw "Stage1 compiler source executable build failed"
    }
    & $Stage1Exe
    if ($LASTEXITCODE -ne 0) {
        throw "Stage1 compiler source executable failed"
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

if (Test-Path $NativeMainSrc) {
    Write-Host ">>> Validating native main entry as executable..." -ForegroundColor Yellow
    & $CompilerExe $NativeMainSrc -o $NativeMainExe --emit-bin
    if ($LASTEXITCODE -ne 0) {
        throw "Native main executable build failed"
    }
    & $NativeMainExe
    if ($LASTEXITCODE -ne 0) {
        throw "Native main executable failed"
    }
}

if (Test-Path $Stage1DeepProbeSrc) {
    Write-Host ">>> Validating stage1 deep parser/sema probe..." -ForegroundColor Yellow
    & $CompilerExe $Stage1DeepProbeSrc -o $Stage1DeepProbeExe --emit-bin
    if ($LASTEXITCODE -ne 0) {
        throw "Stage1 deep probe executable build failed"
    }
    $deepOutput = & $Stage1DeepProbeExe
    if ($LASTEXITCODE -ne 0) {
        throw "Stage1 deep probe executable failed"
    }
    $deepText = [string]::Join("`n", $deepOutput)
    if ($deepText -notmatch "p_parser=0 s_parser=0 p_sema=0 s_sema=0") {
        throw "Stage1 deep probe output mismatch (expected all zero strict statuses)"
    }
}

if (Test-Path $Stage1FullProbeSrc) {
    Write-Host ">>> Validating stage1 full status probe..." -ForegroundColor Yellow
    & $CompilerExe $Stage1FullProbeSrc -o $Stage1FullProbeExe --emit-bin
    if ($LASTEXITCODE -ne 0) {
        throw "Stage1 full probe executable build failed"
    }
    $fullOutput = & $Stage1FullProbeExe
    if ($LASTEXITCODE -ne 0) {
        throw "Stage1 full probe executable failed"
    }
    $fullText = [string]::Join("`n", $fullOutput)
    if ($fullText -notmatch "p1=0 s1=0 p2=1") {
        throw "Stage1 full probe output mismatch (expected p1=0 s1=0 p2=1)"
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

$UnexpectedWarnings = @()
if ($TranscriptStarted) {
    Stop-Transcript | Out-Null
    $TranscriptStarted = $false
}
$UnexpectedWarnings = Get-UnexpectedWarningLines -Path $BootstrapLog -AllowPatterns $AllowedWarningPatterns
if ($UnexpectedWarnings.Count -gt 0) {
    Write-Host ">>> Unexpected warnings detected during bootstrap:" -ForegroundColor Red
    foreach ($line in ($UnexpectedWarnings | Select-Object -First 20)) {
        Write-Host "    $line" -ForegroundColor Red
    }
    if ($UnexpectedWarnings.Count -gt 20) {
        $remaining = $UnexpectedWarnings.Count - 20
        Write-Host "    ... $remaining additional warning lines omitted" -ForegroundColor Red
    }
    throw "Bootstrap failed due to unexpected warnings (log: $BootstrapLog)."
}

Write-Host ">>> warning audit [ok]" -ForegroundColor Green
Write-Host ">>> Native bootstrap complete." -ForegroundColor Green
Write-Host "    Compiler: $CompilerExe" -ForegroundColor Green
Write-Host "    Bin copy: $(Join-Path $BinDir 'fuc.exe')" -ForegroundColor Green
