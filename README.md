# vortex 👁️

[![en](https://img.shields.io/badge/lang-en-red.svg)](/README.md)
[![ua](https://img.shields.io/badge/lang-ua-green.svg)](/readmes/README.ua.md)

## Description 📄

The vortex is a high-velocity, precision tool designed for swiftly detecting open ports on target system. Engineered with Rust 🦀

## Installation ⚙️

Here is the detailed process of installation:

1. Clone the repository - `git clone https://github.com/pyc4che/vortex.git | cd vortex`
2. Build project - `cargo build`

After this steps you are ready to launch the program, for this type: `cargo run -- -h`. The help message will be displayed

![help](/imgs/help.png)

## Packages 📦

For the scanner we need: parse arguments, proceed network operations and form the output

1. Bpaf - Parse Arguments;
2. Colored - Colorize the Output;
3. Tokio - Asynchronous Runtime for Network Manipulation;

## Demo 🎞️

Here is how the code looks in action

![demo](/imgs/demo.png)
