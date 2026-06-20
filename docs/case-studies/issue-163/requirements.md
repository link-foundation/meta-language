# Requirements

## User-Facing Requirements

- Provide a JavaScript implementation of the meta-language core features.
- Make the JavaScript package usable from npm consumers, especially
  `relative-meta-logic`.
- Search for JavaScript/Rust associative dependencies instead of reimplementing
  text grammars from scratch where an ecosystem option exists.
- Add CI/CD coverage for JavaScript and Rust.
- Add a rule that prevents Rust and JavaScript feature drift.
- Collect issue data, analysis, requirements, plans, and library choices under
  `docs/case-studies/issue-163`.

## Executable Scope

The first synchronized JavaScript feature set is captured in
`parity/language-features.json`:

- links-network core
- lossless text parse and reconstruction
- JavaScript identifier query support
- S-expression query predicates
- query-based source transforms
- structural substitution
- link-cli substitution text
- LiNo topology serialization
- snapshots
- translation rules
- verification reports
- Peggy grammar emission
- JavaScript parser module emission
- API-style fixture registry

## Layout Decision

The issue requested `./js` and `./rust` folders. This PR adds `./js` and keeps
Rust in the repository root because the existing published crate, docs site,
examples, benchmarks, and release workflow use the root manifest. The existing
Rust helper scripts already detect either `./Cargo.toml` or `./rust/Cargo.toml`,
so a later mechanical Rust move can be done separately without mixing it into
the JavaScript implementation.
