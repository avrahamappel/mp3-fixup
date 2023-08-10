{
  description = "A script using ffmpeg to add metadata to mp3/m4a files";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            ffmpeg
            rust-bin.stable.latest.default
            rust-analyzer
          ];

          buildInputs = [ ];
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "projectname"; # Same that is in Cargo.toml

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      }
    );
}
