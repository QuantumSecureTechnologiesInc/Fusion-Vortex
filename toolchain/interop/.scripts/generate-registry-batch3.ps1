# Fusion Package Registry - Batch Crate Generator (Batch 3 - Final Sweep)
# Generates final set of crates to ensure 100% file coverage

$ErrorActionPreference = "Stop"

$SourceDir = "C:\Projects\Fusion - Programming Language\Source Files\Ecosystem\Fusion Crates"
$RegistryDir = "C:\Projects\Fusion - Programming Language\registry\crates"

# Define the Batch 3 crates
$CrateDefinitions = @(
    @{Name = "finite-fields"; Config = ""; Lib = "Finite Fields.txt" },
    @{Name = "gate-decomposition"; Config = ""; Lib = "Gate Decomposition.txt" },
    @{Name = "layout-builder"; Config = "Layout Builder Config.txt"; Lib = "Layout Builder Implementation.txt" },
    @{Name = "stream-monitor"; Config = "Stream Monitor Config.txt"; Lib = "" },
    @{Name = "supply-chain"; Config = "Supply Chain Config.txt"; Lib = "" },
    @{Name = "trie-search"; Config = "Trie Search Config.txt"; Lib = "Trie Search.txt" },
    @{Name = "ops"; Config = ""; Lib = "ops.rs" },
    @{Name = "transform"; Config = ""; Lib = "transform.rs" },
    @{Name = "qubo"; Config = ""; Lib = "qubo.rs" },
    @{Name = "block"; Config = ""; Lib = "block.rs" },
    @{Name = "carver"; Config = ""; Lib = "carver.rs" }
)

function Clean-RustCode {
    param($content)
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

Write-Host "=== Fusion Registry Crate Generator (Batch 3) ===" -ForegroundColor Cyan
Write-Host "Generating final crates..." -ForegroundColor Yellow

$successCount = 0
$errorCount = 0

foreach ($crate in $CrateDefinitions) {
    $crateName = $crate.Name
    $crateDir = Join-Path $RegistryDir $crateName
    $srcDir = Join-Path $crateDir "src"
    
    try {
        Write-Host "`nProcessing: $crateName" -ForegroundColor Green
        
        New-Item -ItemType Directory -Force -Path $srcDir | Out-Null
        
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
        
        $libPath = Join-Path $srcDir "lib.rs"
        if ($crate.Lib -and (Test-Path (Join-Path $SourceDir $crate.Lib))) {
            $libContent = Get-Content (Join-Path $SourceDir $crate.Lib) -Raw
            $libContent = Clean-RustCode $libContent
            Set-Content -Path $libPath -Value $libContent -NoNewline
        }
        else {
            $libContent = "/// $crateName implementation`npub struct ${crateName}Module;"
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
