{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    rust-analyzer
    clippy
    hyperfine
  ];

  shellHook = ''
    # make rustc behave like nightly (allow unstable features/flags)
    export RUSTC_BOOTSTRAP=1
  '';
}
