{ lib
, rustPlatform
, makeWrapper
, xorg
, libxkbcommon
, libGL
, xclip }:

let
  libs = [
    libxkbcommon
    libGL
  ] ++ (with xorg; [
    libX11
    libXcursor
    libxcb
    libXi
  ]);
in rustPlatform.buildRustPackage rec {
  pname = "dont-repeat-yourself";
  version = "2.0.1";

  src = ./.;

  nativeBuildInputs = [
    makeWrapper
  ];

  buildInputs = [
    xclip
  ] ++ libs;

  postFixup = ''
    wrapProgram $out/bin/${pname} --set LD_LIBRARY_PATH ${lib.makeLibraryPath libs}
  '';

  cargoLock = {
    lockFile = ./Cargo.lock;

    outputHashes = {
      "x11-clipboard-0.9.3" = "sha256-FQEBzs1hl2oXr0qrUmN2C/AmM4bds4+97uXuaO5BvPc=";
    };
  };

  doCheck = false;

  meta = with lib; {
    homepage = "https://github.com/aurakle/${pname}";
    description = "Keyboard-only clipboard manager";
    license = licenses.mit;
  };
}
