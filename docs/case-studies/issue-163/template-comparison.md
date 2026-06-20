# Template Comparison

The issue asked to compare JavaScript, Rust, Python, and C# CI/CD template
practices. File inventories captured during investigation are stored in
`template-data/`.

## Shared Practices

- Keep language-specific workflows small and explicit.
- Cache package manager artifacts by lockfile.
- Run format/lint before tests where the language has standard tooling.
- Include a package dry-run or build artifact check when publication is a goal.
- Prefer path filters so docs-only changes do not run all language jobs.

## Applied Here

- JavaScript workflow:
  - `npm ci`
  - `npm test`
  - `npm run check:parity`
  - `npm pack --dry-run`
- Rust workflow:
  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets --all-features`
  - `cargo test --all-features --verbose`
  - parity manifest check in manifest-only mode

## Not Applied Yet

- Python and C# workflows were not added because this issue only introduced a
  JavaScript implementation and existing Rust crate coverage.
- A repository-wide workspace move into `rust/` was deferred to avoid combining
  a package relocation with the feature implementation.
