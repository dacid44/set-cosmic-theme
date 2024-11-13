{
  description = "A small utility to programmatically set a COSMIC Desktop theme from a file";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
  flake-utils.lib.eachDefaultSystem (system:
  let pkgs = nixpkgs.legacyPackages.${system}; in
  {
    packages.default = import ./default.nix { pkgs = pkgs; };
  });
}
