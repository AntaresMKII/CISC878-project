let
  pkgs = import <nixpkgs> {};
in
pkgs.callPackage (
  {
    mkShell,
    cargo,
    rustc,
  }:
  mkShell {
    strictDeps = true;
    nativeBuildInputs = [
      cargo
      rustc
    ];
  }
) { }
