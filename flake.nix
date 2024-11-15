{
  description = "A small utility to programmatically set a COSMIC Desktop theme from a file";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs }:
  let
    forAllSystems = nixpkgs.lib.genAttrs [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];
  in {
    packages = forAllSystems (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        default = import ./default.nix { pkgs = pkgs; };
      }
    );
    homeManagerModules.default = import ./home.nix;
  };
}
