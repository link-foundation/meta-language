---
title: "Wire the next programming-language grammar wave (PHP, Swift, Kotlin, Scala, Lua, Perl)"
labels: enhancement
---

## Context

Issue #47 requires "CST/AST for all popular programming languages". 17
grammars are wired today (TIOBE top-10 plus Rust, Go, Ruby, TypeScript/TSX,
HTML, CSS), but popular languages immediately below the top-10 - PHP, Swift,
Kotlin, Scala, Lua, Perl - have no grammar and fall back to plain text. See
[`requirements.md`](../requirements.md) **R-2** and
[`solution-plans.md`](../solution-plans.md) **S-3**.

## Scope

- Wire `tree-sitter-php`, `tree-sitter-swift`, `tree-sitter-kotlin-ng`,
  `tree-sitter-scala`, `tree-sitter-lua`, `ts-parser-perl` through
  `src/tree_sitter_adapter.rs`, following the PR #44-#46 acquisition pattern
  (verify each crate's tree-sitter binding compatibility first).
- Extend `PROGRAMMING_LANGUAGE_TARGETS` (or add an explicit second-tier
  registry) plus per-language `LANGUAGE_FIXTURES` with UTF-8 and recovery
  fixtures.
- Update `docs/parity-roadmap.md` coverage tables and source citations.

## Acceptance criteria

- [ ] Each newly wired language parses and reconstructs byte-for-byte,
      including a recovery fixture with error/missing diagnostics.
- [ ] Registry gates in `tests/unit/link_network.rs` cover the new entries.
- [ ] Roadmap records the selected crate and root node for each language.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) R-2
- Solution: [`solution-plans.md`](../solution-plans.md) S-3
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
