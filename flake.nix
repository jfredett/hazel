{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    devenv.url = "github:cachix/devenv";
    bugstalker.url = "github:godzie44/BugStalker";
  };

  outputs = { self, nixpkgs, bugstalker, devenv, fenix, ... } @ inputs:
    let
      systems = [ "x86_64-linux" "i686-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
      forAllSystems = f: builtins.listToAttrs (map (name: { inherit name; value = f name; }) systems);
    in
      {
      packages.x86_64-linux.devenv-up = self.devShells.x86_64-linux.default.config.procfileScript;
      devShells = forAllSystems
        (system: let
          pkgs = import nixpkgs { inherit system; };
        in {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;


            modules = [{
              languages.rust = {
                enable = true;
                mold.enable = true;
                channel = "nightly";
                components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" "miri" "llvm-tools" ];
                # FIXME: I would love for this to be part of the Cargo.toml, and not the flake.
                rustflags = "--cfg tokio_unstable -Ctarget-feature=+bmi2 -Ctarget-feature=+bmi1";
              };

              languages.ruby = {
                enable = true;
                bundler.enable = true;
              };

              enterShell = ''
                mkdir -p .parsers
                rm .parsers/*
                ln -s "${pkgs.tree-sitter-grammars.tree-sitter-rust}/parser" .parsers/rust.so
                ln -s "${pkgs.tree-sitter-grammars.tree-sitter-rust}/queries" .parsers/rust_queries
              '';


              packages = with pkgs; let 
                ts = tree-sitter.withPlugins (p: [ p.tree-sitter-rust ] );
              in [
                bacon
                cargo-llvm-cov
                cargo-mutants
                cargo-nextest
                cargo-insta
                cloc
                gnuplot
                imhex
                just
                linuxKernel.packages.linux_6_6.perf
                tree-sitter
                tree-sitter-grammars.tree-sitter-rust
                tokio-console
                ts
                mold
                perf-tools
                stockfish
              ];
            }];
          };
        });
    };
}
