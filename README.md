# sppg
![Rust](https://github.com/mtelahun/sppg/actions/workflows/rust.yml/badge.svg)
[![codecov](https://codecov.io/gh/mtelahun/sppg/branch/main/graph/badge.svg?token=A1P9I5E2LU)](https://codecov.io/gh/trevi-software/rhodos)
[![License](https://img.shields.io/badge/License-BSD_2--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)

Secure passphrase generator using the diceware method.

For a long time the [`apg`](https://github.com/jabenninghoff/apg) command line program has allowed users to create random, pronounceable passwords that are also hard to guess. This program attempts to do the same thing for pass phrases.

It uses the [diceware](https://theworld.com/~reinhold/diceware.html) method to derive random passphrases and takes some precautions to ensure that all the phrases it displays are as secure as possible within the confines of the user's request. By default it outputs a 5 word passphrase and won't output anything with less than 4 words unless you specify the quality option (-q | --quality). However, even with the quality option it won't display any phrases with less than 8 characters (including spaces).

Pre-requisites
--------------
1. Git source code versioning system
`https://git-scm.com/book/en/v2/Getting-Started-Installing-Git`
2. Rust programming language [Office install guide](https://www.rust-lang.org/tools/install)
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

To insure it was installed correctly type the following commands and make sure you get a successful output:
```
rustc --version
cargo --version
```

Build
-----
From a terminal:
1. Clone this repository

```git clone https://github.com/mtelahun/sppg.git```
2. Change into the cloned directory and type:

```cargo run --release```

Installation
------------
It currently does not have any installation scripts so you have to copy it manually to a directory in your PATH.

Use
---
```
Usage: sppg [OPTIONS]

Options:
  -e, --eff                        
  -n, --num-of-pass <NUM_OF_PASS>  \[default: 6\]
  -w, --word-count <WORD_COUNT>    \[default: 5\]
  -c, --use-capital-char           
  -s, --use-special-char           
  -q, --quality                    Implies -c and -s
  -h, --help                       Print help
  -V, --version                    Print version
```
