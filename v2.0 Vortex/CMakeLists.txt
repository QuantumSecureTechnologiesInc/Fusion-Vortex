cmake_minimum_required(VERSION 3.21)
project(HyperCyclePQC VERSION 2.0.2 LANGUAGES C CXX)

# Options to control optional backends
option(HC_ENABLE_CUDA "Enable compilation of the NVIDIA CUDA backend" ON)
option(HC_ENABLE_ROCM "Enable compilation of the AMD ROCm backend" ON)
option(HC_BUILD_TESTS "Build tests" OFF)

# Always build position independent code for shared libraries
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# Set C standard
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Security hardening flags
if(CMAKE_C_COMPILER_ID MATCHES "GNU|Clang")
    add_compile_options(
        -fstack-protector-strong # Stack buffer overflow protection
        -Wformat-security # Format string vulnerability detection
        -fPIE # Position independent executable
    )
    add_compile_definitions(_FORTIFY_SOURCE=2) # Buffer overflow detection
    add_link_options(
        -Wl,-z,relro # Read-only relocations
        -Wl,-z,now # Immediate binding
    )
elseif(MSVC)
    add_compile_options(
        /GS # Buffer security check
        /guard:cf # Control flow guard
        /DYNAMICBASE # ASLR support
    )
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} /std:c11" CACHE STRING "" FORCE)
    add_compile_definitions(
        _CRT_SECURE_NO_WARNINGS
    )
endif()

# Main library: universal loader, CPU backend and vacuum engine (C)
# Build as STATIC library for easier linking with Fusion
add_library(hypercycle_pqc STATIC
    src/hc_gpu_universal.c
    # src/hc_vacuum_engine.c - excluded: requires x86intrin.h (not available on MSVC)
    src/hc_vacuum_portable.c  # Portable fallback for MSVC
    src/vortex_integration.c # Vortex v2.0 Integration
    src/vortex_pqc_api.c # Vortex v2.0 PQC API
    src/hc_ed25519.c
    # Internal Ed25519 implementation (original, not third-party)
    src/ed25519/ed25519_api.c
    src/ed25519/ed25519_core.c
    src/ed25519/ed25519_field.c
    # SHA-3 implementation (needed by hc_vacuum_portable)
    src/vortex/sha3.c
)

# Include directories
target_include_directories(hypercycle_pqc PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
    # Expose vortex headers under include/vortex
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/vortex>
    # Expose ed25519 internal headers
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/ed25519>
)

# Compiler-specific flags for AVX-512 support
if(MSVC)
    target_compile_options(hypercycle_pqc PRIVATE /arch:AVX512)
else()
    target_compile_options(hypercycle_pqc PRIVATE
        -mavx512f -mavx512ifma -mavx512dq -mfma)
endif()

# Link dependencies (OpenSSL is optional - we have internal SHA-3)
find_package(OpenSSL)
if(OpenSSL_FOUND)
    target_link_libraries(hypercycle_pqc PRIVATE OpenSSL::Crypto)
    target_compile_definitions(hypercycle_pqc PRIVATE HAVE_OPENSSL)
    message(STATUS "OpenSSL found - using OpenSSL SHA-3")
else()
    message(STATUS "OpenSSL not found - using internal SHA-3 implementation")
endif()

if(UNIX)
    target_link_libraries(hypercycle_pqc PRIVATE dl m pthread)
elseif(MINGW OR MSYS)
    # MinGW/MSYS on Windows
    target_link_libraries(hypercycle_pqc PRIVATE winpthread)
    target_compile_options(hypercycle_pqc PRIVATE -pthread)
endif()

# Link vortex object files into the main library
if(TARGET hypercycle_vortex_objs)
    target_sources(hypercycle_pqc PRIVATE $<TARGET_OBJECTS:hypercycle_vortex_objs>)
endif()

# Install targets for main library
install(TARGETS hypercycle_pqc
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
    PUBLIC_HEADER DESTINATION include
)

install(DIRECTORY include/ DESTINATION include)

# -----------------------------------------------------------------------------
# Vortex v2.0 Dashboard Tools
# -----------------------------------------------------------------------------

# Terminal Dashboard
add_executable(vortex_dashboard tools/vortex_dashboard.c)
target_link_libraries(vortex_dashboard PRIVATE hypercycle_pqc)
if(OpenSSL_FOUND)
    target_link_libraries(vortex_dashboard PRIVATE OpenSSL::Crypto)
endif()
if(UNIX)
    target_link_libraries(vortex_dashboard PRIVATE m pthread)
elseif(MINGW OR MSYS)
    target_link_libraries(vortex_dashboard PRIVATE winpthread)
endif()

# Secure Dashboard
add_executable(secure_dashboard tools/secure_dashboard.c)
target_link_libraries(secure_dashboard PRIVATE hypercycle_pqc)
if(OpenSSL_FOUND)
    target_link_libraries(secure_dashboard PRIVATE OpenSSL::Crypto)
endif()
if(UNIX)
    target_link_libraries(secure_dashboard PRIVATE m pthread)
elseif(MINGW OR MSYS)
    target_link_libraries(secure_dashboard PRIVATE winpthread)
