---
title: "Pluggable language-parser registry (user-registrable grammars and parser dispatch)"
labels: enhancement
---

## Context

Issue #47 demands that "everything in our system should be replaceable,
configurable and so on" and that translation support be "expandable and
configurable by end user". The `LanguageParser` trait exists
(`src/language_parser.rs`), but `LinkNetwork::parse` hardwires
`BuiltInLanguageParser`: users cannot register a new language, override a
grammar, or replace dispatch. See [`requirements.md`](../requirements.md)
**R-17**/**R-18** and [`solution-plans.md`](../solution-plans.md) **S-4**.

Competitor precedent: TXL grammar overrides and SWC's plugin lesson
([`competitors-code-tooling.md`](../competitors-code-tooling.md)) - user
registrations should shadow built-ins for the same key, not fork the pipeline.

## Scope

- Add a `ParserRegistry` mapping language key → `Arc<dyn LanguageParser>`,
  pre-populated with the built-in set.
- `ParseConfiguration::with_parser_registry(...)` (or equivalent) makes every
  parse entry point honor the registry; user entries shadow built-ins.
- Registered parsers must emit links (terminology translation stays at the
  boundary, per `docs/parity-roadmap.md`).
- Document a minimal custom-parser example under `examples/`.

## Acceptance criteria

- [ ] A test registers a custom parser for a new language key and parses
      through the public API without touching built-ins.
- [ ] A test shadows an existing key and observes the override.
- [ ] Default behavior without a custom registry is unchanged.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) R-17, R-18
- Solution: [`solution-plans.md`](../solution-plans.md) S-4
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
