---
bump: minor
---

### Added
- The dependency delivery gate accepts an item behind its current stable release only at its verified newest compatible release. The audit records that release and the holders that exclude the current one, derived from the registries, the Cargo resolve graph, the emscripten version that tree-sitter pins, and the ocamlfind bound on OCaml. Every holder must itself be delivered.
- `check-dependencies.mjs --delivery --live` refreshes the audit from the registries and fails when the recorded audit is outdated or the registries cannot be reached. Live mode is the default in CI, and the clean-consumer acceptance job runs it against the installed npm artifact.
