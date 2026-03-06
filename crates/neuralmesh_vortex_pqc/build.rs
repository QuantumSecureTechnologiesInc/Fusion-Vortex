//! Build script for neuralmesh_vortex_pqc
//!
//! Compiles the HyperCycle Vortex C library and links it to the Rust crate.

use std::env;
use std::path::PathBuf;

fn main() {
    // Get the manifest directory (where Cargo.toml is)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    
    // Path to the Vortex v2.0 C library source
    let vortex_dir = manifest_dir.join("../../v2.0 Vortex");
    
    // Tell Cargo to rerun if the C sources change
    println!("cargo:rerun-if-changed={}", vortex_dir.display());
    
    // Source files for the Vortex library (minimal set for Weave-SIG and Weave-KEM)
    let vortex_sources = vec![
        "src/vortex/weave_sig.c",
        "src/vortex/weave_kem.c",
        "src/vortex/hc_quaternion.c",
        "src/vortex/cemqc.c",
        "src/vortex/sha3.c",
        "src/vortex/system_entropy.c",
        "src/vortex/shim_rand.c",
        "src/vortex/hc_vacuum.c",
        "src/vortex/hc_vacuum_jitter.c",
        "src/vortex/hc_cpu_features.c",
        "src/vortex/hc_constant_time.c",
        "src/vortex/hc_health_tests.c",
        "src/vortex/hc_core.c",
        "src/vortex/hc_octonion.c",
        "src/vortex/fixed_point.c",
        "src/vortex/hc_sbox16.c",
        "src/vortex/hc_secure_memory.c",
        "src/hc_vacuum_engine.c",
        "src/vortex_integration.c",
        "src/vortex_pqc_api.c",
        "src/hc_gpu_universal.c",
    ];
    
    let mut build = cc::Build::new();
    
    // Set C standard
    build.std("c11");
    
    // Include directories
    build.include(vortex_dir.join("include"));
    build.include(vortex_dir.join("include/vortex"));
    build.include(vortex_dir.join("include/vortex/public"));
    build.include(vortex_dir.join("include/vortex/internal"));
    build.include(vortex_dir.join("include/ed25519"));
    
    // Compiler flags for AVX-512 (x86_64 only)
    if cfg!(target_arch = "x86_64") {
        if cfg!(target_env = "msvc") {
            build.flag("/arch:AVX512");
        } else {
            build.flag("-mavx512f");
            build.flag("-mavx512dq");
            build.flag("-mavx512ifma");
            build.flag("-mfma");
        }
    }
    
    // Platform-specific flags
    if cfg!(target_os = "linux") {
        build.flag("-pthread");
        build.flag("-O3");
    } else if cfg!(target_os = "windows") {
        build.flag("-pthread");
        build.flag("/O2");
    }
    
    // Add source files that exist
    for src in &vortex_sources {
        let src_path = vortex_dir.join(src);
        if src_path.exists() {
            build.file(&src_path);
            println!("cargo:rerun-if-changed={}", src_path.display());
        } else {
            println!("cargo:warning=Source file not found: {}", src_path.display());
        }
    }
    
    // Compile the library
    build.compile("hypercycle_vortex");
    
    // Link pthread on Unix
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=m");
    } else if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=bcrypt");
    }
    
    // Output the include path for downstream crates
    println!("cargo:include={}", vortex_dir.join("include").display());
}
