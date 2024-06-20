{
  description = "Generators for random ids";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, fenix, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        cargoBuildInputs = with pkgs; lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.CoreServices
        ];
      in
      {
        devShell = pkgs.mkShell {
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

          buildInputs = with pkgs; [
            cargo
            rustc
            rustup
            rust-analyzer
            rustfmt
            python312
          ] ++ cargoBuildInputs;
        };

        packages.x86_64-linux.default = fenix.packages.x86_64-linux.minimal.toolchain;

        nixosConfigurations.nixos = nixpkgs.lib.nixosSystem {
          system = "x86_64-linux";
          modules = [
            ({ pkgs, ... }: {
              nixpkgs.overlays = [ fenix.overlays.default ];
              environment.systemPackages = with pkgs; [
                (fenix.complete.withComponents [
                  "cargo"
                  "clippy"
                  "rust-src"
                  "rustc"
                  "rustfmt"
                  "rustup"
                ])
                rust-analyzer-nightly
              ];
            })
          ];
        };
        
      });
}
