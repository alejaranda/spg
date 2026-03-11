# spg — Simple Password Generator 🦀

[![CI](https://github.com/alejandro/spg/actions/workflows/ci.yml/badge.svg)](https://github.com/alejandro/spg/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)

A fast, minimal CLI tool to generate secure passwords — built in Rust.

## Features

- 🔐 Cryptographically random password generation
- 🔣 Optional special characters
- 📋 Automatically copies the password to your clipboard
- ⚙️ Configurable length

## Installation

### From source

```bash
git clone https://github.com/alejaranda/spg
cd spg
cargo install --path .
```

## Usage

```
spg [OPTIONS]

Options:
  -l, --length <LENGTH>   Length of the generated password [default: 12]
  -s, --symbols           Include special symbols [default: true]
  -h, --help              Print help
  -V, --version           Print version
```

### Examples

```bash
# Generate a 12-character password with symbols (default)
spg

# Generate a 24-character password
spg --length 24

# Generate a password without special characters
spg --no-symbols

# Combine options
spg -l 32 --no-symbols
```

The generated password is printed to stdout **and** automatically copied to your clipboard.

## Development

```bash
cargo build       # build
cargo test        # run tests
cargo clippy      # lint
cargo fmt         # format
```

## License

This project is released under the [MIT License](LICENSE).
