# Overview
The Command Line Calculator (clc) is a simple, lightweight calculator that evaluates mathematical expressions provided via command line arguments, written on Rust. It supports basic arithmetic operations, several mathematical functions, and utilizes the shunting yard algorithm to handle operator precedence and parentheses.

# Usage
To use the calculator, run the program with a mathematical expression as a command line argument:

```sh
clc <math operatings>
```
(it works best on [nushell](https://github.com/nushell/nushell) due to the lack of regular expressions).

# Features
- Basic Arithmetic Operations: +, -, *, /, %, ^
- Mathematical Functions: sin, cos, ln, log10, exp, sqrt, abs
- Floating Point Numbers: Supports both integer and floating-point numbers
- Operator Precedence: Correctly handles operator precedence using the shunting yard algorithm
- Error Handling: Gracefully handles errors in expression parsing and evaluation
- Easy usage with nix

# How to install
```sh
git clone https://github.com/Gidrex/calc.rs
cd ./calc.rs/clc
cargo build --release
cp ./target/build/release/clc /usr/bin # or other dicectory with your binares
```
![usage with command clc](./preview/nushell.png)

Alternatively, you can use wrapper.sh for this:
```sh
./wrapper.sh
```
![wrapper.sh](./preview/wrapper.png)

# How to use on Nix
```sh
git clone https://github.com/Gidrex/calc.rs
cd ./calc.rs/clc
cargo build --release
nix-shell
```
![usage with nix-shell, integraned with nushell](./preview/nixshell.png)

# TODO
- fix auto build in nixshell

