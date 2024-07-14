## calc.rs calc.rs

# Overview
The Command Line Calculator (clc) is a simple, lightweight calculator that evaluates mathematical expressions provided via command line arguments, written on Rust. It supports basic arithmetic operations, several mathematical functions, and utilizes the shunting yard algorithm to handle operator precedence and parentheses.

# Usage
To use the calculator, run the program with a mathematical expression as a command line argument:

```sh
clc <expression>
```
(it works best on nutshell due to the lack of regular expressions).

# Features

    Basic Arithmetic Operations: +, -, *, /, %, ^
    Mathematical Functions: sin, cos, ln, log10, exp, sqrt, abs
    Floating Point Numbers: Supports both integer and floating-point numbers
    Operator Precedence: Correctly handles operator precedence using the shunting yard algorithm
    Error Handling: Gracefully handles errors in expression parsing and evaluation

# How to install
```sh
git clone https://github.com/Gidrex/calc.rs
cd ./calc.rc/clc
cargo build --release
cp ./target/build/release/clc /usr/bin # or other dicectory with your binares
```
