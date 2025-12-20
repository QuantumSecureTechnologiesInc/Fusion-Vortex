param(
    [string]$Pattern = "registry\crates\nn-*"
)

$ErrorActionPreference = "Continue" # Don't stop on single file error

Write-Host "Starting batch polish for pattern: $Pattern" -ForegroundColor Cyan

# Resolve path to absolute to avoid relative path confusion
$Root = Get-Location
$SearchPath = Join-Path $Root $Pattern

Write-Host "Searching in: $SearchPath" -ForegroundColor Gray

$Crates = Get-ChildItem -Path $SearchPath -Directory

if (-not $Crates) {
    Write-Warning "No crates found matching pattern $Pattern"
    exit
}

Write-Host "Found $($Crates.Count) crates to process." -ForegroundColor Green

$CommonMeta = @{
    Authors    = '["Fusion Team"]'
    Edition    = "2021"
    License    = "MIT"
    Repository = "https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language"
    Categories = '["algorithms", "science"]'
    Readme     = "README.md"
}

foreach ($crate in $Crates) {
    $CargoPath = Join-Path $crate.FullName "Cargo.toml"
    if (-not (Test-Path $CargoPath)) { 
        Write-Warning "No Cargo.toml in $($crate.Name)"
        continue 
    }

    Write-Host "  -> Polishing $($crate.Name)..." -ForegroundColor Cyan

    $Content = Get-Content $CargoPath -Raw -Encoding utf8
    
    # Extract existing name/version
    $Name = if ($Content -match 'name\s*=\s*"([^"]+)"') { $matches[1] } else { $crate.Name }
    $Version = if ($Content -match 'version\s*=\s*"([^"]+)"') { $matches[1] } else { "0.2.0" }
    
    # Determine Keywords/Desc
    $Prefix = "Algorithm"
    $Keywords = @("algorithm", "fusion")
    $DescPart = "Fusion ecosystem crate"

    if ($crate.Name -like "nn-*") {
        $Keywords += "neural-network"
        $Keywords += "deep-learning"
        $DescPart = "Neural Network layer/module: $($crate.Name)"
    }
    elseif ($crate.Name -like "q-*") {
        $Keywords += "quantum"
        $Keywords += "quantum-computing"
        $DescPart = "Quantum algorithm/module: $($crate.Name)"
    }
    elseif ($crate.Name -like "llm-*") {
        $Keywords += "llm"
        $Keywords += "generative-ai"
        $DescPart = "Large Language Model component: $($crate.Name)"
    }

    $KeywordsStr = '["' + ($Keywords -join '", "') + '"]'
    
    # Check if already polished (contains "Algorithm:")
    if ($Content -match 'description\s*=\s*"Algorithm:') {
        Write-Host "     Skipping (already polished)" -ForegroundColor DarkGray
        continue
    }

    # Construct New Package Section
    $NewPackage = @"
[package]
name = "$Name"
version = "$Version"
edition = "$($CommonMeta.Edition)"
license = "$($CommonMeta.License)"
description = "$($Prefix): $DescPart. Optimized for performance."
authors = $($CommonMeta.Authors)
repository = "$($CommonMeta.Repository)"
keywords = $KeywordsStr
categories = $($CommonMeta.Categories)
readme = "$($CommonMeta.Readme)"
"@

    # Smart replace [package] block using Regex
    # Matches [package] ... until specific next section header or simplistic fallback
    $NewContent = $Content -replace '(?ms)^\[package\].*?(?=\n\[)', $NewPackage

    if ($NewContent -eq $Content) {
        # Fallback: Just replace top lines if they look standard
        $NewContent = $NewPackage + "`n" + ($Content -replace '(?ms)^\[package\].*?(\n\[dependencies\])', '$1')
        if ($NewContent -eq $Content) {
            # Last resort: Prepend if file is empty or weird? No, skipping to be safe
            Write-Warning "     Regex replace failed for $($crate.Name), skipping."
            continue
        }
    }

    # Clean up structure
    if ($NewContent -notmatch "fusion_core") {
        $NewContent = $NewContent -replace "\[dependencies\]", "[dependencies]`nfusion_core = { workspace = true }"
    }
    
    # Ensure benches exist in metadata
    if ($NewContent -notmatch "\[dev-dependencies\]") {
        $NewContent += "`n`n[dev-dependencies]`ncriterion = { workspace = true }"
    }

    Set-Content -Path $CargoPath -Value $NewContent -Encoding utf8
}
Write-Host "Batch complete." -ForegroundColor Green
