---
title: "Adopt external competitor test corpora (tree-sitter, LibCST, Recast, jscodeshift, rowan/cstree, Roslyn)"
labels: enhancement
---

## Context

Issue #3 says to "copy **all** the tests from competitors." Each external upstream
is currently represented by a single illustrative fixture; the bulk corpus adoption
is outstanding. Test locations, formats, licenses, and per-project adaptation plans
are documented in
[`competitor-test-suites.md`](../competitor-test-suites.md). All six upstreams are
MIT or Apache-2.0/MIT — compatible with this repo's Unlicense.

## Scope

Port the canonical assertion shapes into `PARITY_FIXTURES`, retaining a provenance
comment (upstream path + license) on each ported case. Use the four universal
pillars as the template for every fixture:

1. **Lossless round-trip** — tree-sitter `:cst` corpus, LibCST
   `assertEqual(mod.code, src)`, Recast `strictEqual(source, code)`, Roslyn
   `ToFullString()`.
2. **Explicit trivia** — comment + blank-line cases under each
   `TriviaAttachmentPolicy`.
3. **Error recovery** — tree-sitter `error_corpus`, Roslyn `_MissingIdentifiers`,
   LibCST `test_parse_errors` (assert error/missing links + round-trip of broken
   source).
4. **Query + transform** — tree-sitter S-expr queries + jscodeshift
   `__testfixtures__` input/output pairs.

Prefer LibCST `_nodes/tests/` (plain MIT) over `_parser/parso/` (dual MIT/PSF).

## Acceptance criteria

- [ ] Each of the six upstreams contributes multiple ported fixtures (not just one),
      each with a provenance comment.
- [ ] All four pillars are represented for at least tree-sitter, LibCST, Recast/
      jscodeshift, and Roslyn.
- [ ] `cargo test --all-features` passes; changelog fragment (`bump: minor`).

## References

- Suites: [`competitor-test-suites.md`](../competitor-test-suites.md)
- Solution: [`solution-plans.md`](../solution-plans.md) Solution 9
- Requirements: I3-2, PAR-1…6
- Blocked by: `#03` (real grammars make many fixtures meaningful)
