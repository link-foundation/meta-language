# Issue #90 — Solution Plans

One plan per requirement cluster, each noting existing components/libraries
reused rather than reinvented. IDs cross-reference
[`requirements.md`](./requirements.md).

## S-1 — Make the Pages root work (R-1, R-15)

**Problem:** uploading `target/doc` gives Pages no root `index.html`.

**Plan:** stop uploading `cargo doc` directly. Assemble an explicit site tree in
`_site/` and upload that:

- `_site/index.html` ← `docs/site/index.html` (real landing page)
- `_site/demo/` ← `web/pkg` (wasm bundle) + demo loader
- `_site/api/` ← `target/doc`, plus a generated `_site/api/index.html` redirect
  to `api/meta_language/index.html` so `/api/` never 404s
- `_site/.nojekyll` so Pages serves rustdoc's `_`-prefixed assets

**Reuse:** `actions/configure-pages@v6`, `actions/upload-pages-artifact@v5`,
`actions/deploy-pages@v5` (already in the repo — only the artifact *contents*
change). `scripts/build-site.rs` is a `rust-script` matching the repo's existing
CI-script convention, so no new toolchain is introduced.

**Verification:** the deploy job runs `find _site -maxdepth 2 -print` before
upload, logging the published tree on every run (R-14).

## S-2 — Description / landing page (R-2)

**Plan:** a static `docs/site/index.html` + `styles.css` with hero, about,
feature cards, quickstart (cargo add / library / CLI), a self-description LiNo
example, and docs links. Dark GitHub-style theme via CSS variables; responsive
grid. No build step, no framework — trivially served by Pages.

**Reuse:** plain HTML/CSS; no dependency. Content sourced from the repo README so
the description stays consistent with the crate.

## S-3 — Interactive demo (R-3, R-5)

**Plan:** a standalone `web/` crate (`meta-language-web`, `publish = false`,
`crate-type = ["cdylib","rlib"]`) exposing `parse_links_notation(input) ->
JSON` via `#[wasm_bindgen]`. `docs/site/app.js` loads the wasm module, runs the
parser on input, and renders the resulting LiNo tree + formatted round-trip.

**Reuse:**
- [`links-notation`](https://crates.io/crates/links-notation) — pure-Rust LiNo
  parser/formatter, wasm-compatible; this is the demo's engine.
- [`wasm-bindgen`](https://crates.io/crates/wasm-bindgen) +
  [`wasm-pack`](https://github.com/rustwasm/wasm-pack) (installed in CI via
  `taiki-e/install-action`) — the standard Rust→wasm toolchain.
- `serde_json` for the JSON bridge to JS.

**Constraint handled:** the full `meta-language` crate cannot target wasm (native
tree-sitter grammars), so the demo wraps the wasm-compatible substrate and shows
richer features via pre-rendered examples. `[profile.release] opt-level = "s",
lto = true` keeps the bundle small. The `web/` crate is excluded from the
published `.crate` via the Cargo `include` allowlist.

## S-4 — Docs (R-4)

**Plan:** keep `cargo doc` as the docs generator (the issue forbids replacing
*automated docs generation*), but mount it under a stable `/api/` with a root
redirect, instead of leaking the crate-named path as the site root.

**Reuse:** `cargo doc --no-deps --all-features` (unchanged generator).

## S-5 — Template best-practice comparison & upstream report (R-6, R-7)

**Plan:** compare the Pages/CI practices of the four pipeline templates; document
which share the bug and why. File an upstream issue on the affected template with
a reproducible example, a workaround, and a code-level fix.

**Findings:** only the rust template ships the `cargo doc → upload target/doc`
pattern → same root-404 bug. See [`template-comparison.md`](./template-comparison.md)
and [`proposed-issues/01-pages-root-404-cargo-doc.md`](./proposed-issues/01-pages-root-404-cargo-doc.md).

## S-6 — Case study & data capture (R-8 … R-13)

**Plan:** snapshot issue/PR/Pages/HTTP/CI data into `raw-data/`, then write the
analysis docs (this folder). Mirror the structure of the existing
`docs/case-studies/issue-47/` case study for consistency.

**Reuse:** `gh` CLI + `curl` for capture; the issue-47 case study as the template
for layout.
