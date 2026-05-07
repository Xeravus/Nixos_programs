{
  description = "Monorepo Flake für mehrere Rust Projekt";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    nixpkgs,
    naersk,
  }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    naerskLib = pkgs.callPackage naersk {};
  in {
    devShells."x86_64-linux" = {
      parser = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
          glib
        ];
        nativeBuildInputs = [
          pkgs.pkg-config
        ];
        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
      switcher = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
          glib
        ];
        nativeBuildInputs = [
          pkgs.pkg-config
        ];
        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
      timetracker = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
        ];
        nativeBuildInputs = [
          pkgs.pkg-config
        ];
        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    };
    packages."x86_64-linux" = {
      parser = naerskLib.buildPackage {
        src = ./nix_parser/.;
        buildInputs = [
          pkgs.glib
        ];
        nativeBuildInputs = [
          pkgs.pkg-config
        ];
      };
      switcher = naerskLib.buildPackage {
        src = ./nix_switcher/.;
        buildInputs = [
          pkgs.pkg-config
        ];
        nativeBuildInputs = [
          pkgs.pkg-config
        ];
      };
      timetracker = naerskLib.buildPackage {
        src = ./nix_timetracker/.;
        buildInputs = [
          pkgs.pkg-config
        ];
        nativeBuildInputs = [
          pkgs.pkg-config
        ];
      }
    };
  };
}
