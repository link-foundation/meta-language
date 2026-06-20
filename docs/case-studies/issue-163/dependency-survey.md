# Dependency Survey

## JavaScript

- `links-notation@0.13.0`
  - Existing npm package from the Link Foundation ecosystem.
  - Provides an ESM `Parser`, `Link`, and formatter surface.
  - Used for link-cli-style substitution text parsing.
- `peggy@5.1.0`
  - Maintained PEG parser generator for JavaScript.
  - Used as the runtime dependency for generated parser modules emitted by the
    JavaScript grammar surface.
- Node.js built-in test runner
  - Avoids a larger test framework dependency for the first package surface.

## Rust

- Existing Rust dependencies remain unchanged.
- The Rust side already uses `links-notation = "0.13"` and the grammar/importer
  stack already present in `Cargo.toml`.

## Related Repository Need

`link-foundation/relative-meta-logic` PR 182 described the practical blocker:
the meta-language capability existed only as Rust code and no npm package was
available. The JS package in this PR addresses that bridge directly.
