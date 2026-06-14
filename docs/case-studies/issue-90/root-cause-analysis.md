# Issue #90 — Root Cause Analysis

## Symptom

`https://link-foundation.github.io/meta-language` is "not working". HTTP probes
(2026-06-14, [`raw-data/website-http-probes.txt`](./raw-data/website-http-probes.txt)):

```
GET /                  -> 404
GET /index.html        -> 404
GET /api/              -> 404
GET /meta_language/    -> 200
```

The root and every "friendly" path 404; only the crate-named docs path works.

## Root cause

The Pages deploy job (`Deploy Rust Documentation`) published the **raw `cargo
doc` output directory** as the site:

```yaml
- run: cargo doc --no-deps --all-features
- uses: actions/upload-pages-artifact@v5
  with:
    path: target/doc        # <-- the whole rustdoc output dir
- uses: actions/deploy-pages@v5
```

`rustdoc` writes its HTML under a **crate-named subdirectory**, not at the root.
From the last pre-fix run ([`raw-data/deploy-docs-job-prefix.log`](./raw-data/deploy-docs-job-prefix.log)):

```
Generated /home/runner/work/meta-language/meta-language/target/doc/meta_language/index.html
...
INPUT_PATH: target/doc
Uploading artifact: github-pages.zip
...
Reported success!
```

So the uploaded artifact contained `meta_language/index.html` but **no
`target/doc/index.html`**. GitHub Pages serves `index.html` at the requested
path; with nothing at the root, `GET /` has no document to serve and returns 404.
The docs are reachable only at `/meta_language/`, which matches the probe results
exactly.

Historically `cargo doc` wrote a root `index.html` redirect only when the target
directory contained exactly one crate *and* under older toolchains; current
stable `rustdoc` does not emit a root `index.html` for a normal `cargo doc
--no-deps` build, so relying on it is fragile regardless.

## Why it was silent (the deeper cause)

The deploy was **green on every one of the ten releases** between 2026-06-07 and
2026-06-14 ([`raw-data/deployments-github-pages.json`](./raw-data/deployments-github-pages.json)).
`actions/deploy-pages` reports success when the artifact is accepted and
published — it does **not** fetch the root URL to confirm it loads. The job's
implicit success criterion ("artifact uploaded") did not match the real
requirement ("root URL serves a page"). With no post-deploy smoke check, the only
detector left was a human visiting the advertised URL — which is precisely how
#90 was opened.

## Secondary gaps (also required by #90)

Even at the working `/meta_language/` path, the site was **only auto-generated
API documentation**. Issue #90 asks for a **description + demo + docs**. The
pre-fix site had:

- no landing page describing what meta-language is (description) — **missing**;
- no interactive demo — **missing**;
- docs — present, but mislocated at `/meta_language/` instead of a stable `/api/`.

## Fix

Replace "upload raw `cargo doc`" with "assemble a real site root", implemented by
`scripts/build-site.rs` and the renamed `Deploy Website` job:

- `/` → `docs/site/index.html` landing page (description, features, quickstart,
  docs links) + an embedded WebAssembly demo;
- `/demo/` → the `web/` wasm crate built with `wasm-pack` (Links-Notation
  playground);
- `/api/` → `cargo doc` output, with a generated root `index.html` redirect into
  the crate subdirectory so `/api/` itself never 404s;
- `.nojekyll` so Pages serves the rustdoc assets verbatim.

The deploy job also runs `find _site -maxdepth 2 -print` before upload so future
runs log the exact published tree — a lightweight, always-on verification that
the root `index.html` exists. This is the "add verbose output for the next
iteration" requirement, applied preventively even though the root cause was
already conclusive from existing logs.

## Verification

- Local build of `_site/` produces `_site/index.html`, `_site/demo/…`, and
  `_site/api/index.html` (verified before commit).
- The wasm demo parses `(1: 1 1)` correctly in a real browser (Playwright),
  proving the demo path is functional end to end.
- After merge, `GET /` will serve the landing page instead of 404; this can be
  re-confirmed with the same probe script in `raw-data/`.
