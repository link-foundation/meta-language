---
title: "Acquire SQL dialect grammars"
labels: enhancement
---

## Context

SQL is TIOBE #9. tree-sitter SQL support is **fragmented across dialect-specific
grammars** (`tree-sitter-sequel` is the most maintained; also
`tree-sitter-sql-bigquery`, DerekStride's `tree-sitter-sql`). There is no single
grammar covering all SQL. See
[`rust-libraries-survey.md`](../rust-libraries-survey.md) §A.

## Scope

- Treat SQL as a **language family**: register dialect keys (e.g. `sql-ansi`,
  `sql-postgres`, `sql-sqlite`, `sql-bigquery`) rather than one `sql` grammar
  (NFR-5: coexisting dialects → register each).
- Adopt `tree-sitter-sequel` as the baseline; map at least one dialect end-to-end
  through the `#03` adapter.
- Document which constructs each adopted dialect grammar covers/misses.

## Acceptance criteria

- [ ] At least one SQL dialect parses with a real grammar and reconstructs
      byte-for-byte.
- [ ] The SQL `LANGUAGE_FIXTURES` entry is keyed to a specific dialect.
- [ ] Dialect coverage/gaps documented; grammar license recorded.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Survey: [`rust-libraries-survey.md`](../rust-libraries-survey.md) §A
- Solution: [`solution-plans.md`](../solution-plans.md) Solution 2
- Requirement: LANG-PL (SQL)
- Blocked by: `#03`
