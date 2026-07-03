@echo off
setlocal enabledelayedexpansion

:: Simple Windows build script for CEMQC Core (MSVC or MinGW)
:: Usage: build_windows.bat [msvc|mingw]

set BUILD=%1
if "%BUILD%"=="" set BUILD=msvc

if "%BUILD%"=="msvc" (
    echo Building with Microsoft Visual C++ (MSVC)...
    if not exist build-msvc mkdir build-msvc
    cd build-msvc
    cmake -G "NMake Makefiles" -DCMAKE_BUILD_TYPE=Release -DCEMQC_SELFTEST=ON ..
    if errorlevel 1 exit /b 1
    nmake
    cd ..
    echo.
    echo Build complete! Run: build-msvc\cemqc_selftest_runner.exe
    exit /b 0
)

if "%BUILD%"=="mingw" (
    echo Building with MinGW-w64 GCC...
    if not exist build-mingw mkdir build-mingw
    cd build-mingw
    cmake -G "MinGW Makefiles" -DCMAKE_BUILD_TYPE=Release -DCEMQC_SELFTEST=ON ..
    if errorlevel 1 exit /b 1
    mingw32-make -j
    cd ..
    echo.
    echo Build complete! Run: build-mingw\cemqc_selftest_runner.exe
    exit /b 0
)

echo Unknown option. Usage: build_windows.bat [msvc|mingw]
exit /b 1