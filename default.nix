{ pkgs ? import <nixpkgs> {} }:

pkgs.stdenv.mkDerivation {
  pname = "program";
  version = "1.0";

  src = ./.;

  buildInputs = [ pkgs.curl ];

  buildPhase = ''
    make
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp program $out/bin/
  '';

  meta = with pkgs.stdenv.lib; {
    description = "Small C program that uses libcurl";
    license = licenses.mit;
    maintainers = [];
  };
}