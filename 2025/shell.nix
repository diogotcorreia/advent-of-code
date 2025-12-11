{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    rust-analyzer
    clippy
    hyperfine
    cargo-flamegraph

    z3
    rustPlatform.bindgenHook
  ];

  shellHook = ''
    # make rustc behave like nightly (allow unstable features/flags)
    export RUSTC_BOOTSTRAP=1

    export Z3_SYS_Z3_HEADER="${pkgs.z3.dev}/include/z3.h"
  '';
}
