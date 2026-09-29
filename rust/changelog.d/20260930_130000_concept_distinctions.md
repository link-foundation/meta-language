---
bump: minor
---

### Added

- `concept_correspondence` relates two source spellings through the concept records and the foundation register: `Shared` only when each spelling means exactly one concept and it is the same one (with the definition that justifies sharing it), `Distinct` with the recorded reason when the concepts differ, `Ambiguous` when a spelling has several meanings, and `Unknown` when it has none. Pest's `|` and BNF's `|`, Rust's and Lean's `panic!`, and Lean's and Rocq's `Prop` therefore never merge.
- `check_concept_distinctions` keeps ordered and unordered choice, lexical and syntactic precedence, binding and assignment, and the integer, overflow, effect, universe, proof and logic models distinct, and requires every concept several sources share to state the meaning that justifies the correspondence.
- `grammar_precedence_concepts` classifies each tree-sitter precedence use as lexical precedence (inside a token) or syntactic precedence (between parse alternatives). The concept records gain `grammar.lexical-precedence` and `grammar.syntactic-precedence`, and the foundation register records why Lean's non-cumulative and Rocq's cumulative universes differ.
