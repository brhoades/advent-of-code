{
  description = "advent-of-code flake with shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }: let
    pkgsFor = system: import nixpkgs {
      inherit system;
    }; in (flake-utils.lib.eachDefaultSystem (system: with (pkgsFor system); {
      # envrc
      devShells.default = mkShell {
        buildInputs = [
          # ghc
          rustup
          go
          godef
        ];
        GOROOT = "${go}/share/go";
      };
    }));
}
