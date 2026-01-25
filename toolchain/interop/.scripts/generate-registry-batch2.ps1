# Fusion Package Registry - Batch Crate Generator (Batch 2)
# Generates remaining 40 crates from source files into registry/crates/

$ErrorActionPreference = "Stop"

$SourceDir = "C:\Projects\Fusion - Programming Language\Source Files\Ecosystem\Fusion Crates"
$RegistryDir = "C:\Projects\Fusion - Programming Language\registry\crates"

# Define the remaining 40 crates to create
$CrateDefinitions = @(
    # Hardware & Kernels
    @{Name = "cuda-interface"; Config = "CUDA Kernel Interface.txt"; Lib = "" },
    @{Name = "cuda-kernels"; Config = "CUDA Kernel Lib Config.txt"; Lib = "" },
    @{Name = "gpu-scheduler"; Config = "GPU Scheduler Config.txt"; Lib = "" },
    @{Name = "tensor-optim"; Config = "Tensor Optim Config.txt"; Lib = "" },
    @{Name = "tensor-parallel"; Config = "Tensor Parallel Config.txt"; Lib = "Tensor Parallel Logic.txt" },
    @{Name = "tensor-sparse"; Config = "Tensor Sparse Ops Config.txt"; Lib = "Sparse Matrix Ops.txt" },

    # Web & WASM
    @{Name = "wasm-server"; Config = "WASM Server Config.txt"; Lib = "WASM Runtime Implementation.txt" },
    @{Name = "webasm-renderer"; Config = "WebASM Renderer Config.txt"; Lib = "WebASM Renderer.txt" },
    @{Name = "react-hooks"; Config = "React Bridge Config.txt"; Lib = "React Hooks Bridge.txt" },

    # Advanced AI & Training
    @{Name = "dynamic-batch"; Config = "Dynamic Batch Config.txt"; Lib = "Dynamic Batch Scheduler.txt" },
    @{Name = "error-correction"; Config = "Error Correction Config.txt"; Lib = "Error Correction.txt" },
    @{Name = "prompt-prefill"; Config = "Prompt Prefill Config.txt"; Lib = "" },
    @{Name = "auto-prompt"; Config = "Prompt Optimization (AutoPrompt).txt"; Lib = "Prompt Optimization (AutoPrompt).txt" },
    @{Name = "clustering"; Config = ""; Lib = "Production Clustering.txt" },
    @{Name = "resnet"; Config = ""; Lib = "Production ResNet Layers.txt" },
    @{Name = "rl-algorithms"; Config = ""; Lib = "crates_rl-gym_src_algorithms.rs.txt" },

    # Security & Policy
    @{Name = "policy-engine"; Config = "Policy Engine Config.txt"; Lib = "Policy Engine.txt" },
    @{Name = "trusted-anchor"; Config = "Trusted Anchor Config.txt"; Lib = "" },
    @{Name = "pqc-proxy"; Config = ""; Lib = "PQC Proxy Implementation.txt" },
    @{Name = "auth"; Config = ""; Lib = "auth.rs" },

    # Integration & Tools
    @{Name = "cargo-converter"; Config = "Cargo Converter Config.txt"; Lib = "" },
    @{Name = "compiler-passes"; Config = "Compiler Passes Config.txt"; Lib = "" },
    @{Name = "python-converter"; Config = "Python Converter Config.txt"; Lib = "" },
    @{Name = "python-pkg"; Config = ""; Lib = "Python Package Manager.txt" },
    @{Name = "data-vis"; Config = "Data Vis Config.txt"; Lib = "Data Visualization.txt" },
    @{Name = "diagnostics"; Config = "Diagnostics Config.txt"; Lib = "" },
    @{Name = "observability"; Config = "Observability Config.txt"; Lib = "" },
    @{Name = "crate-analyzer"; Config = ""; Lib = "Rust Crate Analyzer.txt" },

    # Core Utilities
    @{Name = "kv-cache"; Config = ""; Lib = "kv_cache.rs" },
    @{Name = "safetensors"; Config = ""; Lib = "safetensors.rs" },
    @{Name = "qaoa"; Config = ""; Lib = "qaoa.rs" },
    @{Name = "jordan-wigner"; Config = ""; Lib = "jordan_wigner.rs" },
    @{Name = "density-matrix"; Config = ""; Lib = "density_matrix.rs" },
    @{Name = "graph"; Config = ""; Lib = "graph.rs" },
    @{Name = "tree"; Config = ""; Lib = "tree.rs" },
    @{Name = "vault"; Config = ""; Lib = "vault.rs" },
    @{Name = "solver"; Config = ""; Lib = "solver.rs" },
    @{Name = "retry"; Config = ""; Lib = "retry.rs" }
)

