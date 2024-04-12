{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
  ];
  buildInputs = with pkgs; [
    rustc
    rust-analyzer
    rustfmt
    curl
  ] ++ lib.optional stdenv.isDarwin libiconv;
}
