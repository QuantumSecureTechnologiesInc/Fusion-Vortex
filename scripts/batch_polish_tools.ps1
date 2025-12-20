param(
    [string]$Pattern = "registry\crates\sec-*"
)

$ErrorActionPreference = "Continue"

Write-Host "Starting batch polish for TOOLS: $Pattern" -ForegroundColor Cyan
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
    Categories = '["development-tools", "command-line-utilities"]'
    Readme     = "README.md"
}

foreach ($crate in $Crates) {
    $CargoPath = Join-Path $crate.FullName "Cargo.toml"
    if (-not (Test-Path $CargoPath)) { continue }

    Write-Host "  -> Polishing Tool: $($crate.Name)..." -ForegroundColor Cyan

    $Content = Get-Content $CargoPath -Raw -Encoding utf8
    
    $Name = if ($Content -match 'name\s*=\s*"([^"]+)"') { $matches[1] } else { $crate.Name }
    $Version = if ($Content -match 'version\s*=\s*"([^"]+)"') { $matches[1] } else { "0.2.0" }
    
    $Prefix = "Tool"
    $Keywords = @("tool", "cli", "fusion")
    $DescPart = "Fusion tool: $($crate.Name)"

    if ($crate.Name -like "sec-*") {
        $Keywords += "security"
        $DescPart = "Security tool: $($crate.Name)"
    }
    elseif ($crate.Name -match "generator|compiler|validator|audit|linter") {
        $Keywords += "dev-tool"
        $DescPart = "Development tool: $($crate.Name)"
    }

    $KeywordsStr = '["' + ($Keywords -join '", "') + '"]'

    if ($Content -match 'description\s*=\s*"Tool:') {
        Write-Host "     Skipping (already polished)" -ForegroundColor DarkGray
        continue
    }

    $NewPackage = @"
[package]
name = "$Name"
version = "$Version"
edition = "$($CommonMeta.Edition)"
license = "$($CommonMeta.License)"
description = "$($Prefix): $DescPart. Excellent error reporting."
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

    # Ensure dependencies common for tools
    if ($NewContent -notmatch "clap") {
        $NewContent = $NewContent -replace "\[dependencies\]", "[dependencies]`nclap = { workspace = true }"
    }
    if ($NewContent -notmatch "anyhow") {
        if ($NewContent -match "\[dependencies\]") {
            $NewContent = $NewContent -replace "\[dependencies\]", "[dependencies]`nanyhow = { workspace = true }"
        }
        else {
            $NewContent += "`n`n[dependencies]`nanyhow = { workspace = true }"
        }
    }
    
    # Clean up double newlines
    $NewContent = $NewContent -replace "\n{3,}", "`n`n"

    Set-Content -Path $CargoPath -Value $NewContent -Encoding utf8
}
Write-Host "Tools Polish Complete." -ForegroundColor Green
