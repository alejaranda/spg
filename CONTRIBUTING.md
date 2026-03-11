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

## Commit Messages — Conventional Commits

This project uses [Conventional Commits](https://www.conventionalcommits.org/) to automate versioning and changelog generation via Release Please.

| Prefix | Effect | Example |
|---|---|---|
| `feat:` | minor bump (0.**x**.0) | `feat: add --count flag` |
| `fix:` | patch bump (0.0.**x**) | `fix: handle empty charset` |
| `feat!:` / `BREAKING CHANGE` | major bump (**x**.0.0) | `feat!: remove --no-symbols` |
| `perf:`, `refactor:`, `docs:`, `chore:` | no version bump | `chore: update deps` |

### Examples

```bash
git commit -m "feat: add option to generate multiple passwords"
git commit -m "fix: prevent panic on zero-length input"
git commit -m "feat!: rename --symbols to --special-chars"
```

## Submitting a Pull Request

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Make your changes and add tests if applicable
4. Use conventional commit messages (see above)
5. Ensure CI checks pass locally (fmt + clippy + test)
6. Open a PR with a clear description of the change
