{pkgs, ...}: {
  packages = with pkgs; [
    clang
    mold
    bacon
    rust-analyzer

    sea-orm-cli

    just
  ];
}
