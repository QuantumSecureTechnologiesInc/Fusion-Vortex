# Cleanup Redundant Registry Folders
# Now that we have the unified Global Registry at registry/crates

$Root = "C:\Projects\Fusion - Programming Language"

Write-Host "=== Cleaning up redundant registry folders ===" -ForegroundColor Cyan

# Folders to remove
$FoldersToRemove = @(
    "$Root\ecosystem\crates",  # Empty after migration
    "$Root\Source Files\Ecosystem\Fusion Crates",  # Source files used for generation
    "$Root\Source Files\Ecosystem\Fusion Package Registry"  # Old registry structure
)

foreach ($folder in $FoldersToRemove) {
    if (Test-Path $folder) {
        Write-Host "Removing: $folder" -ForegroundColor Yellow
        Remove-Item -Path $folder -Recurse -Force
        Write-Host "  Removed successfully" -ForegroundColor Green
    } else {
        Write-Host "Skipping (not found): $folder" -ForegroundColor Gray
    }
}

Write-Host "`n=== Cleanup Complete ===" -ForegroundColor Green
Write-Host "The unified Global Package Manager Registry is now at:" -ForegroundColor Cyan
Write-Host "  $Root\registry\crates" -ForegroundColor White
