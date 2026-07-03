# inventory_walk.ps1 - Phase 0: File inventory walker
param(
    [string]$RootDir = ".",
    [string]$OutputManifest = "inventory/manifest.json",
    [string]$OutputReport = "inventory/INVENTORY_REPORT.md"
)

$ErrorActionPreference = "Stop"

# Ensure output directory exists
$outDir = Split-Path $OutputManifest -Parent
if (-not (Test-Path $outDir)) {
    New-Item -ItemType Directory -Force -Path $outDir | Out-Null
}

# Classification function
function Get-FileClassification {
    param([string]$RelativePath, [string]$Extension, [string]$ParentDir)

    # Extension-based classification
    $ext = $Extension.ToLower()
    
    if ($ext -eq ".fu") {
        if ($RelativePath -match "\\tests\\|\\test_|\\fixtures\\") {
            return "test-fixture"
        }
        if ($RelativePath -match "\\examples\\converted\\") {
            return "converted-rust-masquerade"
        }
        return "source-fusion"
    }
    if ($ext -eq ".rs") {
        if ($RelativePath -match "\\target\\") {
            return "build-artifact"
        }
        return "source-rust"
    }
    if ($ext -in @(".c", ".h", ".cpp", ".hpp")) {
        return "source-c"
    }
    if ($ext -in @(".md", ".mdx")) {
        if ($RelativePath -match "\\AI Training\\|\\docs\\launch\\|\\docs\\roadmap\\|\\docs\\plans\\") {
            return "aspirational-doc"
        }
        return "documentation"
    }
    if ($ext -in @(".o", ".obj", ".lib", ".a", ".so", ".dll", ".exe", ".dylib", ".wasm")) {
        if ($RelativePath -match "^bin\\") {
            return "binary"
        }
        return "build-artifact"
    }
    if ($ext -in @(".json", ".toml", ".yaml", ".yml", ".cfg", ".lock")) {
        return "configuration"
    }
    if ($ext -in @(".log", ".pdb", ".ilk", ".exp")) {
        return "build-artifact"
    }
    if ($ext -in @(".png", ".jpg", ".jpeg", ".svg", ".ico", ".gif", ".bmp", ".webp")) {
        return "binary"
    }
    if ($ext -in @(".vsix", ".zip", ".tar", ".gz", ".tgz", ".7z", ".rar")) {
        return "build-artifact"
    }
    if ($ext -in @(".ps1", ".sh", ".bat", ".py", ".pl")) {
        return "configuration"
    }
    if ($ext -in @(".dot", ".csv")) {
        return "build-artifact"
    }
    if ($ext -in @(".txt", ".out", ".exit", ".ll", ".s")) {
        if ($RelativePath -match "\\tests\\|\\fixtures\\") {
            return "test-fixture"
        }
        return "build-artifact"
    }
    if ($RelativePath -match "\\registry\\crates\\") {
        return "registry-crate"
    }
    if ($RelativePath -match "\\target\\") {
        return "build-artifact"
    }
    return "unknown"
}

# Walk the directory (skip build artifact dirs for speed)
Write-Host "[inventory] Walking workspace at $RootDir..."
$allFiles = Get-ChildItem -Path $RootDir -Recurse -File -Force | Where-Object {
    $full = $_.FullName
    $full -notlike "*\.git\*" -and
    $full -notlike "*\target\*" -and
    $full -notlike "*\target_fuc\*" -and
    $full -notlike "*\target_fuc2\*" -and
    $full -notlike "*\target_fuc_native\*" -and
    $full -notlike "*\target_fusion_cli\*" -and
    $full -notlike "*\node_modules\*" -and
    $full -notlike "*\build\*" -and
    $full -notlike "*\cmake_build\*" -and
    $full -notlike "*\dist\*" -and
    $full -notlike "*\vcpkg\*" -and
    $full -notlike "*\source_archives\*" -and
    $full -notlike "*\artifacts\*" -and
    $full -notlike "*\New folder\*" -and
    $full -notlike "*\.qodo\*" -and
    $full -notlike "*\clang+llvm-20.1.0*"
}

$totalFiles = $allFiles.Count
Write-Host "[inventory] Found $totalFiles files (excluding .git)"

# Build manifest entries
$manifestEntries = @()
$processed = 0
$totalBytes = 0
$classificationCounts = @{}
$duplicates = @{}   # key: "stem|size" -> list of paths
$largest = @()
$oldest = @()

