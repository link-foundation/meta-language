# Filed Implementation Issues

This directory contains the source specifications used to file the implementation
issues that decompose [issue #3](https://github.com/link-foundation/meta-language/issues/3)
(and the founding vision [issue #1](https://github.com/link-foundation/meta-language/issues/1))
into actionable work. Each `NN-slug.md` file is a complete issue body with YAML
frontmatter (`title`, `labels`).

## Filing status

Issue #3 asks us to "create issues in this repository." The first draft kept
these as source specs because creating public GitHub issues is an
outward-facing action. After maintainer feedback on PR #4 explicitly requested
creation, the specs were filed with `gh` on 2026-06-06 as issues
[#5](https://github.com/link-foundation/meta-language/issues/5) through
[#20](https://github.com/link-foundation/meta-language/issues/20).

After the June 7 default-branch audit, all 16 filed implementation issues were
confirmed closed via merged PRs
[#21](https://github.com/link-foundation/meta-language/pull/21) through
[#36](https://github.com/link-foundation/meta-language/pull/36). These files
remain as the immutable source specs that were filed, not as the current
implementation status. See [`../README.md`](../README.md) and
[`../requirements.md`](../requirements.md) for the current audit.

The script remains in the tree for traceability and repeatability:

```sh
# Preview (default — creates nothing):
./create-issues.sh

# File missing issues (idempotent: skips any whose exact title already exists):
./create-issues.sh --create
```

The script reads `title`/`labels` from each file's frontmatter, strips the
frontmatter, and creates the issue with the remaining markdown as the body. It only
uses labels that already exist in the repo (`enhancement`, `documentation`).

## Index & phasing

Phasing mirrors [`../solution-plans.md`](../solution-plans.md).

| Spec | GitHub issue | Title | Phase | Requirement IDs | Blocked by |
|---|---|---|---|---|---|
| 01 | [#5](https://github.com/link-foundation/meta-language/issues/5) | Add plain-text (`txt`) as a first-class container target | 0 | LANG-TXT | — |
| 02 | [#6](https://github.com/link-foundation/meta-language/issues/6) | Reconcile natural-language target ordering with Ethnologue 2025 | 0 | (online-research) | — |
| 03 | [#7](https://github.com/link-foundation/meta-language/issues/7) | Integrate tree-sitter as the universal lossless parser front-end | 1 | LANG-PL×7, CORE-1/3/4/5/7, PAR-1 | — |
| 04 | [#8](https://github.com/link-foundation/meta-language/issues/8) | Mixed-mode embedding via tree-sitter injection | 1 | LANG-MIX, CORE-7 | [#7](https://github.com/link-foundation/meta-language/issues/7) |
| 05 | [#9](https://github.com/link-foundation/meta-language/issues/9) | Natural-language segmentation & identification layer | 1 | LANG-NL, CORE-5/6 | — |
| 06 | [#10](https://github.com/link-foundation/meta-language/issues/10) | Acquire a Visual Basic grammar | 2 | LANG-PL (VB) | [#7](https://github.com/link-foundation/meta-language/issues/7) |
| 07 | [#11](https://github.com/link-foundation/meta-language/issues/11) | Acquire SQL dialect grammars | 2 | LANG-PL (SQL) | [#7](https://github.com/link-foundation/meta-language/issues/7) |
| 08 | [#12](https://github.com/link-foundation/meta-language/issues/12) | Acquire a Delphi/Object Pascal grammar | 2 | LANG-PL (Delphi) | [#7](https://github.com/link-foundation/meta-language/issues/7) |
| 09 | [#13](https://github.com/link-foundation/meta-language/issues/13) | Enrich the substitution/query matcher | 3 | CORE-13/14/15, PAR-8 | — |
| 10 | [#14](https://github.com/link-foundation/meta-language/issues/14) | Persistent immutable snapshots (green/red model) | 3 | CORE-11/12, PAR-5 | — |
| 11 | [#15](https://github.com/link-foundation/meta-language/issues/15) | Unified query + transform surface over links | 3 | CORE-14, PAR-1/8 | [#7](https://github.com/link-foundation/meta-language/issues/7), [#13](https://github.com/link-foundation/meta-language/issues/13) |
| 12 | [#16](https://github.com/link-foundation/meta-language/issues/16) | Materialize self-description roots as links | 4 | CORE-8/9 | — |
| 13 | [#17](https://github.com/link-foundation/meta-language/issues/17) | Seed the common concept ontology from meta-expression | 4 | CORE-10 | [#16](https://github.com/link-foundation/meta-language/issues/16) |
| 14 | [#18](https://github.com/link-foundation/meta-language/issues/18) | Cross-language reconstruction + configurable formalization | 4 | CORE-16/17, PAR-12 | [#17](https://github.com/link-foundation/meta-language/issues/17) |
| 15 | [#19](https://github.com/link-foundation/meta-language/issues/19) | Adopt external competitor test corpora | 5 | I3-2, PAR-1…6 | [#7](https://github.com/link-foundation/meta-language/issues/7) |
| 16 | [#20](https://github.com/link-foundation/meta-language/issues/20) | Adopt ecosystem test corpora | 5 | I3-2, PAR-7…12 | — |

All 16 trace back to a requirement ID in [`../requirements.md`](../requirements.md)
and a solution in [`../solution-plans.md`](../solution-plans.md).
