# Fusion Package Registry - Batch Crate Generator (Batch 5 - Final Verificaton)
# Generates final 8 missing crates found in deep audit

$ErrorActionPreference = "Stop"

$SourceDir = "C:\Projects\Fusion - Programming Language\Source Files\Ecosystem\Fusion Crates"
$RegistryDir = "C:\Projects\Fusion - Programming Language\registry\crates"

# Define the Batch 5 crates
$CrateDefinitions = @(
    @{Name = "cloud-agent"; Config = "Cloud Agent Config.txt"; Lib = "Cloud Agent.txt" },
    @{Name = "component-lib"; Config = "Component Lib Config.txt"; Lib = "Component Library.txt" },
    @{Name = "custom-tokenizer"; Config = "Custom Tokenizer Config.txt"; Lib = "" },
    @{Name = "executor"; Config = ""; Lib = "executor.rs" },
    @{Name = "training"; Config = ""; Lib = "train.rs" },
    @{Name = "toolchain-ext"; Config = ""; Lib = "crates_toolchain_src_build.rs.txt" },
    @{Name = "version"; Config = ""; Lib = "version.rs" },
    @{Name = "std-ext"; Config = ""; Lib = "lib.rs" }
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

Write-Host "=== Fusion Registry Crate Generator (Batch 5) ===" -ForegroundColor Cyan
Write-Host "Generating final 8 crates..." -ForegroundColor Yellow

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
        
        Write-Host "  Created $crateName" -ForegroundColor DarkGreen
        $successCount++
        
    }
    catch {
        Write-Host "  Error creating $crateName : $_" -ForegroundColor Red
    }
}

Write-Host "`n=== Summary ===" -ForegroundColor Cyan
Write-Host "Created: $successCount crates" -ForegroundColor Green
