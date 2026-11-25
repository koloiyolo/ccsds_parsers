{
  description = "Rust devshell for minihashcat";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              rust-bin.stable.latest.default
              cargo-tarpaulin
              pre-commit
              cargo-llvm-cov
              cargo-nextest
              llvmPackages_23.llvm
            ];

            shellHook = ''
              export LLVM_COV="${pkgs.llvmPackages_23.llvm}/bin/llvm-cov"
              export LLVM_PROFDATA="${pkgs.llvmPackages_23.llvm}/bin/llvm-profdata"
              pre-commit install
            '';
          };
      }
    );
}
