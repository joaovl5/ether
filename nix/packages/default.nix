{pkgs, ...}: let
  src-path = ../../.;
  cargo-toml =
    src-path + "/crates/ether-cli/Cargo.toml"
    |> builtins.readFile
    |> builtins.fromTOML;
  project-name = cargo-toml.package.name;
  src = pkgs.lib.fileset.toSource {
    root = src-path;
    fileset = pkgs.lib.fileset.gitTracked src-path;
  };
in rec {
  default = pkgs.rustPlatform.buildRustPackage {
    pname = project-name;
    version = cargo-toml.package.version;
    inherit src;
    cargoLock.lockFile = src-path + "/Cargo.lock";
  };
  ${project-name} = default;

  rust-toolchain =
    pkgs.rust-bin.fromRustupToolchainFile (src-path + "/rust-toolchain.toml");
}
