use std::{env, path::PathBuf};

fn main() {
    let cuda_path: PathBuf = env::var_os("CUDA_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/usr/local/cuda"));

    let cuda_include: PathBuf = cuda_path.join("include");

    cc::Build::new()
        .file("csrc/nvml_wrapper.c")
        .include("csrc")
        .include(cuda_include)
        .flag_if_supported("-std=c17")
        .warnings(true)
        .compile("rook_nvml_wrapper");

    // NVMLへリンク
    println!("cargo::rustc-link-lib=dylib=nvidia-ml");

    println!("cargo::rerun-if-changed=csrc/nvml_wrapper.c");
    println!("cargo::rerun-if-changed=csrc/nvml_wrapper.h");
}
