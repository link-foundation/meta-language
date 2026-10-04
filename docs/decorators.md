# Decorators

A decorator changes what one pipeline level produces without changing the
code of that level. Both runtimes share one decorator model, one Links
Notation form and one corpus of cases (`parity/decorators/`), so a decorator
set written once has the same effect in JavaScript and in Rust.

## The model

Each level shows its items to a decorator as **records**: maps of string
fields, where a missing field reads as the empty text. A decorator has:

- `id`, a name of letters, digits, `.`, `_` and `-`, unique in its set;
- `level`, one of the ten levels below;
- `order`, an integer (default 0);
- `when`, a list of `[field, value]` pairs a record must all match;
- `actions`, applied in order: `set field value`, `replace field from to`
  (every occurrence of a non-empty `from`) and `drop`.

A set composes the decorators of a level in a defined order: by `order`, then
by `id`. Each one decorates what the one before produced, and `when` is
checked against that record. A `drop` ends the chain. Sets are immutable, and
`add` and `remove` return new sets, so removing a decorator gives exactly the
output of the set without it.

## Links Notation

A set is stored as links, one decorator per line, in a canonical form both
runtimes read and write:

```lino
(decorator rename-item (level importer) (order 0) (when (format gbnf) (name item)) (set name letter))
(decorator spaced-definition (level emitter) (order 0) (replace line %3A%3A%3D %3A%3A%3D%20))
(decorator drop-comment (level emitter) (order 4) (when (line %23)) (drop))
```

Values are percent-encoded the same way grammar link texts are, so any text
survives, and the empty text is written `%`. Use
`DecoratorSet.fromLino` / `toLino` in JavaScript and `DecoratorSet::from_lino`
/ `to_lino` in Rust.

## Levels

| Level | Record | `drop` | JavaScript hook | Rust hook |
| --- | --- | --- | --- | --- |
| `importer` | `{ format, name, kind, concept }` per rule | removes the rule | `importGbnf(text, { decorators })` and every other importer | `decorate_grammar(&grammar, &set, DecoratorLevel::Importer)` on what an importer returns |
| `grammar-rule` | `{ name, kind, concept }` per rule | removes the rule | `decorateGrammar`, `compileGrammar(grammar, { decorators })` | `decorate_grammar`, `FeatureParseOptions::decorators` |
| `merge-decision` | `{ kind, name, members, basis, definition }` per decision | removes the decision | `mergeGrammars(sources, { decorators })` | `GrammarMergeOptions::decorators` |
| `concept-mapping` | `{ from, rule, to, relation, concept, rules }` (rules joined by spaces) | the construct becomes `unknown` | `translateNativeConstruct(from, rule, to, { decorators })` | `translate_native_construct_decorated` |
| `executor` | `{ type, kind, field, text }` per node and token | unwraps a node into its children; a token cannot be dropped | parser `{ decorators }` | `FeatureParseOptions::decorators` |
| `recovery` | `{ type: error, reason, text }` and `{ type: missing, kind }` | removes a MISSING node; an ERROR node cannot be dropped | parser `{ decorators }` | `FeatureParseOptions::decorators` |
| `cst-to-ast` | `{ term, text }` per syntax fact | leaves the fact out of the model | `analyzeProgram(source, language, project, { decorators })` | `analyze_program_decorated` |
| `transformation` | `{ capture, old, new }` per replacement | leaves the match as it is | `network.replace(matches, rule, { decorators })` | `LinkNetwork::replace_decorated` |
| `emitter` | `{ format, number, line }` per emitted line | removes the line | `emitGbnf(grammar, { decorators })` and every other emitter | `decorate_emitted(format, &source, &set)` |
| `translation-rule` | `{ rule, language, text }` per template | removes the template, so the language falls back | `rules.render(language, network, root, { decorators })`, `rules.decorated(set)` | `TranslationRuleSet::decorated` |

A hook refuses a decoration that would lose input: dropping a token, an ERROR
node or the root of a tree is a `DecoratorError`.

## Making translated output match hand-written code

The translation-rule and emitter levels exist so a generated translation can
be brought to match hand-written source without editing the translator: a
decorator rewrites the template or the emitted line that differs, and
removing it restores the generic output. The `bare-await` and
`spaced-definition` decorators of `parity/decorators/levels.lino` are
minimal examples.

## Tests

`js/tests/decorators.test.js` and `rust/tests/unit/decorators.rs` run every
hook with `parity/decorators/levels.lino` and compare the runtime-neutral
view recorded in `parity/decorators/cases.json`. They also check composition
order (`composition.lino`), the Links Notation round trip, removal, and the
rejected decorations. Both record the `I195-DECORATORS-EVERY-LEVEL` ledger
row.
