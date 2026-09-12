{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # <https://github.com/nix-systems/nix-systems>
    systems.url = "github:nix-systems/default-linux";
  };

  outputs =
    {
      nixpkgs,
      treefmt-nix,
      systems,
      ...
    }:
    let
      eachSystem = nixpkgs.lib.genAttrs (import systems);
      pkgs = eachSystem (system: import nixpkgs { inherit system; });

      tge = eachSystem (system: pkgs.${system}.callPackage ./tge.nix { });
    in
    {
      packages = eachSystem (system: {
        default = tge.${system};
        tge = tge.${system};
      });

      devShells = eachSystem (system: {
        default = pkgs.${system}.mkShell {
          name = "tge-dev-shell";

          packages = with pkgs.${system}; [
            git
            rustup
            gettext
            (import ./tools/nix/treefmt.nix {
              inherit treefmt-nix;
              pkgs = pkgs.${system};
            })
          ];

          shellHook = ''
            export ROOT_DIR=$(git rev-parse --show-toplevel)
            export PATH="$PATH:$ROOT_DIR/tools/bin"

            git config commit.template commit-template
          '';
        };
      });
    };
}
