{ pkgs ? import <nixpkgs> { } }:

let
  inherit (pkgs) fetchzip stdenv lib patchelf fetchFromGitHub python38;

  dart-sass = stdenv.mkDerivation rec {
    pname = "dart-sass";
    version = "1.26.8";
    src = fetchzip {
      sha256 = "0c3mj0imd2r8hdakqzwhdds95dm1kazb8p1lva9rrj63a2zrp9lj";
      url =
        "https://github.com/sass/${pname}/releases/download/${version}/${pname}-${version}-linux-x64.tar.gz";
    };

    installPhase = ''
      mkdir -p $out/bin/
      echo "#! /bin/bash" >> $out/bin/sass
      echo "exec" \
         "\"$out/share/dart-sass/dart\"" \
         "\"$out/share/dart-sass/sass.snapshot\"" \
         "\"\$@\"" \
         >> $out/bin/sass
      chmod +x $out/bin/sass

      ln -s sass $out/bin/dart-sass

      mkdir -p $out/share/dart-sass
      cp src/* $out/share/dart-sass

      ${patchelf}/bin/patchelf \
        --set-interpreter "$(cat $NIX_CC/nix-support/dynamic-linker)" \
        $out/share/dart-sass/dart
    '';
  };

  sfz = stdenv.mkDerivation rec {
    pname = "sfz";
    version = "0.1.0";

    inherit (stdenv.targetPlatform) isDarwin;

    src = fetchzip {
      sha256 = if isDarwin then
        "0ndmyibdzyab50l1rczskxyf73ar6c84wg60namspcmzd9xr823q"
      else
        "1z26ls2jq6mg8xphy6kkxnlc6qzg8iqq4nxhgrd8181hiqs99ii1";
      url =
        let platform = if isDarwin then "apple-darwin" else "unknown-linux-gnu";
        in "https://github.com/weihanglo/${pname}/releases/download/${version}/${pname}-${version}-x86_64-${platform}.tar.gz";
    };

    installPhase = ''
      mkdir -p $out/bin/
      cp sfz $out/bin/
      if [[ -z "$isDarwin" ]]
      then
        ${patchelf}/bin/patchelf \
          --set-interpreter "$(cat $NIX_CC/nix-support/dynamic-linker)" \
          $out/bin/sfz
      fi
    '';
  };

  reveal-js = stdenv.mkDerivation rec {
    pname = "reveal.js";
    version = "4.0.2";

    src = fetchFromGitHub {
      owner = "hakimel";
      repo = "reveal.js";
      rev = "15815efe05ca69c35ce66cfdbf93316e1db66ecb";
      sha256 = "1g3h710rhpyq4vnh6rgyay2dyjpw4rw99p062yhwhgrjkgjyzrc2";
    };

    # nodeModules = ./node_modules-reveal.js;

    # buildInputs = [ pkgs.nodejs_latest ];

    installPhase = ''
      # cp -r $nodeModules node_modules
      # chmod -R +w node_modules
      # HOME=$PWD npm install
      mkdir -p $out
      cp -r . $out/
    '';
  };

  py = python38.withPackages (pkgs: with pkgs; [ jedi ]);

in stdenv.mkDerivation {
  pname = "rust-for-non-systems-programmers";
  version = "1.0.0";
  src = if lib.inNixShell then null else ./.;
  buildInputs = [ sfz dart-sass py ];
}
