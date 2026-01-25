$Unpolished = @()
$Crates = Get-ChildItem -Path "registry/crates" -Directory

foreach ($crate in $Crates) {
    $CargoPath = Join-Path $crate.FullName "Cargo.toml"
    if (-not (Test-Path $CargoPath)) { continue }
    
    $Content = Get-Content $CargoPath -Raw
    if ($Content -notmatch 'description\s*=\s*"(Foundation|Algorithm|Integration|Framework|Tool|Experimental):') {
        $Unpolished += $crate.Name
    }
}

Write-Host "Unpolished Crates ($($Unpolished.Count)):" -ForegroundColor Yellow
$Unpolished | ForEach-Object { Write-Host " - $_" }
