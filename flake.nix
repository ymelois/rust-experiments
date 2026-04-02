{
  description = "rust experiments development environemnt";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };
  };

  outputs =
    {
      nixpkgs,
      fenix,
      ...
    }:
    let
      system = "x86_64-linux";

      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          fenix.overlays.default
        ];
      };

      toolchainFile = pkgs.fenix.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-zC8E38iDVJ1oPIzCqTk/Ujo9+9kx9dXq7wAwPMpkpg0=";
      };

      rustToolchain = pkgs.fenix.combine [
        pkgs.rust-analyzer
        pkgs.fenix.latest.rustfmt
        toolchainFile
      ];
    in
    {
      packages."${system}".default = pkgs.stdenv.mkDerivation {
        name = "main";
        src = ./main.rs;
        dontUnpack = true;
        nativeBuildInputs = [ rustToolchain ];
        buildPhase = ''
          rustc $src -o main \
            --target x86_64-unknown-none \
            -C opt-level=3 \
            -C debuginfo=none \
            -C strip=symbols \
            -C lto=fat \
            -C panic=abort \
            -C codegen-units=1
        '';
        installPhase = ''
          mkdir -p $out/bin
          cp main $out/bin/main
        '';
      };

      devShells."${system}".default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          rustToolchain
          cargo-show-asm
        ];
      };
    };
}
