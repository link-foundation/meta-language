---
title: "Reconcile natural-language target ordering with Ethnologue 2025"
labels: documentation
---

## Context

`NATURAL_LANGUAGE_TARGETS` (`src/parity.rs`) and `docs/parity-roadmap.md` pin the
10 natural languages to the Britannica/Ethnologue total-speaker list. The **set of
10 is correct and verified**, but the **ordering** drifts from Ethnologue 2025 in
two places (see
[`docs/case-studies/issue-3/online-research.md`](../online-research.md) §2):

| Position | Repo | Ethnologue 2025 |
|---|---|---|
| 5 | French | Modern Standard Arabic |
| 6 | Modern Standard Arabic | French |
| 8 | Russian | Portuguese |
| 9 | Portuguese | Russian |

Because fixtures are keyed by language name (not rank), this is **cosmetic** — no
code behavior depends on it — but the doc comment should not claim an order the
cited source contradicts.

## Scope

Pick one:
- **(a)** Reorder to match Ethnologue 2025 (Arabic #5, French #6, Bengali #7,
  Portuguese #8, Russian #9), or
- **(b)** Add a note that ranks are approximate (sources/years disagree on the
  Arabic/French and Portuguese/Russian pairs) and the *set* is what is pinned.

## Acceptance criteria

- [ ] `NATURAL_LANGUAGE_TARGETS` ordering and `docs/parity-roadmap.md` agree with the
      cited source, or carry an explicit "approximate order" note.
- [ ] Parity gate still passes (all 10 present).
- [ ] Changelog fragment added (`bump: patch`).

## References

- Data: [`online-research.md`](../online-research.md) §2
- Source: <https://www.britannica.com/topic/languages-by-total-number-of-speakers-2228881>
