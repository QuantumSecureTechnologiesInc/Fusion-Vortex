#!/usr/bin/env pwsh
# Advanced Markdown Lint Fixer
# Handles more nuanced markdown linting issues

param(
    [Parameter(Position = 0)]
    [string]$Path = ".",
    
    [switch]$DryRun
)

function Repair-AdvancedMarkdownIssues {
    param([string]$FilePath)
    
    Write-Host "Processing: $FilePath" -ForegroundColor Cyan
    
    $content = Get-Content -Path $FilePath -Raw
    $originalContent = $content
    
    # Fix MD026: Remove trailing punctuation from headings (except ?)
    # Remove ! from headings
    $content = $content -replace '(#{1,6}\s[^!\r\n]+)!(\s*\r\n)', "$1$2"
    # Remove : from headings  
    $content = $content -replace '(#{1,6}\s[^:\r\n]+):(\s*\r\n)', "$1$2"
    
    # Fix MD036: Emphasis used as heading - convert to proper heading or remove emphasis
    # This is context-dependent, so we'll handle common patterns
    # Convert **Text** at end of section to <!-- Comment -->
    $content = $content -replace '(\r\n\r\n)\*\*([^*]+)\*\*(\s*\r\n\r\n|\s*$)', "$1<!-- $2 -->$3"
    
    # Fix MD034: Bare URLs - wrap in angle brackets or markdown links
    # Match http(s):// URLs not already in markdown link format
    $content = $content -replace '(?<![(\[])https?://[^\s)\]]+(?![)\]])', '<$0>'
    
    # Fix MD042: Empty links - remove or add URL
    $content = $content -replace '\[([^\]]+)\]\(\)', '`$1`'
    
    # Fix MD029: Ordered list prefix - ensure consistent numbering (1. 2. 3. style)
    # This is complex and requires parsing, so we'll skip automatic fixing
    
    # Fix MD024: Duplicate headings - this requires manual review
    # We'll just note it in output
    
    if ($content -ne $originalContent) {
        if ($DryRun) {
            Write-Host "  Would fix advanced issues" -ForegroundColor Yellow
        }
        else {
            Set-Content -Path $FilePath -Value $content -NoNewline
            Write-Host "  Fixed advanced issues" -ForegroundColor Green
        }
        return $true
    }
    else {
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
    if (Repair-AdvancedMarkdownIssues -FilePath $file.FullName) {
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
