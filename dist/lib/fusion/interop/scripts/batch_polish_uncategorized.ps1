$ErrorActionPreference = "Continue"

# Categorization of uncategorized crates based on their purpose
$UncategorizedMapping = @{
    # Integration / FFI / Connectors
    "fusion_bridge_c"                 = @{
        Archetype   = "Integration"
        Description = "C FFI bridge for Fusion interoperability"
        Keywords    = @("ffi", "c-bindings", "integration")
    }
    "fusion_interop_cargo_converter"  = @{
        Archetype   = "Integration"
        Description = "Converts Rust Cargo metadata to Fusion FFI definitions"
        Keywords    = @("cargo", "converter", "interop")
    }
    "fusion-deploy"                   = @{
        Archetype   = "Integration"
        Description = "Cloud deployment adapters for AWS, GCP, Azure"
        Keywords    = @("deployment", "cloud", "devops")
    }
    "fusion-github"                   = @{
        Archetype   = "Integration"
        Description = "GitHub API integration and workflow automation"
        Keywords    = @("github", "api", "integration")
    }
    "fusion_cuda_interface"           = @{
        Archetype   = "Integration"
        Description = "CUDA kernel interface for GPU operations"
        Keywords    = @("cuda", "gpu", "ffi")
    }

    # Frameworks
    "fusion-agents"                   = @{
        Archetype   = "Framework"
        Description = "Multi-agent orchestration framework for parallel AI workflows"
        Keywords    = @("agents", "orchestration", "framework")
    }
    "fusion-ai-core"                  = @{
        Archetype   = "Framework"
        Description = "AI infrastructure core with adapters and safety checks"
        Keywords    = @("ai", "infrastructure", "framework")
    }
    "fusion-ai-models"                = @{
        Archetype   = "Framework"
        Description = "Local model runners (llama.cpp, ONNX) with unified interface"
        Keywords    = @("models", "inference", "framework")
    }
    "fusion_server_event_bus"         = @{
        Archetype   = "Framework"
        Description = "Asynchronous message queue and event-driven architecture"
        Keywords    = @("event-bus", "messaging", "framework")
    }
    "fusion_server_faas"              = @{
        Archetype   = "Framework"
        Description = "Serverless Function-as-a-Service runtime and emulator"
        Keywords    = @("faas", "serverless", "framework")
    }
    "fusion_server_router_mesh"       = @{
        Archetype   = "Framework"
        Description = "Service mesh with dynamic discovery and routing"
        Keywords    = @("service-mesh", "routing", "framework")
    }
    "fusion_llm_batch_scheduler"      = @{
        Archetype   = "Framework"
        Description = "Dynamic batching scheduler for LLM inference"
        Keywords    = @("scheduler", "batching", "llm")
    }
    "flux-resolve-v2-hive-mind"       = @{
        Archetype   = "Framework"
        Description = "Distributed dependency resolution with GPU acceleration"
        Keywords    = @("dependency-resolution", "distributed", "framework")
    }

    # Algorithms
    "fusion_embeddings"               = @{
        Archetype   = "Algorithm"
        Description = "Token and positional embeddings for sequence models"
        Keywords    = @("embeddings", "nlp", "algorithm")
    }
    "fusion_q_compiler_pass"          = @{
        Archetype   = "Algorithm"
        Description = "Quantum compiler optimization passes and gate synthesis"
        Keywords    = @("quantum", "compiler", "optimization")
    }
    "fusion_q_error_corr"             = @{
        Archetype   = "Algorithm"
        Description = "Quantum Error Correction (surface codes, stabilizer)"
        Keywords    = @("quantum", "error-correction", "algorithm")
    }
    "fusion_ui_data_vis"              = @{
        Archetype   = "Algorithm"
        Description = "High-performance data visualization (charts, graphs)"
        Keywords    = @("visualization", "charts", "algorithm")
    }
    "fusion_llm_cuda_kernel_lib"      = @{
        Archetype   = "Algorithm"
        Description = "Optimized CUDA kernels (Attention, RMSNorm, MatMul)"
        Keywords    = @("cuda", "kernels", "optimization")
    }

    # Tools
    "fusion_crate_analyzer"           = @{
        Archetype   = "Tool"
        Description = "Static analysis and dependency graph tool for Fusion crates"
        Keywords    = @("analyzer", "static-analysis", "tool")
    }
    "fusion-ai-cli"                   = @{
        Archetype   = "Tool"
        Description = "AI-powered CLI with subcommands and workspace management"
        Keywords    = @("cli", "ai", "tool")
    }
    "fusion-ai-daemon"                = @{
        Archetype   = "Tool"
        Description = "Background daemon for heavy LLM inference workloads"
        Keywords    = @("daemon", "background-service", "tool")
    }
    "fusion-debugger"                 = @{
        Archetype   = "Tool"
        Description = "Debug Adapter Protocol (DAP) implementation for Fusion"
        Keywords    = @("debugger", "dap", "tool")
    }
    "fusion-docgen"                   = @{
        Archetype   = "Tool"
        Description = "Documentation generator with search indexing"
        Keywords    = @("documentation", "generator", "tool")
    }
    "fusion_llm_custom_tokenizer"     = @{
        Archetype   = "Tool"
        Description = "BPE/SentencePiece training utility for custom vocabularies"
        Keywords    = @("tokenizer", "training", "tool")
    }

    # Foundation
    "fusion_ui_component_lib"         = @{
        Archetype   = "Foundation"
        Description = "Reusable UI component primitives (buttons, grids, modals)"
        Keywords    = @("ui", "components", "primitives")
    }
    "fusion_finance"                  = @{
        Archetype   = "Foundation"
        Description = "High-frequency trading primitives with sub-10us latency"
        Keywords    = @("finance", "trading", "primitives")
    }

    # Experimental (no clear description or purpose)
    "fusion_carver"                   = @{
        Archetype   = "Experimental"
        Description = "EXPERIMENTAL: Code carving and extraction utilities"
        Keywords    = @("experimental", "utilities")
    }
    "fusion_client"                   = @{
        Archetype   = "Experimental"
        Description = "EXPERIMENTAL: Generic client interface"
        Keywords    = @("experimental", "client")
    }
    "fusion_executor"                 = @{
        Archetype   = "Experimental"
        Description = "EXPERIMENTAL: Task execution engine"
        Keywords    = @("experimental", "executor")
    }
    "fusion_graph"                    = @{
        Archetype   = "Experimental"
        Description = "EXPERIMENTAL: Graph data structures and algorithms"
        Keywords    = @("experimental", "graph")
    }
    "fusion_experimental_diagnostics" = @{
        Archetype   = "Experimental"
        Description = "EXPERIMENTAL: Mixture-of-Experts diagnostics and profiling"
        Keywords    = @("experimental", "diagnostics", "moe")
    }
    "haft_fusion"                     = @{
        Archetype   = "Experimental"
        Description = "EXPERIMENTAL: Research prototype"
        Keywords    = @("experimental", "research")
    }
}

