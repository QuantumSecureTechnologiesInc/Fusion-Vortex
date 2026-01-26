# Visual Studio 2026 + CUDA Configuration Guide

## System Configuration

**Visual Studio:** 2026 (v18) - Community Edition  
**MSVC Version:** 14.50.35717  
**CUDA Toolkit:** v13.1  
**CMake:** 4.2+  
**Windows SDK:** 10.0.26100.0

---

## Installation Paths

```
Visual Studio 2026:
C:\Program Files\Microsoft Visual Studio\18\Community

MSVC Compiler (x64):
C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\bin\Hostx64\x64\cl.exe

CUDA Toolkit:
C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.1

NVCC Compiler:
C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.1\bin\nvcc.exe

OpenSSL (if installed):
C:\Program Files\OpenSSL-Win64
```

---

## Environment Setup

### Option 1: Command Line Build

```batch
@echo off
REM Activate VS 2026 x64 environment
call "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Auxiliary\Build\vcvars64.bat"

REM Now CUDA can find the correct compiler
nvcc --version
cl

REM Compile CUDA code
nvcc your_file.cu -o output.exe
```

### Option 2: CMake with Visual Studio 2026

**CMakeLists.txt configuration:**

```cmake
cmake_minimum_required(VERSION 3.18)
project(YourProject LANGUAGES CXX CUDA)

# Specify Visual Studio 2026 toolset
set(CMAKE_GENERATOR_TOOLSET "v145")

# CUDA Configuration
set(CMAKE_CUDA_ARCHITECTURES "75;86;89")  # RTX 3000/4000 series
set(CMAKE_CUDA_STANDARD 17)
set(CMAKE_CXX_STANDARD 17)

# Your targets here
add_executable(your_target main.cu)
```

**Build commands:**

```powershell
# Configure
cmake -S . -B build -G "Visual Studio 18 2026" -T v145

# Build
cmake --build build --config Release
```

---

## VS Code IntelliSense Configuration

**File:** `.vscode/c_cpp_properties.json`

```json
{
    "configurations": [
        {
            "name": "Win32-VS2026-CUDA",
            "includePath": [
                "${workspaceFolder}/**",
                "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v13.1/include",
                "C:/Program Files/OpenSSL-Win64/include"
            ],
            "defines": [
                "_DEBUG",
                "UNICODE",
                "_UNICODE",
                "__CUDACC__",
                "CUDA_VERSION=13010"
            ],
            "windowsSdkVersion": "10.0.26100.0",
            "compilerPath": "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v13.1/bin/nvcc.exe",
            "cStandard": "c17",
            "cppStandard": "c++17",
            "intelliSenseMode": "windows-msvc-x64"
        }
    ],
    "version": 4
}
```

---

## Troubleshooting

### Issue: NVCC Can't Find Visual Studio

**Symptom:** `nvcc fatal error: Cannot find compiler 'cl.exe'`

**Solution:**
```batch
REM Always run vcvars64.bat first
call "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Auxiliary\Build\vcvars64.bat"
```

### Issue: Wrong VS Version Detected

**Solution:** Use explicit CMake generator:
```powershell
cmake -G "Visual Studio 18 2026" -T v145 -S . -B build
```

### Issue: IDE Shows CUDA Errors

**Solution:** 
1. Ensure `.vscode/c_cpp_properties.json` exists with correct paths
2. Reload VS Code window: `Ctrl+Shift+P` → "Developer: Reload Window"

---

## Quick Reference

### Build HyperCycle Projects

```powershell
# Navigate to project
cd "C:\Users\Matth\.gemini\antigravity\scratch\HyperCycle\v1.1 Origin"

# Option A: Direct CUDA compilation
cmd /c '"C:\Program Files\Microsoft Visual Studio\18\Community\VC\Auxiliary\Build\vcvars64.bat" && nvcc cuda_test.cu -o cuda_test.exe'

# Option B: CMake build
cmake -S . -B build -G "Visual Studio 18 2026" -T v145
cmake --build build --config Release

# Option C: Use provided batch file
.\build_cuda.bat
```

### Verify Setup

```powershell
# Check CUDA
nvcc --version

# Check MSVC (after vcvars64.bat)
cl

# Check CMake
cmake --version
```

---

## CUDA Compute Capabilities

| GPU Series | Compute Capability | CMake Flag |
| ---------- | ------------------ | ---------- |
| RTX 30xx   | 8.6                | 86         |
| RTX 40xx   | 8.9                | 89         |
| GTX 16xx   | 7.5                | 75         |

**Set in CMakeLists.txt:**
```cmake
set(CMAKE_CUDA_ARCHITECTURES "75;86;89")
```

---

## Notes

- **Always** use VS 2026 (folder `18`, not `2026`)
- MSVC toolset version: **v145** (not v143 or v142)
- CUDA 13.1 is compatible with VS 2026
- For IntelliSense, reload window after changing `c_cpp_properties.json`

---

**Last Updated:** 2026-01-14  
**VS Version:** 2026 (18.0.0)  
**CUDA Version:** 13.1  
**MSVC Toolset:** v145 (14.50.35717)
