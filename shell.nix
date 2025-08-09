{ pkgs ? import <nixpkgs> { } }:

let
  libs = with pkgs; [
    libxkbcommon
    libGL
  ] ++ (with xorg; [
    libX11
    libXcursor
    libxcb
    libXi
  ]);
in pkgs.mkShell {
  name = "rust";

  buildInputs = libs ++ (with pkgs; [
    cargo
    rustc
    gcc
    rustfmt
    python3
  ]);

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  RUST_BACKTRACE = "full";
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libs;
}
