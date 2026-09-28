---
bump: patch
---

### Fixed
- Lean parses on Windows again. The `tree-sitter-lean4` crate compiles its generated parser without MSVC's `/utf-8`, so its non-ASCII node kind names (`×`, `→`, `∀`, `⟨` …) were re-encoded in the Windows code page: `Node::kind` panicked on `×` and silently returned `?` for the others. The same parser (upstream `wvhulle/tree-sitter-lean` revision `bd942cd2`, byte-identical to the crate's sources) is now vendored under `rust/vendor/tree-sitter-lean` and compiled with `-utf-8` like the other vendored grammars.
- The issue #195 conformance and generative fixtures keep LF line endings on Windows checkouts, since their manifests pin file digests and byte offsets.

### Added
- `grammar_names` (Rust) and `grammarNames` (JavaScript) return a default grammar's node kind and field names. The default CST expectations record a digest of them for every grammar, and both runtimes check it, so a compiler that garbles a grammar's names fails on every platform CI runs.
