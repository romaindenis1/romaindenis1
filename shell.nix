{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [ stdenv.cc curl pkgconfig gnumake ];

  shellHook = ''
    echo "Entered nix shell with gcc (stdenv.cc), make, pkg-config and libcurl available."
    echo "Run: make  # to build the project"
  '';
}