endif()

# -----------------------------------------------------------------------------
# Existing backend logic...


# Optionally build CUDA backend
if(HC_ENABLE_CUDA)
    enable_language(CUDA)
    if(EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/src/hc_backend_cuda.cu")
        add_library(hc_cuda MODULE src/hc_backend_cuda.cu)
        set_target_properties(hc_cuda PROPERTIES
            OUTPUT_NAME "hc_cuda"
            PREFIX "lib"
            SUFFIX ".so"
        )
        target_include_directories(hc_cuda PRIVATE include)
        target_link_libraries(hc_cuda PRIVATE CUDA::cudart)
        install(TARGETS hc_cuda
            LIBRARY DESTINATION lib
        )
    endif()
endif()

# Optionally build ROCm backend
if(HC_ENABLE_ROCM)
    find_package(HIP QUIET)
    if(HIP_FOUND AND EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/src/hc_backend_rocm.hip.cpp")
        add_library(hc_rocm MODULE src/hc_backend_rocm.hip.cpp)
        set_target_properties(hc_rocm PROPERTIES
            OUTPUT_NAME "hc_rocm"
            PREFIX "lib"
            SUFFIX ".so"
        )
        target_include_directories(hc_rocm PRIVATE include)
        target_link_libraries(hc_rocm PRIVATE HIP::hip_runtime)
        install(TARGETS hc_rocm
            LIBRARY DESTINATION lib
        )
    endif()
endif()

# -----------------------------------------------------------------------------
# Vortex Modules
#
# To unify the current and original codebases, we build all C files under
# src/vortex/ as an object library.  These objects are then linked into
# the main hypercycle_pqc library.
#

file(GLOB_RECURSE VORTEX_SOURCES CONFIGURE_DEPENDS "src/vortex/*.c")
# Exclude files with GCC-only inline asm or MSVC-incompatible complex types
list(FILTER VORTEX_SOURCES EXCLUDE REGEX ".*hc_final\\.c$")
list(FILTER VORTEX_SOURCES EXCLUDE REGEX ".*hc_vacuum_avx512\\.c$")
list(FILTER VORTEX_SOURCES EXCLUDE REGEX ".*hc_vacuum_engine\\.c$")
# Exclude files with MSVC-incompatible C11 atomic usage
list(FILTER VORTEX_SOURCES EXCLUDE REGEX ".*hc_core\\.c$")
list(FILTER VORTEX_SOURCES EXCLUDE REGEX ".*hc_cpu_features\\.c$")
list(FILTER VORTEX_SOURCES EXCLUDE REGEX ".*hc_introspection\\.c$")
list(FILTER VORTEX_SOURCES EXCLUDE REGEX ".*hc_sbox16\\.c$")
list(FILTER VORTEX_SOURCES EXCLUDE REGEX ".*hc_telemetry\\.c$")
list(FILTER VORTEX_SOURCES EXCLUDE REGEX ".*ns_entropy_pool\\.c$")
if(VORTEX_SOURCES)
    add_library(hypercycle_vortex_objs OBJECT ${VORTEX_SOURCES})
    set_target_properties(hypercycle_vortex_objs PROPERTIES C_STANDARD 11 C_STANDARD_REQUIRED ON)
    target_include_directories(hypercycle_vortex_objs PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/include
        ${CMAKE_CURRENT_SOURCE_DIR}/include/vortex
        ${CMAKE_CURRENT_SOURCE_DIR}/include/vortex/internal
        ${CMAKE_CURRENT_SOURCE_DIR}/include/vortex/public
    )

    # Compiler-specific flags
    if(MSVC)
        target_compile_options(hypercycle_vortex_objs PRIVATE /arch:AVX512 /std:c11)
        target_compile_definitions(hypercycle_vortex_objs PRIVATE
            _CRT_SECURE_NO_WARNINGS
            CLOCK_MONOTONIC=1
        )
    else()
        target_compile_options(hypercycle_vortex_objs PRIVATE
            -mavx512f -mavx512ifma -mavx512dq -mfma -O3 -pthread)
    endif()

    # Add pthread for MinGW
    if(MINGW OR MSYS)
        target_compile_options(hypercycle_vortex_objs PRIVATE -pthread)
    endif()
endif()
# Tests could be added here if HC_BUILD_TESTS is ON

# Build test programs if requested
if(HC_BUILD_TESTS)
    enable_testing()
    file(GLOB TEST_SOURCES CONFIGURE_DEPENDS "tests/*.c")
    foreach(test_src ${TEST_SOURCES})
        get_filename_component(test_name ${test_src} NAME_WE)
        add_executable(${test_name} ${test_src})
        target_include_directories(${test_name} PRIVATE
            ${CMAKE_CURRENT_SOURCE_DIR}/include
            ${CMAKE_CURRENT_SOURCE_DIR}/include/vortex
            ${CMAKE_CURRENT_SOURCE_DIR}/include/ed25519
        )
        target_link_libraries(${test_name} hypercycle_pqc)
        add_test(NAME ${test_name} COMMAND ${test_name})
    endforeach()
endif()
