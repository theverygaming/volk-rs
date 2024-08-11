with import (fetchTarball https://github.com/NixOS/nixpkgs/archive/5e0ca22929f3342b19569b21b2f3462f053e497b.tar.gz) { };
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
