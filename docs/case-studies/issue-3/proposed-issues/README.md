# Proposed Implementation Issues

This directory contains **ready-to-file issue specifications** that decompose
[issue #3](https://github.com/link-foundation/meta-language/issues/3) (and the
founding vision [issue #1](https://github.com/link-foundation/meta-language/issues/1))
into actionable work. Each `NN-slug.md` file is a complete issue body with YAML
frontmatter (`title`, `labels`).

## Why specs, not auto-filed issues

Issue #3 asks us to "create issues in this repository." Filing 16 GitHub issues is
an outward-facing, hard-to-reverse action, so — per this repository's operating
guidance — the case study ships the specs plus an idempotent creation script and
leaves the actual filing to a maintainer's single command:

```sh
# Preview (default — creates nothing):
./create-issues.sh

# Actually file them (idempotent: skips any whose exact title already exists):
./create-issues.sh --create
```

The script reads `title`/`labels` from each file's frontmatter, strips the
frontmatter, and creates the issue with the remaining markdown as the body. It only
uses labels that already exist in the repo (`enhancement`, `documentation`).

## Index & phasing

Phasing mirrors [`../solution-plans.md`](../solution-plans.md).

| # | Title | Phase | Requirement IDs | Blocked by |
|---|---|---|---|---|
| 01 | Add plain-text (`txt`) as a first-class container target | 0 | LANG-TXT | — |
| 02 | Reconcile natural-language target ordering with Ethnologue 2025 | 0 | (online-research) | — |
| 03 | Integrate tree-sitter as the universal lossless parser front-end | 1 | LANG-PL×7, CORE-1/3/4/5/7, PAR-1 | — |
| 04 | Mixed-mode embedding via tree-sitter injection | 1 | LANG-MIX, CORE-7 | 03 |
| 05 | Natural-language segmentation & identification layer | 1 | LANG-NL, CORE-5/6 | — |
| 06 | Acquire a Visual Basic grammar | 2 | LANG-PL (VB) | 03 |
| 07 | Acquire SQL dialect grammars | 2 | LANG-PL (SQL) | 03 |
| 08 | Acquire a Delphi/Object Pascal grammar | 2 | LANG-PL (Delphi) | 03 |
| 09 | Enrich the substitution/query matcher | 3 | CORE-13/14/15, PAR-8 | — |
| 10 | Persistent immutable snapshots (green/red model) | 3 | CORE-11/12, PAR-5 | — |
| 11 | Unified query + transform surface over links | 3 | CORE-14, PAR-1/8 | 03, 09 |
| 12 | Materialize self-description roots as links | 4 | CORE-8/9 | — |
| 13 | Seed the common concept ontology from meta-expression | 4 | CORE-10 | 12 |
| 14 | Cross-language reconstruction + configurable formalization | 4 | CORE-16/17, PAR-12 | 13 |
| 15 | Adopt external competitor test corpora | 5 | I3-2, PAR-1…6 | 03 |
| 16 | Adopt ecosystem test corpora | 5 | I3-2, PAR-7…12 | — |

All 16 trace back to a requirement ID in [`../requirements.md`](../requirements.md)
and a solution in [`../solution-plans.md`](../solution-plans.md).
