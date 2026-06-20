# Issue 163 Case Study: JavaScript Meta-Language

Issue 163 asked for a JavaScript implementation of the meta-language feature
surface, dependency research, CI/CD split by language, and a guard that keeps
Rust and JavaScript behavior in sync.

This case study records the investigation and implementation decisions for PR
164. Raw GitHub and npm discovery output is stored in `raw-data/`; template
repository file inventories are stored in `template-data/`.

## Result

- Added `js/` as an npm package named `@link-foundation/meta-language`.
- Implemented the parity operation families in JavaScript: parse, query,
  transform, substitute, serialize, snapshot, translate, and verify.
- Added `parity/language-features.json` and
  `js/scripts/check-js-rust-parity.mjs` as the cross-language sync guard.
- Added `.github/workflows/js.yml` and `.github/workflows/rust.yml`.
- Kept the existing Rust crate at repository root for this PR because release,
  documentation, examples, and website workflows already assume that layout.
  Existing scripts already support a future `rust/` root through `RUST_ROOT`.

## Related Documents

- [Requirements](requirements.md)
- [Dependency Survey](dependency-survey.md)
- [Template Comparison](template-comparison.md)
- [Solution Plan](solution-plans.md)
