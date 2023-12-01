{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { fenix, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system;
        overlays = [fenix.overlays.default];};
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            (pkgs.fenix.complete.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            # rust-analyzer-nightly
            rust-analyzer
          ];
        };
      }
    );
}
