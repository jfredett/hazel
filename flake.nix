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
            in {
                devshells.default = {
                    motd = ''Double double, toil and trouble.'';

                    packages = with pkgs; [
                        bacon
                        cargo-llvm-cov
                        cargo-mutants
                        cargo-nextest
                        clang
                        cloc
                        gnuplot
                        imhex
                        just
                        libcxx
                        linuxKernel.packages.linux_6_6.perf
                        mold
                        perf-tools
                        plantuml
                        timg
                        rustpkg
                        stockfish
                    ];

                };
            };
        };
}
