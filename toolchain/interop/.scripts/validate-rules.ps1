# Validation Script - Rules Compliance Checker
# Run this after every AI delivery to enforce coding standards

param(
    [string]$ProjectPath = ".",
    [switch]$Verbose
)

$ErrorActionPreference = "Continue"
$errors = @()
$warnings = @()

Write-Host "`n🔍 Fusion Visual Compiler - Rules Compliance Validation" -ForegroundColor Cyan
Write-Host "=" * 60 -ForegroundColor Cyan

# Rule 2: Zero Placeholders (STRICTEST)
Write-Host "`n[Rule 2] Checking for placeholders..." -ForegroundColor Yellow
$placeholderPatterns = @("TODO", "FIXME", "placeholder", "stub", "mock", "unimplemented!", "todo!()")
foreach ($pattern in $placeholderPatterns) {
    $matches = rg -i $pattern $ProjectPath --type rust --type typescript 2>$null
    if ($matches) {
        $errors += "RULE 2 VIOLATION: Found '$pattern' in code`n$matches"
    }
}

# Check for fake APIs
$fakeAPIs = rg "NeuralParser::load|template!.*\{" "$ProjectPath/cmd/fusion-visual-pure" 2>$null
if ($fakeAPIs) {
    # Verify these have implementations
    $neuralParserExists = Test-Path "$ProjectPath/cmd/fusion-visual-pure/src/neural_parser.fsn"
    $templateMacroExists = Test-Path "$ProjectPath/cmd/fusion-visual-pure/src/template_macro.fsn"
    
    if (-not $neuralParserExists) {
        $errors += "RULE 2 VIOLATION: NeuralParser used but not implemented"
    }
    if (-not $templateMacroExists) {
        $errors += "RULE 2 VIOLATION: template! macro used but not implemented"
    }
}

# Rule 5: Lint-Free Code
Write-Host "[Rule 5] Running linters..." -ForegroundColor Yellow
Push-Location $ProjectPath

# Cargo clippy
$clippyOutput = cargo clippy --all-targets --all-features -- -D warnings 2>&1
if ($LASTEXITCODE -ne 0) {
    $errors += "RULE 5 VIOLATION: Clippy found issues`n$clippyOutput"
}

# Cargo fmt check
$fmtOutput = cargo fmt -- --check 2>&1
if ($LASTEXITCODE -ne 0) {
    $warnings += "Code formatting issues found. Run 'cargo fmt' to fix."
}

Pop-Location

# Rule 6: 7-Step Response Format
Write-Host "[Rule 6] Checking response format..." -ForegroundColor Yellow
# This is checked manually by user

# Rule 8: Folder Structure
Write-Host "[Rule 8] Validating folder structure..." -ForegroundColor Yellow
$requiredFiles = @(
    "README.md",
    "QuickStartGuide.md",
    "ChangeLog.md"
)

$requiredDirs = @(
    "docs/guides",
    "docs/test",
    "docs/security",
    "artifacts"
)

foreach ($file in $requiredFiles) {
    if (-not (Test-Path "$ProjectPath/$file")) {
        $errors += "RULE 8 VIOLATION: Missing required file: $file"
    }
}

foreach ($dir in $requiredDirs) {
    if (-not (Test-Path "$ProjectPath/$dir")) {
        $errors += "RULE 8 VIOLATION: Missing required directory: $dir"
    }
}

# Check Diátaxis documentation
$docsIndex = Test-Path "$ProjectPath/docs/DocumentIndex.md"
if (-not $docsIndex) {
    $warnings += "Missing docs/DocumentIndex.md (recommended)"
}

# Rule 3: Narrative Code Excellence
Write-Host "[Rule 3] Checking code narrative..." -ForegroundColor Yellow
$filesWithoutComments = rg -l "^fn\s+\w+.*\{$" --type rust $ProjectPath 2>$null | Select-Object -First 5
if ($filesWithoutComments) {
    $warnings += "Some functions lack narrative comments (Rule 3)"
}

# Rule 4: Complexity Check
Write-Host "[Rule 4] Checking cyclomatic complexity..." -ForegroundColor Yellow
# Note: Requires cargo-complexity
if (Get-Command cargo-complexity -ErrorAction SilentlyContinue) {
    $complexityOutput = cargo complexity --all 2>&1 | Select-String "complexity.*[1-9][0-9]"
    if ($complexityOutput) {
        $errors += "RULE 4 VIOLATION: Functions with complexity > 10 found`n$complexityOutput"
    }
}
else {
    $warnings += "cargo-complexity not installed. Run: cargo install cargo-complexity"
}

# Generate Report
Write-Host "`n" + ("=" * 60) -ForegroundColor Cyan
Write-Host "COMPLIANCE REPORT" -ForegroundColor Cyan
Write-Host ("=" * 60) -ForegroundColor Cyan

if ($errors.Count -eq 0 -and $warnings.Count -eq 0) {
    Write-Host "`n✅ ALL RULES PASSED - Code is compliant!" -ForegroundColor Green
    Write-Host "`nCompliance Status:" -ForegroundColor Green
    Write-Host "  ✅ Rule 2: Zero placeholders" -ForegroundColor Green
    Write-Host "  ✅ Rule 3: Narrative code" -ForegroundColor Green
    Write-Host "  ✅ Rule 4: Complexity < 10" -ForegroundColor Green
    Write-Host "  ✅ Rule 5: Lint-free" -ForegroundColor Green
    Write-Host "  ✅ Rule 8: Complete structure" -ForegroundColor Green
    exit 0
}

if ($errors.Count -gt 0) {
    Write-Host "`n❌ CRITICAL VIOLATIONS FOUND ($($errors.Count)):" -ForegroundColor Red
    $errors | ForEach-Object { 
        Write-Host "`n$_" -ForegroundColor Red 
    }
}

if ($warnings.Count -gt 0) {
    Write-Host "`n⚠️  WARNINGS ($($warnings.Count)):" -ForegroundColor Yellow
    $warnings | ForEach-Object { 
        Write-Host "  - $_" -ForegroundColor Yellow 
    }
}

if ($errors.Count -gt 0) {
    Write-Host "`n❌ VALIDATION FAILED - Fix violations before proceeding" -ForegroundColor Red
    exit 1
}
else {
    Write-Host "`n⚠️  VALIDATION PASSED WITH WARNINGS" -ForegroundColor Yellow
    exit 0
}
