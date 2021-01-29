{ pkgs ? import <nixpkgs> { } }:
with pkgs;

stdenv.mkDerivation {
    name = "env";
    nativeBuildInputs = [ 
    ];
    buildInputs = [ 
        cargo
        carnix
        wasm-pack
    ];
}