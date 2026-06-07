---
title: "Natural-language segmentation & identification layer"
labels: enhancement
---

## Context

Issue #3 requires "in and out" for the 10 natural languages. Byte-exact
reconstruction is already a storage property (the scaffold keeps UTF-8 bytes +
spans), so the added value is a **segmentation + identification** link layer that
annotates the text without ever mutating the stored bytes. See
[`rust-libraries-survey.md`](../rust-libraries-survey.md) §D and
[`solution-plans.md`](../solution-plans.md) Solution 4.

## Scope

- **Segmentation:** `unicode-segmentation` (UAX#29) for the 9 space/delimiter
  scripts; `lindera` (CJK dictionary) for **Mandarin**, which UAX#29 cannot
  word-segment.
- **Identification:** `lingua` (default; 75 languages incl. all 10) with `whatlang`
  as a configurable alternative (NFR-5).
- **Normalization/bidi:** `unicode-normalization` (NFC/NFD) and `unicode-bidi`
  (Arabic/Urdu RTL) as annotation only.
- Emit `Token`/`Semantic`/`Language` links over the existing lossless text via a
  script-aware dispatch (CJK → lindera, else → unicode-segmentation).

## Acceptance criteria

- [ ] All 10 natural-language fixtures still reconstruct byte-for-byte (stored bytes
      untouched).
- [ ] Mandarin (`你好。`) is word-segmented via `lindera`; a Latin sample via
      `unicode-segmentation`; an Arabic/Urdu sample carries bidi/normalization
      annotation.
- [ ] Language identification populates a `Language` link per region; detector is
      switchable (`lingua`/`whatlang`).
- [ ] `cargo test --all-features` passes.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Survey: [`rust-libraries-survey.md`](../rust-libraries-survey.md) §D
- Solution: [`solution-plans.md`](../solution-plans.md) Solution 4
- Requirements: LANG-NL, CORE-5/6
