# Contributing to spg

Thank you for your interest in contributing! 🎉

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)

## Getting Started

```bash
git clone https://github.com/alejaranda/spg
cd spg
cargo build
```

## Running Tests

```bash
cargo test
```

## Code Style

This project uses `rustfmt` and `clippy`. Before submitting a PR, make sure:

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

## Submitting a Pull Request

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Make your changes and add tests if applicable
4. Ensure CI checks pass locally (fmt + clippy + test)
5. Open a PR with a clear description of the change
