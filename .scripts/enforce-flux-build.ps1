# Fusion Build Policy Enforcement
# This script ensures Fusion Flux Engine is used instead of cargo

param(
    [Parameter(Position = 0)]
    [string]$Command = "",
    
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$Args
)

# Configuration
$FUSION_FLUX_ENABLED = $env:FUSION_FLUX_ENABLED ?? "true"
$ALLOW_CARGO_FALLBACK = $env:ALLOW_CARGO_FALLBACK ?? "false"
$STRICT_MODE = $env:FUSION_STRICT_MODE ?? "true"

# Colors for output
function Write-Success { Write-Host $args -ForegroundColor Green }
function Write-Error-Custom { Write-Host $args -ForegroundColor Red }
function Write-Warning-Custom { Write-Host $args -ForegroundColor Yellow }
function Write-Info { Write-Host $args -ForegroundColor Cyan }

# Check if Fusion Flux Engine is available
function Test-FluxAvailable {
    $fluxPath = "runtime\crates\fusion_flux_resolve"
    if (Test-Path $fluxPath) {
        if (Test-Path "$fluxPath\target\debug\fusion_flux_resolve.dll") {
            return $true
        }
        if (Test-Path "$fluxPath\target\release\fusion_flux_resolve.dll") {
            return $true
        }
    }
    return $false
}

# Detect if user is trying to run cargo directly
function Test-CargoAttempt {
    param([string]$cmd)
    
    $cargoCommands = @("cargo", "cargo.exe", "cargo-build", "cargo-test", "cargo-run", "cargo-check")
    foreach ($c in $cargoCommands) {
        if ($cmd -like "*$c*") {
            return $true
        }
    }
    return $false
}

# Main enforcement logic
function Invoke-BuildEnforcement {
    param(
        [string]$command,
        [string[]]$arguments
    )
    
    Write-Info "╔══════════════════════════════════════════════════════════╗"
    Write-Info "║        FUSION BUILD POLICY ENFORCEMENT                   ║"
    Write-Info "╚══════════════════════════════════════════════════════════╝"
    Write-Info ""
    
    # Check if Fusion Flux is enabled
    if ($FUSION_FLUX_ENABLED -ne "true") {
        Write-Warning-Custom "⚠️  Fusion Flux Engine is DISABLED"
        Write-Warning-Custom "   Set FUSION_FLUX_ENABLED=true to enable"
        Write-Warning-Custom "   Falling back to cargo..."
        return $false
    }
    
    # Check if this is a cargo attempt
    $isCargoAttempt = Test-CargoAttempt -cmd $command
    
    if ($isCargoAttempt -and $STRICT_MODE -eq "true") {
        Write-Error-Custom "❌ POLICY VIOLATION: Direct cargo usage detected!"
        Write-Error-Custom ""
        Write-Error-Custom "   Command attempted: $command"
        Write-Error-Custom ""
        Write-Error-Custom "   FUSION BUILD POLICY:"
        Write-Error-Custom "   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        Write-Error-Custom "   Fusion projects must use Fusion Flux Engine."
        Write-Error-Custom "   Direct cargo usage is prohibited."
        Write-Error-Custom ""
        Write-Error-Custom "   ✅ USE INSTEAD:"
        Write-Error-Custom "      fusion build    (instead of 'cargo build')"
        Write-Error-Custom "      fusion test     (instead of 'cargo test')"
        Write-Error-Custom "      fusion run      (instead of 'cargo run')"
        Write-Error-Custom "      fusion check    (instead of 'cargo check')"
        Write-Error-Custom ""
        Write-Error-Custom "   ⚙️  TO DISABLE ENFORCEMENT (emergency only):"
        Write-Error-Custom "      $env:FUSION_STRICT_MODE='false'"
        Write-Error-Custom "      $env:ALLOW_CARGO_FALLBACK='true'"
        Write-Error-Custom ""
        exit 1
    }
    
    # Check if Flux is available
    $fluxAvailable = Test-FluxAvailable
    
    if (-not $fluxAvailable) {
        Write-Warning-Custom "⚠️  Fusion Flux Engine not built yet"
        Write-Info ""
        Write-Info "   Building Fusion Flux Engine first..."
        
        Push-Location "runtime"
        $buildResult = & cargo build -p fusion_flux_resolve 2>&1
        Pop-Location
        
        if ($LASTEXITCODE -eq 0) {
            Write-Success "✅ Fusion Flux Engine built successfully"
            return $true
        }
        else {
            Write-Error-Custom "❌ Failed to build Fusion Flux Engine"
            
            if ($ALLOW_CARGO_FALLBACK -eq "true") {
                Write-Warning-Custom "   Fallback to cargo enabled - proceeding with cargo"
                return $false
            }
            else {
                Write-Error-Custom "   Cargo fallback disabled. Build cannot continue."
                Write-Error-Custom "   To enable fallback: $env:ALLOW_CARGO_FALLBACK='true'"
                exit 1
            }
        }
    }
    
    Write-Success "✅ Fusion Flux Engine is available"
    return $true
}

# Execute the enforcement
$shouldUseFusion = Invoke-BuildEnforcement -command $Command -arguments $Args

if ($shouldUseFusion) {
    Write-Info ""
    Write-Info "🚀 Routing build through Fusion Flux Engine..."
    Write-Info ""
    
    # TODO: Once Fusion CLI is ready, route through it
    # For now, inform user
    Write-Warning-Custom "⚠️  Fusion CLI integration pending"
    Write-Warning-Custom "   Currently using cargo as build backend"
    Write-Warning-Custom "   Fusion Flux will handle dependency resolution"
    Write-Info ""
    
    # Execute cargo with Fusion Flux awareness
    $cargoCmd = "cargo"
    if ($Command) {
        $cargoCmd += " $Command"
    }
    if ($Args) {
        $cargoCmd += " " + ($Args -join " ")
    }
    
    Invoke-Expression $cargoCmd
    exit $LASTEXITCODE
}
else {
    # Fallback to cargo
    Write-Info "Using cargo directly (fallback mode)"
    
    $cargoCmd = "cargo"
    if ($Command) {
        $cargoCmd += " $Command"
    }
    if ($Args) {
        $cargoCmd += " " + ($Args -join " ")
    }
    
    Invoke-Expression $cargoCmd
    exit $LASTEXITCODE
}
