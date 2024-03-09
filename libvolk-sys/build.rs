fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    if cfg!(target_os = "windows") {
        let volkdir = std::path::PathBuf::from(std::env::var("VOLK_LIB_DIR").expect("VOLK_LIB_DIR must be set to a valid path"));

        if !out_dir.join("volk.dll").exists() {
            std::fs::copy(volkdir.join("volk.dll"), out_dir.join("volk.dll")).unwrap();
        }
        if !out_dir.join("volk.lib").exists() {
            std::fs::copy(volkdir.join("volk.lib"), out_dir.join("volk.lib")).unwrap();
        }
        println!("cargo:rustc-link-lib=volk");
        println!("cargo:rustc-link-search={}", out_dir.display());
    } else {
        println!("cargo:rustc-link-lib=volk");
    }
}
