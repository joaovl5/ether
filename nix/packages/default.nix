{pkgs, ...}: let
  src-path = ../../.;
  project-name = cargo-toml.package.name;
  cargo-toml =
    builtins.readFile "${src-path}/Cargo.toml"
    |> builtins.fromTOML;
in rec {
  default = pkgs.rustPlatform.buildRustPackage {
    pname = project-name;
    version = cargo-toml.package.version;
    src = src-path;
    cargoLock.lockFile = "${src-path}/Cargo.lock";
  };
  ${project-name} = default;

  rust-toolchain =
    pkgs.rust-bin.fromRustupToolchainFile
    "${src-path}/rust-toolchain.toml";
}
