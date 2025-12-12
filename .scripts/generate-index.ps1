$RegistryDir = "C:\Projects\Fusion - Programming Language\registry\crates"
$IndexFile = "C:\Projects\Fusion - Programming Language\registry\index.json"

$Crates = Get-ChildItem -Directory $RegistryDir
$Index = @{}

foreach ($crate in $Crates) {
    # Read Cargo.toml to get actual package name
    $cargoPath = Join-Path $crate.FullName "Cargo.toml"
    if (Test-Path $cargoPath) {
        $content = Get-Content $cargoPath -Raw
        if ($content -match 'name\s*=\s*"([^"]+)"') {
            $pkgName = $matches[1]
            $Index[$pkgName] = "crates/" + $crate.Name
        }
    }
}

$Index | ConvertTo-Json -Depth 2 | Set-Content $IndexFile
Write-Host "Generated registry index with $($Index.Count) crates." -ForegroundColor Green
