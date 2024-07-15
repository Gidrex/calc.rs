{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rust-up cargo
    nushell
  ];

  shellHook = ''
    exec nu
  '';
}
