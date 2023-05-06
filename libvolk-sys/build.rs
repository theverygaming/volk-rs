fn main() {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-search={}", "./");
        println!(
            "cargo:rustc-link-search={}",
            std::path::PathBuf::from(
                std::env::var("VOLK_LIB_DIR").expect("VOLK_LIB_DIR must be set to a valid path")
            )
            .display()
        );
        println!("cargo:rustc-link-lib=volk");
    } else {
        println!("cargo:rustc-link-lib=volk");
    }
}
