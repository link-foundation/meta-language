# issue-93 sub-issue filing scripts

Reproducible record of how the 34 planned sub-issues for
[#93](https://github.com/link-foundation/meta-language/issues/93) were filed from
the specs in [`docs/case-studies/issue-93/proposed-issues/`](../../docs/case-studies/issue-93/proposed-issues/).

Each issue body is the full spec with relative links rewritten to commit-pinned
blob URLs, plus a tracking header that resolves the spec's "Blocked by" line to
real issue numbers.

## Scripts (run in order)

1. `phase1-create.sh` — create the 34 issues (#95–#128) in topological order
   (blockers first), recording `specid → number → dbid → title` into
   `issue-map.tsv`. Idempotent: dedupes by exact title.
2. `phase23-wire.sh` — Phase 2 attaches every issue as a native sub-issue of #93
   (`POST /issues/93/sub_issues`, `-F sub_issue_id=<dbid>`). (Phase 3 here hit a
   bash empty-array bug; use the standalone script below.)
3. `phase3-blockedby.sh` — wire all 51 `blocked_by` edges per the DAG
   (`POST /issues/{n}/dependencies/blocked_by`, `-F issue_id=<blocker dbid>`).
   Idempotent: skips edges already present.
4. `update-readme.sh` — fill the "Filed as" column in the proposed-issues README
   with live issue links.

## Key API notes

- Both endpoints require **typed integer** ids — use `gh api -F` (not `-f`, which
  sends a string and yields HTTP 422).
- `sub_issue_id` / `issue_id` are the issue **database ids** (`gh api
  repos/.../issues/N --jq .id`), not the per-repo issue numbers.

## Artifacts

- `issue-map.tsv` — the canonical `specid → number → dbid → title` mapping.
- `*.log` — run logs (gitignored).
- `bodies/` — generated issue bodies (gitignored; derived from the specs).
