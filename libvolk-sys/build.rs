fn main() {
    if cfg!(target_os = "windows") {
        let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("lib");
        std::fs::create_dir_all(&out_dir).unwrap();
        let volkdir = std::path::PathBuf::from(std::env::var("VOLK_LIB_DIR").expect("VOLK_LIB_DIR must be set to a valid path"));

        if !out_dir.join("volk.dll").exists() {
            if volkdir.join("volk.dll").exists() {
                std::fs::copy(volkdir.join("volk.dll"), out_dir.join("volk.dll")).unwrap();
            } else {
                std::fs::copy(volkdir.join("libvolk.dll"), out_dir.join("volk.dll")).unwrap();
            }
        }
        if !out_dir.join("volk.lib").exists() {
            if volkdir.join("volk.lib").exists() {
                std::fs::copy(volkdir.join("volk.lib"), out_dir.join("volk.lib")).unwrap();
            } else {
                std::fs::copy(volkdir.join("libvolk.lib"), out_dir.join("volk.lib")).unwrap();
            }
        }
        println!("cargo:rustc-link-search={}", out_dir.display());
        println!("cargo:rustc-link-lib=volk");
    } else {
        println!("cargo:rustc-link-lib=volk");
    }
}
