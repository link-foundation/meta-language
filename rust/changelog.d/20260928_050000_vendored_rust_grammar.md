---
bump: patch
---

### Fixed
- Rust sources from real published crates no longer produce error nodes: both runtimes now parse Rust with `tree-sitter-rust` v0.24.2 vendored under `rust/vendor/tree-sitter-rust` with a patch for `~` and bare `$` in macro token trees, unit structs with a where clause, attributes on struct-pattern fields and later tuple elements, `as T <= e`, unit types in where predicates, turbofish calls of functions named like primitive types, the 2015 `try!` macro and cargo script frontmatter.
