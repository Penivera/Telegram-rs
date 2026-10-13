# Contributing to telegram-rs

Thank you for your interest in contributing to `telegram-rs`! We welcome contributions of all kinds, including bug reports, documentation improvements, new features, and code reviews.

This guide will help you set up your development environment and understand our workflows and standards.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Fork & Clone](#fork--clone)
- [Development Workflow](#development-workflow)
  - [Building](#building)
  - [Running Tests](#running-tests)
  - [Code Formatting](#code-formatting)
  - [Clippy & Linting](#clippy--linting)
  - [Documentation](#documentation)
- [Coding & Design Standards](#coding--design-standards)
  - [Testing Standards](#testing-standards)
  - [Error Handling](#error-handling)
  - [Feature Gating](#feature-gating)
- [Commit Guidelines](#commit-guidelines)
- [Submitting Pull Requests](#submitting-pull-requests)
- [License & Contributions](#license--contributions)

---

## Code of Conduct

All contributors and participants are expected to uphold our [Code of Conduct](CODE_OF_CONDUCT.md). Please report unacceptable behavior to project maintainers.

---

## Getting Started

### Prerequisites

- **Rust**: Ensure you have a recent stable version of Rust installed (MSRV is `1.75.0`). We recommend using [rustup](https://rustup.rs/):
  ```bash
  rustup update stable
  rustup component add rustfmt clippy
  ```

### Fork & Clone

1. Fork the repository on GitHub: [https://github.com/Penivera/Telegram-rs](https://github.com/Penivera/Telegram-rs)
2. Clone your fork locally:
   ```bash
   git clone https://github.com/<your-username>/Telegram-rs.git
   cd Telegram-rs
   ```
3. Create a descriptive branch for your work:
   ```bash
   git checkout -b feat/my-new-feature
   # or
   git checkout -b fix/issue-description
   ```

---

## Development Workflow

### Building

Build the library with default features:
```bash
cargo build
```

Build with all features enabled (`mini-app`, `wallet`, `webhook`):
```bash
cargo build --all-features
```

### Running Tests

Run the test suite across all feature permutations:
```bash
# Default features
cargo test

# Mini App feature
cargo test --features mini-app

# Wallet feature
cargo test --features wallet

# All features and targets
cargo test --all-targets --all-features
```

### Code Formatting

We follow the standard Rust code style enforced by `rustfmt`. Check formatting before committing:
```bash
cargo fmt -- --check
```

To automatically format your code:
```bash
cargo fmt
```

### Clippy & Linting

We enforce zero Clippy warnings across all targets:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Documentation

Ensure all public items are documented with rustdoc comments (`///`). Build and verify docs locally:
```bash
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features --open
```

---

## Coding & Design Standards

### Testing Standards

- **Integration & Separate Modules**: Place test suites in the `tests/` directory as separate test modules rather than inline inside production source modules (`src/`).
- **Feature Flags**: When testing feature-gated code, guard the test file or module using `#![cfg(feature = "...")]`.
- **Determinism**: Avoid tests that depend on live network connections without mocks or explicit opt-in flags.

### Error Handling

- Use idiomatic `thiserror` derives defined in `src/errors.rs`.
- Avoid using `unwrap()` or `expect()` in library code; bubble up errors using `?` and descriptive `Error` variants.

### Feature Gating

- Modular features (`mini-app`, `wallet`, `webhook`) should remain truly modular. Ensure the core library builds cleanly without optional dependencies.

---

## Commit Guidelines

We recommend using [Conventional Commits](https://www.conventionalcommits.org/) for clear git history:

- `feat:` A new feature or capability
- `fix:` A bug fix
- `docs:` Documentation-only changes
- `style:` Code style, formatting, missing semicolons
- `refactor:` Code changes that neither fix a bug nor add a feature
- `perf:` Performance improvements
- `test:` Adding or updating tests
- `chore:` Maintenance tasks, dependency updates, CI workflows

Example:
```
feat(mini_app): add initData signature verification
fix(wallet): resolve session expiry timestamp comparison
docs(readme): add crate badge and quickstart example
```

---

## Submitting Pull Requests

Before opening a pull request, run the following verification checklist:

1. [ ] `cargo check --all-targets --all-features` passes cleanly.
2. [ ] `cargo test --all-targets --all-features` succeeds with 0 failures.
3. [ ] `cargo fmt -- --check` reports no formatting differences.
4. [ ] `cargo clippy --all-targets --all-features -- -D warnings` reports 0 warnings.
5. [ ] Relevant tests have been added in `tests/`.
6. [ ] New public APIs are fully documented.

When submitting:
- Reference any related issues (e.g. `Closes #42`).
- Describe what was changed and why.

---

## License & Contributions

`telegram-rs` is dual-licensed under:
- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `telegram-rs` by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
