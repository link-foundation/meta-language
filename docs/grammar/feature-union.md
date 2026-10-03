# Grammar feature union: representation and native executor

> This document is subordinate to the authoritative
> [vision and architecture specification](../vision.md). It specifies the
> representation and the executor behind requirement row
> `I195-GRAMMAR-FEATURE-UNION` ([grammar feature union](../vision.md#grammar-feature-union),
> [concrete and abstract syntax trees](../vision.md#concrete-and-abstract-syntax-trees)).
> Where the two disagree, the vision is the contract and this page is a defect to fix.

Status: both packages implement this page. The JavaScript package does so in
[`js/src/grammar-feature-forms.js`](../../js/src/grammar-feature-forms.js),
[`js/src/grammar-runtime.js`](../../js/src/grammar-runtime.js) and
[`js/src/grammar-runtime/`](../../js/src/grammar-runtime/); the Rust crate in
[`rust/src/grammar/feature.rs`](../../rust/src/grammar/feature.rs),
[`rust/src/grammar/interchange/`](../../rust/src/grammar/interchange/) and the
native executor [`rust/src/grammar/feature_runtime/`](../../rust/src/grammar/feature_runtime/)
(`compile_feature_grammar`, `FeatureGrammarParser`), which uses no parser
generator. The shared fixture
[`parity/fixtures/grammar-feature-union.json`](../../parity/fixtures/grammar-feature-union.json)
is the executable check both ports pass byte for byte
([`js/tests/issue-195-grammar-feature-union.test.js`](../../js/tests/issue-195-grammar-feature-union.test.js),
[`rust/tests/unit/issue_195_grammar_feature_union.rs`](../../rust/tests/unit/issue_195_grammar_feature_union.rs)
and
[`rust/tests/unit/issue_195_grammar_feature_union_interchange.rs`](../../rust/tests/unit/issue_195_grammar_feature_union_interchange.rs)).
Its `interchange` section covers grammar merging and lowering: a merge keeps
the declarations and rule fields (it unites imports, modes, extras and
conflict groups, keeps the first matching, macro and scanner of a name and
reports a different later one as a `declaration-conflict`), and a lowering
carries them as `declarations` and `attributes` metadata steps, which makes it
`approximate` because no lowered executable honors them.

Known gaps: the merge of both packages refuses rule bodies that use the
feature forms (`unsupported grammar expression kind`). In the Rust crate,
grammar inference, sampling and recognition do not evaluate the feature forms,
the link-network codec does not carry rule fields or declarations, and the
older pest-based `GrammarParser` only diagnoses the feature forms instead of
running them.

## Contents

- [Feature map](#feature-map)
- [Representation](#representation)
- [Native listing](#native-listing)
- [Links form](#links-form)
- [Operation language](#operation-language)
- [Loading](#loading)
- [Executor](#executor)
- [Syntax tree](#syntax-tree)
- [Tree notation](#tree-notation)
- [Rejections, options and resource limits](#rejections-options-and-resource-limits)
- [Fixture and tests](#fixture-and-tests)

## Feature map

Each bullet of the vision's feature list is one fixture feature. The forms
column names the representation the fixture grammar for that feature must use
(the test checks their presence); the cases column counts the positive and the
negative cases of the fixture (load-error negatives in parentheses). Every
feature also carries one grammar mutation whose outcome must differ.

| Fixture id | Vision feature | Forms exercised | Positive | Negative |
| --- | --- | --- | --- | --- |
| `alternatives` | ordered and unordered alternatives | `choice` ordered and unordered | 3 | 2 |
| `recursion` | recursion, including left recursion | direct and indirect left recursion | 3 | 4 |
| `precedence` | precedence and associativity | `precedence` | 5 | 2 |
| `ambiguity` | ambiguity and conflicts | `conflict` declaration, `dynamicPrecedence` | 3 | 2 |
| `lexical` | lexical priority and longest-match rules | `longest`, `lexicalPrecedence` | 2 | 2 |
| `unicode` | Unicode and byte classes | class `category` and `script` items, `byteClass` | 2 | 3 |
| `trivia` | token boundaries and trivia | `extra` declaration, `token`, `immediateToken` | 2 | 3 |
| `modes` | lexer modes, channels and state | `mode` declaration, rule `modes` and `channel`, `pushMode`, `popMode` | 2 | 3 |
| `layout` | indentation and layout | `scanner` declaration, `emit`, `push`, `pop`, `predicate` | 3 | 3 |
| `predicates` | context-sensitive predicates | `predicate`, rule `action`, `fieldText` | 3 | 2 |
| `actions` | semantic actions and attributes | rule `action`, `setAttribute`, `buildNode`, `sumOf` | 3 | 2 |
| `fields` | captures, fields and aliases | `capture`, `alias` | 2 | 2 |
| `parameterization` | parameterization | rule `parameters`, `parameter`, `ref` arguments | 2 | 4 (2) |
| `imports` | imports and inheritance | `import` declaration | 2 | 4 (2) |
| `macros` | macros and notation | `macro` declaration, `expand` | 2 | 4 (2) |
| `embedded` | embedded languages | `embed` | 2 | 3 (1) |
| `recovery` | error and missing nodes, and recovery | `recover`, `missing` | 3 | 3 |
| total | | | 43 | 48 (7) |

## Representation

A grammar is `Grammar(start, rules, sourceFormat, declarations)`;
`grammar.normalized()` (and so `serializeGrammar`) carries `declarations` next
to the rules, and drops empty declaration lists so a grammar without any keeps
its earlier shape.

### Expressions

The base kinds are unchanged: `empty`, `any`, `literal`, `literalInsensitive`,
`ref`, `seq`, `choice` (`ordered: boolean`), `optional`, `repeat0`, `repeat1`,
`repeat` (`min`, `max` or `null`), `and`, `not`, `capture` (`label`),
`charRange`, `charClass` and `regex` (the last only from importers). The
feature union adds:

| Kind | Fields | Meaning |
| --- | --- | --- |
| `ref` | `name`, `arguments?` | a rule call; `arguments` instantiate a parameterized rule |
| `charClass` items | `{kind: 'category', value}`, `{kind: 'script', value}` | a Unicode general category (`Lu`, `L`, `Nd`) or script (`Greek`, `Han`) |
| `byteClass` | `negated`, `items` of `{kind: 'byte', value}` and `{kind: 'byteRange', start, end}` | one byte, 0 to 255 |
| `precedence` | `level` (integer), `associativity` (`left`, `right` or `none`), `item` | static precedence of the item's binary shape |
| `dynamicPrecedence` | `level`, `item` | adds `level` to the score that settles an ambiguity |
| `lexicalPrecedence` | `level`, `item` | the priority a `longest` tie uses |
| `longest` | `items` (at least one) | the longest match among the alternatives, as one token |
| `token` | `item` | the item's longest match as one token, after trivia |
| `immediateToken` | `item` | as `token`, with no trivia before it |
| `alias` | `name`, `item` | renames the item's node or token |
| `parameter` | `name` | a parameter of the enclosing rule or macro |
| `predicate` | `item`, `condition` | keeps the item's results the condition accepts |
| `recover` | `item`, `synchronize` | on failure, skips to `synchronize` and records an ERROR node |
| `missing` | `item` | on failure, records a zero-width MISSING node |
| `embed` | `language`, `item` | parses the item's region with another grammar |
| `expand` | `name`, `arguments` | a macro call, expanded at load |

Integers are JavaScript safe integers written with at most 15 digits and an
optional `-`.

### Rules

A rule is `{name, kind, expression, parameters?, channel?, modes?, action?}`.
`kind` is `normal`, `token`, `atomic` or `silent` (`nonterminal` and
`terminal` load as `normal` and `token`). `parameters` is a non-empty list of
distinct names. `channel` other than `default` makes the rule trivia (below).
`modes` lists the lexer modes the rule may match in. `action` is a statement
list of the [operation language](#operation-language).

### Declarations

`declarations` holds, each optional:

- `matching`: `generalized` (every alternative, the default) or `peg` (first
  and greedy; the default when `sourceFormat` is `peg`);
- `imports`: names of grammars whose rules and declarations this one inherits;
- `modes`: the lexer modes beyond `default`;
- `extras`: trivia expressions allowed between any two tokens;
- `conflicts`: groups of rule names whose ambiguity is expected and not reported;
- `macros`: `{name, parameters, expression}`;
- `scanners`: external scanners, `{name, tokens, operations}`; each token is
  referenced as `ref(TOKEN)` like a rule.

## Native listing

The line listing of `renderNativeGrammar` and `parseNativeGrammar` writes, in
this order: an optional `format TAG` line, `start NAME`, then the declaration
lines `matching M`, `import NAME`, `mode NAME`, `extra EXPRESSION`,
`conflict NAME NAME...`, `macro NAME(P, ...) = EXPRESSION` and
`scanner NAME tokens(T, ...) operations(OPERATION, ...)`, then one rule line
per rule:

```text
rule NAME(P, ...) = KIND EXPRESSION channel(NAME) modes(MODE, ...) action(OPERATION, ...)
```

The parameter list and each attribute are present only when set, attributes
in the order shown. A name renders bare unless it is empty or contains a stop
character, in which case it is a JSON string. A feature form renders as
`head(field, field, ...)` with fields in the order of the table above, texts as
JSON strings, and a form or operation with no fields as its bare head
(`advance`, `column`, `popMode`). Lists (`longest`, `expand` arguments, `all`,
`some`) are the trailing fields. A `byteClass` is
`byteClass(byte(N), byteRange(A, B))` or `notByteClass(...)`; class items add
`category("Lu")` and `script("Han")`; a call with arguments is
`ref(NAME, ARGUMENT, ...)`. Blocks are `then(...)`, `else(...)` and `do(...)`.
Example from the fixture:

```text
start s
mode string
rule open = token literal("\"") action(pushMode(string))
rule close = token literal("\"") modes(string) action(popMode)
rule space = token repeat1(literal(" ")) channel(hidden) modes(default)
```

## Links form

`renderGrammarLinks` and `parseGrammarLinks` write one link per line. The
header is `(grammar (format F) (start N) (matching M))`, each part only when
set. Declaration links follow, before the first rule (a declaration after a
rule is rejected), in the listing order:

```text
(import NAME)
(mode NAME)
(extra EXPRESSION)
(conflict NAME NAME ...)
(macro NAME (parameters P ...) EXPRESSION)
(scanner NAME (tokens T ...) (operations OPERATION ...))
```

A rule is `(rule NAME KIND EXPRESSION (parameters ...) (channel N) (modes M ...) (action OPERATION ...) (doc TEXT))`,
the optional parts in exactly that order. A feature form is `(head field ...)`,
a form without fields is its bare head, and names and texts are
percent-encoded (`[A-Za-z0-9._-]` stay, every other UTF-8 byte is `%XX` in
upper case, the empty text is `%`). Integers are written as is, negative ones
with `-`. Additions to the earlier links form: `(ref NAME ARGUMENT ...)`,
class items `(category V)` and `(script V)`, and
`(byteClass plain|negated (byte N) (byteRange A B) ...)`. Blocks are
`(then ...)`, `(else ...)` and `(do ...)`. For example the layout scanner of the
fixture begins:

```text
(scanner layout (tokens newline indent dedent) (operations (if (valid newline) (then (consume (literal %0A)) ...
```

The listing, the links form and the serialized JSON all round-trip every form:
`parse(render(g))` yields a normalized grammar equal to `g`, and rendering it
again yields the same text. Both codecs are driven by one table,
`FEATURE_EXPRESSION_FORMS` and `OPERATION_FORMS` in
[`grammar-feature-forms.js`](../../js/src/grammar-feature-forms.js), which a
port should transcribe rather than re-derive.

## Operation language

External scanners, semantic actions and predicate conditions are written in a
small, deterministic operation language. An operation is plain data,
`{operation: NAME, ...fields}`; there are no host callbacks, no `eval`, no
source text and no unbounded host computation: every operation costs a step
of the [step budget](#rejections-options-and-resource-limits).

Statements (run in order; a statement list ends early on `emit`):

| Operation | Fields | Effect |
| --- | --- | --- |
| `advance` | | move the scanner cursor one code point (or one stray byte); fails at the end |
| `consume` | `item` | move the cursor past the longest match of `item`; fails without one |
| `skip` | `item` | as `consume`, but only before anything was consumed; the skipped text becomes a trivia leaf and the token starts after it |
| `mark` | | the token ends here, whatever is consumed later |
| `emit` | `token` | stop and produce `token` |
| `fail` | | fail the run |
| `if` | `condition`, `consequent` (`then`), `alternative?` (`else`) | branch |
| `while` | `condition`, `body` (`do`) | loop |
| `push` | `stack`, `value` | push onto a named stack |
| `pop` | `stack` | pop; fails on an empty stack |
| `set` | `variable`, `value` | assign a named variable |
| `pushMode`, `popMode`, `setMode` | `mode` (not for `popMode`) | change the mode stack; `popMode` fails when one mode is left |
| `setAttribute` | `attribute`, `value` | set an attribute of the node being built |
| `buildNode` | `kind` | rename the node being built |

Conditions: `valid(token)` (the scanner was asked for `token`),
`next(item)` (the item matches at the cursor, consuming nothing), `atEnd`,
`equal(left, right)` (same type and value), `less`, `greater` (integers),
`all(...)`, `some(...)` (short-circuit), `not(condition)`.

Values: `integer`, `text`, `variable(name)` (0 when unset), `top(stack)` (0
when empty), `depth(stack)`, `column` (0-based, in code points since the last
line feed), `matched` (the matched text), `mode` (the current mode),
`attribute(field, attribute)`, `sumOf(field, attribute)`, `fieldText(field)`,
`length(value)` (code points of a text), `number(value)` (a text of at most 15
digits with an optional `-`), `add`, `subtract`, `multiply`. Arithmetic on a
non-integer, or a result outside the safe integer range, fails.

A failing operation (`fail`, a failed `advance`, `consume`, `skip` or `pop`, a
type error, a missing attribute) fails the scanner run, the action or the
predicate; it never throws to the caller.

Each context admits a subset (`OPERATION_CONTEXTS` in
[`operations.js`](../../js/src/grammar-runtime/operations.js)); loading rejects
any other operation with reason `operation`:

- scanner: every statement except `setAttribute` and `buildNode`; every
  condition; every value except `matched`, `attribute`, `sumOf` and `fieldText`;
- action: every statement except `advance`, `consume`, `skip`, `mark` and
  `emit`; every condition except `valid` and `next`; every value;
- predicate: conditions except `valid` and `next`, and values except
  `attribute`, `sumOf` and `fieldText`.

### Parser state

Every match threads an immutable state: the mode stack (initially
`['default']`), named stacks of values (empty stacks dropped) and named
variables. Its canonical key is the JSON text of
`[modes, stacks sorted by name as [name, values] pairs, variables sorted by name as [name, value] pairs]`.
Two results with equal end and equal state key are the same result. Scanners
and actions run on a working copy of the state and settle it back when they
succeed; predicates only read it.

## Loading

`createGrammarParser(grammar, options)` (and `compileGrammar`) loads the
grammar once; a load error is a `GrammarRuntimeError` with a `reason`. The
steps, in order:

1. The grammar may be a `Grammar`, its normalized document or that document's
   JSON text.
2. Imports: for each name in `imports`, `options.resolveGrammar(name)` gives
   the grammar, loaded recursively. Imported rules come first in import order,
   local rules replace imported ones of the same name in place, `modes`,
   `extras`, `conflicts` and `scanners` accumulate, and a local macro replaces
   an imported one. A cycle, or a name that cannot be resolved, is `import`.
3. Macros expand inline: `expand(NAME, ARG...)` becomes the macro body with
   each `parameter(P)` replaced by the expanded argument. An undefined macro,
   a recursive expansion or a wrong argument count is `macro`.
4. Every scanner token must be declared once and must not also be a rule
   (`declaration`).
5. The modes are `default` plus the declared modes.
6. Parameterized rules are instantiated per distinct argument list: an
   instance is the rule `NAME(ARG, ...)` (each argument as its JSON text) whose
   nodes keep the kind `NAME`. A wrong argument count, an unbound
   `parameter`, arguments to a scanner token or more than 1000 instances is
   `parameter`; an undefined rule is `reference`.
7. The start rule must exist (`reference`) and take no parameters
   (`parameter`).
8. Conflicts must name rules (`declaration`). `matching` is the declared one,
   else `peg` for a `peg` source format, else `generalized`.
9. Checks over every expression and operation: an `expand` or `parameter`
   that survives is `macro`; a `recover` without `synchronize`, a mode
   operation or rule `modes` naming an undeclared mode is `declaration`; an
   operation outside its context, an `emit` or `valid` naming a token the
   scanner does not declare, and a `setAttribute` or `buildNode` in a
   `silent` rule's action is `operation`; an unknown Unicode category or
   script, or an invalid regular expression, is `pattern`.
10. Trivia: the `extras` in order (a `ref` extra's leaves take the rule name as
    kind, others have none), then every rule whose `channel` is not `default`,
    limited to that rule's `modes`.
11. A rule whose expression is `lexicalPrecedence` has that level as its
    lexical priority (0 otherwise).
12. Each embedded language is resolved with `options.resolveGrammar(name)`
    (`embed` when it cannot be) and loaded in the same context, once.

## Executor

The executor ([`executor.js`](../../js/src/grammar-runtime/executor.js)) is a
memoizing interpreter over the input bytes. Every offset is a byte offset;
text is UTF-8, and a byte that does not start a well-formed sequence is one
unit with no code point. Evaluating an expression at a position, in a state,
in syntactic or token context yields a list of results
`{end, state, children, dynamic, precedence, ambiguous}`.

**Result sets.** Results are deduplicated by `end` and state key. Of two equal
results the one with the higher `dynamic` stays; on a tie the first stays and
is marked ambiguous. Under `matching peg` a sequence keeps only its first
result.

**Terminals.** In syntactic context a terminal first skips trivia; in token
context it does not. Matchers: `literal` compares UTF-8 bytes;
`literalInsensitive` compares the per-code-point lower case of as many code
points as the literal has; `charRange` and `charClass` test one code point
(category and script items by the Unicode property); `byteClass` tests one
byte; `any` takes one code point or stray byte; `regex` runs a JavaScript
regular expression (sticky, Unicode when it compiles so) over the UTF-16 view
of the input with stray bytes as U+FFFD. A failed terminal records an
expectation at its start: `"lit"`, `"lit"i`, `"a".."z"`, `character class`,
`byte class`, `any character` or `/re/`.

**Trivia.** Skipping trivia repeatedly takes the longest match, in token
context, of any trivia expression allowed in the current mode (the top of the
mode stack); the first expression wins a tie. Each match is a trivia leaf
(`trivia: true`) placed before the next leaf. Skips are memoized by position
and state.

**Sequences and choices.** A sequence joins every left result with every
result of the next item. An ordered choice returns the results of the first
alternative that has any. An unordered choice under `peg` returns the longest
first result among the alternatives (the first on a tie); under
`generalized` it returns the union of all alternatives.

**Repetition.** Under `peg` a repetition is greedy and possessive, and a
zero-width iteration ends it. Under `generalized` it is a breadth-first
frontier by iteration count: once `min` is met each new end and state is a
result, a result reached again marks ambiguity and is not extended, and a
zero-width iteration only pads up to `min`.

**Lookahead.** `and` and `not` evaluate the item without recording
expectations and yield a zero-width result.

**Captures and aliases.** `capture` sets `field` on each non-trivia child.
`alias` renames the only non-trivia child, or wraps several in a node of the
alias name.

**Precedence.** `precedence(level, associativity, item)` filters the item's
results before they are merged: a result whose first or last non-trivia child
is a node tagged with an inner level is invalid when the inner level is lower,
or equal while the associativity is not `left` (first child) or not `right`
(last child). A rejected result records the expectation `precedence`. Kept
results are tagged with the level and associativity, and a rule node carries
the tag of its result. `dynamicPrecedence` adds its level to `dynamic`.
`lexicalPrecedence` matches its item and only matters to `longest`.

**Tokens.** `longest` skips trivia, then takes the longest token-context match
of its alternatives; a tie goes to the higher lexical priority (the level of a
`lexicalPrecedence` alternative or of a rule's lexical priority), then to the
first. The leaf's kind is the winning rule's node kind for a `ref`, else none.
`token` skips trivia and makes one leaf of the item's longest token-context
match; `immediateToken` does the same without skipping trivia.

**Predicates.** `predicate(item, condition)` keeps the item's results whose
condition holds; `column` and `matched` are measured from the first
non-trivia child. When none is kept it records `predicate` at the position
after trivia.

**Recovery.** `recover(item, synchronize)` returns the item's results when it
has any. Otherwise it skips trivia and then advances at least one code point
until `synchronize` matches (without consuming it) and records the skipped
bytes as an ERROR node; it fails when the end is reached first. `missing(item)`
returns the item's results, or else a zero-width MISSING node whose kind is
the rule's node kind for a `ref`, the literal text for a `literal` (marked
`literal`), and none otherwise.

**Embedded languages.** `embed(language, item)` skips trivia, matches the
item in token context and parses each region with a nested executor for the
embedded grammar over the same bytes, so offsets stay absolute. The nested
parse shares the step budget, has the remaining depth, starts in the initial
state and must cover its region. Outcomes are memoized by language and region.
A failed region forwards its expectations.

**Rule calls.** A `ref` to a scanner token runs the scanner. A rule whose
`modes` exclude the current mode fails, recording its name. Otherwise the call
is memoized by rule, position, state key and context. Left recursion is
handled by seed growing: a call that meets itself in progress answers with the
current seed, and the calls in between are not memoized; the outer call then
grows the seed. Under `peg` it repeats while the end increases; under
`generalized` it merges results until no new end and state appears. Each
call past `maxDepth` nesting aborts the parse.

**Rule bodies.** A `token` or `atomic` rule skips trivia and evaluates its
expression in token context without recording inner expectations; a `token`
rule, or any rule under `peg`, keeps the longest result; each result becomes a
leaf of the rule's node kind, and when none survives the rule's name is
recorded as the expectation. A `normal` rule wraps each result in a node of
its kind. A `silent` rule, and any rule in token context, splices its
children into the caller. A rule's action runs on each result's leaf or node:
`attribute`, `sumOf` and `fieldText` select the direct non-trivia children
captured under the field name, or else those of that kind; `fieldText` of a
node starts at its first non-trivia child; `setAttribute` and `buildNode`
change the fresh node; a failing action drops the result; the result's state
becomes the action's settled state.

**Scanners.** A scanner runs once per token name, start position (after
trivia) and state key. Its cursor and token start begin at the position;
`next`, `consume` and `skip` match in token context; `column` is the cursor's
column. The run succeeds only when it ends with `emit` of the requested token;
the token spans the token start to the mark, or to the cursor without a mark,
and the leaves are the skipped trivia followed by the token leaf.

**Whole parse.** The start rule is called at offset 0 in the initial state.
Each result skips trailing trivia and must reach the end of the input, or it
records `end of input`. The first complete result is the tree; the parse is
ambiguous when several results complete or that result is ambiguous. The root
is the result's single node, or else a node of the start rule's kind around
its children; trailing trivia is appended to it. A failed parse reports the
farthest offset any expectation was recorded at and the expectations recorded
there, sorted by UTF-16 code units.

**Automatic recovery.** With `errorRecovery: true` a failed parse is repaired
without recovery rules in the grammar. An *element* is a terminal, a `token`
or `immediateToken`, a `longest` choice, a token or atomic rule, or a scanner
token, matched outside a token. Each result carries a repair cost, the sum of
its repairs; of two results with the same end and state the cheaper one
stays, and only results without repairs are marked ambiguous. The parse runs
in rounds. Each round notes the farthest offset after trivia at which an
element failed (outside `not`, `and` and other quiet matches, which never
repair), and when no element failed, the farthest offset of the failed whole
parse. That offset becomes a *repair point* and the input is parsed again. At
a repair point, a failing element yields two results:

- a zero-width MISSING leaf, at cost 2. Its kind is the literal for a literal
  (marked `literal`), the node kind for a token or atomic rule, the scanner
  token's name for a scanner token, and none otherwise;
- the element's match at the first later code point boundary where it
  matches, behind an ERROR leaf over the skipped bytes, at a cost of one per
  skipped byte.

Trailing input that a result leaves unmatched becomes an ERROR leaf. The
result counts as complete when that input starts at a repair point. Otherwise
it costs one per byte and is kept as the partial tree. The cheapest complete
result is the tree; on a tie the first one. Rounds stop when a parse
completes, when a round notes no new offset, or after `maxRepairs` repair
points (default 32). The last round's partial tree, the result that reaches
farthest, then stands. When the start rule matches nothing, the root holds one
ERROR leaf over the whole input. Every round has its own step budget. A
repaired tree holds an ERROR or MISSING leaf, so it is reported as
`recovered`. The Rust executor takes the same options as
`FeatureParseOptions::error_recovery` and `max_repairs` and builds the same
trees.

## Syntax tree

The public tree (`SyntaxTreeNode` in
[`index.d.ts`](../../js/src/index.d.ts)) is lossless: concatenating the token,
trivia and ERROR leaves in order reproduces the input bytes.

- `{type: 'node', kind, field?, start, end, children, attributes?}`
- `{type: 'token', kind (or null), field?, trivia?, start, end, attributes?, text, hex?}`
- `{type: 'error', start, end, text, hex?, reason?}`
- `{type: 'missing', kind (or null), start, end, literal?}`
- `{type: 'embed', language, start, end, root}`

`text` is `null` and `hex` holds lower-case hexadecimal when the bytes are not
well-formed UTF-8. Ambiguities are reported as `{rule, start, end}` for each
ambiguous node, in preorder, outside the rules named by `conflicts`.

## Tree notation

`renderSyntaxTree(node)` is the stable text form the fixture records:

- node: `(kind {a=1,b="x"} child ...)`, attributes sorted by name, omitted when absent;
- captured child: `field:` before it;
- anonymous token: its quoted text; named token: `(kind "text")`; unnamed
  token with attributes: `(_ {...} "text")`; trivia: `~` before it;
- `(ERROR@S..E "text")`, `(MISSING@P kind)`, `(MISSING@P "literal")`, `(MISSING@P)`;
- `(EMBED language@S..E root)`;
- invalid UTF-8: `<hex>` in place of the quoted text.

A name is bare when it matches `[A-Za-z_][A-Za-z0-9_-]*` and quoted otherwise.
Quoting is a JSON string with a fixed escaping: `\"`, `\\`, `\b`, `\f`, `\n`,
`\r`, `\t`, every other control character and U+007F as lower-case `\u00xx`,
and everything else literal.

## Rejections, options and resource limits

`parser.parseTree(source, options)` takes a string or UTF-8 bytes and returns
`{ok, tree, ambiguities, rejection}`; `parser.parse` returns the tree or
throws `GrammarParseError` (with `reason`, `offset`, `line`, `column` and
`expected`). Lines and columns are 1-based, columns in code points.
Rejections:

- `{reason: 'syntax', offset, line, column, expected}`: no complete parse;
- `{reason: 'stepLimit', limit}`: the step budget ran out;
- `{reason: 'nestingDepth', limit}`: rule nesting exceeded `maxDepth` (or the
  host stack overflowed); the tree is a single ERROR node over the input;
- `{reason: 'recovered', offset, line, column}`: the tree holds an ERROR or
  MISSING node, unless `recovery: 'accept'`;
- `{reason: 'ambiguity', offset, line, column}`: only with `ambiguity: 'reject'`.

Options, given to the parser and overridable per parse: `resolveGrammar`,
`startRule`, `maxDepth` (default 1000), `stepLimit` (default
100000 + 1000 per input byte, one step per expression evaluation and per
operation, shared with embedded parses), `memoLimit` (default 1000000 memo
entries; past it new entries are dropped rather than kept), `ambiguity`,
`recovery`, `errorRecovery` (default off) and `maxRepairs` (default 32). `compileGrammar` and `parseWithGrammar` run on this executor, and
`emitJavascriptParser` emits a module that calls
`compileGrammar(deserializeGrammar(GRAMMAR))`; the package no longer runs
Peggy, which is only a development dependency of tests that compare against it.

## Fixture and tests

[`parity/fixtures/grammar-feature-union.json`](../../parity/fixtures/grammar-feature-union.json)
lists, per feature, a native listing, optional parser options, positive cases
(`input` or `inputHex`, and the expected `tree` in the tree notation),
negative cases (a `rejection`, with the `tree` and `ambiguities` when one was
built, or a `listing` with the expected `loadError` reason) and a `mutation`
(`from`, `to`, `input`, `before`, `after`). `languages` holds the grammars
that imports and embeds resolve. It is written by
[`experiments/issue-195-grammar-feature-union-fixture.mjs`](../../experiments/issue-195-grammar-feature-union-fixture.mjs)
and reviewed by hand.

[`js/tests/issue-195-grammar-feature-union.test.js`](../../js/tests/issue-195-grammar-feature-union.test.js)
checks the representation and round trips, runs every case through the
listing and the links form, checks the negatives and the mutations, and checks
that scanners and actions run from their links form with no host code. A Rust
test against the same fixture is open work.
