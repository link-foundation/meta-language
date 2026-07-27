---
bump: minor
---

### Added
- Made translation rule sets recursively evaluate captured links, compose different rules across network roots, and support variadic placeholders, optional segments, target sub-language fallbacks, and indentation-aware substitutions in both Rust and JavaScript.
- Added public root-link rendering and documented that consumers provide parser front-ends for languages without a built-in syntax profile.
- Added slice-based Rust link and syntax-node insertion for parser nodes whose arity is known only at run time.
