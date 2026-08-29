let
  sources = import ./lon.nix;
  pkgs = import sources.nixpkgs { };
in
pkgs.mkShell {
  packages = [
    pkgs.nixfmt
    pkgs.nix-prefetch-git
    pkgs.clippy
    pkgs.rustfmt
    pkgs.rust-analyzer
    pkgs.leptosfmt
    pkgs.trunk
    pkgs.dart-sass
    pkgs.rustc
    pkgs.lld
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
