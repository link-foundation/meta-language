---
title: "Mixed-mode embedding via tree-sitter injection (one unified links network)"
labels: enhancement
---

## Context

Issue #3 requires txt/markdown/html "in **mixed mode**." The crate already tracks
the four `GRAMMAR_EMBEDDING_TARGETS` (Markdown+code, Markdown+HTML, HTML+JS,
HTML+CSS) and has detection-only scaffolding in `src/mixed_regions.rs`. The
remaining work is to actually parse each embedded region with its own grammar and
attach the results to the host as **one** links network with cross-language links
on shared byte ranges — the explicit divergence from tree-sitter's N-disjoint-trees
model named in issue #1 §1. See [`solution-plans.md`](../solution-plans.md)
Solution 5.

## Scope

- Use tree-sitter **injection** to model fenced-code / `<script>` / `<style>`
  embedding.
- Detect regions **both** ways (NFR-5): name-driven (fence tag, `<script type>`,
  extension) **and** content-driven (sniff the body; `lingua`/`whatlang` for prose,
  lightweight signatures for code). Default documented.
- Parse each region via the appropriate grammar adapter (from
  [#7](https://github.com/link-foundation/meta-language/issues/7)) and link it
  into the host network keyed on its byte range.
- Whole-document `reconstruct_text()` stays byte-exact.
- Fall back to a `txt` region ([#5](https://github.com/link-foundation/meta-language/issues/5))
  when sniffing fails.

## Acceptance criteria

- [ ] Each of the four embedding targets produces **one connected** network (not
      separate parse results), with cross-language links on shared byte ranges.
- [ ] Whole-document reconstruction is byte-for-byte exact for all four fixtures.
- [ ] Both name-driven and content-driven detection are exercised by fixtures.
- [ ] `cargo test --all-features` passes.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Solution: [`solution-plans.md`](../solution-plans.md) Solution 5
- Requirements: LANG-MIX, CORE-7
- Blocked by: [#7](https://github.com/link-foundation/meta-language/issues/7)
