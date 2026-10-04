---
bump: patch
---

### Fixed

- The native TypeScript and TSX grammars pass the naming gate. The rule
  `omitting_type_annotation` is now `required_type_annotation` and `asserts`
  is now `assertion_signature`; both keep their tree-sitter names as source
  names and oracle kinds. `type_arguments` and `type_parameters` resolve to
  the Rust concepts `grammar.type-argument-list` and
  `grammar.type-parameter-list`, which are now shared constructs. The
  technical vocabulary gains the reviewed words "mapped", "predefined" and
  "readonly", and a type assertion is declared distinct from a case statement.
