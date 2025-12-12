# Fusion Registry Migration Script
# Moves ecosystem/crates into registry/crates with overwrite priority

$EcosystemDir = "C:\Projects\Fusion - Programming Language\ecosystem\crates"
$RegistryDir = "C:\Projects\Fusion - Programming Language\registry\crates"

$EcosystemCrates = Get-ChildItem -Directory $EcosystemDir

Write-Host "=== Migrating Ecosystem Crates to Registry ===" -ForegroundColor Cyan

foreach ($crate in $EcosystemCrates) {
    $crateName = $crate.Name
    $destPath = Join-Path $RegistryDir $crateName
    
    if (Test-Path $destPath) {
        Write-Host "Overwriting existing registry crate: $crateName" -ForegroundColor Yellow
        Remove-Item -Recurse -Force $destPath
    }
    else {
        Write-Host "Moving unique crate: $crateName" -ForegroundColor Green
    }
    
    Move-Item -Path $crate.FullName -Destination $RegistryDir
}

Write-Host "Migration Complete." -ForegroundColor Group
