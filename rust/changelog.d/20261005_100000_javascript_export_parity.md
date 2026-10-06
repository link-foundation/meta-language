---
bump: minor
---

### Added

- Every public export of the JavaScript package now has a public Rust counterpart, listed in `parity/language-features.json` and checked by a generated test. New public Rust items: `sequence`, `choice` and `canonical_repeat` (with `RepetitionBoundsError`) build grammar expressions; `display_grammar_expression`, `GRAMMAR_DIAGNOSTIC_KINDS`, `canonical_rule_definition`, `accepts_text` and `carry_rule_docs` validate, compare and document grammars; `parse_with_grammar` (with `ParseWithGrammarError`) compiles and parses in one call; `render_declaration_links` and `render_rule_fields` render grammar links; `QueryIndex`, `query_by_concept_term` and `RejectPredicateHost` query networks; `detect_embedded_regions` detects the embedded regions of source text and `detect_embedded_regions_in_tree` those of a parsed document; `sniff_language` guesses a language from content.
