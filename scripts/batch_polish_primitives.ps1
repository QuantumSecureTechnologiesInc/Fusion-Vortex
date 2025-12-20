param(
    [string]$Pattern = "registry\crates\math-*"
)

$ErrorActionPreference = "Continue"

Write-Host "Starting batch polish for PRIMITIVES: $Pattern" -ForegroundColor Cyan
$Root = Get-Location
$SearchPath = Join-Path $Root $Pattern
$Crates = Get-ChildItem -Path $SearchPath -Directory

if (-not $Crates) {
    Write-Warning "No crates found."
    exit
}

$CommonMeta = @{
    Authors    = '["Fusion Team"]'
    Edition    = "2021"
    License    = "MIT"
    Repository = "https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language"
    Categories = '["development-tools", "mathematics"]'
    Readme     = "README.md"
}

foreach ($crate in $Crates) {
    $CargoPath = Join-Path $crate.FullName "Cargo.toml"
    if (-not (Test-Path $CargoPath)) { continue }

    Write-Host "  -> Polishing Primitive: $($crate.Name)..." -ForegroundColor Cyan

    $Content = Get-Content $CargoPath -Raw -Encoding utf8
    
    $Name = if ($Content -match 'name\s*=\s*"([^"]+)"') { $matches[1] } else { $crate.Name }
    $Version = if ($Content -match 'version\s*=\s*"([^"]+)"') { $matches[1] } else { "0.2.0" }
    
    $Prefix = "Foundation"
    $Keywords = @("foundation", "primitive", "fusion")
    $DescPart = "Fusion primitive: $($crate.Name)"

    if ($crate.Name -like "math-*") {
        $Keywords += "math"
        $Keywords += "linear-algebra"
        $DescPart = "Mathematical primitive: $(($crate.Name -replace 'math-', '').Replace('-', ' '))"
        $CommonMeta.Categories = '["mathematics", "science"]'
    }
    elseif ($crate.Name -eq "fusion-id-provider") {
        $Keywords += "id"
        $Keywords += "uuid"
        $DescPart = "Identity provider and generation primitives"
        $CommonMeta.Categories = '["development-tools"]'
    }
    elseif ($crate.Name -eq "fusion-regex") {
        $Keywords += "regex"
        $Keywords += "parser"
        $DescPart = "Regular expression engine optimized for Fusion"
        $CommonMeta.Categories = '["text-processing"]'
    }

    $KeywordsStr = '["' + ($Keywords -join '", "') + '"]'

    if ($Content -match 'description\s*=\s*"Foundation:') {
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
    
    # Ensure fusion_core dependency if not std/core itself
    if ($crate.Name -ne "fusion_std" -and $crate.Name -ne "fusion_core") {
        if ($NewContent -notmatch "fusion_core") {
            $NewContent = $NewContent -replace "\[dependencies\]", "[dependencies]`nfusion_core = { workspace = true }"
        }
    }

    # Clean up double newlines
    $NewContent = $NewContent -replace "\n{3,}", "`n`n"

    Set-Content -Path $CargoPath -Value $NewContent -Encoding utf8
}
Write-Host "Primitive Polish Complete." -ForegroundColor Green
