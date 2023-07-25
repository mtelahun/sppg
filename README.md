# sppg
![Rust](https://github.com/mtelahun/sppg/actions/workflows/rust.yml/badge.svg)
[![codecov](https://codecov.io/gh/mtelahun/sppg/branch/main/graph/badge.svg?token=A1P9I5E2LU)](https://codecov.io/gh/trevi-software/rhodos)
[![License](https://img.shields.io/badge/License-BSD_2--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)

Secure passphrase generator using the diceware method.

For a long time the [`apg`](https://github.com/jabenninghoff/apg) command line program has allowed users to create random, pronounceable passwords that are also hard to guess. This program attempts to do the same thing for pass phrases.

It uses the [diceware](https://theworld.com/~reinhold/diceware.html) method to derive random passphrases and takes some precautions to ensure that all the phrases it displays are as secure as possible within the confines of the user's request. By default it outputs a plain 5 word passphrase and won't output anything with less than 4 words unless you specify the quality option (-q | --quality). However, even with the quality option it won't display any phrases with less than 8 characters (including spaces).

A Note about security
---
- A four word passphrase should suffice for the average computer user. A five or six word passphrase will suffice for someone who's position in their organization might make them a legitimate target to compromise. If your threat model includes adversaries who can dedicate a large amount of money and resources against you the length of your passphrase is the least of your worries.
- ~~This program includes the spaces between words in determining the length of a passphrase. You should also include them when you type your passphrase. Theoretically, it is possible for an adversary to guess the number and length of the words in your passphrase by listening for the sound of the space bar, but if that is a realistic part of your theat model why are you even reading this?~~ The `-S` option can now be used to supply your own separator character.
- This program uses the original word list from Arnold Reinhold by default. However, there is an option to use the EFF's revised list. Both lists are exactly the same security-wise. The difference is in the words included in the list. The EFF's list includes longer words, removes some Americanisms, and removes a broader range of potentially offensive words. My personal preference is for the original list because it's shorter to type on average.
- If you want shorter passphrases you can use the `--quality` option to insert one special character and convert one letter to upper case at random.

    `sppg --word-count 2 --quality`

- Ultimately the security of your passphrase is only as good as the sytem it's stored on. If the webservice you're accessing stores passwords in plaintext in its database and it gets hacked it will do you no good to have a 12 word passphrase. You should **never** reuse passphrases. Better yet, use a password manager.
- "This is all well and good," you might say. "But how's **your** password hygiene?" you might ask. I'm only some rando on the internet that plays at being a software developer. I use a password manager and for its master password I use a plain, unadorned 4 word passphrase from the original list generated thusly:

    `sppg -w 4`

- These resources [offer](https://xkcd.com/936/) [further](https://palant.info/2023/01/30/password-strength-explained/) [information](https://proton.me/blog/protonmail-com-blog-password-vs-passphrase) on the subject of passphrase security.

Pre-requisites
--------------
1. Git source code versioning system

`https://git-scm.com/book/en/v2/Getting-Started-Installing-Git`

2. Rust programming language [Official install guide](https://www.rust-lang.org/tools/install)

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
To install this package switch to the root of repository directory and type:

`cargo install --path .`

Use
---
```
Usage: sppg [OPTIONS]

Options:
  -e, --eff                        Use EFF wordlist
  -n, --num-of-pass <NUM_OF_PASS>  Number of phrases to output [default: 6]
  -w, --word-count <WORD_COUNT>    Number of words in a phrase [default: 5]
  -c, --use-capital-char           Convert one letter at random to uppercase
  -s, --use-special-char           Insert one special character at random
  -q, --quality                    Implies -c and -s
  -S, --separator <SEPARATOR>      Use SEPARATOR (instead of ' ') to separate words
  -h, --help                       Print help
  -V, --version                    Print version
```