# Search in both registry/crates and crates directories
$SearchPaths = @("registry/crates", "crates")
$PolishedCount = 0

foreach ($searchPath in $SearchPaths) {
    if (-not (Test-Path $searchPath)) { continue }
    
    $Crates = Get-ChildItem -Path $searchPath -Directory

    foreach ($crate in $Crates) {
        $CargoPath = Join-Path $crate.FullName "Cargo.toml"
        if (-not (Test-Path $CargoPath)) { continue }

        $Content = Get-Content $CargoPath -Raw -Encoding utf8
        $PackageName = if ($Content -match 'name\s*=\s*"([^"]+)"') { $matches[1] } else { $crate.Name }

        if (-not $UncategorizedMapping.ContainsKey($PackageName)) { continue }
        
        $Config = $UncategorizedMapping[$PackageName]

        Write-Host "Polishing as $($Config.Archetype): $PackageName (in $($crate.Name))..." -ForegroundColor Cyan
        
        # Check if already polished
        if ($Content -match "description\s*=\s*`"$($Config.Archetype):") {
            Write-Host "  Skipping (already polished)" -ForegroundColor DarkGray
            continue
        }

        $Name = $PackageName
        $Version = if ($Content -match 'version\s*=\s*"([^"]+)"') { $matches[1] } else { "0.2.0" }
        
        $CommonMeta = @{
            Authors    = '["Fusion Team"]'
            Edition    = "2021"
            License    = "MIT"
            Repository = "https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language"
            Readme     = "README.md"
        }

        $Keywords = @("fusion", $Config.Archetype.ToLower()) + $Config.Keywords
        $KeywordsStr = '["' + ($Keywords -join '", "') + '"]'

        # Set categories based on archetype
        $Categories = switch ($Config.Archetype) {
            "Foundation" { '["development-tools", "no-std"]' }
            "Algorithm" { '["algorithms", "science"]' }
            "Integration" { '["external-ffi-bindings", "network-programming"]' }
            "Framework" { '["development-tools", "framework"]' }
            "Tool" { '["command-line-utilities", "development-tools"]' }
            "Experimental" { '["experimental"]' }
            default { '["development-tools"]' }
        }

        $NewPackage = @"
[package]
name = "$Name"
version = "$Version"
edition = "$($CommonMeta.Edition)"
license = "$($CommonMeta.License)"
description = "$($Config.Archetype): $($Config.Description)."
authors = $($CommonMeta.Authors)
repository = "$($CommonMeta.Repository)"
keywords = $KeywordsStr
categories = $Categories
readme = "$($CommonMeta.Readme)"
"@

        $NewContent = $Content -replace '(?ms)^\[package\].*?(?=\n\[)', $NewPackage
        if ($NewContent -eq $Content) {
            # Fallback if regex failed
            $NewContent = $NewPackage + "`n" + ($Content -replace '(?ms)^\[package\].*?(\n\[)', '$1')
        }

        # Add fusion_core dependency if not present and not core
        if ($NewContent -notmatch "fusion_core" -and $PackageName -ne "fusion-core" -and $PackageName -ne "fusion_core") {
            if ($NewContent -match "\[dependencies\]") {
                $NewContent = $NewContent -replace "(\[dependencies\])", "`$1`nfusion_core = { workspace = true }"
            }
        }

        Set-Content -Path $CargoPath -Value $NewContent -Encoding utf8
        $PolishedCount++
    }
}
Write-Host "`nUncategorized crates polish complete!" -ForegroundColor Green
Write-Host "Total polished: $PolishedCount crates" -ForegroundColor Cyan
