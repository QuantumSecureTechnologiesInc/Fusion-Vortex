$ErrorActionPreference = "Continue"

$Mapping = @{
    # Integration / Glue
    "fusion_net"         = "Integration"
    "fusion_financer"    = "Integration"
    "k8s-operator"       = "Integration"
    "githubent-lib"      = "Integration"
    "graphql"            = "Integration"
    "rest-server"        = "Integration"
    "wasm-server"        = "Integration"
    "webasm-renderer"    = "Integration"
    "python-converter"   = "Integration"
    "python-pkg"         = "Integration"
    "kv-cache"           = "Integration"
    "react-hooks"        = "Integration"
    "vault"              = "Integration"
    "trusted-anchor"     = "Integration"

    # Algorithms / Engines
    "fusion_quantum"     = "Algorithm"
    "quantum-sdk"        = "Algorithm"
    "pqc-proxy"          = "Algorithm"
    "qubo"               = "Algorithm"
    "solver"             = "Algorithm"
    "rl-algorithms"      = "Algorithm"
    "gate-decomposition" = "Algorithm"
    "auto-prompt"        = "Algorithm"
    "inference-graph"    = "Algorithm"
    "prompt-prefill"     = "Algorithm"
    "safetensors"        = "Algorithm"
    "tensor-optim"       = "Algorithm"
    "tensor-parallel"    = "Algorithm"
    "tokenizers"         = "Algorithm"
    "transform"          = "Algorithm"
    "tree"               = "Algorithm"
    "trie-search"        = "Algorithm"
    "layout-builder"     = "Algorithm"
    "ops"                = "Algorithm"
    "model-server-core"  = "Algorithm"
    "resnet"             = "Algorithm"
    "block"              = "Algorithm"

    # Frameworks
    "sentinel-tribrid"   = "Framework"
    "telemetry-ingestor" = "Framework"
    "observability"      = "Framework"
    "profiler"           = "Framework"
    "safety-monitor"     = "Framework"
    "policy-engine"      = "Framework"
    "sandbox-manager"    = "Framework"
    "stream-monitor"     = "Framework"
    "vram-scheduler"     = "Framework"
    "gpu-scheduler"      = "Framework"
    "offload"            = "Framework"
    "training"           = "Framework"
    "metrics"            = "Framework"
    "rate-limiter"       = "Framework"

    # Tooling
    "audit"              = "Tool"
    "auth"               = "Tool"
    "formatter"          = "Tool"
    "sbom-generator"     = "Tool"
    "sdk-generator"      = "Tool"
    "schema-validator"   = "Tool"
    "supply-chain"       = "Tool"
    "tester"             = "Tool"
    "version"            = "Tool"
    "toolchain-ext"      = "Tool"

    # Primitives
    "std-ext"            = "Foundation"
    "retry"              = "Foundation"
    "verifier"           = "Foundation"
}

$Crates = Get-ChildItem -Path "registry/crates" -Directory

foreach ($crate in $Crates) {
    if (-not $Mapping.ContainsKey($crate.Name)) { continue }
    
    $Archetype = $Mapping[$crate.Name]
    $CargoPath = Join-Path $crate.FullName "Cargo.toml"
    if (-not (Test-Path $CargoPath)) { continue }

    Write-Host "Polishing as $Archetype`: $($crate.Name)..." -ForegroundColor Cyan

    $Content = Get-Content $CargoPath -Raw -Encoding utf8
    
    # Corrected regex for already polished check
    if ($Content -match "description\s*=\s*`"$($Archetype):") {
        Write-Host "  Skipping (already polished)" -ForegroundColor DarkGray
        continue
    }

    $Name = if ($Content -match 'name\s*=\s*"([^"]+)"') { $matches[1] } else { $crate.Name }
    $Version = if ($Content -match 'version\s*=\s*"([^"]+)"') { $matches[1] } else { "0.2.0" }
    
    $CommonMeta = @{
        Authors    = '["Fusion Team"]'
        Edition    = "2021"
        License    = "MIT"
        Repository = "https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language"
        Readme     = "README.md"
    }

    $Keywords = @("fusion", $Archetype.ToLower())
    $DescPart = "Fusion $Archetype crate: $($crate.Name)"
    $Categories = '["development-tools"]'

    switch ($Archetype) {
        "Foundation" { $Categories = '["development-tools", "primitives"]' }
        "Algorithm" { $Categories = '["algorithms", "science"]' }
        "Integration" { $Categories = '["network-programming", "external-ffi-bindings"]' }
        "Framework" { $Categories = '["development-tools", "framework"]' }
        "Tool" { $Categories = '["command-line-utilities", "development-tools"]' }
    }

    $KeywordsStr = '["' + ($Keywords -join '", "') + '"]'

    # Corrected string interpolation for description
    $NewPackage = @"
[package]
name = "$Name"
version = "$Version"
edition = "$($CommonMeta.Edition)"
license = "$($CommonMeta.License)"
description = "$($Archetype): $DescPart."
authors = $($CommonMeta.Authors)
repository = "$($CommonMeta.Repository)"
keywords = $KeywordsStr
categories = $Categories
readme = "$($CommonMeta.Readme)"
"@

    $NewContent = $Content -replace '(?ms)^\[package\].*?(?=\n\[)', $NewPackage
    if ($NewContent -eq $Content) {
        $NewContent = $NewPackage + "`n" + ($Content -replace '(?ms)^\[package\].*?(\n\[dependencies\])', '$1')
    }

    if ($NewContent -notmatch "fusion_core") {
        $NewContent = $NewContent -replace "\[dependencies\]", "[dependencies]`nfusion_core = { workspace = true }"
    }

    Set-Content -Path $CargoPath -Value $NewContent -Encoding utf8
}
