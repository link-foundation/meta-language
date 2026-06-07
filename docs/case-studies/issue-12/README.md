# Issue 12: Delphi/Object Pascal Grammar

Issue #12 asked for a Pascal/Delphi grammar adapter, byte-exact
reconstruction, gap documentation, a license record, and a decision on whether
to fork for fuller Delphi coverage.

## Candidate Evaluation

### Published `tree-sitter-pascal`

- Crate: <https://crates.io/crates/tree-sitter-pascal/0.10.2>
- Repository: <https://github.com/Isopod/tree-sitter-pascal>
- License: MIT. MIT is permissive and compatible with this repository's
  Unlicense distribution model.
- Binding shape: exposes the same safe `LANGUAGE` `LanguageFn` constant used by
  the existing grammar crates.
- Coverage signal: the crate README describes Pascal dialect coverage for
  Delphi and Free Pascal, including classes, records, interfaces, helpers,
  nested declarations, variant records, Delphi- and FPC-flavored generics,
  anonymous procedures/functions, inline assembler, extended RTTI attributes,
  and FPC PasCocoa extensions.
- Result: selected as the initial grammar-backed baseline for
  `Delphi/Object Pascal`.

## Integration

The adapter recognizes `Delphi/Object Pascal`, `Delphi`, `Object Pascal`, and
`Pascal` labels and loads `tree-sitter-pascal`. The executable fixture is now a
Delphi-style unit with a generic class, RTTI-style attribute, and property. The
grammar-backed fixture test verifies byte-exact reconstruction, a clean
verification report, syntax spans, named syntax metadata, and a Pascal `unit`
node. A focused fixture test also checks for `declClass`, `declProp`, and
`rttiAttributes` nodes.

## Coverage Decision

Decision: accept the published generic Pascal grammar for now rather than fork.
The selected crate already covers the issue's highest-risk Delphi syntax areas
well enough for an initial adapter: units, properties, attributes, generics, and
inline local variables all parse cleanly in local probes or automated fixtures.

This repository still does not claim full Delphi compiler coverage. Remaining
gaps are tracked as explicit scope limits:

- No compatibility matrix by Delphi, Free Pascal, or Object Pascal compiler
  version.
- No exhaustive fixture suite for conditional compilation directives, package
  and library projects, resource directives, or design-time artifacts such as
  `.dfm` files.
- No semantic validation for symbol resolution, visibility, overload binding,
  property accessor correctness, generic constraints, or compiler-mode rules.
- Unsupported Delphi syntax will still preserve source text through recovery or
  the lossless fallback, but only clean `tree-sitter-pascal` parses are
  advertised as grammar-backed coverage.
