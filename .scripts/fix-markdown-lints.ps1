#!/usr/bin/env pwsh
# Markdown Lint Fixer Script
# Systematically fixes common markdown linting issues

param(
    [Parameter(Position=0)]
    [string]$Path = ".",
    
    [switch]$DryRun
)

function Fix-MarkdownFile {
    param([string]$FilePath)
    
    Write-Host "Processing: $FilePath" -ForegroundColor Cyan
    
    $content = Get-Content -Path $FilePath -Raw
    $originalContent = $content
    
    # Fix MD032: Lists should be surrounded by blank lines
    # Add blank line before list if not preceded by blank line or start of file
    $content =-replace '(?<!\r\n\r\n)(\r\n)(\s*[-*+]\s|\s*\d+\.\s)', "$1$1$2"
    
    # Add blank line after list if not followed by blank line
    $content = $content -replace '(\r\n\s*[-*+]\s[^\r\n]+)(\r\n)(?!\r\n|\s*[-*+]\s|\s*\d+\.\s)', "$1$2$2"
    $content = $content -replace '(\r\n\s*\d+\.\s[^\r\n]+)(\r\n)(?!\r\n|\s*\d+\.\s)', "$1$2$2"
    
    # Fix MD031: Fenced code blocks should be surrounded by blank lines
    $content = $content -replace '(?<!\r\n\r\n)(\r\n)(```)', "$1$1$2"
    $content = $content -replace '(```\r\n)(?!\r\n)', "$1$1"
    
    # Fix MD040: Add language to unlabeled code blocks
    $content = $content -replace '(\r\n```)(\r\n)(?![a-z])', "$1text$2"
    
    # Fix MD009: Remove trailing spaces (but preserve intentional double spaces for line breaks)
    $content = $content -replace ' +(\r\n)', "$1"
    
    # Fix MD022: Headings should be surrounded by blank lines
    $content = $content -replace '(?<!\r\n\r\n)(\r\n)(#{1,6}\s)', "$1$1$2"
    $content = $content -replace '(#{1,6}\s[^\r\n]+)(\r\n)(?!\r\n)', "$1$2$2"
   
    # Fix MD030: List marker space (only one space after marker)
    $content = $content -replace '(\r\n\s*[-*+])\s{2,}', "$1 "
    $content = $content -replace '(\r\n\s*\d+\.)\s{2,}', "$1 "
    
    if ($content -ne $originalContent) {
        if ($DryRun) {
            Write-Host "  Would fix linting issues" -ForegroundColor Yellow
        } else {
            Set-Content -Path $FilePath -Value $content -NoNewline
            Write-Host "  Fixed linting issues" -ForegroundColor Green
        }
        return $true
    } else {
        Write-Host "  No changes needed" -ForegroundColor Gray
        return $false
    }
}

# Get all markdown files
$markdownFiles = Get-ChildItem -Path $Path -Filter "*.md" -Recurse | 
    Where-Object { 
        $_.FullName -notlike "*node_modules*" -and 
        $_.FullName -notlike "*.gemini*"
    }

Write-Host "Found $($markdownFiles.Count) markdown files" -ForegroundColor Magenta
Write-Host ""

$fixedCount = 0
foreach ($file in $markdownFiles) {
    if (Fix-MarkdownFile -FilePath $file.FullName) {
        $fixedCount++
    }
}

Write-Host ""
Write-Host "Summary:" -ForegroundColor Magenta
Write-Host "  Total files: $($markdownFiles.Count)" -ForegroundColor White
Write-Host "  Files fixed: $fixedCount" -ForegroundColor Green
if ($DryRun) {
    Write-Host "  (DRY RUN - no changes made)" -ForegroundColor Yellow
}
