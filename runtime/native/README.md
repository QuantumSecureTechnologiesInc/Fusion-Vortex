# Fusion Native Runtime

Cross-platform C runtime library providing FFI functions for the Fusion standard library.

## Build

### Windows (MSVC)
```powershell
mkdir build
cd build
cmake .. -G "Visual Studio 17 2022" -A x64
cmake --build . --config Release
cmake --install . --prefix ../../../dist/runtime
```

### Windows (MinGW)
```bash
mkdir build && cd build
cmake .. -G "MinGW Makefiles" -DCMAKE_BUILD_TYPE=Release
cmake --build .
cmake --install . --prefix ../../../dist/runtime
```

### Linux/macOS
```bash
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make
sudo make install  # or: cmake --install . --prefix ../../../dist/runtime
```

## Usage

Link your Fusion-compiled objects with `libfusionrt.a` (or `fusionrt.lib` on Windows):

```bash
# Linux/macOS
gcc -o myapp myapp.o -L./dist/runtime/lib -lfusionrt

# Windows MSVC
link /OUT:myapp.exe myapp.obj dist\runtime\lib\fusionrt.lib

# Windows MinGW
gcc -o myapp.exe myapp.o -Ldist/runtime/lib -lfusionrt
```

## String Pool

All `const char*` returns are allocated from a 16MB internal string pool. Strings remain valid until `fusion_rt_shutdown()` is called. Do not `free()` returned strings.

## Status

✅ **Implemented**:
- I/O (read_line)
- File system (read, write, append, exists, mkdir, remove)
- Path manipulation (join, basename, dirname)
- Environment variables
- Time functions
- Simple RNG
- Hash/HMAC functions
- JSON/formatting helpers

🚧 **Stubs** (TODO):
- TCP/UDP networking
- Mutex synchronization

## API Reference

See `fusionrt.h` for complete API documentation.
