{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        fenix.url = "github:nix-community/fenix";
        devshell.url = "github:numtide/devshell";
        flake-parts.url = "github:hercules-ci/flake-parts";

        rust-manifest = {
            url = "https://static.rust-lang.org/dist/2025-02-12/channel-rust-nightly.toml";
            flake = false;
        };
    };

    outputs = { self, nixpkgs, fenix, rust-manifest, devshell, flake-parts, ... } @ inputs:
        flake-parts.lib.mkFlake { inherit inputs; } {
            imports = [
                devshell.flakeModule
            ];

            systems = [
                "x86_64-linux"
            ];

            perSystem = { pkgs, system, ... }: let
                rustpkg = (fenix.packages.${system}.fromManifestFile rust-manifest).completeToolchain;
                fastchess = pkgs.callPackage ./nix/fastchess.nix { };
                deps = with pkgs; rec {
                    dev = [
                        bacon
                        cloc
                        gnuplot
                        imhex
                        linuxKernel.packages.linux_6_6.perf
                        perf-tools
                        plantuml
                        timg
                    ] ++ ci;
                    ci = [
                        cargo-insta
                        cargo-llvm-cov
                        cargo-mutants
                        cargo-nextest
                        cargo-udeps
                        clang
                        fastchess
                        just
                        libcxx
                        mold
                        rustpkg
                        stockfish
                    ];
                };
            in {
                packages = {
                    inherit fastchess;

                    ci = pkgs.writeShellApplication {
                        name = "ci";
                        runtimeInputs = deps.ci;
                        text = /* bash */ ''
                            just ci
                        '';
                    };


                };
                devshells.default = {
                    motd = ''Double double, toil and trouble.'';
                    packages = deps.dev;
                };
            };
        };
}
