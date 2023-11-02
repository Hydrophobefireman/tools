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
    flake-utils.lib.eachSystem [ "x86_64-linux" ]
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };

        # maxmind
          maxMindKey = builtins.readFile ./maxmind_license.txt;
          maxMindBaseUrl ="https://download.maxmind.com/app/geoip_download?license_key=${maxMindKey}";
          
          geolite_asn = fetchTarball { url = "${maxMindBaseUrl}&edition_id=GeoLite2-ASN&suffix=tar.gz"; sha256 = "0raph0bsy5lpsg762fdxazkp1by1n5ap6s5jv5slyk6nqb9jqc2h"; };
          geolite_asn_city = fetchTarball { url = "${maxMindBaseUrl}&edition_id=GeoLite2-City&suffix=tar.gz"; sha256 = "0hi9lc8wnq3f5kwqxw9llnxrm9lh7s9a6hah41107yr66qpdivsd";  };

    
          nativeBuildInputs = [pkgs.glibc];
          buildInputs = with pkgs; [ rust-bin.stable.latest.default  ];
          craneLib = crane.lib.${system};
          src = craneLib.cleanCargoSource (craneLib.path ./backend);
          commonArgs = {
              inherit src buildInputs;
            };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          bin = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
            cargoLock = ./backend/Cargo.lock;
            cargoToml = ./backend/Cargo.toml;
            CARGO_BUILD_TARGET = "x86_64-unknown-linux-gnu";
            # RUSTFLAGS="-C target-feature=+crt-static";
            });
          dockerImage = pkgs.dockerTools.buildImage {
            name = "tools-backend";
            tag = "latest";
            copyToRoot = with pkgs.dockerTools; [
                  bin 
                  geolite_asn 
                  geolite_asn_city
                  binSh
                  usrBinEnv
                  # pkgs.coreutils
                  # caCertificates
                  # fakeNss
                  # pkgs.wget
            ];
            
            runAsRoot = ''
                #!${pkgs.runtimeShell}
                mkdir -p /data/GeoIP
                mv *.mmdb /data/GeoIP
                '';
            config = {
              Cmd = [ "/bin/tool-api" ];
            };
        };
        in with pkgs;
        {
          packages  = {
              inherit bin dockerImage;
              default = bin;
          };
          devShells.default = mkShell {
            inputsFrom = [bin];
            buildInputs = with pkgs; [dive git-crypt flyctl just];
          };
        }
      );
}