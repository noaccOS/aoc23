{
  description = "Advent of Code 2023";
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = rust-overlay.packages.${system}.rust-nightly.override {
          extensions = [ "rust-src" "rust-analyzer-preview" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
      in
      {
        packages.default = craneLib.buildPackage {
          src = pkgs.lib.cleanSourceWith {
            src = craneLib.path ./.;
            filter =
              path: type:
              (builtins.match ".*/(input|sample_a|sample_b)$" path != null)
              || (craneLib.filterCargoSources path type);
          };
          strictDeps = true;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.default ];
          RUST_SRC_PATH = "${toolchain}";
        };

        formatter = pkgs.nixpkgs-fmt;
      });
}
