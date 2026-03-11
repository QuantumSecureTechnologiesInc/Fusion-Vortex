param(
    [switch]$SkipRefresh
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $false

function Write-Utf8NoBom {
    param(
        [string]$Path,
        [string]$Content
    )
    $encoding = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllText($Path, $Content, $encoding)
}

function Invoke-StrictStep {
    param(
        [string]$Name,
        [scriptblock]$Action
    )
    Write-Host ">>> $Name" -ForegroundColor Yellow
    & $Action
    Write-Host ">>> $Name [ok]" -ForegroundColor Green
}

function Invoke-CompilerWithRetry {
    param(
        [string]$Compiler,
        [string[]]$CompilerArgs,
        [string]$FailureMessage,
        [int]$MaxAttempts = 5
    )

    for ($attempt = 1; $attempt -le $MaxAttempts; $attempt++) {
        $old = $ErrorActionPreference
        $ErrorActionPreference = "Continue"
        try {
            $output = & $Compiler @CompilerArgs 2>&1
        } finally {
            $ErrorActionPreference = $old
        }
        if ($null -ne $output) {
            foreach ($line in $output) {
                Write-Host ([string]$line)
            }
        }

        $exitCode = $LASTEXITCODE
        if ($exitCode -eq 0) {
            return
        }

        $hasMappedSectionLock = $false
        if ($null -ne $output) {
            foreach ($line in $output) {
                if ([string]$line -match '(?i)user-mapped section open') {
                    $hasMappedSectionLock = $true
                    break
                }
            }
        }

        if ($hasMappedSectionLock -and $attempt -lt $MaxAttempts) {
            Write-Host ">>> transient file lock detected; retrying compiler invocation ($attempt/$MaxAttempts)" -ForegroundColor Yellow
            Start-Sleep -Milliseconds 300
            continue
        }

        throw $FailureMessage
    }

    throw $FailureMessage
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

Write-Host ">>> Fusion Strict Self-Host Gate" -ForegroundColor Cyan

$Root = (Get-Item $PSScriptRoot).Parent.FullName
$Compiler = Join-Path $Root "target\release\fuc.exe"
$Artifacts = Join-Path $Root "artifacts\selfhost-gate"
$ModProbeDir = Join-Path $Artifacts "mod_probe"
$Stage1Exe = Join-Path $Artifacts "pure_fusion_compiler_minimal_direct.exe"
$GateLog = Join-Path $Artifacts "strict_selfhost_gate.log"
$AllowedWarningPatterns = @()
if (-not [string]::IsNullOrWhiteSpace($env:FUSION_ALLOWED_WARNING_REGEX)) {
    $AllowedWarningPatterns = $env:FUSION_ALLOWED_WARNING_REGEX -split ";"
}

New-Item -ItemType Directory -Force -Path $Artifacts | Out-Null
if (Test-Path $GateLog) {
    Remove-Item $GateLog -Force
}
$env:FUSION_ACTIVE_COMPILER = $Compiler
$env:FUSION_STRICT_UNRESOLVED_CALLS = "1"

$TranscriptStarted = $false
try {
    Start-Transcript -Path $GateLog -Force | Out-Null
    $TranscriptStarted = $true

    Invoke-StrictStep -Name "1/5 refreshed compiler build" -Action {
        if (-not $SkipRefresh) {
            $oldRefresh = $env:FUC_REFRESH_STAGE0_FROM_SOURCE
            $env:FUC_REFRESH_STAGE0_FROM_SOURCE = "1"
            try {
                & python (Join-Path $Root "tools\build_fuc_from_fu.py")
                if ($LASTEXITCODE -ne 0) {
                    throw "Refreshed compiler build failed"
                }
            } finally {
                if ($null -eq $oldRefresh) {
                    Remove-Item Env:FUC_REFRESH_STAGE0_FROM_SOURCE -ErrorAction SilentlyContinue
                } else {
                    $env:FUC_REFRESH_STAGE0_FROM_SOURCE = $oldRefresh
                }
            }
        }

        if (-not (Test-Path $Compiler)) {
            throw "Active compiler missing after refresh: $Compiler"
        }
    }

    Invoke-StrictStep -Name "2/5 mod probe pass" -Action {
        New-Item -ItemType Directory -Force -Path $ModProbeDir | Out-Null

        $helperSrc = Join-Path $ModProbeDir "helper.fu"
        $mainSrc = Join-Path $ModProbeDir "main.fu"
        $modProbeExe = Join-Path $ModProbeDir "mod_probe_gate.exe"

        $helperContent = @"
pub fn add_one(x: int) -> int {
    return x + 1;
}
"@
        Write-Utf8NoBom -Path $helperSrc -Content $helperContent

        $mainContent = @"
mod helper;
extern fn printf(fmt: string, ...) -> int;

pub fn main() -> int {
    let v = helper::add_one(4);
    printf("v=%d\n", v);
    return 0;
}
"@
        Write-Utf8NoBom -Path $mainSrc -Content $mainContent

        Invoke-CompilerWithRetry -Compiler $Compiler -CompilerArgs @($mainSrc, "-o", $modProbeExe, "--emit-bin") -FailureMessage "mod probe compile failed"

        $probeOutput = & $modProbeExe
        if ($LASTEXITCODE -ne 0) {
            throw "mod probe executable failed"
        }

        $probeText = [string]::Join("`n", $probeOutput)
        if ($probeText -notmatch "v=5") {
            throw "mod probe output mismatch (expected v=5)"
        }
    }

    Invoke-StrictStep -Name "3/5 stage1 direct API pass" -Action {
        $Stage1Src = Join-Path $Root "crates\fuc\src\pure_fusion_compiler_minimal.fu"
        $NativeMainSrc = Join-Path $Root "crates\fuc\src\main.fu"
        $NativeMainExe = Join-Path $Artifacts "main_native_gate.exe"
        $DeepProbeSrc = Join-Path $Root "crates\fuc\src\stage1_status_deep_probe.fu"
        $DeepProbeExe = Join-Path $Artifacts "stage1_status_deep_probe_gate.exe"
        $FullProbeSrc = Join-Path $Root "crates\fuc\src\stage1_full_status_probe.fu"
        $FullProbeExe = Join-Path $Artifacts "stage1_full_status_probe_gate.exe"

        if (-not (Test-Path $Stage1Src)) {
            throw "Missing stage1 source: $Stage1Src"
        }

        Invoke-CompilerWithRetry -Compiler $Compiler -CompilerArgs @($Stage1Src, "-o", $Stage1Exe, "--emit-bin") -FailureMessage "stage1 direct API compile failed"

        & $Stage1Exe
        if ($LASTEXITCODE -ne 0) {
            throw "stage1 direct API normal run failed"
        }

        & $Stage1Exe "--emit-bin" "--opt" "--bad-target"
        if ($LASTEXITCODE -eq 0) {
            throw "stage1 direct API negative path did not fail"
        }

        if (Test-Path $NativeMainSrc) {
            Invoke-CompilerWithRetry -Compiler $Compiler -CompilerArgs @($NativeMainSrc, "-o", $NativeMainExe, "--emit-bin") -FailureMessage "native main emit-bin compile failed"
            & $NativeMainExe
            if ($LASTEXITCODE -ne 0) {
                throw "native main executable failed"
            }
        }

        if (Test-Path $DeepProbeSrc) {
            Invoke-CompilerWithRetry -Compiler $Compiler -CompilerArgs @($DeepProbeSrc, "-o", $DeepProbeExe, "--emit-bin") -FailureMessage "stage1 deep probe compile failed"
            $deepOutput = & $DeepProbeExe
            if ($LASTEXITCODE -ne 0) {
                throw "stage1 deep probe executable failed"
            }
            $deepText = [string]::Join("`n", $deepOutput)
            if ($deepText -notmatch "p_parser=0 s_parser=0 p_sema=0 s_sema=0") {
                throw "stage1 deep probe output mismatch (expected all zero strict statuses)"
            }
        }

        if (Test-Path $FullProbeSrc) {
            Invoke-CompilerWithRetry -Compiler $Compiler -CompilerArgs @($FullProbeSrc, "-o", $FullProbeExe, "--emit-bin") -FailureMessage "stage1 full probe compile failed"
            $fullOutput = & $FullProbeExe
            if ($LASTEXITCODE -ne 0) {
                throw "stage1 full probe executable failed"
            }
            $fullText = [string]::Join("`n", $fullOutput)
            if ($fullText -notmatch "p1=0 s1=0 p2=1") {
                throw "stage1 full probe output mismatch (expected p1=0 s1=0 p2=1)"
            }
        }
    }

    Invoke-StrictStep -Name "4/5 full native regression" -Action {
        & powershell -ExecutionPolicy Bypass -File (Join-Path $Root "scripts\run_native_regression.ps1")
        if ($LASTEXITCODE -ne 0) {
            throw "native regression failed"
        }
    }

    Invoke-StrictStep -Name "5/5 bootstrap and packaging" -Action {
        & powershell -ExecutionPolicy Bypass -File (Join-Path $Root "scripts\bootstrap_native.ps1")
        if ($LASTEXITCODE -ne 0) {
            throw "bootstrap/package failed"
        }
    }
}
finally {
    if ($TranscriptStarted) {
        Stop-Transcript | Out-Null
    }
}

$UnexpectedWarnings = Get-UnexpectedWarningLines -Path $GateLog -AllowPatterns $AllowedWarningPatterns
if ($UnexpectedWarnings.Count -gt 0) {
    Write-Host ">>> Unexpected warnings detected during strict gate:" -ForegroundColor Red
    foreach ($line in ($UnexpectedWarnings | Select-Object -First 20)) {
        Write-Host "    $line" -ForegroundColor Red
    }
    if ($UnexpectedWarnings.Count -gt 20) {
        $remaining = $UnexpectedWarnings.Count - 20
        Write-Host "    ... $remaining additional warning lines omitted" -ForegroundColor Red
    }
    throw "Strict self-host gate failed due to unexpected warnings (log: $GateLog)."
}

Write-Host ">>> warning audit [ok]" -ForegroundColor Green
Write-Host ">>> Strict self-host gate passed." -ForegroundColor Green
