with import <nixpkgs> { };
stdenv.mkDerivation {
  name = "volk-rs";
  buildInputs = [
    rustc
    cargo
    volk
  ];
}
