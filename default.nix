{ pkgs ? import <nixpkgs> {} }:
pkgs.rustPlatform.buildRustPackage {
  pname = "set-cosmic-theme";
  version = "0.1.0";
  doCheck = false;
  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "atomicwrites-0.4.2" = "sha256-QZSuGPrJXh+svMeFWqAXoqZQxLq/WfIiamqvjJNVhxA=";
      "clipboard_macos-0.1.0" = "sha256-tovB4fjPVVRY8LKn5albMzskFQ+1W5ul4jT5RXx9gKE=";
      "cosmic-config-0.1.0" = "sha256-QsDox6AHUNQ0VM4XHF6qxH9QNVK7Fc1SNywW0gRqb9I=";
      "smithay-clipboard-0.8.0" = "sha256-4InFXm0ahrqFrtNLeqIuE3yeOpxKZJZx+Bc0yQDtv34=";
    };
  };
  src = pkgs.lib.cleanSource ./.;
}
