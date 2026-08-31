{
  pkgs ? import <nixpkgs> { },
}:

let
  cargoToml = fromTOML (builtins.readFile ./Cargo.toml);
in
pkgs.rustPlatform.buildRustPackage {
  pname = cargoToml.package.name;
  version = cargoToml.package.version;

  src = ./.;

  cargoHash = "sha256-Pf7aOdabBtukwx9TcEsYWPY4xEGGTHxafENg7nKZVts=";
}
