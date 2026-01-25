# Fusion Package Registry - Batch Crate Generator (Batch 4 - Deep Clean)
# Generates final missing crates identified from deep audit

$ErrorActionPreference = "Stop"

$SourceDir = "C:\Projects\Fusion - Programming Language\Source Files\Ecosystem\Fusion Crates"
$RegistryDir = "C:\Projects\Fusion - Programming Language\registry\crates"
$NewFolderDir = Join-Path $SourceDir "New folder"

# Define the Batch 4 crates
$CrateDefinitions = @(
    @{Name = "tokenizers"; Config = "Tokenizers Config.txt"; Lib = "Tokenizers Interface.txt" },
    @{Name = "embeddings"; Config = "Embeddings Config.txt"; Lib = "" },
    @{Name = "metrics"; Config = "Metrics Config.txt"; Lib = "" },
    @{Name = "attention"; Config = ""; Lib = "attention.rs" },
    @{Name = "client"; Config = ""; Lib = "client.rs" },
    @{Name = "inference-graph"; Config = ""; Lib = "Inference Graph.txt" },
    
    # Handling logical groupings
    @{Name = "model-server-core"; Config = ""; Lib = "" } # Placeholder for New folder content
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

Write-Host "=== Fusion Registry Crate Generator (Batch 4) ===" -ForegroundColor Cyan
Write-Host "Generating final 7 crates..." -ForegroundColor Yellow

$successCount = 0

foreach ($crate in $CrateDefinitions) {
    $crateName = $crate.Name
    $crateDir = Join-Path $RegistryDir $crateName
    $srcDir = Join-Path $crateDir "src"
    
    try {
        Write-Host "`nProcessing: $crateName" -ForegroundColor Green
        New-Item -ItemType Directory -Force -Path $srcDir | Out-Null
        
        $cargoPath = Join-Path $crateDir "Cargo.toml"
        $libPath = Join-Path $srcDir "lib.rs"
        
        # Special handling for New Folder content
        if ($crateName -eq "model-server-core") {
            if (Test-Path (Join-Path $NewFolderDir "Model Server Config.txt")) {
                $configContent = Get-Content (Join-Path $NewFolderDir "Model Server Config.txt") -Raw
                $configContent = Clean-RustCode $configContent
                Set-Content -Path $cargoPath -Value $configContent -NoNewline
            }
            if (Test-Path (Join-Path $NewFolderDir "Server Logic.txt")) {
                $libContent = Get-Content (Join-Path $NewFolderDir "Server Logic.txt") -Raw
                $libContent = Clean-RustCode $libContent
                Set-Content -Path $libPath -Value $libContent -NoNewline
            }
        }
        else {
            # Standard logic
            $packageName = "fusion_" + ($crateName -replace '-', '_')
            
            if ($crate.Config -and (Test-Path (Join-Path $SourceDir $crate.Config))) {
                $configContent = Get-Content (Join-Path $SourceDir $crate.Config) -Raw
                $configContent = Clean-RustCode $configContent
                $configContent = $configContent -replace 'path = "\.\./core"', 'path = "../../crates/core"'
                Set-Content -Path $cargoPath -Value $configContent -NoNewline
            }
            else {
                $cargoContent = "[package]`nname = `"$packageName`"`nversion = `"0.1.0`"`nedition = `"2021`"`nlicense = `"MIT`"`n`n[dependencies]`nfusion_core = { path = `"../../crates/core`" }`n"
                Set-Content -Path $cargoPath -Value $cargoContent -NoNewline
            }
            
            if ($crate.Lib -and (Test-Path (Join-Path $SourceDir $crate.Lib))) {
                $libContent = Get-Content (Join-Path $SourceDir $crate.Lib) -Raw
                $libContent = Clean-RustCode $libContent
                Set-Content -Path $libPath -Value $libContent -NoNewline
            }
            else {
                $libContent = "/// $crateName implementation`npub struct ${crateName}Module;"
                Set-Content -Path $libPath -Value $libContent -NoNewline
            }
        }
        
        Write-Host "  Created $crateName" -ForegroundColor DarkGreen
        $successCount++
        
    }
    catch {
        Write-Host "  Error creating $crateName : $_" -ForegroundColor Red
    }
}

Write-Host "`n=== Summary ===" -ForegroundColor Cyan
Write-Host "Created: $successCount crates" -ForegroundColor Green
