# Update Cargo Dependencies to Point to Registry
# 
$Root = "C:\Projects\Fusion - Programming Language"
$RegistryBase = "$Root\registry"
$IndexFile = "$RegistryBase\index.json"

$Index = Get-Content $IndexFile | ConvertFrom-Json
$IndexInfos = @{} # map Name -> RelativePathFromRegistry

# Convert output object to hashtable
$Index.PSObject.Properties | ForEach-Object {
    $IndexInfos[$_.Name] = $_.Value
}

# Target directories to update
$Targets = @("crates", "runtime\crates", "cmd", "services", "mcp", "registry\crates")

function Get-RelativePath {
    param($Path, $To)
    $Path = [System.IO.Path]::GetFullPath($Path)
    $To = [System.IO.Path]::GetFullPath($To)
    
    $PathUri = [Uri]$Path
    # Ensure folder paths end with slash for correct URI calculation
    if (-not $To.EndsWith("\")) { $To = $To + "\" }
    $ToUri = [Uri]$To
    
    $RelativeUri = $PathUri.MakeRelativeUri($ToUri)
    $RelativePath = [System.Uri]::UnescapeDataString($RelativeUri.ToString())
    
    return $RelativePath.Replace('/', '\')
}

$Files = Get-ChildItem -Path ($Targets | ForEach-Object { Join-Path $Root $_ }) -Recurse -Filter "Cargo.toml"

foreach ($file in $Files) {
    # Combined check moved or removed
    if ($file.FullName -like "*\registry\index.json*") { continue } # Skip index itself if it was scanned

    
    $content = Get-Content $file.FullName
    $newContent = @()
    $modified = $false
    
    foreach ($line in $content) {
        $newLine = $line
        
        # Regex to find: package_name = { ... path = "..." ... }
        # Simplified: look for package names known in index followed by path
        
        foreach ($pkgName in $IndexInfos.Keys) {
            # Pattern: matches "pkg_name = { ... path =" or "pkg_name = { path ="
            # We want to replace the path value.
            
            if ($line -match "$pkgName\s*=\s*\{.*path\s*=\s*`"([^`"]+)`"") {
                $oldPath = $matches[1]
                
                # New Path Calculation
                $registryCratePath = Join-Path $RegistryBase $IndexInfos[$pkgName]
                # We need relative path from $file.DirectoryName TO $registryCratePath
                
                # Using .NET relative path calculation
                $relPath = Get-RelativePath -Path ($file.DirectoryName + "\") -To $registryCratePath
                # Rust requires forward slashes
                $relPath = $relPath.Replace('\', '/')
                
                # Replace in line
                $newLine = $line -replace "path\s*=\s*`"[^`"]+`"", "path = `"$relPath`""
                $modified = $true
                Write-Host "Updating $pkgName in $($file.Name)" -ForegroundColor Gray
                break # processed this line
            }
        }
        $newContent += $newLine
    }
    
    if ($modified) {
        $newContent | Set-Content $file.FullName
        Write-Host "Updated $($file.FullName)" -ForegroundColor Green
    }
}
