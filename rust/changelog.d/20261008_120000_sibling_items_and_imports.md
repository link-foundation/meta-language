---
bump: minor
---

### Added

- Self-translation into Rust binds the other top-level items of a module: an item may call a function and read a constant declared before or after it, and the Rust names them as their own translations do (#202).
- A relative named import, `import { a, b as c } from './m.mjs'`, translates as `use crate::m::{a, b as c};`. `selfTranslationSignatures` (`self_translation_signatures`) gives a module's exported signatures, and the `imports` option (`SelfTranslationOptions` with `self_translate_with` in Rust) binds them in the modules that import it (#203).
- A module whose only statement declares a top-level constant translates into Rust as a constant in both runtimes: a literal number or boolean is a `const` of its type, a literal string a `&str`, and any other value a lazily computed `static`. Its signature lets the modules that import it read it. The Rust root had only the JavaScript half (`emit_rust_constants`), and the new `math-to-rust` case, which `imports-to-rust` reads its items from, shows both roots agree.

### Changed

- Default and namespace imports, Node.js built-in modules and packages are refused with their own diagnostics instead of the advice to import the assertion module.
