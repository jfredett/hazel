{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    devenv.url = "github:cachix/devenv";
  };

  outputs = { self, nixpkgs, devenv, fenix, ... } @ inputs:
  let
    systems = [ "x86_64-linux" "i686-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
    forAllSystems = f: builtins.listToAttrs (map (name: { inherit name; value = f name; }) systems);
  in
  {
    devShells = forAllSystems
    (system: let
      pkgs = import nixpkgs { inherit system; };
    in {
      default = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [{
          languages.rust.enable = true;
          languages.rust.channel = "nightly";

          packages = with pkgs; [ 
            gnuplot 
            perf-tools
            linuxKernel.packages.linux_6_6.perf
          ];
        }];
      };
    });
  };
}
