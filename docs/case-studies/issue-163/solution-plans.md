# Solution Plan

## Implemented Plan

1. Capture issue, PR, npm, and template research under
   `docs/case-studies/issue-163`.
2. Add a JavaScript package under `js/`.
3. Write tests that express the missing JavaScript feature surface.
4. Implement the core links-network behavior needed by the Rust parity fixtures.
5. Add `parity/language-features.json` as a feature-level contract.
6. Add a JavaScript parity checker that validates feature evidence and API
   operation families.
7. Add separate JavaScript and Rust workflows that both run the parity guard.
8. Update PR 164 from draft/WIP with the implementation and verification.

## Follow-Up Plan

- Decide whether the Rust crate should be mechanically moved into `rust/`.
- If moved, update release, docs, website, examples, benches, and package
  include paths in a dedicated PR.
- Expand the parity manifest as future Rust-only features are ported to
  JavaScript.
