---
title: "Adopt data-exchange format grammars (JSON, YAML, TOML, XML, INI, protobuf, GraphQL)"
labels: enhancement
---

## Context

Issue #47 requires "CST/AST for ... data exchange formats", but no mainstream
interchange format has a wired grammar - only LiNo parses structurally; JSON,
YAML, XML, TOML, CSV, INI, protobuf, and GraphQL fall back to plain-text
tokens. See [`requirements.md`](../requirements.md) **R-3**/**R-4** and
[`solution-plans.md`](../solution-plans.md) **S-2**.

Research verified modern-binding grammar crates compatible with the project's
tree-sitter 0.25.8 ([`formats-storage-apis.md`](../formats-storage-apis.md)
Part A): `tree-sitter-json` 0.24.8, `tree-sitter-yaml` 0.7.2,
`tree-sitter-toml-ng` 0.7.0, `tree-sitter-xml` 0.7.0 (XML + DTD),
`tree-sitter-ini` 1.4.0, `tree-sitter-proto` 0.4.0, `tree-sitter-graphql`
0.1.0. CSV and JSON5 crates still pin tree-sitter ~0.20, so PR #48 handles
them with in-repo lossless parsers validated by `csv` and `json5_nodes`.

## Scope

- Wire the seven compatible grammar crates through
  `src/tree_sitter_adapter.rs`, following the PR #44-#46 pattern.
- Add a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` with parity-gate
  tests, mirroring `MARKUP_LANGUAGE_TARGETS`.
- Add `LANGUAGE_FIXTURES` round-trip fixtures per format (UTF-8 + recovery
  cases) and mixed-region cases (e.g. JSON in Markdown fences, YAML
  frontmatter).
- Record CSV/JSON5 status in `docs/parity-roadmap.md` with the in-repo parser
  rationale and compatibility notes.

## Acceptance criteria

- [ ] Each wired format parses and reconstructs byte-for-byte via
      `LinkNetwork::parse` / `reconstruct_text`.
- [ ] `DATA_FORMAT_TARGETS` is gated by tests like the existing registries.
- [ ] At least one mixed-region fixture embeds a data format inside Markdown
      or HTML.
- [ ] Roadmap documents CSV/JSON5 handling; no silent gap.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) R-3, R-4
- Solution: [`solution-plans.md`](../solution-plans.md) S-2
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
