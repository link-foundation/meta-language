---
bump: patch
---

### Added
- Four-language conformance fixtures (`parity/fixtures/issue-195-conformance/`). They include the upstream tree-sitter test corpora of JavaScript, Lean, Rocq and Rust at the pinned grammar revisions, and files of real projects at pinned tags with their licenses. There are also hand-authored construct, Unicode, malformed and mixed-language (Markdown fences, HTML scripts) inputs. Each case carries the concrete syntax tree the native tree-sitter 0.27.0 CLI prints, and `manifest.json` records its provenance. `js/scripts/generate-issue-195-conformance.mjs` regenerates them, and `--check` verifies them. The JavaScript suite `js/tests/issue-195-conformance.test.js` and the Rust suite `rust/tests/unit/issue_195_conformance.rs` compare every public tree with the CLI tree. They check structure, kinds, fields, spans, error and missing nodes, trivia coverage, verification diagnostics and exact reconstruction, and compares against the upstream expected trees where they agree with the CLI.

### Fixed
- The JavaScript runtime now attaches trivia to whitespace before and after the grammar root (for example the leading blank line of a file), as the Rust runtime does. Before, that whitespace had a source token but no trivia link.
- Both runtimes now treat U+200B, U+2060 and U+FEFF around hidden-rule text as whitespace trivia. The JavaScript grammar lexes these characters as extras, but they were kept as non-trivia hidden text.
