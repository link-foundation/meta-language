# Issue #90 — CI/CD Template Comparison

Issue #90 asks to compare the GitHub Pages / CI practices across the four
AI-driven-development pipeline templates, adopt the best ones, and file an
upstream issue wherever the same root-404 bug exists. Snapshot taken 2026-06-14
via the GitHub Contents API.

## Workflow inventory

| Template | Workflows with a Pages deploy | Pages source | What gets uploaded |
|---|---|---|---|
| [`js`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template) | `example-app.yml` | `actions/upload-pages-artifact@v5` | `examples/universal-app/dist` — a **Vite build** (`npm run example:web:build`) that emits a root `index.html`. |
| [`rust`](https://github.com/link-foundation/rust-ai-driven-development-pipeline-template) | `release.yml` (`deploy-docs` job) | `actions/upload-pages-artifact@v5` | **`target/doc`** — raw `cargo doc` output, **no root `index.html`**. |
| [`python`](https://github.com/link-foundation/python-ai-driven-development-pipeline-template) | `docs.yml` | `actions/upload-pages-artifact@v5` | **`_site`** — a **Sphinx build** (`sphinx-build -b html docs _site`) that emits a root `index.html`. |
| [`csharp`](https://github.com/link-foundation/csharp-ai-driven-development-pipeline-template) | `docs.yml` | `actions/upload-pages-artifact@v5` | **`_site`** — a **DocFX build** (`docfx docfx.json -o _site`) that emits a root `index.html`. |

## Root-404 bug presence

| Template | Site root has `index.html`? | Affected by #90's bug? |
|---|---|---|
| js | ✅ Vite `dist/index.html` | No |
| **rust** | ❌ `target/doc/<crate>/index.html` only | **Yes — same bug** |
| python | ✅ Sphinx `_site/index.html` | No |
| csharp | ✅ DocFX `_site/index.html` | No |

Only the **rust** template reproduces meta-language's failure: it uploads
`cargo doc`'s `target/doc` directly, which never contains a top-level
`index.html`, so the Pages root 404s. (meta-language's own pre-fix
`release.yml` was derived from this template — same job name "Deploy Rust
Documentation", same `path: target/doc`.) An upstream issue spec is in
[`proposed-issues/01-pages-root-404-cargo-doc.md`](./proposed-issues/01-pages-root-404-cargo-doc.md).

## Best practices adopted from the templates

1. **Build a real site root, then upload it** (js/python/csharp pattern) — the
   core fix. meta-language now assembles `_site/` (landing page + demo + `/api/`
   redirect) instead of uploading `target/doc`.
2. **`find _site -maxdepth N -print` before upload** — the **csharp** `docs.yml`
   already logs its `_site` tree (`find _site -maxdepth 3 -print`). Adopted into
   meta-language's `Deploy Website` job (`find _site -maxdepth 2 -print`) as a
   permanent post-build verification of the published tree.
3. **Keep the docs deploy independent of package/release publication** — already
   present in the rust template's comment ("still updates when the release path
   fails"); preserved.
4. **One-time "Settings → Pages → Source = GitHub Actions" note** — present in
   all templates' Pages comments; preserved in meta-language's job and README.
5. **Echo `page_url` after deploy** — the **csharp** `docs.yml`
   (`echo "Pages deployed to: ${{ steps.deployment.outputs.page_url }}"`) is a
   nice touch worth keeping in mind for the meta-language job.

## Cross-checking the other workflow files

Beyond Pages, the templates' `release.yml` package-publish jobs upload language
artifacts (`dist/`, `*.nupkg`, `target` package files) — those `path:` lines are
unrelated to the Pages bug and are correct. `js/links.yml` and
`js/example-app.yml`'s non-Pages steps are app/build specific. No other shared
CI bug surfaced in the comparison.
