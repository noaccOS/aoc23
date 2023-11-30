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
        toolchain = rust-overlay.packages.${system}.rust-nightly;
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
      in
      {
        packages.default = craneLib.buildPackage
          {
            src = craneLib.cleanCargoSource (craneLib.path ./.);
            strictDeps = true;
          };

      });
}
