{
  description = "UTPM is a package manager for local and remote Typst packages.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";

    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
  };

  outputs = { self, ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
        };

        craneLib = inputs.crane.mkLib pkgs;
        src = ./.;

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;
          buildInputs = [ pkgs.openssl ];
          nativeBuildInputs = [ pkgs.pkg-config  ];
          OPENSSL_NO_VENDOR = 1;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          inputsFrom = [ self.packages.${pkgs.system}.default ];
          packages = [
            #lsp
            pkgs.rust-analyzer
          ];
        };

        packages = {
          utpm = self.packages.${pkgs.system}.default;

          default = craneLib.buildPackage {
            buildInputs = [ pkgs.openssl ];
            nativeBuildInputs = [ pkgs.pkg-config  ];
            OPENSSL_NO_VENDOR = 1;
            inherit cargoArtifacts src;
            meta = {
              homepage = "https://github.com/Thumuss/utpm";
              licence = pkgs.stdenv.lib.licences.MIT;
              description = "UTPM is a package manager for local and remote Typst packages.";
              longDescription = "UTPM is a package manager for local and remote Typst packages. Quickly create and manage projects and templates on your system, and publish them directly to Typst Universe.";
              mainProgram = "utpm";
            };
          };
        };
      }
    );
}
