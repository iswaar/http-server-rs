{
  description = "http-server";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgsBase = nixpkgs.legacyPackages."${system}";
      pkgs = pkgsBase.extend rust-overlay.overlays.default;
      release = pkgs.rustPlatform.buildRustPackage (final: {
        name = "http-server";
        pname = "${final.name}";

        checkType = "release";
        buildType = "release";

        buildNoDefaultFeatures = false;
        buildFeatures = []; # Extra features

        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
          allowBuiltinFetchGit = true;
        };
      });
      debug = release.overrideAttrs {
        buildType = "debug";
        checkType = "debug";
      };
    in {
      packages = rec {
        inherit release debug;
        default = debug;
      };
      devShells.default = with pkgs;
        mkShell {
          buildInputs = [
            (rust-bin.stable.latest.default.override {
              targets = ["x86_64-unknown-linux-gnu"];
            })
            zsh
          ];

          shellHook = ''
            exec "${pkgs.zsh}/bin/zsh"
          '';
        };
    });
}
