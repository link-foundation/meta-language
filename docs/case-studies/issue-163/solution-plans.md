# Solution Plan

## Implemented Plan

1. Capture issue, PR, npm, and template research under
   `docs/case-studies/issue-163`.
2. Add a JavaScript package under `js/` with its own README and badges.
3. Write tests that express the missing JavaScript feature surface.
4. Implement the core links-network behavior needed by the Rust parity fixtures.
5. Add `parity/language-features.json` as a feature-level contract.
6. Add a JavaScript parity checker that validates feature evidence and API
   operation families.
7. Add separate JavaScript and Rust workflows that both run the parity guard.
8. Move the Rust crate into `rust/` so no language sits at the repository root,
   updating every path that referenced the old root layout: the Rust helper
   scripts (`build-site.rs`, `detect-code-changes.rs`), `.gitattributes`,
   `.gitignore`, `.pre-commit-config.yaml`, the grammar doc cross-links, and the
   parity manifest's Rust evidence paths.
9. Fully convert `release.yml` into `rust.yml` with the correct per-step
   working directories (git-diff scripts run from the repo root, filesystem
   scripts run from `rust/`).
10. Add a Rust-native parity test (`rust/tests/unit/parity_manifest.rs`) so the
    bidirectional guard is enforced from both languages, not just Node.
11. Rewrite the root README as a language-neutral overview and give each
    language folder its own README and badges.
12. Update PR 164 with the implementation and verification.

## Follow-Up Plan

- Expand the parity manifest as future Rust-only features are ported to
  JavaScript; both the Node checker and the Rust test will flag any new family
  that is missing from either side.
