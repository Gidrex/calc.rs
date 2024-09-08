#!/usr/bin/env bash
cargo build --release 
while true; do
  read -p "> " cmd
  if [ "$cmd" = "exit" ]; then
    break
  fi
  nu -c "./target/release/clc \"$cmd\""
done
