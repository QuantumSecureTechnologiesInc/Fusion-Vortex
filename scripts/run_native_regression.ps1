$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $false

Write-Host ">>> Fusion Native Regression" -ForegroundColor Cyan

$Root = (Get-Item $PSScriptRoot).Parent.FullName
$Compiler = Join-Path $Root "target\release\fuc.exe"
if (-not (Test-Path $Compiler)) {
    throw "Compiler not found: $Compiler (run scripts/bootstrap_native.ps1 first)"
}

$Artifacts = Join-Path $Root "artifacts\native-regression"
$ObjDir = Join-Path $Artifacts "objects"
New-Item -ItemType Directory -Force -Path $ObjDir | Out-Null

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
        & $Compiler @Args 1>$null 2>$null
        return $LASTEXITCODE
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

$fixturesDir = Join-Path $Root "crates\fuc\tests\fixtures"
$unsupportedAdvanced = @(
    "closures.fu",
    "concurrency.fu",
    "control_flow.fu",
    "generics.fu",
    "memory.fu"
)

# Valid fixtures
$validFixtures = Get-ChildItem $fixturesDir -File -Filter *.fu | Sort-Object Name
foreach ($fixture in $validFixtures) {
    $isUnsupported = $unsupportedAdvanced -contains $fixture.Name
    $expectedPass = -not $isUnsupported

    $parseExit = Invoke-Fuc "--parse-only" $fixture.FullName
    Add-Result -Name $fixture.Name -Phase "parse-only" -ExitCode $parseExit -ExpectedPass $expectedPass

    $semaExit = Invoke-Fuc "--sema-only" $fixture.FullName
    Add-Result -Name $fixture.Name -Phase "sema-only" -ExitCode $semaExit -ExpectedPass $expectedPass

    if ($expectedPass) {
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

Write-Host ">>> Regression passed." -ForegroundColor Green
