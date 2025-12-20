// Build script for Flux-Resolve v2.0
// Handles optional CUDA kernel compilation

#[cfg(feature = "gpu")]
use std::env;
#[cfg(feature = "gpu")]
use std::path::PathBuf;
#[cfg(feature = "gpu")]
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/gpu/kernel.cu");

    // Only compile CUDA kernel if GPU feature is enabled
    #[cfg(feature = "gpu")]
    {
        compile_cuda_kernel();
    }
}

#[cfg(feature = "gpu")]
fn compile_cuda_kernel() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let kernel_src = PathBuf::from("src/gpu/kernel.cu");
    let ptx_output = out_dir.join("flux_kernel_v2.ptx");

    // Check if nvcc is available
    let nvcc_check = Command::new("nvcc").arg("--version").output();

    if nvcc_check.is_err() {
        println!("cargo:warning=NVCC not found. GPU acceleration will be disabled at runtime.");
        println!("cargo:warning=To enable GPU features, install CUDA Toolkit 11.0+");
        return;
    }

    // Compile CUDA kernel to PTX
    let status = Command::new("nvcc")
        .args(&[
            "--ptx",
            "-arch=sm_60", // Minimum compute capability 6.0 (Pascal+)
            "-O3",
            kernel_src.to_str().unwrap(),
            "-o",
            ptx_output.to_str().unwrap(),
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("cargo:rustc-env=KERNEL_PTX_PATH={}", ptx_output.display());
            println!("cargo:warning=Successfully compiled CUDA kernel to PTX");
        }
        _ => {
            println!("cargo:warning=Failed to compile CUDA kernel. GPU acceleration disabled.");
            println!("cargo:warning=Ensure CUDA Toolkit is properly installed");
        }
    }
}
