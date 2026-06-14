# Issue #90 — Timeline / Sequence of Events

All times UTC. Sources are in [`raw-data/`](./raw-data/).

| When | Event | Evidence |
|---|---|---|
| 2026-06-07 22:27:40 | First `github-pages` deployment recorded for the repo. The Pages source is configured as `build_type: workflow` (GitHub Actions), not branch-based. | `raw-data/deployments-github-pages.json` (oldest entry), `raw-data/pages-config.json` |
| 2026-06-07 → 2026-06-14 | Ten `github-pages` deployments run, one per release. Each runs the `Deploy Rust Documentation` job: `cargo doc` → `upload-pages-artifact path: target/doc` → `deploy-pages`. Every run **reports success** — the broken root is never flagged. | `raw-data/deployments-github-pages.json` (10 entries) |
| 2026-06-14 11:55:48 | Latest pre-fix deployment for release v0.42.0 (commit `ba2389e`). | `raw-data/deployments-github-pages.json` (latest entry) |
| 2026-06-14 11:57:26 | In that run, `cargo doc` reports `Generated …/target/doc/meta_language/index.html` — note there is **no** `target/doc/index.html`. The artifact is uploaded with `path: target/doc`. | `raw-data/deploy-docs-job-prefix.log.txt` |
| 2026-06-14 11:57:33 | `deploy-pages` prints `Reported success!`. The deploy is green even though the site root has no `index.html`. | `raw-data/deploy-docs-job-prefix.log.txt` |
| 2026-06-14 12:14:34 | Issue #90 filed: "`https://link-foundation.github.io/meta-language` is not working". | `raw-data/issue-90.json` |
| 2026-06-14 12:15:15 | PR #91 opened automatically as a WIP solution draft for #90. | `raw-data/pr-91.json` |
| 2026-06-14 (investigation) | HTTP probes confirm live state: `GET /` → 404, `GET /index.html` → 404, `GET /api/` → 404, `GET /meta_language/` → 200. | `raw-data/website-http-probes.txt` |
| 2026-06-14 (fix) | Branch `issue-90-5532f0ba547e`: new `web/` wasm crate, `docs/site/` landing page, `scripts/build-site.rs`, and the `Deploy Website` job that assembles a real site root (`/`, `/demo/`, `/api/`). | PR #91 diff |

## Reading of the sequence

The site was *technically deployed* from the very first release, and every
deploy was green, which is exactly why the breakage went unnoticed for a week
and ten releases. The job's success criterion was "did `deploy-pages` accept the
artifact", not "does the site root load". Because `cargo doc` emits documentation
under a crate-named subdirectory and never a top-level `index.html`, the artifact
was structurally incapable of serving the root URL the badges and links pointed
at. The first signal was a human opening the advertised URL (issue #90), not CI.
