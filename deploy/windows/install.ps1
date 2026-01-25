Param(
    [string]$ZipPath = "fusion-toolchain.zip",
    [string]$InstallRoot = "$env:LOCALAPPDATA\Fusion"
)

$ErrorActionPreference = "Stop"

if (!(Test-Path $ZipPath)) {
    Write-Error "Bundle not found: $ZipPath"
    exit 1
}

if (!(Test-Path $InstallRoot)) {
    New-Item -ItemType Directory -Path $InstallRoot | Out-Null
}

$distPath = Join-Path $InstallRoot "dist"
if (Test-Path $distPath) {
    Remove-Item -Recurse -Force $distPath
}

Expand-Archive -Path $ZipPath -DestinationPath $InstallRoot

$binPath = Join-Path $InstallRoot "dist\bin"
$envPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($envPath -notlike "*$binPath*") {
    [Environment]::SetEnvironmentVariable("Path", "$envPath;$binPath", "User")
}

Write-Host "Fusion installed to $InstallRoot"
Write-Host "Open a new terminal and run: fusion --version"
