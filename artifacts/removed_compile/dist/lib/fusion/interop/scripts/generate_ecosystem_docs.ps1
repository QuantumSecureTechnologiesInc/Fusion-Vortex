$ErrorActionPreference = "Continue"

# Collect all crate metadata
$AllCrates = @()
$Crates = Get-ChildItem -Path "registry/crates" -Directory

foreach ($crate in $Crates) {
    $CargoPath = Join-Path $crate.FullName "Cargo.toml"
    if (-not (Test-Path $CargoPath)) { continue }
    
    $Content = Get-Content $CargoPath -Raw
    
    # Extract metadata
    $Name = if ($Content -match 'name\s*=\s*"([^"]+)"') { $matches[1] } else { $crate.Name }
    $Version = if ($Content -match 'version\s*=\s*"([^"]+)"') { $matches[1] } else { "unknown" }
    $Description = if ($Content -match 'description\s*=\s*"([^"]+)"') { $matches[1] } else { "No description" }
    
    # Categorize by archetype prefix
    $Archetype = "Uncategorized"
    if ($Description -match '^(Foundation|Algorithm|Integration|Framework|Tool|Experimental):') {
        $Archetype = $matches[1]
    }
    
    $AllCrates += [PSCustomObject]@{
        Name        = $Name
        Version     = $Version
        Description = $Description
        Archetype   = $Archetype
        FolderName  = $crate.Name
    }
}

# Group by archetype
$ByArchetype = $AllCrates | Group-Object -Property Archetype | Sort-Object Name

# Generate Markdown
$Markdown = @"
# Fusion Crate Ecosystem Documentation

**Generated**: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss UTC")  
**Total Crates**: $($AllCrates.Count)

## Overview

The Fusion Programming Language ecosystem consists of $($AllCrates.Count) crates organized into six primary archetypes:

"@

foreach ($group in $ByArchetype) {
    $count = $group.Count
    $archetype = $group.Name
    $Markdown += "- **$archetype**: $count crates`n"
}

$Markdown += @"

---

## Crates by Archetype

"@

foreach ($group in $ByArchetype | Sort-Object { 
        # Custom sort order
        switch ($_.Name) {
            "Foundation" { 1 }
            "Algorithm" { 2 }
            "Integration" { 3 }
            "Framework" { 4 }
            "Tool" { 5 }
            "Experimental" { 6 }
            default { 7 }
        }
    }) {
    $archetype = $group.Name
    $crates = $group.Group | Sort-Object Name
    
    $Markdown += @"

### $archetype ($($crates.Count) crates)

"@

    if ($archetype -eq "Foundation") {
        $Markdown += "> **Foundation crates** provide core primitives and building blocks. They are dependency-minimal, panic-free, and designed for composition.`n`n"
    }
    elseif ($archetype -eq "Algorithm") {
        $Markdown += "> **Algorithm crates** implement specific computational methods with documented complexity guarantees.`n`n"
    }
    elseif ($archetype -eq "Integration") {
        $Markdown += "> **Integration crates** connect Fusion to external services, languages, and protocols.`n`n"
    }
    elseif ($archetype -eq "Framework") {
        $Markdown += "> **Framework crates** provide opinionated, batteries-included platforms for specific domains.`n`n"
    }
    elseif ($archetype -eq "Tool") {
        $Markdown += "> **Tool crates** are CLI utilities and development tools with excellent error reporting.`n`n"
    }
    
    $Markdown += "| Crate | Version | Description |`n"
    $Markdown += "|-------|---------|-------------|`n"
    
    foreach ($crate in $crates) {
        $desc = $crate.Description -replace '\|', '\|'  # Escape pipes for markdown
        $Markdown += "| ``$($crate.Name)`` | $($crate.Version) | $desc |`n"
    }
}

$Markdown += @"

---

## Quick Reference

### Core Infrastructure
- **`fusion_core`**: Foundation type system
- **`fusion_std`**: Standard library extensions
- **`fusion_runtime_core`**: Heterogeneous runtime
- **`fusion_ai_core`**: AI/ML framework

### Quantum Computing
- **`fusion_quantum`**: Quantum primitives
- **`q-sim`**: Quantum circuit simulator
- **`qaoa`**: Quantum optimization algorithm
- **`q-error-correction`**: Error correction codes

### Neural Networks
- **`nn-lstm`**: LSTM layers
- **`nn-attention-block`**: Attention mechanisms
- **`nn-layer-norm`**: Layer normalization
- **`resnet`**: ResNet implementation

### Large Language Models
- **`llm-tokenizers`**: BPE/WordPiece tokenizers
- **`llm-quantization`**: Model quantization
- **`llm-beam-search`**: Beam search decoding
- **`llm-rag`**: Retrieval-Augmented Generation

### Cloud Integration
- **`cloud-aws`**: AWS connector
- **`cloud-gcp`**: Google Cloud connector
- **`cloud-azure`**: Azure connector

### Security Tools
- **`sec-penetration`**: Penetration testing
- **`sec-forensics`**: Security forensics
- **`sec-policy-engine`**: Policy enforcement

---

## Getting Started

### Using a Crate

Add to your `Cargo.toml`:

```toml
[dependencies]
fusion_core = { workspace = true }
# Or specify version if not in Fusion workspace
fusion_ai_core = "0.2.0"
```

### Building from Source

```bash
# Build entire workspace
fusion build --workspace

# Build specific crate
fusion build -p fusion_runtime_core

# Run tests
fusion test --workspace
```

### Documentation

Generate docs for all crates:

```bash
fusion doc --workspace --no-deps --open
```

---

**For detailed crate documentation, see individual README.md files in each crate directory.**
"@

# Write to file
$Markdown | Out-File -FilePath "docs/CRATE_ECOSYSTEM_OVERVIEW.md" -Encoding utf8

Write-Host "Documentation generated: docs/CRATE_ECOSYSTEM_OVERVIEW.md" -ForegroundColor Green
Write-Host "Total crates documented: $($AllCrates.Count)" -ForegroundColor Cyan
