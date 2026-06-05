# Contributing to meta-language

This repository contains the Rust core for a self-describing meta language over
a links network. Contributions should keep the public API small, tested, and
aligned with the issue requirements.

## Development Setup

1. Install Rust with `rustup`.
2. Install the standard tooling:

   ```bash
   rustup component add rustfmt clippy
   cargo install rust-script
   ```

3. Build and test:

   ```bash
   cargo build
   cargo test
   ```

## Code Standards

- Model structural data as links and references to links.
- Keep external parser terminology at API boundaries and translate it into the
  links-network model internally.
- Prefer explicit metadata links or typed metadata over side tables.
- Add focused tests for each new behavior.
- Use Rust documentation comments for public APIs.
- Keep generated experiments in `experiments/` and real usage examples in
  `examples/`.

## Local Checks

Run these before pushing code:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test --all-features
rust-script scripts/check-file-size.rs
rust-script scripts/check-crate-size.rs
```

## Changelog

User-facing changes need a fragment in `changelog.d/`:

```markdown
---
bump: minor
---

### Added
- Short description of the change.
```

Use `major` for incompatible API changes, `minor` for new backward-compatible
features, and `patch` for fixes.
