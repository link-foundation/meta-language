# Case Study: Issue #90 — `https://link-foundation.github.io/meta-language` is not working

## Summary

Issue [#90](https://github.com/link-foundation/meta-language/issues/90) reports
that the project's GitHub Pages site at
<https://link-foundation.github.io/meta-language> is not working, and asks to
**fix automated documentation and website publishing so the site offers a
description + demo + docs**. The issue further requires:

- using best practices from the four AI-driven-development pipeline templates
  (`js`, `rust`, `python`, `csharp`), comparing every workflow / CI file, and
  filing upstream issues where the same bug exists;
- downloading all logs and data into `docs/case-studies/issue-90/` and doing a
  deep case-study analysis (timeline, requirements register, root causes,
  per-requirement solution plans, library reuse, online research);
- adding debug/verbose output if data is insufficient to find the root cause;
- applying the fix everywhere the problem occurs;
- executing everything inside the single existing pull request
  [PR #91](https://github.com/link-foundation/meta-language/pull/91) on branch
  `issue-90-5532f0ba547e`.

This folder is the issue #90 case-study record: raw GitHub/Pages/HTTP evidence,
the requirement register, root-cause analysis, per-requirement solution plans,
the timeline, the CI/CD template comparison, and the source spec for the filed
upstream issue.

Investigation date: 2026-06-14.

## Key findings

- **Root cause (confirmed):** the deploy job uploaded raw `cargo doc` output as
  the Pages artifact. `rustdoc` only emits `target/doc/<crate>/index.html`
  (here `target/doc/meta_language/index.html`); it never writes a root
  `target/doc/index.html`. With `path: target/doc` uploaded to Pages, the site
  root has no `index.html`, so `GET /` returns **404** while the docs are only
  reachable at `/meta_language/`. The deploy job itself **reported success**, so
  the failure was silent — nothing in CI flagged it. See
  [`root-cause-analysis.md`](./root-cause-analysis.md).
- **Live evidence (2026-06-14):**
  `GET /` → **404**, `GET /index.html` → **404**, `GET /api/` → **404**,
  `GET /meta_language/` → **200**. Captured in
  [`raw-data/website-http-probes.txt`](./raw-data/website-http-probes.txt).
- **CI log evidence:** the last pre-fix deploy run shows
  `Generated …/target/doc/meta_language/index.html` (no root `index.html`),
  `actions/upload-pages-artifact@v5` with `path: target/doc`, and
  `Reported success!`. Captured in
  [`raw-data/deploy-docs-job-prefix.log.txt`](./raw-data/deploy-docs-job-prefix.log.txt).
- **No description, no demo:** even when reached at `/meta_language/`, the site
  was only auto-generated API docs — it had no landing page describing the
  project and no interactive demo, which the issue explicitly requires.
- **Template comparison:** of the four templates, only
  [`rust-ai-driven-development-pipeline-template`](https://github.com/link-foundation/rust-ai-driven-development-pipeline-template)
  ships the same `cargo doc → upload target/doc` Pages deploy and therefore the
  same root-404 bug. The `js`, `python`, and `csharp` templates build a real
  site root, so they are not affected. See
  [`template-comparison.md`](./template-comparison.md). Filed upstream as
  [rust-template#79](https://github.com/link-foundation/rust-ai-driven-development-pipeline-template/issues/79);
  spec in [`proposed-issues/`](./proposed-issues/).

## Issue #90 requirements → deliverables

| Ask | Delivered by |
|---|---|
| Fix automated docs & website publishing (root must work) | `Deploy Website` job in `.github/workflows/release.yml` assembles a real site root via `scripts/build-site.rs` |
| Website has a description | Landing page `docs/site/index.html` (+ `styles.css`) served at `/` |
| Website has a demo | WebAssembly Links-Notation playground (`web/` crate) served at `/demo/` |
| Website has docs | `rustdoc` served at `/api/` with a root redirect |
| Reuse best practices from 4 CI/CD templates; compare all workflow files | [`template-comparison.md`](./template-comparison.md) |
| File upstream issue where the same bug exists | Filed [rust-template#79](https://github.com/link-foundation/rust-ai-driven-development-pipeline-template/issues/79); spec in [`proposed-issues/01-pages-root-404-cargo-doc.md`](./proposed-issues/01-pages-root-404-cargo-doc.md) |
| Download all logs/data to `docs/case-studies/issue-90` | This folder, including [`raw-data/`](./raw-data/) |
| Deep case-study analysis (timeline, requirements, root causes, plans) | [`timeline.md`](./timeline.md), [`requirements.md`](./requirements.md), [`root-cause-analysis.md`](./root-cause-analysis.md), [`solution-plans.md`](./solution-plans.md) |
| Add debug/verbose output if data was insufficient | Not needed — root cause confirmed from existing CI logs + HTTP probes; the new deploy job adds a `find _site -maxdepth 2 -print` verification step for future runs |
| Apply fix everywhere | Single Pages deploy path in this repo; the only other affected place is the rust template (upstream issue filed) |
| One PR on the issue branch | Branch `issue-90-5532f0ba547e`, [PR #91](https://github.com/link-foundation/meta-language/pull/91) |

## Document index

| File | Purpose |
|---|---|
| [`timeline.md`](./timeline.md) | Reconstructed sequence of events from first Pages deploy to the fix. |
| [`requirements.md`](./requirements.md) | Traceable register of every issue #90 requirement with status and evidence. |
| [`root-cause-analysis.md`](./root-cause-analysis.md) | Why the root 404'd, with CI-log and HTTP evidence and why it was silent. |
| [`solution-plans.md`](./solution-plans.md) | One plan per requirement, with library/component reuse and reasoning. |
| [`template-comparison.md`](./template-comparison.md) | Comparison of Pages/CI practices across the four pipeline templates. |
| [`proposed-issues/`](./proposed-issues/) | Source spec for the filed upstream issue (rust template). |
| [`raw-data/`](./raw-data/) | Raw snapshots: issue #90, comments, PR #91, Pages config, deployments, deploy job log, HTTP probes, pre-fix workflow snippet. |

## Status

Requirement extraction, root-cause analysis, solution planning, the website +
demo + docs implementation, the CI deploy fix, the template comparison, and the
upstream issue spec are complete on branch `issue-90-5532f0ba547e`
([PR #91](https://github.com/link-foundation/meta-language/pull/91)). The live
site will only switch from 404 to the new landing page after this PR merges and
the `Deploy Website` job runs on the default branch.
