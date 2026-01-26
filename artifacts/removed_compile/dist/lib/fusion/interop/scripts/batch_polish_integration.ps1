param(
    [string]$Pattern = "registry\crates\cloud-*"
)

$ErrorActionPreference = "Continue"

Write-Host "Starting batch polish for INTEGRATION: $Pattern" -ForegroundColor Cyan
$Root = Get-Location
$SearchPath = Join-Path $Root $Pattern
$Crates = Get-ChildItem -Path $SearchPath -Directory

if (-not $Crates) {
    Write-Warning "No crates found."
    exit
}

$CommonMeta = @{
    Authors = '["Fusion Team"]'
    Edition = "2021"
    License = "MIT"
    Repository = "https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language"
    Categories = '["network-programming", "external-ffi-bindings"]'
    Readme = "README.md"
}

foreach ($crate in $Crates) {
    $CargoPath = Join-Path $crate.FullName "Cargo.toml"
    if (-not (Test-Path $CargoPath)) { continue }

    Write-Host "  -> Polishing Integration: $($crate.Name)..." -ForegroundColor Cyan

    $Content = Get-Content $CargoPath -Raw -Encoding utf8
    
    $Name = if ($Content -match 'name\s*=\s*"([^"]+)"') { $matches[1] } else { $crate.Name }
    $Version = if ($Content -match 'version\s*=\s*"([^"]+)"') { $matches[1] } else { "0.2.0" }
    
    $Prefix = "Integration"
    $Keywords = @("integration", "fusion")
    $DescPart = "Integration crate: $($crate.Name)"

    if ($crate.Name -like "cloud-*") {
        $Keywords += "cloud"
        $Keywords += ($crate.Name -replace "cloud-", "")
        $DescPart = "Cloud provider connector for $(($crate.Name -replace 'cloud-', '').ToUpper())"
        $CommonMeta.Categories = '["network-programming", "web-programming"]'
    } elseif ($crate.Name -like "interop-*") {
        $Keywords += "interop"
        $Keywords += "ffi"
        $DescPart = "Language interoperability layer for $(($crate.Name -replace 'interop-', ''))"
        $CommonMeta.Categories = '["external-ffi-bindings"]'
    }

    $KeywordsStr = '["' + ($Keywords -join '", "') + '"]'

    if ($Content -match 'description\s*=\s*"Integration:') {
        Write-Host "     Skipping (already polished)" -ForegroundColor DarkGray
        continue
    }

    $NewPackage = @"
[package]
name = "$Name"
version = "$Version"
edition = "$($CommonMeta.Edition)"
license = "$($CommonMeta.License)"
description = "$($Prefix): $DescPart."
authors = $($CommonMeta.Authors)
repository = "$($CommonMeta.Repository)"
keywords = $KeywordsStr
categories = $($CommonMeta.Categories)
readme = "$($CommonMeta.Readme)"
"@

    $NewContent = $Content -replace '(?ms)^\[package\].*?(?=\n\[)', $NewPackage
    if ($NewContent -eq $Content) {
         $NewContent = $NewPackage + "`n" + ($Content -replace '(?ms)^\[package\].*?(\n\[dependencies\])', '$1')
    }

    # Ensure fusion_core dependency
    if ($NewContent -notmatch "fusion_core") {
        $NewContent = $NewContent -replace "\[dependencies\]", "[dependencies]`nfusion_core = { workspace = true }"
    }
    
    # Clean up double newlines
    $NewContent = $NewContent -replace "\n{3,}", "`n`n"

    Set-Content -Path $CargoPath -Value $NewContent -Encoding utf8
}
Write-Host "Integration Polish Complete." -ForegroundColor Green
