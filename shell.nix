with import <nixpkgs> { };
stdenv.mkDerivation {
  name = "volk-rs";
  buildInputs = [
    rustc
    cargo
    volk
    rust-bindgen
    rustfmt
  ];
  shellHook = "export VOLK_PATH=${volk}";
}
