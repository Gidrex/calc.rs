{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup cargo
    nushell
];

shellHook = ''
  cargo build --release
  cat << 'EOF' > ~/command_wrapper.sh
#!/usr/bin/env bash
while true; do
  read -p "> " cmd
  if [ "$cmd" = "exit" ]; then
    break
  fi
  nu -c "./target/release/clc \"$cmd\""
done
EOF
  chmod +x ~/command_wrapper.sh
  ~/command_wrapper.sh
  '';
}

