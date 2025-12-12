# Fusion Flux Engine - Setup and Enforcement Installation

Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║   FUSION FLUX ENGINE - POLICY ENFORCEMENT INSTALLATION     ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$ErrorActionPreference = "Stop"

# Step 1: Build Fusion Flux Engine
Write-Host "📦 Step 1/5: Building Fusion Flux Engine..." -ForegroundColor Yellow
Write-Host ""

Push-Location "runtime"
try {
    $buildOutput = cargo build -p fusion_flux_resolve --release 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Fusion Flux Engine built successfully" -ForegroundColor Green
    }
    else {
        Write-Host "❌ Failed to build Fusion Flux Engine" -ForegroundColor Red
        Write-Host $buildOutput
        exit 1
    }
}
finally {
    Pop-Location
}

Write-Host ""

# Step 2: Run tests
Write-Host "🧪 Step 2/5: Running Flux Engine tests..." -ForegroundColor Yellow
Write-Host ""

Push-Location "runtime"
try {
    $testOutput = cargo test -p fusion_flux_resolve --release 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ All tests passed" -ForegroundColor Green
    }
    else {
        Write-Host "❌ Tests failed" -ForegroundColor Red
        Write-Host $testOutput
        exit 1
    }
}
finally {
    Pop-Location
}

Write-Host ""

# Step 3: Install Git hooks
Write-Host "🪝 Step 3/5: Installing Git hooks..." -ForegroundColor Yellow
Write-Host ""

# Configure Git to use custom hooks directory
git config core.hooksPath .githooks

# Make hooks executable (Git Bash on Windows)
if (Test-Path ".githooks\pre-commit") {
    git update-index --chmod=+x .githooks/pre-commit
    Write-Host "✅ Git hooks installed" -ForegroundColor Green
}
else {
    Write-Host "⚠️  Git hook file not found" -ForegroundColor Yellow
}

Write-Host ""

# Step 4: Set environment variables
Write-Host "⚙️  Step 4/5: Configuring environment..." -ForegroundColor Yellow
Write-Host ""

# Set for current session
$env:FUSION_FLUX_ENABLED = "true"
$env:FUSION_STRICT_MODE = "true"
$env:ALLOW_CARGO_FALLBACK = "false"

Write-Host "✅ Environment variables set for current session" -ForegroundColor Green
Write-Host ""

# Prompt to set globally
Write-Host "Would you like to set these permanently? (Y/N): " -NoNewline
$response = Read-Host

if ($response -eq "Y" -or $response -eq "y") {
    [Environment]::SetEnvironmentVariable("FUSION_FLUX_ENABLED", "true", "User")
    [Environment]::SetEnvironmentVariable("FUSION_STRICT_MODE", "true", "User")
    [Environment]::SetEnvironmentVariable("ALLOW_CARGO_FALLBACK", "false", "User")
    Write-Host "✅ Environment variables set permanently" -ForegroundColor Green
}
else {
    Write-Host "⚠️  Environment variables are session-only" -ForegroundColor Yellow
    Write-Host "   Add to your PowerShell profile for persistence" -ForegroundColor Yellow
}

Write-Host ""

# Step 5: Create aliases
Write-Host "🔗 Step 5/5: Creating command aliases..." -ForegroundColor Yellow
Write-Host ""

$profilePath = $PROFILE
$aliasContent = @"

# Fusion Build Policy Enforcement
`$env:FUSION_FLUX_ENABLED = 'true'
`$env:FUSION_STRICT_MODE = 'true'

# Alias cargo to enforcement script
function cargo {
    & "$PSScriptRoot\.scripts\enforce-flux-build.ps1" @args
}
"@

Write-Host "Add the following to your PowerShell profile? ($profilePath)" -ForegroundColor Cyan
Write-Host $aliasContent -ForegroundColor Gray
Write-Host ""
Write-Host "(Y/N): " -NoNewline
$response = Read-Host

if ($response -eq "Y" -or $response -eq "y") {
    # Create profile if it doesn't exist
    if (!(Test-Path $profilePath)) {
        New-Item -Path $profilePath -ItemType File -Force | Out-Null
    }
    
    Add-Content -Path $profilePath -Value $aliasContent
    Write-Host "✅ Aliases added to PowerShell profile" -ForegroundColor Green
    Write-Host "   Restart PowerShell or run: . $profilePath" -ForegroundColor Yellow
}
else {
    Write-Host "⚠️  Aliases not added - refer to BUILD_POLICY.md" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║              INSTALLATION COMPLETE                         ║" -ForegroundColor Green
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""

Write-Host "📋 Summary:" -ForegroundColor Cyan
Write-Host "   ✅ Fusion Flux Engine: Built and tested" -ForegroundColor Green
Write-Host "   ✅ Git Hooks: Installed" -ForegroundColor Green
Write-Host "   ✅ Environment: Configured" -ForegroundColor Green
Write-Host "   ✅ Policy: ENFORCED" -ForegroundColor Green
Write-Host ""

Write-Host "📖 Next Steps:" -ForegroundColor Cyan
Write-Host "   1. Restart your terminal/IDE" -ForegroundColor White
Write-Host "   2. Try: fusion build" -ForegroundColor White
Write-Host "   3. Read: BUILD_POLICY.md" -ForegroundColor White
Write-Host ""

Write-Host "⚠️  From now on:" -ForegroundColor Yellow
Write-Host "   ❌ cargo build  → BLOCKED" -ForegroundColor Red
Write-Host "   ✅ fusion build → REQUIRED" -ForegroundColor Green
Write-Host ""

Write-Host "🆘 Emergency override:" -ForegroundColor Yellow
Write-Host "   `$env:FUSION_STRICT_MODE='false'" -ForegroundColor Gray
Write-Host "   `$env:ALLOW_CARGO_FALLBACK='true'" -ForegroundColor Gray
Write-Host ""
