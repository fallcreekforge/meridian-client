{
  description = "Meridian Client builds and development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, fenix, ... }:
    let
      ciSystem = "x86_64-linux";
      developmentSystems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
      forAllDevelopmentSystems = nixpkgs.lib.genAttrs developmentSystems;
      workspaceManifest = fromTOML (builtins.readFile ./Cargo.toml);
      version = workspaceManifest.workspace.package.version;

      pkgsFor = system: import nixpkgs { inherit system; };
      rustToolchainFor =
        system:
        fenix.packages.${system}.stable.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
        ];
      rustfmtFor = system: fenix.packages.${system}.complete.withComponents [ "rustfmt" ];
      rustPlatformFor =
        system:
        let
          pkgs = pkgsFor system;
          rustToolchain = rustToolchainFor system;
        in
        pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      rustSource =
        let
          inherit (pkgsFor ciSystem) lib;
        in
        lib.fileset.toSource {
          root = ./.;
          fileset = lib.fileset.unions [
            ./Cargo.lock
            ./Cargo.toml
            (lib.fileset.fileFilter (file: file.hasExt "rs" || file.name == "Cargo.toml") ./crates)
          ];
        };

      meridianClient =
        let
          pkgs = pkgsFor ciSystem;
          rustPlatform = rustPlatformFor ciSystem;
        in
        rustPlatform.buildRustPackage {
          pname = "meridian-client";
          inherit version;

          src = rustSource;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [
            "--workspace"
            "--all-features"
          ];
          cargoTestFlags = [
            "--workspace"
            "--all-features"
          ];
          strictDeps = true;

          meta = {
            description = "Customer-controlled Meridian Client runtime";
            homepage = "https://github.com/fallcreekforge/meridian-client";
            license = pkgs.lib.licenses.asl20;
            mainProgram = "meridian";
          };
        };

      meridianClientWindowsX86_64 =
        let
          crossPkgs = import nixpkgs {
            localSystem = ciSystem;
            crossSystem = nixpkgs.lib.systems.examples.mingwW64;
          };
          target = crossPkgs.stdenv.hostPlatform.rust.rustcTarget;
          rustToolchain =
            with fenix.packages.${ciSystem};
            combine [
              stable.cargo
              stable.rustc
              targets.${target}.stable.rust-std
            ];
          rustPlatform = crossPkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };
        in
        rustPlatform.buildRustPackage {
          pname = "meridian-client-windows";
          inherit version;

          src = rustSource;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [
            "--workspace"
            "--all-features"
          ];
          doCheck = false;
          strictDeps = true;

          meta = {
            description = "Windows x86-64 GNU portability build for Meridian Client";
            homepage = "https://github.com/fallcreekforge/meridian-client";
            license = crossPkgs.lib.licenses.asl20;
            mainProgram = "meridian";
          };
        };

      repositoryQuality =
        let
          pkgs = pkgsFor ciSystem;
          rustPlatform = rustPlatformFor ciSystem;
          rustToolchain = rustToolchainFor ciSystem;
          rustfmt = rustfmtFor ciSystem;
        in
        pkgs.stdenv.mkDerivation {
          pname = "meridian-client-repository-quality";
          inherit version;

          src = ./.;
          cargoDeps = rustPlatform.importCargoLock {
            lockFile = ./Cargo.lock;
          };
          nativeBuildInputs = [
            pkgs.actionlint
            pkgs.just
            pkgs.nixfmt
            pkgs.nushell
            pkgs.statix
            pkgs.taplo
            rustPlatform.cargoSetupHook
            rustToolchain
            rustfmt
          ];
          strictDeps = true;
          dontConfigure = true;

          buildPhase = ''
            runHook preBuild

            export CLIPPY_CONF_DIR="$PWD"
            export RUSTFMT="${rustfmt}/bin/rustfmt"
            export TAPLO_CONFIG="$PWD/taplo.toml"
            just fmt lint nix-fmt nix-lint toml-fmt
            if [ -f .github/workflows/windows.yml ]; then
              just gha-lint
            fi

            runHook postBuild
          '';

          installPhase = ''
            runHook preInstall
            touch "$out"
            runHook postInstall
          '';
        };
    in
    {
      packages.${ciSystem} = {
        default = meridianClient;
        meridian-client = meridianClient;
        meridian-client-windows-x86_64 = meridianClientWindowsX86_64;
      };

      checks.${ciSystem} = {
        meridian-client = meridianClient;
        repository-quality = repositoryQuality;
        meridian-client-windows-x86_64 = meridianClientWindowsX86_64;
      };

      devShells = forAllDevelopmentSystems (
        system:
        let
          pkgs = pkgsFor system;
          rustToolchain = rustToolchainFor system;
          rustfmt = rustfmtFor system;
          clippyConfDir = pkgs.linkFarm "meridian-clippy-configuration" {
            "clippy.toml" = ./clippy.toml;
          };
        in
        {
          default = pkgs.mkShell {
            packages = [
              rustToolchain
              rustfmt
              fenix.packages.${system}.rust-analyzer
              pkgs.actionlint
              pkgs.nushell
              pkgs.git
              pkgs.just
              pkgs.nixfmt
              pkgs.stdenv.cc
              pkgs.statix
              pkgs.cargo-deny
              pkgs.cargo-expand
              pkgs.cargo-fuzz
              pkgs.cargo-nextest
              pkgs.evcxr
              pkgs.taplo
            ];

            CARGO_NET_GIT_FETCH_WITH_CLI = "true";
            CLIPPY_CONF_DIR = "${clippyConfDir}";
            RUSTFMT = "${rustfmt}/bin/rustfmt";
            TAPLO_CONFIG = "${./taplo.toml}";

            shellHook = ''
              # Make an interactive development shell Nushell by default. The
              # TTY check preserves `nix develop -c <command>` for automation.
              if [ -t 0 ] && [ -z "''${NU_VERSION:-}" ]; then
                exec nu
              fi
            '';
          };
        }
      );

      formatter = forAllDevelopmentSystems (system: (pkgsFor system).nixfmt);

      herculesCI.ciSystems = [ ciSystem ];
    };
}
