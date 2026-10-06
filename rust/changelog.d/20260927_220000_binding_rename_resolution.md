---
bump: minor
---

### Added
- Binding resolution follows Rust `self::`, `super::`, `crate::`, and module paths and single-item `use` aliases, declares Rust `let`/`for`/closure pattern bindings and `macro_rules!` metavariables (`$name` only sees metavariables), and resolves identifiers inside format-macro strings (`{x}`, not `{{x}}`) and Lean `s!` interpolations.
- Lean and Rocq binder groups, `fun`, `∀`/`forall`, and `let … in` binders are scoped to their extent, so shadowing proof binders resolve to the innermost one.
- `rename_binding` rewrites Rust struct shorthand `S { x }` as `S { x: y }` and rejects any rename that changes how another name resolves, reporting the first changed offset as a capture conflict.
- Shared `bindingRenameCorpus` evidence for JavaScript, Rust, Lean, and Rocq, with JavaScript/Rust runtime parity of each fixture's resolution and rename outcome.
