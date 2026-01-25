Param(
    [string]$DistPath = "..\..\dist",
    [string]$OutputPath = "fusion-toolchain.msi"
)

$ErrorActionPreference = "Stop"

if (!(Test-Path $DistPath)) {
    Write-Error "dist not found: $DistPath"
    exit 1
}

$wix = Get-Command candle.exe -ErrorAction SilentlyContinue
if (!$wix) {
    Write-Error "WiX toolset not found. Install WiX Toolset v3+ and ensure candle.exe/light.exe are in PATH."
    exit 1
}

$wxs = Join-Path $PSScriptRoot "wix\FusionToolchain.wxs"
$candle = "candle.exe"
$light = "light.exe"

& $candle -dDistPath=$DistPath -out FusionToolchain.wixobj $wxs
& $light -out $OutputPath FusionToolchain.wixobj

Write-Host "MSI built: $OutputPath"
