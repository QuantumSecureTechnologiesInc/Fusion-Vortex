$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $false

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

Write-Host ">>> Fusion Native Regression" -ForegroundColor Cyan

$Root = (Get-Item $PSScriptRoot).Parent.FullName
$Compiler = Join-Path $Root "target\release\fuc.exe"
if (-not (Test-Path $Compiler)) {
    throw "Compiler not found: $Compiler (run scripts/bootstrap_native.ps1 first)"
}
$env:FUSION_ACTIVE_COMPILER = $Compiler
$env:FUSION_STRICT_UNRESOLVED_CALLS = "1"

$Artifacts = Join-Path $Root "artifacts\native-regression"
$ObjDir = Join-Path $Artifacts "objects"
$RegressionLog = Join-Path $Artifacts "run_native_regression.log"
$FucOutputLog = Join-Path $Artifacts "run_native_regression_fuc_output.log"
$AllowedWarningPatterns = @()
if (-not [string]::IsNullOrWhiteSpace($env:FUSION_ALLOWED_WARNING_REGEX)) {
    $AllowedWarningPatterns = $env:FUSION_ALLOWED_WARNING_REGEX -split ";"
}
$TranscriptStarted = $false
New-Item -ItemType Directory -Force -Path $ObjDir | Out-Null
if (Test-Path $RegressionLog) {
    Remove-Item $RegressionLog -Force
}
if (Test-Path $FucOutputLog) {
    Remove-Item $FucOutputLog -Force
}

trap {
    if ($TranscriptStarted) {
        Stop-Transcript | Out-Null
        $TranscriptStarted = $false
    }
    throw
}

Start-Transcript -Path $RegressionLog -Force | Out-Null
$TranscriptStarted = $true

$allPass = $true
$results = @()

function Add-Result {
    param(
        [string]$Name,
        [string]$Phase,
        [int]$ExitCode,
        [bool]$ExpectedPass
    )
    $passed = if ($ExpectedPass) { $ExitCode -eq 0 } else { $ExitCode -ne 0 }
    if (-not $passed) { $script:allPass = $false }
    $script:results += [pscustomobject]@{
        name = $Name
        phase = $Phase
        exit_code = $ExitCode
        expected_pass = $ExpectedPass
        passed = $passed
    }
}

