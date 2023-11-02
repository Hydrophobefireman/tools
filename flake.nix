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
        maxMindKey = builtins.readFile ./maxmind_license.txt;
        MAXMIND_BASE_URL="https://download.maxmind.com/app/geoip_download?license_key=${maxMindKey}";
        buildInputs = with pkgs; [ rust-bin.stable.latest.default flyctl nodejs ];
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
          });
        dockerImage = pkgs.dockerTools.buildImage {
          name = "tools-backend";
          tag = "latest";
          copyToRoot = [bin];
          runAsRoot = ''
              mkdir /GeoLite2/
              cd /GeoLite2/
              URL="${MAXMIND_BASE_URL}"
              URL="$URL&edition_id=GeoLite2-ASN&suffix=tar.gz"
              wget "$URL" -O GeoLite2-ASN.tar.gz

              URL="${MAXMIND_BASE_URL}"
              URL="$URL&edition_id=GeoLite2-ASN&suffix=tar.gz.sha256"
              wget "$URL" -O GeoLite2-ASN.tar.gz.sha256

              URL="${MAXMIND_BASE_URL}"
              URL="$URL&edition_id=GeoLite2-City&suffix=tar.gz"
              wget "$URL" -O GeoLite2-City.tar.gz

              URL="${MAXMIND_BASE_URL}"
              URL="$URL&edition_id=GeoLite2-City&suffix=tar.gz.sha256"
              wget "$URL" -O GeoLite2-City.tar.gz.sha256

              sed 's/GeoLite2-ASN_[0-9]*.tar.gz/GeoLite2-ASN.tar.gz/g' -i GeoLite2-ASN.tar.gz.sha256
              sha256sum -c GeoLite2-ASN.tar.gz.sha256
              tar xvf GeoLite2-ASN.tar.gz --strip 1

              sed 's/GeoLite2-City_[0-9]*.tar.gz/GeoLite2-City.tar.gz/g' -i GeoLite2-City.tar.gz.sha256
              sha256sum -c GeoLite2-City.tar.gz.sha256
              tar xvf GeoLite2-City.tar.gz --strip 1
              mkdir -p /data/GeoIP
              mv /GeoLite2/*.mmdb /data/GeoIP
              '';
          config = {
            Cmd = [ "${bin}/bin/tool-api" ];
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
            buildInputs = [pkgs.dive pkgs.git-crypt];
          };
        }
      );
}