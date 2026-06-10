# Proposed Implementation Issues - Issue #47

This directory contains the source specifications used to file the
implementation sub-issues that decompose
[issue #47](https://github.com/link-foundation/meta-language/issues/47) into
actionable work, following the
[issue #3 precedent](../../issue-3/proposed-issues/README.md). Each
`NN-slug.md` file is a complete issue body with YAML frontmatter (`title`,
`labels`).

Issue #47 explicitly instructs: "For each aspect of this task, we should
create issues on GitHub, with clear blocked by markings ... all tasks should
also be subtasks of this task. All tasks will be executed on the branch for
this task, so everything will be merged into single big pull request." On the
issue #3 round, maintainer feedback on PR #4 confirmed that the issues should
actually be filed, so this round files them directly and attaches them as
GitHub sub-issues of #47 with native blocked-by dependency relationships.

Cross-references between specs are written as code-styled `#NN` (the local
two-digit spec number) because real GitHub numbers are unknown until filing;
the real relationships are wired with the GitHub sub-issue and dependency
APIs after filing, and the filed-issue table in
[`../README.md`](../README.md) records the mapping.

## Filing status

The 15 specs were filed on 2026-06-10 as issues
[#49](https://github.com/link-foundation/meta-language/issues/49) through
[#63](https://github.com/link-foundation/meta-language/issues/63) (spec `#01`
→ #49 ... spec `#15` → #63), attached as sub-issues of #47, with the
blocked-by table below wired via the native dependencies API. The mapping
table with real numbers lives in [`../README.md`](../README.md). These files
remain the immutable source specs that were filed.

## Filing

```sh
# Preview (default - creates nothing):
./create-issues.sh

# File missing issues (idempotent: skips any whose exact title already exists):
./create-issues.sh --create
```

The script reads `title`/`labels` from each file's frontmatter, strips the
frontmatter, absolutizes relative links to `main`-branch blob URLs, and files
the issue. It only uses labels that already exist in the repo (`enhancement`,
`documentation`).

## Index and dependency order

| Spec | Title | Blocked by |
|---|---|---|
| `#01` | Readonly or mutable engine as per user configuration | - |
| `#02` | Adopt data-exchange format grammars | - |
| `#03` | Wire the next programming-language grammar wave | - |
| `#04` | Pluggable language-parser registry | - |
| `#05` | Serialize any links network to links-notation text | - |
| `#06` | Binary doublets storage backend (doublets-rs) | `#01`, `#05` |
| `#07` | Rust traits/types representation as links | `#05` |
| `#08` | Natural-language grammatical-correctness parsing | - |
| `#09` | Exact-match shared concept space | - |
| `#10` | Configurable translation-rule registry | `#04`, `#09` |
| `#11` | Single-language restriction profiles | `#10` |
| `#12` | Query/transform algebra enrichment | - |
| `#13` | Incremental re-parse and structural sharing | - |
| `#14` | API-style parity contract | `#06`, `#10` |
| `#15` | Competitor corpora wave 2 + coverage gate | `#02`, `#08`, `#12`, `#14` |

Phasing: specs without dependencies form phase 1 (parallelizable); `#06`,
`#07`, `#10` form phase 2; `#11`, `#14` phase 3; `#15` is the closing gate.
See [`../solution-plans.md`](../solution-plans.md) "Phasing".
