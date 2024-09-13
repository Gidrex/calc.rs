#!/usr/bin/env bash
cd ~/github/calc.rs

cargo build --release
while true; do
  read -p "> " cmd
  if [ "$cmd" = "exit" ]; then
    break
  fi
  nu -c "./target/release/clc \"$cmd\""
done

cd -
