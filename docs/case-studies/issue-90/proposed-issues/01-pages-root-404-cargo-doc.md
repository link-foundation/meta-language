# Upstream issue: GitHub Pages root 404s — `deploy-docs` uploads `cargo doc` output with no root `index.html`

**Target repository:** `link-foundation/rust-ai-driven-development-pipeline-template`
**Filed as:** [#79](https://github.com/link-foundation/rust-ai-driven-development-pipeline-template/issues/79) (2026-06-14)
**Found via:** downstream issue `link-foundation/meta-language#90`
**File:** `.github/workflows/release.yml`, job `deploy-docs` ("Deploy Rust Documentation")

---

## Summary

The `deploy-docs` job publishes the raw `cargo doc` output directory to GitHub
Pages:

```yaml
- name: Build documentation
  run: cargo doc --no-deps --all-features

- name: Upload GitHub Pages artifact
  uses: actions/upload-pages-artifact@v5
  with:
    path: target/doc          # <-- no root index.html lives here
```

`rustdoc` writes its HTML under a **crate-named subdirectory**
(`target/doc/<crate_name>/index.html`) and does **not** emit a top-level
`target/doc/index.html` for a normal `cargo doc --no-deps` build. GitHub Pages
serves `index.html` at the requested path, so with `target/doc` as the artifact
the **site root 404s**; docs are only reachable at
`https://<owner>.github.io/<repo>/<crate_name>/`.

The job still **reports success** (`actions/deploy-pages` confirms the artifact
was published, not that the root URL loads), so the breakage is silent — it
surfaces only when a human opens the advertised Pages URL.

## Reproduction

1. Use the template; enable Pages with Source = "GitHub Actions".
2. Let `deploy-docs` run on `main`.
3. Probe the site:

   ```console
   $ curl -s -o /dev/null -w '%{http_code}\n' https://<owner>.github.io/<repo>/
   404
   $ curl -s -o /dev/null -w '%{http_code}\n' https://<owner>.github.io/<repo>/<crate_name>/
   200
   ```

   Local proof that no root `index.html` exists:

   ```console
   $ cargo doc --no-deps --all-features
   $ ls target/doc/index.html
   ls: cannot access 'target/doc/index.html': No such file or directory
   $ ls target/doc/<crate_name>/index.html
   target/doc/<crate_name>/index.html
   ```

(Confirmed downstream: `meta-language#90`, CI log shows
`Generated …/target/doc/meta_language/index.html` and `path: target/doc`, deploy
`Reported success!`, yet `GET /` → 404.)

## Workaround (no template change)

Add a root redirect into the crate docs before upload:

```yaml
- name: Build documentation
  run: cargo doc --no-deps --all-features

- name: Add root redirect
  run: |
    crate=$(cargo metadata --no-deps --format-version 1 \
      | python3 -c 'import json,sys; print(json.load(sys.stdin)["packages"][0]["name"].replace("-","_"))')
    echo "<meta http-equiv=\"refresh\" content=\"0; url=${crate}/index.html\">" > target/doc/index.html
    touch target/doc/.nojekyll
```

## Suggested fix (in the template)

Bake the redirect + `.nojekyll` into the job so every consumer gets a working
root automatically. Minimal, robust version:

```yaml
- name: Build documentation
  run: cargo doc --no-deps --all-features

- name: Generate Pages root index
  run: |
    # rustdoc emits target/doc/<crate>/index.html but no root index.html,
    # which makes the Pages root 404. Redirect the root to the crate docs.
    crate=$(cargo metadata --no-deps --format-version 1 \
      | python3 -c 'import json,sys; print(json.load(sys.stdin)["packages"][0]["name"].replace("-","_"))')
    printf '<!doctype html><meta http-equiv="refresh" content="0; url=%s/index.html">\n' "$crate" \
      > target/doc/index.html
    touch target/doc/.nojekyll   # serve rustdoc's _-prefixed assets verbatim

- name: Verify site tree
  run: find target/doc -maxdepth 2 -print   # log the published tree (matches docs.yml in the csharp template)

- name: Upload GitHub Pages artifact
  uses: actions/upload-pages-artifact@v5
  with:
    path: target/doc
```

Optionally echo the deployed URL after `deploy-pages` (as the csharp template's
`docs.yml` already does) so the root URL is visible in the run summary:

```yaml
- name: Deploy to GitHub Pages
  id: deployment
  uses: actions/deploy-pages@v5
- run: echo "Pages deployed to ${{ steps.deployment.outputs.page_url }}"
```

## Why this matters / cross-reference

- The `python` (`sphinx-build -b html docs _site`), `csharp`
  (`docfx … -o _site`), and `js` (Vite `dist/`) templates all build a site that
  *does* contain a root `index.html`, so they don't have this bug. Only the rust
  template uploads `cargo doc` output directly.
- The csharp template's `docs.yml` already includes a `find _site … -print`
  verification step; mirroring it here would have caught this earlier.
- Downstream fix for reference: `link-foundation/meta-language` PR #91 replaced
  "upload `target/doc`" with an assembled `_site/` (landing page + demo + `/api/`
  with a root redirect).
