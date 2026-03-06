$ErrorActionPreference = "Stop"

Write-Host ">>> Fusion Native Packaging (Windows x64)" -ForegroundColor Cyan

$Root = (Get-Item $PSScriptRoot).Parent.FullName
$CompilerExe = Join-Path $Root "target\release\fuc.exe"
if (-not (Test-Path $CompilerExe)) {
    throw "Compiler not found: $CompilerExe"
}

$Stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$DistRoot = Join-Path $Root "dist\native\windows-x64"
$PackagesDir = Join-Path $Root "artifacts\packages"
$OutZip = Join-Path $PackagesDir ("Fusion-native-windows-x64-" + $Stamp + ".zip")
$ManifestPath = Join-Path $DistRoot "manifest.json"

if (Test-Path $DistRoot) {
    Remove-Item -Recurse -Force $DistRoot
}
New-Item -ItemType Directory -Force -Path $DistRoot | Out-Null
New-Item -ItemType Directory -Force -Path $PackagesDir | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $DistRoot "bin") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $DistRoot "stdlib") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $DistRoot "registry\crates\fusion-iot\src") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $DistRoot "docs\guides") | Out-Null

Copy-Item -Force $CompilerExe (Join-Path $DistRoot "bin\fuc.exe")
if (Test-Path (Join-Path $Root "bin\fusionrt.lib")) {
    Copy-Item -Force (Join-Path $Root "bin\fusionrt.lib") (Join-Path $DistRoot "bin\fusionrt.lib")
}
if (Test-Path (Join-Path $Root "bin\libfusionrt.a")) {
    Copy-Item -Force (Join-Path $Root "bin\libfusionrt.a") (Join-Path $DistRoot "bin\libfusionrt.a")
}

Get-ChildItem -Path (Join-Path $Root "stdlib") -File -Filter *.fu | ForEach-Object {
    Copy-Item -Force $_.FullName (Join-Path $DistRoot ("stdlib\" + $_.Name))
}

Copy-Item -Force (Join-Path $Root "registry\crates\fusion-iot\src\lib.fu") (Join-Path $DistRoot "registry\crates\fusion-iot\src\lib.fu")
Copy-Item -Force (Join-Path $Root "README.md") (Join-Path $DistRoot "README.md")
if (Test-Path (Join-Path $Root "docs\guides\QuickStartGuide.md")) {
    Copy-Item -Force (Join-Path $Root "docs\guides\QuickStartGuide.md") (Join-Path $DistRoot "docs\guides\QuickStartGuide.md")
}
if (Test-Path (Join-Path $Root "docs\meta\ChangeLog.md")) {
    New-Item -ItemType Directory -Force -Path (Join-Path $DistRoot "docs\meta") | Out-Null
    Copy-Item -Force (Join-Path $Root "docs\meta\ChangeLog.md") (Join-Path $DistRoot "docs\meta\ChangeLog.md")
}

$gitCommit = ""
try {
    $gitCommit = (git -C $Root rev-parse --short HEAD).Trim()
} catch {
    $gitCommit = "unknown"
}

$manifest = [pscustomobject]@{
    product = "Fusion v2.0 Vortex"
    package = "native-windows-x64"
    generated_at = (Get-Date).ToString("o")
    git_commit = $gitCommit
    compiler = "bin/fuc.exe"
    stdlib = "stdlib/*.fu"
    compat_modules = @(
        "registry/crates/fusion-iot/src/lib.fu"
    )
    notes = @(
        "Compiler pipeline is native-first with .fu source of truth.",
        "Bootstrap stage is strict no-Cargo and reuses an existing stage0 native compiler binary."
    )
}
$manifest | ConvertTo-Json -Depth 6 | Set-Content -Encoding UTF8 $ManifestPath

if (Test-Path $OutZip) {
    Remove-Item -Force $OutZip
}
Compress-Archive -Path (Join-Path $DistRoot "*") -DestinationPath $OutZip -CompressionLevel Optimal

Write-Host ">>> Package complete: $OutZip" -ForegroundColor Green