foreach ($file in $allFiles) {
    $processed++
    if ($processed % 1000 -eq 0) {
        Write-Host "[inventory] Processed $processed / $totalFiles files..."
    }

    $relativePath = $file.FullName.Substring($RootDir.Length).TrimStart('\', '/')
    $size = $file.Length
    $lastModified = $file.LastWriteTimeUtc.ToString("o")
    $parentDir = (Split-Path $relativePath -Parent) -replace '\\', '/'
    $extension = $file.Extension
    $classification = Get-FileClassification -RelativePath $relativePath -Extension $extension -ParentDir $parentDir

    $totalBytes += $size

    # Track classification counts
    if (-not $classificationCounts.ContainsKey($classification)) {
        $classificationCounts[$classification] = 0
    }
    $classificationCounts[$classification]++

    # SHA256 skipped for performance (77K+ files)

    # Duplicate detection (by stem + size)
    $stem = $file.BaseName
    $dupKey = "$stem|$size"
    if (-not $duplicates.ContainsKey($dupKey)) {
        $duplicates[$dupKey] = @()
    }
    $duplicates[$dupKey] += $relativePath

    $entry = @{
        path = $relativePath
        size_bytes = $size
        last_modified_utc = $lastModified
        classification = $classification
        parent_directory = $parentDir
    }

    $manifestEntries += $entry

    # Track largest
    $largest += @{ path = $relativePath; size = $size }
    # Track oldest
    $oldest += @{ path = $relativePath; lastModified = $file.LastWriteTimeUtc }
}

Write-Host "[inventory] Writing manifest.json..."
$manifest = @{
    total_files = $totalFiles
    total_bytes = $totalBytes
    generated_utc = (Get-Date).ToUniversalTime().ToString("o")
    workspace_root = (Resolve-Path $RootDir).Path
    files = $manifestEntries
}
$manifest | ConvertTo-Json -Depth 3 | Out-File -FilePath $OutputManifest -Encoding UTF8

# Find duplicates (2+ files with same stem and size)
$dupGroups = $duplicates.GetEnumerator() | Where-Object { $_.Value.Count -ge 2 }

# Find largest files
$largestSorted = $largest | Sort-Object -Property size -Descending | Select-Object -First 20

# Find oldest files
$oldestSorted = $oldest | Sort-Object -Property lastModified | Select-Object -First 20

# Build report
Write-Host "[inventory] Writing INVENTORY_REPORT.md..."
$report = @"
# Fusion v2.0 Vortex - Phase 0 Inventory Report

**Generated**: $((Get-Date).ToUniversalTime().ToString("yyyy-MM-dd HH:mm:ss UTC"))
**Workspace**: $(Resolve-Path $RootDir)

---

## Summary

| Metric | Value |
|--------|-------|
| Total files | $totalFiles |
| Total bytes | $($totalBytes.ToString("N0")) ($([math]::Round($totalBytes / 1MB, 2)) MB) |
| Categories with entries | $($classificationCounts.Count) |

---

## Files per Classification

| Classification | Count | Percentage |
|----------------|-------|------------|
"@

$categories = @("source-fusion", "source-rust", "source-c", "build-artifact", "test-fixture", 
    "documentation", "configuration", "binary", "registry-crate", "converted-rust-masquerade", 
    "aspirational-doc", "unknown")

foreach ($cat in $categories) {
    $count = if ($classificationCounts.ContainsKey($cat)) { $classificationCounts[$cat] } else { 0 }
    $pct = if ($totalFiles -gt 0) { [math]::Round($count / $totalFiles * 100, 1) } else { 0 }
    $report += "| $cat | $count | $pct% |`n"
}

$report += @"

---

## Top 20 Largest Files

| Rank | Path | Size (bytes) |
|------|------|-------------|
"@
$rank = 1
foreach ($f in $largestSorted) {
    $report += "| $rank | $($f.path) | $($f.size.ToString("N0")) |`n"
    $rank++
}

$report += @"

---

## Top 20 Oldest Files

| Rank | Path | Last Modified (UTC) |
|------|------|---------------------|
"@
$rank = 1
foreach ($f in $oldestSorted) {
    $report += "| $rank | $($f.path) | $($f.lastModified.ToString("yyyy-MM-dd HH:mm:ss")) |`n"
    $rank++
}

$report += @"

---

## Duplicate Files (same stem + size)

| Stem | Size | Count | Paths |
|------|------|-------|-------|
"@

$dupCount = 0
foreach ($dup in $dupGroups) {
    if ($dupCount -ge 50) { break }
    $stem = $dup.Key.Split('|')[0]
    $sz = $dup.Key.Split('|')[1]
    $count = $dup.Value.Count
    $paths = ($dup.Value | Select-Object -First 5) -join ", "
    if ($dup.Value.Count -gt 5) { $paths += " ... (+$($dup.Value.Count - 5) more)" }
    $report += "| $stem | $sz | $count | $paths |`n"
    $dupCount++
}

if ($dupGroups.Count -eq 0) {
    $report += "| *(none)* | | | |`n"
}

$report += @"

---

## Category Details

*(Categories with zero entries are explicitly noted as empty.)*

"@

foreach ($cat in $categories) {
    $count = if ($classificationCounts.ContainsKey($cat)) { $classificationCounts[$cat] } else { 0 }
    if ($count -eq 0) {
        $report += "- **$cat**: EMPTY (0 files)`n"
    } else {
        $report += "- **$cat**: $count files`n"
    }
}

# Write report
$report | Out-File -FilePath $OutputReport -Encoding UTF8

Write-Host "[inventory] Complete!"
Write-Host "  Manifest: $OutputManifest ($($manifestEntries.Count) entries)"
Write-Host "  Report:   $OutputReport"
Write-Host "  Total files: $totalFiles"
Write-Host "  Total bytes: $($totalBytes.ToString("N0"))"
Write-Host "  Biggest category: $(($classificationCounts.GetEnumerator() | Sort-Object -Property Value -Descending | Select-Object -First 1).Name)"
Write-Host "  Duplicate groups: $($dupGroups.Count)"