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

The issue requested `./js` and `./rust` folders with no language at the
repository root. This PR delivers exactly that:

- The Rust crate moved from the repository root into `rust/`, taking its
  `Cargo.toml`, `src/`, `tests/`, `scripts/`, `web/`, `benches/`, `examples/`,
  `README.md`, and badges with it.
- The JavaScript package lives in `js/` with its own `src/`, `tests/`,
  `scripts/`, `README.md`, and badges.
- Shared assets stay at the root: `parity/` (the cross-language manifest),
  `docs/` (grammar reference, fidelity matrices, website source, case studies),
  and `.github/` (the `rust.yml` and `js.yml` workflows).
- The repository-root `README.md` carries no implementation — it is a
  language-neutral overview that points into `rust/` and `js/`.

CI/CD was split per language: `release.yml` was fully converted into `rust.yml`,
and `js.yml` was added. Both workflows run the parity gate so feature drift in
either direction fails CI.
