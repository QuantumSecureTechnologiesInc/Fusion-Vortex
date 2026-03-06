# Fusion Native Compiler Wrapper (PowerShell)
# Bootstraps and runs the fusion compiler.

$ErrorActionPreference = "Stop"

# Auto-detect LLVM (Backend) - Prioritize 20
if (Test-Path "C:\Program Files\LLVM") {
    # Assume this is the user's intended 20.1.8 install
    if (-not $env:LLVM_SYS_201_PREFIX) {
        Write-Host "[jfuc] Auto-Config: Found LLVM 20 at C:\Program Files\LLVM"
        $env:LLVM_SYS_201_PREFIX = "C:\Program Files\LLVM"
        $env:PATH = "C:\Program Files\LLVM\bin;" + $env:PATH
    }
}
elseif (Test-Path "C:\LLVM\20.1.0-msvc") {
    if (-not $env:LLVM_SYS_201_PREFIX) {
        Write-Host "[jfuc] Auto-Config: Found LLVM 20 at C:\LLVM\20.1.0-msvc"
        $env:LLVM_SYS_201_PREFIX = "C:\LLVM\20.1.0-msvc"
        $env:PATH = "C:\LLVM\20.1.0-msvc\bin;" + $env:PATH
    }
}
elseif (Test-Path "C:\LLVM\18.1.8-msvc") {
    # Fallback to checking if this is actually 18 (unlikely given user context, but safe)
    # We warn if config mismatches, but for now we look for 18 specific logic
}

# Bootstrap (Python Build)
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
python "$ScriptDir\..\tools\build_fuc_from_fu.py"
if ($LASTEXITCODE -ne 0) {
    Write-Error "[jfuc] Bootstrap failed. Exit Code: $LASTEXITCODE"
    exit $LASTEXITCODE
}

# Execute
$FucPath = "$ScriptDir\..\target\release\fuc.exe"
if (Test-Path $FucPath) {
    & $FucPath $args
} else {
    Write-Error "Error: fuc.exe not found usually after build at $FucPath"
    exit 1
}
