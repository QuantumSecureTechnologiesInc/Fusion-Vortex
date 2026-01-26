@echo off
REM
REM build_msvc.bat - Build Ed25519 library with MSVC
REM
REM Usage:
REM   build_msvc.bat              - Build everything
REM   build_msvc.bat release      - Release build (optimized)
REM   build_msvc.bat debug        - Debug build
REM   build_msvc.bat clean        - Clean build artifacts
REM

setlocal enabledelayedexpansion

set BUILD_TYPE=%1
if "%BUILD_TYPE%"=="" set BUILD_TYPE=release

REM MSVC compiler flags
if "%BUILD_TYPE%"=="release" (
    set CFLAGS=/O2 /W4 /TC /GS /Qspectre
    echo [BUILD] Release build with optimizations
) else if "%BUILD_TYPE%"=="debug" (
    set CFLAGS=/Od /W4 /TC /Z7 /D_DEBUG
    echo [BUILD] Debug build with symbols
) else if "%BUILD_TYPE%"=="clean" (
    echo [CLEAN] Removing build artifacts...
    del *.obj *.exe *.lib *.dll *.pdb 2>nul
    echo [OK] Clean complete
    goto :EOF
) else (
    echo Usage: build_msvc.bat [release^|debug^|clean]
    goto :EOF
)

REM Source files
set SRCS=ed25519_field.c ed25519_sha512.c ed25519_scalar.c ed25519_group.c ed25519_api_complete.c
set TEST_SRC=test_ed25519.c

REM Compile source files
echo.
echo [CC] Compiling cryptographic modules...
cl %CFLAGS% %SRCS%
if errorlevel 1 (
    echo [ERROR] Compilation failed
    goto :EOF
)

REM Create static library
echo.
echo [AR] Creating static library...
lib *.obj /out:ed25519.lib >nul 2>&1
if errorlevel 0 echo [OK] Library created: ed25519.lib

REM Compile test program
echo.
echo [CC] Compiling test program...
cl %CFLAGS% /link advapi32.lib %TEST_SRC% ed25519.lib /out:test_ed25519.exe
if errorlevel 1 (
    echo [ERROR] Test compilation failed
    goto :EOF
)

REM Run tests
echo.
echo [RUN] Running test suite...
test_ed25519.exe
if errorlevel 1 (
    echo [FAIL] Tests failed
    goto :EOF
)

echo.
echo ========================================
echo Build successful!
echo ========================================
echo Artifacts:
echo   - ed25519.lib (static library)
echo   - test_ed25519.exe (test program)
echo.
echo To use in your application:
echo   cl myapp.c ed25519.lib /out:myapp.exe
echo.

endlocal
