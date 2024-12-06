{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    rust-analyzer
    clippy
    hyperfine
    cargo-flamegraph
  ];

  shellHook = ''
    # make rustc behave like nightly (allow unstable features/flags)
    export RUSTC_BOOTSTRAP=1
  '';
}