function Invoke-Fuc {
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]]$Args
    )
    $old = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        $maxAttempts = 5
        for ($attempt = 1; $attempt -le $maxAttempts; $attempt++) {
            $output = & $Compiler @Args 2>&1
            if ($null -ne $output) {
                foreach ($line in $output) {
                    Add-Content -Path $FucOutputLog -Value ([string]$line)
                }
            }
            $exitCode = $LASTEXITCODE
            if ($exitCode -eq 0) {
                return 0
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

            if ($hasMappedSectionLock -and $attempt -lt $maxAttempts) {
                Add-Content -Path $FucOutputLog -Value ">>> transient file lock detected; retrying compiler invocation ($attempt/$maxAttempts)"
                Start-Sleep -Milliseconds 300
                continue
            }

            return $exitCode
        }
    } finally {
        $ErrorActionPreference = $old
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

# Core smoke test
$SmokeSrc = Join-Path $Artifacts "smoke_regression.fu"
$SmokeExe = Join-Path $Artifacts "smoke_regression.exe"
Write-Utf8NoBom -Path $SmokeSrc -Content @'
pub fn main() -> int {
    return 0;
}
'@
$smokeCompileExit = Invoke-Fuc $SmokeSrc "-o" $SmokeExe "--emit-bin"
Add-Result -Name "smoke_regression.fu" -Phase "emit-bin" -ExitCode $smokeCompileExit -ExpectedPass $true
if ($smokeCompileExit -eq 0 -and (Test-Path $SmokeExe)) {
    & $SmokeExe *> $null
    Add-Result -Name "smoke_regression.exe" -Phase "run" -ExitCode $LASTEXITCODE -ExpectedPass $true
}

# Stage1 mode transitions (argc-driven behaviour in pure stage1 scaffold)
$Stage1ModeSrc = Join-Path $Root "crates\fuc\src\pure_fusion_compiler_minimal.fu"
$Stage1ModeExe = Join-Path $Artifacts "stage1_mode_regression.exe"
if (Test-Path $Stage1ModeSrc) {
    $stage1CompileExit = Invoke-Fuc $Stage1ModeSrc "-o" $Stage1ModeExe "--emit-bin"
    Add-Result -Name "pure_fusion_compiler_minimal.fu" -Phase "emit-bin-stage1-mode" -ExitCode $stage1CompileExit -ExpectedPass $true
    if ($stage1CompileExit -eq 0 -and (Test-Path $Stage1ModeExe)) {
        & $Stage1ModeExe *> $null
        Add-Result -Name "stage1_mode_regression.exe" -Phase "run-argc1" -ExitCode $LASTEXITCODE -ExpectedPass $true

        & $Stage1ModeExe "--sema-only" *> $null
        Add-Result -Name "stage1_mode_regression.exe" -Phase "run-argc2" -ExitCode $LASTEXITCODE -ExpectedPass $true

        & $Stage1ModeExe "--emit-bin" "--opt" *> $null
        Add-Result -Name "stage1_mode_regression.exe" -Phase "run-argc3" -ExitCode $LASTEXITCODE -ExpectedPass $true

        & $Stage1ModeExe "--emit-bin" "--opt" "--bad-target" *> $null
        Add-Result -Name "stage1_mode_regression.exe" -Phase "run-argc4-negative" -ExitCode $LASTEXITCODE -ExpectedPass $false
    }
}

# Native main entry should emit/run without Rust-runtime symbol leakage.
$MainSrc = Join-Path $Root "crates\fuc\src\main.fu"
$MainExe = Join-Path $Artifacts "main_native_regression.exe"
if (Test-Path $MainSrc) {
    $mainCompileExit = Invoke-Fuc $MainSrc "-o" $MainExe "--emit-bin"
    Add-Result -Name "main.fu" -Phase "emit-bin-native-main" -ExitCode $mainCompileExit -ExpectedPass $true
    if ($mainCompileExit -eq 0 -and (Test-Path $MainExe)) {
        & $MainExe *> $null
        Add-Result -Name "main_native_regression.exe" -Phase "run-native-main" -ExitCode $LASTEXITCODE -ExpectedPass $true
    }
}

# Stage1 parser/sema strict status probes.
$Stage1DeepProbeSrc = Join-Path $Root "crates\fuc\src\stage1_status_deep_probe.fu"
$Stage1DeepProbeExe = Join-Path $Artifacts "stage1_status_deep_probe_regression.exe"
if (Test-Path $Stage1DeepProbeSrc) {
    $deepCompileExit = Invoke-Fuc $Stage1DeepProbeSrc "-o" $Stage1DeepProbeExe "--emit-bin"
    Add-Result -Name "stage1_status_deep_probe.fu" -Phase "emit-bin-stage1-deep-probe" -ExitCode $deepCompileExit -ExpectedPass $true
    if ($deepCompileExit -eq 0 -and (Test-Path $Stage1DeepProbeExe)) {
        & $Stage1DeepProbeExe *> $null
        Add-Result -Name "stage1_status_deep_probe.exe" -Phase "run-stage1-deep-probe" -ExitCode $LASTEXITCODE -ExpectedPass $true
    }
}

$Stage1FullProbeSrc = Join-Path $Root "crates\fuc\src\stage1_full_status_probe.fu"
$Stage1FullProbeExe = Join-Path $Artifacts "stage1_full_status_probe_regression.exe"
if (Test-Path $Stage1FullProbeSrc) {
    $fullCompileExit = Invoke-Fuc $Stage1FullProbeSrc "-o" $Stage1FullProbeExe "--emit-bin"
    Add-Result -Name "stage1_full_status_probe.fu" -Phase "emit-bin-stage1-full-probe" -ExitCode $fullCompileExit -ExpectedPass $true
    if ($fullCompileExit -eq 0 -and (Test-Path $Stage1FullProbeExe)) {
        & $Stage1FullProbeExe *> $null
        Add-Result -Name "stage1_full_status_probe.exe" -Phase "run-stage1-full-probe" -ExitCode $LASTEXITCODE -ExpectedPass $true
    }
}

$fixturesDir = Join-Path $Root "crates\fuc\tests\fixtures"
$parseExpectedFail = @()

$semaExpectedFail = @(
    "generics.fu"
)

$codegenUnsupported = @(
    "closures.fu",
    "concurrency.fu",
    "control_flow.fu",
    "generics.fu",
    "memory.fu",
    "modules.fu"
)

# Valid fixtures
$validFixtures = Get-ChildItem $fixturesDir -File -Filter *.fu | Sort-Object Name
foreach ($fixture in $validFixtures) {
    $parseExpectedPass = -not ($parseExpectedFail -contains $fixture.Name)
    $semaExpectedPass = -not ($semaExpectedFail -contains $fixture.Name)
    $skipCodegen = $codegenUnsupported -contains $fixture.Name

    $parseExit = Invoke-Fuc "--parse-only" $fixture.FullName
    Add-Result -Name $fixture.Name -Phase "parse-only" -ExitCode $parseExit -ExpectedPass $parseExpectedPass

    $semaExit = Invoke-Fuc "--sema-only" $fixture.FullName
    Add-Result -Name $fixture.Name -Phase "sema-only" -ExitCode $semaExit -ExpectedPass $semaExpectedPass

    if (-not $skipCodegen) {
        $objPath = Join-Path $ObjDir ($fixture.BaseName + ".o")
        $codegenExit = Invoke-Fuc "--lib" $fixture.FullName "-o" $objPath
        Add-Result -Name $fixture.Name -Phase "codegen-lib" -ExitCode $codegenExit -ExpectedPass $true
    }
}

# Invalid fixtures
$invalidDir = Join-Path $fixturesDir "invalid"
$invalidFixtures = Get-ChildItem $invalidDir -File -Filter *.fu | Sort-Object Name
foreach ($fixture in $invalidFixtures) {
    if ($fixture.Name -eq "sema_error.fu") {
        $parseExit = Invoke-Fuc "--parse-only" $fixture.FullName
        Add-Result -Name $fixture.Name -Phase "parse-only" -ExitCode $parseExit -ExpectedPass $true
        $semaExit = Invoke-Fuc "--sema-only" $fixture.FullName
        Add-Result -Name $fixture.Name -Phase "sema-only" -ExitCode $semaExit -ExpectedPass $false
    } elseif ($fixture.Name -eq "parser_robustness.fu") {
        $parseExit = Invoke-Fuc "--parse-only" $fixture.FullName
        Add-Result -Name $fixture.Name -Phase "parse-only" -ExitCode $parseExit -ExpectedPass $true
    } else {
        $parseExit = Invoke-Fuc "--parse-only" $fixture.FullName
        Add-Result -Name $fixture.Name -Phase "parse-only" -ExitCode $parseExit -ExpectedPass $false
    }
}

$summary = [pscustomobject]@{
    timestamp = (Get-Date).ToString("o")
    compiler = $Compiler
    total = $results.Count
    passed = ($results | Where-Object { $_.passed }).Count
    failed = ($results | Where-Object { -not $_.passed }).Count
}

$resultsPath = Join-Path $Artifacts "regression_results.json"
$summaryPath = Join-Path $Artifacts "regression_summary.txt"

[pscustomobject]@{
    summary = $summary
    results = $results
} | ConvertTo-Json -Depth 6 | Set-Content -Encoding UTF8 $resultsPath

$summaryLines = @(
    "Fusion Native Regression Summary",
    "Timestamp: $($summary.timestamp)",
    "Compiler: $($summary.compiler)",
    "Total checks: $($summary.total)",
    "Passed: $($summary.passed)",
    "Failed: $($summary.failed)"
)
$summaryLines | Set-Content -Encoding UTF8 $summaryPath

Write-Host ">>> Regression summary: $summaryPath" -ForegroundColor Yellow
Write-Host ">>> Regression details: $resultsPath" -ForegroundColor Yellow

if (-not $allPass) {
    throw "Regression failures detected"
}

$UnexpectedWarnings = New-Object System.Collections.Generic.List[string]
if ($TranscriptStarted) {
    Stop-Transcript | Out-Null
    $TranscriptStarted = $false
}
foreach ($line in (Get-UnexpectedWarningLines -Path $RegressionLog -AllowPatterns $AllowedWarningPatterns)) {
    $UnexpectedWarnings.Add($line)
}
foreach ($line in (Get-UnexpectedWarningLines -Path $FucOutputLog -AllowPatterns $AllowedWarningPatterns)) {
    $UnexpectedWarnings.Add($line)
}
if ($UnexpectedWarnings.Count -gt 0) {
    Write-Host ">>> Unexpected warnings detected during regression:" -ForegroundColor Red
    foreach ($line in ($UnexpectedWarnings | Select-Object -First 20)) {
        Write-Host "    $line" -ForegroundColor Red
    }
    if ($UnexpectedWarnings.Count -gt 20) {
        $remaining = $UnexpectedWarnings.Count - 20
        Write-Host "    ... $remaining additional warning lines omitted" -ForegroundColor Red
    }
    throw "Regression failed due to unexpected warnings (logs: $RegressionLog, $FucOutputLog)."
}

Write-Host ">>> warning audit [ok]" -ForegroundColor Green
Write-Host ">>> Regression passed." -ForegroundColor Green
