{ pkgs ? import (fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/77ef7a29d276c6d8303aece3444d61118ef71ac2.tar.gz";
    sha256 = "0pm4l48jq8plzrrrisimahxqlcpx7qqq9c99hylmf7p3zlc3phsy";
  }) {},
}:

let
  nix-ld-libs = pkgs.buildEnv {
    name = "nix-ld-libs";
    paths = with pkgs; [
      stdenv.cc.cc.lib
      zlib
      openssl
    ];
  };

in pkgs.mkShell {
  packages = with pkgs; [
    mise
    pkg-config
    openssl.dev
    (writeShellScriptBin "fish" ''
      exec ${pkgs.fish}/bin/fish -C 'mise activate fish | source' "$@"
    '')
  ];
}