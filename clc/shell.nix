{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup cargo
    nushell
  ];

  shellHook = ''
    exec nu
  '';
}
