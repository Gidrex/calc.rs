{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup cargo
    nushell
];

shellHook = ''
  cat << 'EOF' > ~/command_wrapper.sh
#!/usr/bin/env bash
while true; do
  read -p "> " cmd
  if [ "$cmd" = "exit" ]; then
    break
  fi
  nu -c "/home/gidrex/github/calc.rs/clc/target/release/clc \"$cmd\""
done
EOF
  chmod +x ~/command_wrapper.sh
  ~/command_wrapper.sh
  '';
}

