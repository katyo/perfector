{ pkgs ? import <nixpkgs> {}, ... }:
with pkgs;

let
  #linker = "lld";
  linker = "mold";

in mkShell {
  buildInputs = [
    cmake
    opencascade-occt
    cargo
    (pkgs.${linker})
  ];

  LD_LIBRARY_PATH = lib.makeLibraryPath [
    stdenv.cc.cc
    opencascade-occt
  ];

  RUSTFLAGS = "-C link-arg=-fuse-ld=${linker}";
}
