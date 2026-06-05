---
title: "Integrate tree-sitter as the universal lossless parser front-end (+7 official grammars)"
labels: enhancement
---

## Context

The crate currently parses every fixture with a generic, language-agnostic
scaffold parser (`src/link_network.rs`); the only dependency is `clap`. To deliver
real "in and out" support for the programming-language targets (issue #3) we need
grammar-aware parsing. tree-sitter is the highest-leverage choice: one adapter
yields lossless spans, error recovery, queries, and language injection across
**every** grammar that exists. See
[`docs/case-studies/issue-3/rust-libraries-survey.md`](../rust-libraries-survey.md)
§A and [`solution-plans.md`](../solution-plans.md) Solution 1.

This single adapter covers **7 of 10** TIOBE targets immediately (Python, C, Java,
C++, C#, JavaScript, R — all have official MIT grammar crates) and underpins
CORE-1/3/4/5/7 and PAR-1.

## Scope

- Define a `LanguageParser` trait and a `tree-sitter` → links adapter that, per
  node, emits a link with `LinkType`, named/anonymous flag, field label
  (tree-sitter fields → `LinkType::Field`), `ByteRange`, and row/col `Point`s;
  synthesizes `Trivia`/`Token` links from inter-child byte gaps; maps `ERROR`/
  `MISSING` nodes to `LinkFlags`.
- Add deps: `tree-sitter` + `tree-sitter-python`, `-c`, `-java`, `-cpp`,
  `-c-sharp`, `-javascript`, `-r`.
- Re-back the 7 corresponding `LANGUAGE_FIXTURES` entries with real parses, keeping
  the byte-exact round-trip assertion.
- Keep the generic scaffold parser as the fallback for languages without a grammar.
- Honor NFR-1: translate tree-sitter "node/tree" vocabulary to links at the
  boundary; never expose it.

## Acceptance criteria

- [ ] The 7 languages parse with their real grammars and `reconstruct_text() ==
      input` byte-for-byte for each fixture.
- [ ] An error fixture (e.g. truncated `if (`) produces `is_error`/`is_missing`
      links **and** still round-trips the original bytes.
- [ ] tree-sitter fields appear as `LinkType::Field` labeled links.
- [ ] `cargo test --all-features` passes; crate-size/file-size gates still pass.
- [ ] Changelog fragment added (`bump: minor`).

## Notes

- Visual Basic, SQL, and Delphi are **not** covered here (no official grammars) —
  see `#06`, `#07`, `#08`.
- A native-Rust JS alternative (`biome_js_parser`) may be added later as a second
  source (NFR-5 "try both"); out of scope for this issue.

## References

- Survey: [`rust-libraries-survey.md`](../rust-libraries-survey.md) §A, §B
- Solution: [`solution-plans.md`](../solution-plans.md) Solution 1
- Requirements: LANG-PL, CORE-1/3/4/5/7, PAR-1
