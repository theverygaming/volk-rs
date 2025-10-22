{
  description = "volk-rs";

  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    { }
    // flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      rec {
        devShells.default = pkgs.stdenv.mkDerivation {
          name = "volk-rs";
          buildInputs = with pkgs; [
            rustc
            cargo
            volk
            rust-bindgen
            rustfmt
          ];
          shellHook = "export VOLK_PATH=${pkgs.volk}";
        };
      }
    );
}
