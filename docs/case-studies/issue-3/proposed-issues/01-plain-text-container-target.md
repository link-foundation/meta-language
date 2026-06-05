---
title: "Add plain-text (txt) as a first-class container target"
labels: enhancement
---

## Context

Issue #3 asks for "full support for **txt**, markdown and html in mixed mode," but
`MARKUP_LANGUAGE_TARGETS` (`src/parity.rs`) currently lists only Markdown and HTML.
Plain text is the one explicitly-named container with no registry entry. See
[`docs/case-studies/issue-3/requirements.md`](../requirements.md) → **LANG-TXT**.

Plain text is also architecturally important: it is the **degenerate container**
(the whole buffer is a single prose/region link) and the **fallback** when
content-sniffing cannot identify an embedded region's language, which makes the
mixed-mode contract total — every byte belongs to some region.

## Scope

- Add a `Txt` entry to `MARKUP_LANGUAGE_TARGETS`.
- Add a `LANGUAGE_FIXTURES` entry: a multi-line UTF-8 plain-text sample that
  round-trips byte-for-byte through `reconstruct_text()`.
- Make the region detector in `src/mixed_regions.rs` fall back to a single `txt`
  region when no language is detected.
- No external dependency required.

## Acceptance criteria

- [ ] `MARKUP_LANGUAGE_TARGETS` includes `txt`; the parity gate in
      `tests/unit/link_network.rs` passes.
- [ ] A `txt` `LANGUAGE_FIXTURES` entry parses and reconstructs exactly (including a
      trailing newline and a non-ASCII line).
- [ ] Content-sniffing with no match yields one `txt` region rather than an error.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) LANG-TXT
- Solution: [`solution-plans.md`](../solution-plans.md) Solution 3
- Founding vision: #1 §3.2 (mixed-language parsing)