function Clean-RustCode {
    param($content)
    # Remove line numbers added by view_file
    $lines = $content -split "`n"
    $cleaned = $lines | ForEach-Object {
        if ($_ -match '^\d+:\s*(.*)$') {
            $matches[1]
        }
        else {
            $_
        }
    }
    return ($cleaned -join "`n")
}

Write-Host "=== Fusion Registry Crate Generator (Batch 2) ===" -ForegroundColor Cyan
Write-Host "Generating remaining crates..." -ForegroundColor Yellow

$successCount = 0
$errorCount = 0

foreach ($crate in $CrateDefinitions) {
    $crateName = $crate.Name
    $crateDir = Join-Path $RegistryDir $crateName
    $srcDir = Join-Path $crateDir "src"
    
    try {
        Write-Host "`nProcessing: $crateName" -ForegroundColor Green
        
        # Create directories
        New-Item -ItemType Directory -Force -Path $srcDir | Out-Null
        
        # Generate or read Cargo.toml
        $cargoPath = Join-Path $crateDir "Cargo.toml"
        $packageName = "fusion_" + ($crateName -replace '-', '_')
        
        if ($crate.Config -and (Test-Path (Join-Path $SourceDir $crate.Config))) {
            $configContent = Get-Content (Join-Path $SourceDir $crate.Config) -Raw
            # Fix paths
            $configContent = $configContent -replace 'path = "\.\./core"', 'path = "../../crates/core"'
            $configContent = $configContent -replace 'path = "\.\./ai-core"', 'path = "../../ecosystem/crates/ai-core"'
            $configContent = $configContent -replace 'path = "\.\./std"', 'path = "../../crates/core"'
            Set-Content -Path $cargoPath -Value $configContent -NoNewline
        }
        else {
            $cargoContent = "[package]`nname = `"$packageName`"`nversion = `"0.1.0`"`nedition = `"2021`"`nlicense = `"MIT`"`n`n[dependencies]`nfusion_core = { path = `"../../crates/core`" }`n"
            Set-Content -Path $cargoPath -Value $cargoContent -NoNewline
        }
        
        # Generate or read lib.rs
        $libPath = Join-Path $srcDir "lib.rs"
        if ($crate.Lib -and (Test-Path (Join-Path $SourceDir $crate.Lib))) {
            $libContent = Get-Content (Join-Path $SourceDir $crate.Lib) -Raw
            $libContent = Clean-RustCode $libContent
            Set-Content -Path $libPath -Value $libContent -NoNewline
        }
        else {
            $libContent = "/// $crateName implementation`n`npub struct ${crateName}Module;`n"
            Set-Content -Path $libPath -Value $libContent -NoNewline
        }
        
        Write-Host "  Created $crateName" -ForegroundColor DarkGreen
        $successCount++
        
    }
    catch {
        Write-Host "  Error creating $crateName : $_" -ForegroundColor Red
        $errorCount++
    }
}

Write-Host "`n=== Summary ===" -ForegroundColor Cyan
Write-Host "Created: $successCount / $($CrateDefinitions.Count) crates" -ForegroundColor Green
Write-Host "Errors: $errorCount crates" -ForegroundColor $(if ($errorCount -gt 0) { "Red" }else { "Green" })
