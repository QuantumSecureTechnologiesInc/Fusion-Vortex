# HyperCycle Vortex v2.0.2 - Local Development Setup
# For Windows PowerShell

Write-Host "=== HyperCycle Vortex v2.0.2 - Local Setup ===" -ForegroundColor Cyan
Write-Host ""

# Check prerequisites
Write-Host "Checking prerequisites..." -ForegroundColor Yellow

# Check CMake
if (Get-Command cmake -ErrorAction SilentlyContinue) {
    $cmakeVersion = cmake --version | Select-Object -First 1
    Write-Host "✅ CMake found: $cmakeVersion" -ForegroundColor Green
} else {
    Write-Host "❌ CMake not found. Please install CMake 3.21+" -ForegroundColor Red
    Write-Host "Download from: https://cmake.org/download/" -ForegroundColor Yellow
    exit 1
}

# Check compiler
$compilerFound = $false
if (Get-Command cl -ErrorAction SilentlyContinue) {
    Write-Host "✅ MSVC found" -ForegroundColor Green
    $compilerFound = $true
} elseif (Get-Command gcc -ErrorAction SilentlyContinue) {
    Write-Host "✅ GCC found (MinGW)" -ForegroundColor Green
    $compilerFound = $true
} elseif (Get-Command clang -ErrorAction SilentlyContinue) {
    Write-Host "✅ Clang found" -ForegroundColor Green
    $compilerFound = $true
}

if (-not $compilerFound) {
    Write-Host "❌ No C compiler found" -ForegroundColor Red
    Write-Host "Please install Visual Studio 2019+ or MinGW-w64" -ForegroundColor Yellow
    exit 1
}

# Check OpenSSL (optional)
if (Test-Path "C:\Program Files\OpenSSL-Win64\bin\openssl.exe") {
    Write-Host "✅ OpenSSL found" -ForegroundColor Green
} else {
    Write-Host "⚠️  OpenSSL not found (optional - will use internal SHA-3)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Configuring build..." -ForegroundColor Yellow

# Navigate to project root
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location (Join-Path $scriptPath "../..")

# Create build directory
if (Test-Path "build") {
    Remove-Item -Recurse -Force "build"
}
New-Item -ItemType Directory -Path "build" | Out-Null
Set-Location "build"

# Detect generator
$generator = "MinGW Makefiles"
if (Get-Command cl -ErrorAction SilentlyContinue) {
    $generator = "Visual Studio 17 2022"
}

# Configure with CMake
cmake .. `
    -G "$generator" `
    -DCMAKE_BUILD_TYPE=Release `
    -DHC_BUILD_TESTS=ON `
    -DHC_ENABLE_CUDA=OFF `
    -DHC_ENABLE_ROCM=OFF

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ CMake configuration failed" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Building..." -ForegroundColor Yellow

# Build
cmake --build . --config Release --parallel

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Running tests..." -ForegroundColor Yellow

# Run tests
ctest -C Release --output-on-failure

Write-Host ""
Write-Host "✅ Setup complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Binaries are in: $(Get-Location)" -ForegroundColor Cyan
Write-Host ""
Write-Host "To run the dashboard:" -ForegroundColor Yellow
Write-Host "  .\Release\vortex_dashboard.exe" -ForegroundColor White
Write-Host ""
Write-Host "To run the secure dashboard:" -ForegroundColor Yellow
Write-Host "  .\Release\secure_dashboard.exe" -ForegroundColor White
Write-Host ""
