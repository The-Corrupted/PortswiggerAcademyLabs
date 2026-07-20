{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      flake-utils,
      fenix,
      nixpkgs,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        toolchainBuild = fenix.packages.${system}.minimal.toolchain;
        toolchainDev =
          with fenix.packages.${system}.complete;
          withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustfmt"
          ];

      in
      {
        packages = rec {
          mfa-force =
            (pkgs.makeRustPlatform {
              cargo = toolchainBuild;
              rustc = toolchainBuild;
            }).buildRustPackage
              {
                pname = "mfa_force";
                version = "0.2.0";

                src = self;
                cargoLock = {
                  lockFile = ./Cargo.lock;
                };

                nativeBuildInputs = with pkgs; [
                  pkg-config
                  autoPatchelfHook
                ];
                buildInputs = with pkgs; [
                  openssl
                  libgcc
                ];

                meta = with pkgs.lib; {
                  description = "mfa-force project";
                  license = licenses.mit;
                  maintainers = [ "The-Corrupted" ];
                };

                postInstall = ''
                  mv $out/bin/protonctl-rs $out/bin/protonctl
                '';
              };

          default = mfa-force;
        };

        devShells.default = pkgs.mkShell {

          buildInputs = with pkgs; [
            openssl
            pkg-config
          ];
          packages = [ toolchainDev ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.openssl
            pkgs.libgcc
          ];

          shellHook = ''
            echo "Welcome to the mfa-force dev shell"
          '';
        };
      }
    );
}
