# selfhost_preprocess.ps1 — Fusion self-hosting preprocessor
# Handles: recursive module resolution, pub stripping, mod/use removal,
#          const inlining, extern dedup, hex literal conversion (string-aware)
param(
    [Parameter(Mandatory=$true)][string]$InputPath,
    [Parameter(Mandatory=$true)][string]$OutputPath,
    [bool]$ResolveModules = $false
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path $InputPath)) {
    Write-Error "Input file not found: $InputPath"
    exit 1
}

# === Stage 0: Recursive Module Resolution ===

# Hardcoded dependency order for topological sort.
# Modules earlier in this list appear first in the flattened output.
$dependencyOrder = @(
    "ast",
    "lexer",
    "parser",
    "sema",
    "ir",
    "codegen",
    "wasm_encoder",
    "wasm_types",
    "wasm",
    "optimizer",
    "cli",
    "stage1_parser_api",
    "stage1_sema_api",
    "dummy",
    "chaos_vacuum"
)

# Files to skip (aspirational or orchestrators)
$skipModules = @("pure_fusion_compiler", "llvm", "dwarf", "lib")

# Map module name to resolved file path
$resolvedPaths = @{}

# Recursively discover module files starting from root directory.
function Resolve-ModuleFiles {
    param([string]$RootDir, [string]$ModuleName)
    if ($resolvedPaths.ContainsKey($ModuleName)) { return }
    if ($skipModules -contains $ModuleName) { return }

    $fuPath = Join-Path $RootDir "$ModuleName.fu"
    $modDir = Join-Path $RootDir $ModuleName
    $modFuPath = Join-Path $modDir "mod.fu"

    $foundPath = $null
    if (Test-Path $fuPath) {
        $foundPath = $fuPath
    } elseif (Test-Path $modFuPath) {
        $foundPath = $modFuPath
    }

    if ($foundPath) {
        $resolvedPaths[$ModuleName] = $foundPath
        Write-Host "  [mod] resolved $ModuleName -> $foundPath"

        # Recursively scan this file for its own mod declarations
        $subLines = Get-Content -Path $foundPath
        foreach ($sl in $subLines) {
            $st = $sl.TrimStart()
            if ($st -match '^mod\s+(\w+)\s*;') {
                $subName = $Matches[1]
                # Sub-modules are relative to the parent file's directory
                $subDir = Split-Path $foundPath -Parent
                Resolve-ModuleFiles -RootDir $subDir -ModuleName $subName
            }
        }
    }
}

$allModuleLines = New-Object System.Collections.Generic.List[string]

if ($ResolveModules) {
    $rootDir = Split-Path $InputPath -Parent

    # Scan root file for mod declarations
    $rootLines = Get-Content -Path $InputPath
    foreach ($line in $rootLines) {
        $trimmed = $line.TrimStart()
        if ($trimmed -match '^mod\s+(\w+)\s*;') {
            Resolve-ModuleFiles -RootDir $rootDir -ModuleName $Matches[1]
        }
    }

    # Add non-module content from root file (extern fns, main fn, etc.)
    # We'll collect this for later inclusion
    $rootNonModLines = New-Object System.Collections.Generic.List[string]
    foreach ($line in $rootLines) {
        $trimmed = $line.TrimStart()
        if ($trimmed -notmatch '^(mod\s+\w+\s*;|use\s+\S+\s*;)') {
            $rootNonModLines.Add($line)
        }
    }

    # Concatenate resolved modules in dependency order
    foreach ($modName in $dependencyOrder) {
        if ($resolvedPaths.ContainsKey($modName)) {
            $modPath = $resolvedPaths[$modName]
            $modContent = Get-Content -Path $modPath
            $allModuleLines.Add("// === begin module: $modName ($modPath) ===")
            foreach ($ml in $modContent) {
                $allModuleLines.Add($ml)
            }
            $allModuleLines.Add("// === end module: $modName ===")
        }
    }

    # Append root file's non-module content
    if ($rootNonModLines.Count -gt 0) {
        $allModuleLines.Add("// === begin root: $InputPath ===")
        foreach ($rl in $rootNonModLines) {
            $allModuleLines.Add($rl)
        }
        $allModuleLines.Add("// === end root ===")
    }

    $lines = $allModuleLines.ToArray()
    Write-Host "  [mod] resolved $($resolvedPaths.Count) modules, $($lines.Count) total lines"
} else {
    $lines = Get-Content -Path $InputPath
}
$out = New-Object System.Collections.Generic.List[string]
$seenExterns = New-Object System.Collections.Generic.HashSet[string]
$consts = @{}

# First pass: collect const definitions
foreach ($line in $lines) {
    $trimmed = $line.TrimStart()
    if ($trimmed -match '^const\s+(\w+)\s*:\s*\w+\s*=\s*(.+?)\s*;') {
        $constName = $Matches[1]
        $constValue = $Matches[2].TrimEnd(';').Trim()
        $consts[$constName] = $constValue
    }
}

# Second pass: process lines
foreach ($line in $lines) {
    $trimmed = $line.TrimStart()
    $stripped = $line

    # Stage 1: Strip 'pub ' keyword
    $stripped = $stripped -replace '\bpub\s+', ''

    # Stage 2: Skip mod/use/const declarations entirely
    if ($trimmed -match '^(mod\s+\w+\s*;|use\s+\S+\s*;|const\s+\w+\s*:\s*\w+\s*=)') {
        continue
    }

    # Stage 3: Inline const references (simple word-boundary replacement)
    foreach ($name in $consts.Keys) {
        $stripped = $stripped -replace "\b$name\b", $consts[$name]
    }

    # Stage 4: Deduplicate extern fn declarations
    if ($trimmed -match '^extern\s+fn\s+(\w+)\s*\(') {
        $fnName = $Matches[1]
        if ($seenExterns.Contains($fnName)) {
            continue
        }
        $seenExterns.Add($fnName) | Out-Null
    }

    # Stage 5: Convert hex literals to decimal (bootstrap compiler crashes on 0xNN)
    # Must NOT convert hex inside string literals. Split on quotes:
    # even-indexed segments are outside strings, odd-indexed are inside.
    $parts = $stripped -split '"'
    for ($i = 0; $i -lt $parts.Length; $i++) {
        if ($i % 2 -eq 0) {
            # Outside string: safe to convert hex literals
            $parts[$i] = [regex]::Replace($parts[$i], '\b0x([0-9a-fA-F]+)\b', {
                param($m)
                [Convert]::ToInt32($m.Groups[1].Value, 16).ToString()
            })
        }
    }
    $stripped = $parts -join '"'

    $out.Add($stripped)
}

# Write flattened output
$encoding = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllLines($OutputPath, $out, $encoding)
Write-Host "  [ps-preprocess] $InputPath -> $OutputPath ($($out.Count) lines)"
exit 0