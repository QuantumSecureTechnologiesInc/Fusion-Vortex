@echo off
set LLVM_ROOT=C:\Program Files\LLVM

:loop
if "%1" == "" goto end

if "%1" == "--version" (
    echo 18.1.8
    goto next
)
if "%1" == "--prefix" (
    echo %LLVM_ROOT%
    goto next
)
if "%1" == "--libdir" (
    echo %LLVM_ROOT%\lib
    goto next
)
if "%1" == "--includedir" (
    echo %LLVM_ROOT%\include
    goto next
)
if "%1" == "--bindir" (
    echo %LLVM_ROOT%\bin
    goto next
)
if "%1" == "--libs" (
    :: Minimal libs Output - simplistic
    echo -L"%LLVM_ROOT%\lib" -lLLVM
    goto next
)
if "%1" == "--ldflags" (
    echo -LIBPATH:"%LLVM_ROOT%\lib"
    goto next
)
if "%1" == "--system-libs" (
    echo.
    goto next
)
if "%1" == "--cxxflags" (
    echo -I"%LLVM_ROOT%\include" -D_DEBUG
    goto next
)
if "%1" == "--link-static" (
    echo true
    goto next
)
if "%1" == "--link-shared" (
    echo false
    goto next
)
if "%1" == "--targets-built" (
    echo X86 AArch64 ARM NVPTX
    goto next
)

:next
shift
goto loop

:end
