# Issue 163 Case Study: JavaScript Meta-Language

Issue 163 asked for a JavaScript implementation of the meta-language feature
surface, dependency research, CI/CD split by language, and a guard that keeps
Rust and JavaScript behavior in sync.

This case study records the investigation and implementation decisions for PR
164. Raw GitHub and npm discovery output is stored in `raw-data/`; template
repository file inventories are stored in `template-data/`.

## Result

- Moved the Rust crate out of the repository root into `rust/` so that **no
  language implementation lives at the root**. The crate keeps its own `src/`,
  `tests/`, `scripts/`, `web/`, `benches/`, `examples/`, `Cargo.toml`,
  `README.md`, and badges.
- Added `js/` as an npm package named `@link-foundation/meta-language` with its
  own `src/`, `tests/`, `scripts/`, `README.md`, and badges.
- Implemented the parity operation families in JavaScript: parse, query,
  transform, substitute, serialize, snapshot, translate, and verify.
- Added `parity/language-features.json` and two enforcement points around it:
  the JavaScript checker `js/scripts/check-js-rust-parity.mjs` and the
  Rust-native test `rust/tests/unit/parity_manifest.rs`. Each language verifies
  its own half of every manifest row, and both CI workflows run the manifest
  gate, so a change in one language that is not mirrored in the other fails CI.
- Converted `.github/workflows/release.yml` fully into
  `.github/workflows/rust.yml` and added `.github/workflows/js.yml`. The two
  workflows have independent path filters so each language builds and releases
  on its own.
- Rewrote the repository-root `README.md` as a language-neutral overview that
  links into `rust/` and `js/`, and gave each language folder its own README and
  badges modeled on the templates referenced in the issue.

## Related Documents

- [Requirements](requirements.md)
- [Dependency Survey](dependency-survey.md)
- [Template Comparison](template-comparison.md)
- [Solution Plan](solution-plans.md)
