# Grammar-format round-trip fidelity matrix

`meta-language` imports external grammar notations into the shared
[`Grammar` IR](architecture.md), emits that IR back to a target notation, and
re-imports the emitted text to check what survived the trip. This page records
whether each grammar construct is lossless, equivalent, or lossy for each
profiled format. The matrix starts with BNF and is generated from the
`grammar_format_profile(format)` API so the documentation cannot drift from the
code.

## Entry point

`grammar_format_profile(format)` returns a `GrammarFormatProfile` for every
entry in `GRAMMAR_FORMATS`. The profile classifies every entry in
`GRAMMAR_CONSTRUCTS` as native support, equivalent support, or one documented
lossy fallback:

```rust
use meta_language::{grammar_format_profile, GrammarFidelityLevel};

let bnf = grammar_format_profile("bnf").expect("BNF profile");

assert_eq!(
    bnf.construct_fidelity("terminal"),
    Some(GrammarFidelityLevel::Lossless),
);
assert_eq!(
    bnf.construct_fidelity("zero-or-more-repetition"),
    Some(GrammarFidelityLevel::Lossy),
);
```

The BNF round trip is measured over `import_bnf` followed by `emit_bnf` and
`import_bnf` again. Native BNF constructs preserve the IR exactly. Constructs
BNF cannot spell directly either synthesize helper productions, drop
BNF-inexpressible metadata with a lossy report, or return an explicit
unsupported-construct error.

## Capability profiles

Each grammar format exposes a `LanguageProfile` over the grammar construct
vocabulary through `GrammarFormatProfile::language_profile()`. The invariant
enforced by the test suite is the same one used by the document fidelity
matrix: for every format and every entry in `GRAMMAR_CONSTRUCTS`, the construct
is either represented without a lossy fallback or has exactly one documented
fallback.

## Per-format construct support

| Construct | BNF |
| --- | :---: |
| empty-expression | ✅ |
| sequence | ✅ |
| ordered-choice | ⚠️ |
| unordered-choice | ✅ |
| optional-expression | ⚠️ |
| zero-or-more-repetition | ⚠️ |
| one-or-more-repetition | ⚠️ |
| counted-repetition | ⚠️ |
| character-range | ⚠️ |
| character-class | ⚠️ |
| any-character | ⚠️ |
| terminal | ✅ |
| case-insensitive-terminal | ⚠️ |
| nonterminal | ✅ |
| positive-predicate | ⚠️ |
| negative-predicate | ⚠️ |
| capture | ⚠️ |
| atomic-rule | ⚠️ |
| silent-rule | ⚠️ |
| token-rule | ⚠️ |

✅ lossless · ≈ equivalent spelling or normalization · ⚠️ documented lossy fallback

## Documented lossy fallbacks

| Format | Construct | Fallback |
| --- | --- | --- |
| bnf | ordered-choice | emitted as an unordered BNF alternative; priority semantics are not preserved |
| bnf | optional-expression | emitted through a synthetic helper production with an empty alternative |
| bnf | zero-or-more-repetition | emitted through a recursive synthetic helper production with an empty alternative |
| bnf | one-or-more-repetition | emitted through a recursive synthetic helper production plus one required item |
| bnf | counted-repetition | emitted as required occurrences plus optional or recursive synthetic helper productions |
| bnf | character-range | expanded to a synthetic helper production enumerating each character when the range is bounded |
| bnf | character-class | expanded to a synthetic helper production for finite non-negated classes; unsupported classes are rejected |
| bnf | any-character | unsupported by BNF emission and rejected instead of silently broadening the language |
| bnf | case-insensitive-terminal | emitted as a case-sensitive literal and reported as lossy |
| bnf | positive-predicate | unsupported by BNF emission and rejected because lookahead has no BNF equivalent |
| bnf | negative-predicate | unsupported by BNF emission and rejected because lookahead has no BNF equivalent |
| bnf | capture | emitted as the captured expression while dropping the capture label |
| bnf | atomic-rule | emitted as a normal BNF production; rule-kind metadata is dropped |
| bnf | silent-rule | emitted as a normal BNF production; rule-kind metadata is dropped |
| bnf | token-rule | emitted as a normal BNF production; rule-kind metadata is dropped |

## Round-trip guarantee

The BNF row is backed by `tests/unit/grammar_fidelity.rs`. The tests assert
that every profiled construct is either supported or has one fallback, that
lossless BNF constructs survive `import_bnf -> emit_bnf -> import_bnf` as the
same rule list, that lossy helper fallbacks re-import without undefined
non-terminals, and that unsupported BNF constructs return documented
`GrammarEmitError::Unsupported` errors instead of silent output.

Later importer/emitter issues can add rows by extending `GRAMMAR_FORMATS`,
adding a `grammar_format_profile` branch, and adding same-format fixtures for
that row.
