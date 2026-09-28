---
bump: patch
---

### Fixed
- The JavaScript runtime recovers from malformed input exactly as the native runtime does. The published web-tree-sitter runtime always parses UTF-16, and tree-sitter charges error recovery per skipped byte, so it chose different recoveries on malformed input (Lean `import` without a module gave `(MISSING identifier)` instead of `(ERROR)`; similar cases in Rocq comments, Rust `extern` and JavaScript `if`). The package now ships a web-tree-sitter 0.25.10 runtime rebuilt with a small patch that makes it read UTF-8 (`js/src/vendor/web-tree-sitter`, reproducible with `js/scripts/build-web-tree-sitter-runtime.mjs`, whose unpatched build equals the npm wasm byte for byte). `parity/fixtures/issue-195-conformance/error-recovery.json` holds the diverging cases with trees from the native tree-sitter CLI, and both runtimes test them.
