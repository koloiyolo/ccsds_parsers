{
  pkgs ? import <nixpkgs> { },
}:

let
  packageToml = fromTOML (builtins.readFile ./ccsds_parsers/Cargo.toml);
  workspaceToml = fromTOML (builtins.readFile ./Cargo.toml);
in
pkgs.rustPlatform.buildRustPackage {
  pname = packageToml.package.name;
  version = workspaceToml.workspace.package.version;

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
