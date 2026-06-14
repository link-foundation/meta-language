---
bump: minor
---

### Fixed
- Fixed the GitHub Pages site returning HTTP 404 at the root URL
  (<https://link-foundation.github.io/meta-language>). The deploy job uploaded
  raw `cargo doc` output, which has no root `index.html` (rustdoc only emits
  `target/doc/<crate>/index.html`), so the Pages root 404'd while docs were only
  reachable at `/meta_language/`. See issue #90.

### Added
- A project website with a landing page (description + docs links) under
  `docs/site/`, deployed at the Pages root.
- An interactive WebAssembly "Links Notation playground" demo in a new
  standalone `web/` crate (wraps the wasm-compatible `links-notation` crate),
  served at `/demo/`.
- `scripts/build-site.rs` to assemble the website (`_site/`) locally and in CI:
  landing page at `/`, demo at `/demo/`, and `rustdoc` under `/api/` with a root
  redirect.
- `docs/case-studies/issue-90` documenting the timeline, requirements, root
  cause analysis, and solution plans.

### Changed
- The `Deploy Website` CI job (formerly `Deploy Rust Documentation`) now builds
  the WebAssembly demo and assembles the full site instead of deploying
  `target/doc` directly.
