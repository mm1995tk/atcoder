{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    treefmt-nix,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };

      treefmtEval = treefmt-nix.lib.evalModule pkgs ./nix/treefmt.nix;

      rustBins = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      rustPlatform = pkgs.makeRustPlatform {
        rustc = rustBins;
        cargo = rustBins;
      };
    in {
      packages = {};

      devShells.default = pkgs.mkShell {
        inputsFrom = [];
        buildInputs = with pkgs;
          [rustBins]
          ++ (
            if pkgs.stdenv.isDarwin
            then [pkgs.darwin.apple_sdk.frameworks.SystemConfiguration]
            else []
          );
      };

      formatter = treefmtEval.config.build.wrapper;
      checks.formatting = treefmtEval.config.build.check self;
    });
}
