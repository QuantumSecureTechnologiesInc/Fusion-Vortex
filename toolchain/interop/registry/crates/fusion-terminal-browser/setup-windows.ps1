# Fusion Terminal Browser - Windows Setup Script
# Downloads Chrome for Testing and sets up the environment

Write-Host "===================================================" -ForegroundColor Cyan
Write-Host " Fusion Terminal Browser - Windows Setup" -ForegroundColor Cyan
Write-Host "===================================================" -ForegroundColor Cyan
Write-Host ""

# Create bin directory
$binDir = Join-Path $PSScriptRoot "bin"
if (-not (Test-Path $binDir)) {
    New-Item -ItemType Directory -Path $binDir | Out-Null
}

# Check if Chrome already exists
$chromeExe = Join-Path $binDir "chrome-win64\chrome.exe"
if (Test-Path $chromeExe) {
    Write-Host "Chrome for Testing already installed at: $chromeExe" -ForegroundColor Green
    Write-Host "Skipping download..." -ForegroundColor Yellow
}
else {
    Write-Host "Downloading Chrome for Testing..." -ForegroundColor Yellow
    
    # Get latest Chrome for Testing URL
    $latestUrl = "https://googlechromelabs.github.io/chrome-for-testing/latest-versions-per-milestone-with-downloads.json"
    $json = Invoke-RestMethod -Uri $latestUrl
    
    # Extract Windows 64-bit chrome download URL (latest stable)
    $chromeUrl = $json.milestones.PSObject.Properties | 
    Sort-Object { [int]$_.Name } -Descending |
    Select-Object -First 1 |
    ForEach-Object { $_.Value.downloads.chrome } |
    Where-Object { $_.platform -eq "win64" } |
    Select-Object -ExpandProperty url
    
    if (-not $chromeUrl) {
        # Fallback to direct URL
        Write-Host "Using fallback URL..." -ForegroundColor Yellow
        $chromeUrl = "https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.85/win64/chrome-win64.zip"
    }
    
    Write-Host "Download URL: $chromeUrl" -ForegroundColor Cyan
    
    # Download
    $zipPath = Join-Path $binDir "chrome.zip"
    try {
        Invoke-WebRequest -Uri $chromeUrl -OutFile $zipPath -UseBasicParsing
        Write-Host "Download complete!" -ForegroundColor Green
        
        # Extract
        Write-Host "Extracting Chrome..." -ForegroundColor Yellow
        Expand-Archive -Path $zipPath -DestinationPath $binDir -Force
        
        # Cleanup
        Remove-Item $zipPath
        Write-Host "Extraction complete!" -ForegroundColor Green
    }
    catch {
        Write-Host "Error downloading Chrome: $_" -ForegroundColor Red
        Write-Host "Please download Chrome for Testing manually from:" -ForegroundColor Yellow
        Write-Host "https://googlechromelabs.github.io/chrome-for-testing/" -ForegroundColor Cyan
        exit 1
    }
}

# Set environment variable (for current session)
$env:CHROME_PATH = $chromeExe
Write-Host ""
Write-Host "Chrome installed at: $chromeExe" -ForegroundColor Green

# Create logs directory
$logsDir = Join-Path $PSScriptRoot "logs"
if (-not (Test-Path $logsDir)) {
    New-Item -ItemType Directory -Path $logsDir | Out-Null
    Write-Host "Created logs directory: $logsDir" -ForegroundColor Green
}

# Create chrome profile directory
$profileDir = Join-Path $PSScriptRoot "chrome_profile"
if (-not (Test-Path $profileDir)) {
    New-Item -ItemType Directory -Path $profileDir | Out-Null
    Write-Host "Created Chrome profile directory: $profileDir" -ForegroundColor Green
}

Write-Host ""
Write-Host "===================================================" -ForegroundColor Cyan
Write-Host " Setup Complete!" -ForegroundColor Green
Write-Host "===================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "To persist CHROME_PATH, add to your PowerShell profile:" -ForegroundColor Yellow
Write-Host "`$env:CHROME_PATH = '$chromeExe'" -ForegroundColor Cyan
Write-Host ""
Write-Host "Or add to system environment variables permanently." -ForegroundColor Yellow
Write-Host ""
Write-Host "Build and run:" -ForegroundColor Cyan
Write-Host "  fusion build --release" -ForegroundColor White
Write-Host "  fusion run --release -- --url https://webgpu.github.io/webgpu-samples/" -ForegroundColor White
Write-Host ""
