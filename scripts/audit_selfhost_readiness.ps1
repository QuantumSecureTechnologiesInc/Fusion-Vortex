$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $false

Write-Host ">>> Fusion Self-Host Readiness Audit" -ForegroundColor Cyan

$Root = (Get-Item $PSScriptRoot).Parent.FullName
$Compiler = Join-Path $Root "target\release\fuc.exe"
if (-not (Test-Path $Compiler)) {
    throw "Compiler not found: $Compiler"
}

$Artifacts = Join-Path $Root "artifacts\selfhost-audit"
New-Item -ItemType Directory -Force -Path $Artifacts | Out-Null

$sourceFiles = Get-ChildItem (Join-Path $Root "crates\fuc\src") -Recurse -File -Filter *.fu |
    Where-Object { $_.Name -notlike "pure_fusion_compiler*.fu" } |
    Sort-Object FullName

$results = @()

function Invoke-Fuc {
    param(
        [string]$FilePath,
        [string]$Phase
    )
    $old = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        & $Compiler "--$Phase" $FilePath 1>$null 2>$null
        return $LASTEXITCODE
    } finally {
        $ErrorActionPreference = $old
    }
}

foreach ($file in $sourceFiles) {
    $parseExit = Invoke-Fuc -FilePath $file.FullName -Phase "parse-only"
    $semaExit = Invoke-Fuc -FilePath $file.FullName -Phase "sema-only"

    $results += [pscustomobject]@{
        file = $file.FullName
        parse_exit = $parseExit
        sema_exit = $semaExit
        parse_pass = ($parseExit -eq 0)
        sema_pass = ($semaExit -eq 0)
    }
}

$summary = [pscustomobject]@{
    timestamp = (Get-Date).ToString("o")
    compiler = $Compiler
    total_files = $results.Count
    parse_pass = ($results | Where-Object { $_.parse_pass }).Count
    sema_pass = ($results | Where-Object { $_.sema_pass }).Count
    parse_fail = ($results | Where-Object { -not $_.parse_pass }).Count
    sema_fail = ($results | Where-Object { -not $_.sema_pass }).Count
}

$jsonPath = Join-Path $Artifacts "selfhost_readiness.json"
$txtPath = Join-Path $Artifacts "selfhost_readiness.txt"

[pscustomobject]@{
    summary = $summary
    files = $results
} | ConvertTo-Json -Depth 6 | Set-Content -Encoding UTF8 $jsonPath

@(
    "Fusion Self-Host Readiness",
    "Timestamp: $($summary.timestamp)",
    "Compiler: $($summary.compiler)",
    "Files audited: $($summary.total_files)",
    "Parse pass: $($summary.parse_pass)",
    "Sema pass: $($summary.sema_pass)",
    "Parse fail: $($summary.parse_fail)",
    "Sema fail: $($summary.sema_fail)"
) | Set-Content -Encoding UTF8 $txtPath

Write-Host ">>> Audit summary: $txtPath" -ForegroundColor Yellow
Write-Host ">>> Audit details: $jsonPath" -ForegroundColor Yellow
