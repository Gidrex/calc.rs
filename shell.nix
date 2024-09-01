{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup cargo
    nushell
];

shellHook = ''
  cat << 'EOF' > ./wrapper.sh

#!/usr/bin/env bash
cargo build --release 
while true; do
  read -p "> " cmd
  if [ "$cmd" = "exit" ]; then
    break
  fi
  nu -c "./target/release/clc \"$cmd\""
done
EOF
  chmod +x ./wrapper.sh
  ./wrapper.sh
  '';
}

