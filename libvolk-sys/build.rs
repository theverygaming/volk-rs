fn main() {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=libvolk");
    } else {
        println!("cargo:rustc-link-lib=volk");
    }
}
