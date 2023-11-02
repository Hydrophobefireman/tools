{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    }; 
 };

 outputs = { self, nixpkgs, flake-utils, rust-overlay, crane}:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        buildInputs = with pkgs; [ rust-bin.stable.latest.default flyctl nodejs ];
        craneLib = crane.lib.${system};
        src = craneLib.cleanCargoSource (craneLib.path ./backend);
        commonArgs = {
            inherit src buildInputs;
          };
        # cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        bin = craneLib.buildPackage (commonArgs // {
          # inherit cargoArtifacts;
          cargoLock = ./backend/Cargo.lock;
          cargoToml = ./backend/Cargo.toml;
          postUnpack = ''
            cd $sourceRoot/backend
            sourceRoot="."
          '';
          });
        in with pkgs;
        {
          packages  = {
              inherit bin;
              default = bin;
          };
          devShells.default = mkShell {
            inputsFrom = [bin];
          };
        }
      );
}