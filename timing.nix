{ pkgs ? import <nixpkgs> { } }:
let
  inherit (pkgs) stdenv lib python38;

  py = python38.withPackages (pypkgs:
    with pypkgs;
    [ beautifulsoup4 html5lib ]
    ++ (lib.optionals lib.inNixShell [ mypy pylint flake8 ptpython jedi ]));

in stdenv.mkDerivation {
  pname = "timing";
  version = "0.0.0";

  src = ./timing.py;
  buildInputs = [ py ];
}
