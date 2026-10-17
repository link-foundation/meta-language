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
| `importer` | `{ format, name, kind, concept, composition }` per rule | removes the rule | `importGbnf(text, { decorators })` and every other importer | `decorate_grammar(&grammar, &set, DecoratorLevel::Importer)` on what an importer returns |
| `grammar-rule` | `{ name, kind, concept, composition }` per rule | removes the rule | `decorateGrammar`, `compileGrammar(grammar, { decorators })` | `decorate_grammar`, `FeatureParseOptions::decorators` |
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

## Composing a grammar rule

`composition` starts empty. Set it to one `any-grammar` or `all-grammar` Links
expression with `empty` as the insertion slot. The hook puts the rule's actual
original expression in that slot. This opt-in form accepts normal syntactic
rules and a nonempty list of uniquely named candidates using one common
`field-ranges` or `json-string` profile. Field candidates must share the same
complete selector. Unsupported profiles, malformed data and replacement bodies
refuse.

This example uses the existing public API and stores its decorator as text
through `DecoratorSet.toLino()`, which handles percent encoding:

```javascript
import { DecoratorSet, decorateGrammar, compileGrammar, parseGrammarLinks } from 'meta-language';

const host = parseGrammarLinks(`
(grammar (start document))
(rule document normal (seq (literal prefix) (capture labeled value (literal 7)) (literal suffix)))
`);
const number = parseGrammarLinks(`
(grammar (start number))
(rule number normal (literal 7))
`);
const policy = new DecoratorSet([{
  id: 'select-value', level: 'grammar-rule',
  when: [['name', 'document']],
  actions: [{ op: 'set', field: 'composition', value:
    '(any-grammar empty (grammar-field-candidate ExampleNumber field-ranges (source-field-selector value include 0 1)))' }],
}]);
const restored = DecoratorSet.fromLino(policy.toLino());
const sources = new Map([
  ['ExampleHost', decorateGrammar(host, restored)],
  ['ExampleNumber', number],
]);
const parser = compileGrammar(sources.get('ExampleHost'), {
  resolveGrammar: name => sources.get(name),
});
const outcome = parser.parseTree('prefix7suffix');
console.log(outcome.ok, outcome.sourcePlanes.length); // true 2
```

Bind each policy to its intended source grammar before adding that grammar to
the resolver or global archive. A matching local rule name does not select a
global source; resolved children need their own policies applied separately.
Use `parseTree` to retain both the physical CST and the derived source-plane
table. Reapplying the same validated selection does not add another wrapper;
removal or replacement starts from the original grammar.

Authoring refuses composition combined with `executor` or `recovery` levels.
Before publishing a tree with retained mapped source planes, the runtime also
refuses any effective level of either kind. This includes fallback success
retaining attempted mappings, parse-time overrides, archived grammars and
resolved child grammars. Identity-only results retain their existing behavior.
See the authoritative
[vision's decorator contract](vision.md#javascript-first-and-self-translation)
and [concrete syntax requirements](vision.md#concrete-and-abstract-syntax-trees).
Native compilation and runtime parity require CI verification.

## Making translated output match hand-written code

The translation-rule and emitter levels exist so a generated translation can
be brought to match hand-written source without editing the translator: a
decorator rewrites the template or the emitted line that differs, and
removing it restores the generic output. The `bare-await` and
`spaced-definition` decorators of `parity/decorators/levels.lino` are
minimal examples. Self-translation applies them too:
`parity/self-translation/decorators.lino` brings the Rust translation of the
corpus's `arithmetic.mjs` to `parity/self-translation/hand-written/arithmetic.rs`
function by function, and the per-module report measures every module with
and without that set ([self-translation](self-translation.md)).

## Tests

`js/tests/decorators.test.js` and `rust/tests/unit/decorators.rs` run every
hook with `parity/decorators/levels.lino` and compare the runtime-neutral
view recorded in `parity/decorators/cases.json`. They also check composition
order (`composition.lino`), the Links Notation round trip, removal, and the
rejected decorations. Both record the `I195-DECORATORS-EVERY-LEVEL` ledger
row.
