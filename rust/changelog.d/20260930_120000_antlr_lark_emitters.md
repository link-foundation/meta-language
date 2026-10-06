---
bump: minor
---

### Added

- `emit_antlr` writes a combined ANTLR v4 grammar. Normal and silent rules become parser rules, and token and atomic rules become lexer rules. `emit_lark` writes a Lark grammar with lowercase rules and uppercase terminals. `?` marks inlined rules and `_` marks filtered ones. Character classes become regex terminals, and `%ignore` is emitted only for the grammar's own ignore rules.
- Both emitters return an `EmitReport` alongside the text. The report records every deterministic rename (collisions, reserved words, identifier case) and every lowering. Lowered constructs include ordered choice, lookahead predicates, case-insensitive literals, ranges, any-character, counted repetition, captures and regex terminals. No construct is dropped without a note. `import_antlr(emit_antlr(g))` and `import_lark(emit_lark(g))` give back `g` modulo those notes, and emission is a fixpoint from the first re-import.
- `emit-grammar --format antlr|lark` and `import-grammar --to antlr|lark` now emit instead of reporting an unsupported format.
