let
  pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {};
in pkgs.mkShell {
  buildInputs = [ 
    (fenix.complete.withComponents [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
    ])
    pkgs.ffmpeg
    pkgs.openssl
    pkgs.pkg-config
  ];
}

