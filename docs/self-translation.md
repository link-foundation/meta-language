# Self-translation

Self-translation ports meta-language's own modules between JavaScript,
TypeScript and Rust with meta-language itself. Both packages have the same
library API and the same CLI command, and their results are the same byte for
byte:

```js
import { selfTranslate } from 'meta-language';
const { code, items } = selfTranslate(source, 'JavaScript', 'Rust');
```

```rust
let translation = meta_language::self_translate(&source, "Rust", "JavaScript")?;
```

```sh
meta-language translate --to rust [--from js] [--items] module.mjs
```

A language is named by name or extension (`rust`, `rs`, `ts`, `mjs`); without
`--from`, the CLI takes it from the file extension. `--items` prints one line
per top-level item instead of the translation.

The current draft also provides `--semantic`, or
`selfTranslate(source, from, to, { semanticTranslation: true })`. Rust selects
`SelfTranslationOptions.semantic_translation`. This reads the actual emitted
code through the ordinary frontend, including its runtime preludes. Saved
originals stay provenance and carried blocks remain explicit implementation
gaps. The default continues to support lossless source restoration. Execution
and parity checks for this new mode are deferred until the full requirement
scope is drafted.

With `semanticTranslation: true`, `selfTranslationSignatures` also reads
unrestricted public free functions from a complete Rust module. The ordinary
Rust frontend checks every body and sibling dependency together, and the
ordinary JavaScript emitter must accept the result. The original concrete
grammar selects actual function owners; the shared generated token policy
selects their public name after attributes. Restricted visibility and external
signature assumptions retain explicit namespace-composition obligations.
The semantic draft can also emit complete flat Rust free-function libraries as
one checked scope, preserving private/forward helpers and actual public export
aliases. Project directory metadata remains available and does not disable this
source-module path. Effectful entry points and other unsupported source items
keep their explicit gaps.

`selfTranslateWithDiagnostics` and Rust `self_translate_with_diagnostics` retain
actual compiler diagnostics beside the unchanged source owners. JavaScript's
`referenceSemantics: true` and Rust's `reference_semantics: true` select shared
reference values across the complete module; false keeps automatic selection
from source operations, inferred identities and imported types. Native context
propagation is a code draft; behavior remains untested in this phase.

`draftProjectTranslation(inputs, { targetLanguage })` drafts a complete supplied
module inventory. Each input supplies its identifier, source language, complete
source and import-specifier-to-source-identifier map. Source hashes, dependency
refinement, products and every unsupported item remain in the returned draft;
`projectTranslationDraftToLinks` stores it as inspectable Links data. Rust exposes
`draft_project_translation` and `project_translation_draft_to_links` over the
same source inventory and generated worklist decisions. The
`js/scripts/draft-project-translation.mjs` author materializes every tracked
JavaScript, TypeScript and Rust source of a supplied project and writes that
draft. This author is intended for the later CI drafting phase; it performs no
product execution. Remaining language and library gaps retain their full scope.

The library can explicitly read actual standard Rust shared-array handles:

```js
const options = {
  rustSharedArrayHandleSourceModel: 'standard Rust shared array identity handles 1',
};
const result = selfTranslate(rustLibrarySource, 'Rust', 'JavaScript', options);
```

Rust selects the same model through
`SelfTranslationOptions.rust_shared_array_handle_source_model`. This finite
profile preserves fresh `Rc<RefCell<Vec<scalar>>>` allocations, explicit clone
aliases and `Rc::ptr_eq`, with actual fully qualified standard owners. Whole
libraries retain private/forward siblings and concrete `pub` visibility. Both
Links codecs retain the checked identity domain, and reverse translation
restores the complete source. Ordinary payload equality and complete executable
runtime shells require separate interpretations. The
[vision](vision.md#four-language-semantics-and-translation) defines the complete
boundary; native compilation and runtime parity run in CI.

Functions beside literal constant arrays/objects, literal standard Map/Set
constructors, and `Object.freeze` on literal containers can retain their real
module binding context. The complete CST proof rejects eager calls, getters,
computed keys, spreads and shadowed standard owners. It checks the neighboring
function normally and preserves the original initializer for exact reverse
translation. That initializer still needs complete executable translation.

Empty standard `Map` and `Set` module constants also retain their binding
context, including `new Map()` and `new Set` without argument parentheses.
Shadowed constructors and evaluated arguments refuse. Unsupported neighboring
function bodies remain explicit translation obligations.

The public `projectTypeScriptTyping(source)` JavaScript function and
`project_type_script_typing(&source)` Rust function return the same JavaScript
source string with supported TypeScript function and alias annotations retained
as JSDoc. Rust returns a `Result` and JavaScript throws on an unsupported source
construct; both preserve the shared refusal reason. This projection feeds the
existing typed frontend. It does not establish support for arbitrary TypeScript
types or runtime constructs, and does not retain the complete source provenance
provided by `selfTranslate` and `self_translate`.

Explicit type modifiers in mixed named imports erase through the same linked
rules in both packages. Runtime specifiers and default imports remain, including
an empty named import's module evaluation; whole `import type` statements erase
completely. Original TypeScript, comments and byte spans remain in restoration
provenance. This uses the explicit modifier behavior of
[verbatim module syntax](https://www.typescriptlang.org/tsconfig/verbatimModuleSyntax.html);
it does not infer whether an unmarked import is used only as a type.

An owned function in an existing checked program can also opt into the closed
Boolean callable model. Select its complete recursive closure first:

```js
const selected = encodeFormalFunctionDefinition(linkedProgram, declarationName);
const programRoot = selected.network.link(selected.root).references()[0];
const definition = encodeFormalConceptDefinition(
  { network: selected.network, root: programRoot },
  { model: 'closed boolean function value' },
);
```

This retains the original linked program and source context. The explicit
Boolean argument contract and its separate namespace observation are described
in the [authoritative vision](vision.md#recursive-formal-concept-definitions).

Supplemental documentation parameter patterns encode each literal comment
terminator `*/` as the visible text `*\/`. The executable parameter pattern is
unchanged. Internal projection records retain that original text and its UTF-8
byte span separately from the documentation text. This display encoding has no
inverse claim: a pre-existing escaped slash can have the same documentation
spelling, while its authoritative executable source remains distinct. Encoding
the display does not establish semantic typing support for arbitrary destructured
parameters; unsupported type annotations keep their existing source obligation.

A supported TypeScript function's first annotated `this` parameter erases with
its following comma, preserving runtime arity and source restoration. Portable
Rust projection admits `this: void`; other receiver types retain a precise
obligation. Methods, constructors and arrow receiver parameters remain outside
this erasure profile. The [vision](vision.md#javascript-first-and-self-translation)
records the interpretation and remaining boundaries.

## The output

A translation starts with a header that records the source language and the
SHA-256 and length of the source:

```
// meta-language:self-translation:v1 source=JavaScript target=Rust sha256=… bytes=508
```

Each top-level item of the source then becomes one block:

- **translated**: the item went through links into the target language. Its
  source is kept above the code as `// |` lines, with the item's own hash:
  `// meta-language:translated JavaScript export_statement items=1 sha256=…`.
- **carried**: the translator cannot express the item in the target yet (it
  has no definition, or uses a construct the portable core does not cover).
  The item is kept as `// |` lines only, with the reason:
  `// meta-language:carried JavaScript import_statement (no definition)`.
  A carried item is a gap in the translator, reported in the item list and in
  the per-module report.
- **comment**: comment groups between items are copied as comments when the
  syntax fits both languages. A comment directly before an item travels with
  that item.

An isolated JavaScript declaration that fails type resolution is retried with
its contiguous declaration run in one checked scope when targeting Rust. A
successful run has one provenance block covering all of its source items;
forward calls and arrow-bound siblings use their actual checked declarations.
A failed run retains the original per-item decisions, so a carried sibling
never acquires a placeholder signature or implementation. Captures of mutable
or non-literal globals and bindings across intervening imports or statements remain separate
translation obligations.

The definitions the translated code needs (helpers and, for Rust, an
`#![allow(...)]` line) sit once between `// meta-language:prelude begin` and
`// meta-language:prelude end`.

## Round trips

Translating back reads the provenance:

- a translated or carried block whose code still matches its hash is restored
  to its source text exactly (status `restored`);
- an edited block drops its provenance and is translated again (status
  `translated`), so edits made in the target language carry over;
- when the header names the target language and the hash of the restored text
  equals the header's, the result is the original source byte for byte.

So JavaScript → Rust → JavaScript and Rust → JavaScript → Rust are lossless
for unedited output, and a translation into the same language returns the
source unchanged. Behavior across languages is checked by calling the
translated functions in both runtimes (see below).

## The shared corpus

`parity/self-translation/cases.lino` lists the cases and calls in Links
Notation, in the manner of
[relative-meta-logic PR #184](https://github.com/link-foundation/relative-meta-logic/pull/184):

```
(case arithmetic-to-rust (source sources/arithmetic.mjs) (from JavaScript) (to Rust) (expected expected/arithmetic-to-rust.rs))
(call arithmetic-to-rust (javascript add) (rust add) (arguments (f64 2) (f64 3.5)) (result 5.5))
```

Each case has its expected output and an `.items.lino` file with one
`(item start end term status "reason")` link per item. Both runtimes check
every case (`js/tests/self-translation.test.js`,
`rust/tests/unit/self_translation.rs`). The Rust test compiles the translated
Rust with clippy at `-D warnings` and runs each call; the JavaScript test
imports the JavaScript side and runs the same calls. Meta-language's own
modules round-trip byte for byte in both tests.

`npm run check:self-translation` checks that the expected files match the
translator; `node js/scripts/generate-self-translation-cases.mjs` rewrites
them after an intended change.

The Unicode-escape and default-parameter decisions in
`js/src/translation/frontend-rules.js` also run inside the Rust translator.
`node js/scripts/generate-frontend-rules.mjs` translates the complete module
through the portable core, applies the shared emitter decorators in
`parity/self-translation/frontend-rules-decorators.lino`, and formats the
result as `rust/src/translation/frontend_rules.rs`. The adapters supply UTF-16
code units and token records to these generated decisions. CI runs
`npm run check:frontend-rules` to reject stale generated code.

`node js/scripts/generate-frontend-rule-styles.mjs` derives style decorators
from machine-applicable Clippy suggestions on temporary copies of the generated
module. It records line context around each replacement, then regeneration
applies formatting and decorators to a fixed point. Run the frontend generator
again after deriving styles. CI also runs `npm run check:frontend-rule-styles`
to compile these generated modules with the crate's lint groups and warnings
denied, without downloading or building the crate's dependencies.

## Translated against hand-written Rust

`js/scripts/generate-self-translation-report.mjs` translates every module of
`js/src` into Rust and measures it against its hand-written counterpart
(`rust/src/<module>.rs` or `rust/src/<module>/mod.rs`, with grammar runtime
hosts under `rust/src/grammar/feature_runtime`): the items by status,
the Rust functions written, how many the hand-written Rust defines under the
same name and how many are identical up to whitespace, and how many translated
code lines the hand-written Rust holds too. The report also accounts for every
UTF-8 source byte using the item ranges and whitespace between them, and lists
translated, carried, comment and layout byte counts. It rejects overlapping,
out-of-bounds or split UTF-8 ranges. A module with code outside those ranges is
listed as refused, including when a parser fallback returned no items, so zero
carried items cannot conceal omitted source. Each module is measured twice: by
the generic translation and by the translation with the shared emitter
[decorators](decorators.md) of `parity/self-translation/decorators.lino`
(`--decorators` names another set), so the report shows how much of the
difference the decorators close. The whole of `js/src` takes over an hour on
one runner, so `--shard K/N` measures one of N shards that hold about the same
number of source bytes, and `--list` prints a shard's modules. CI's
`Self-Translation Report` jobs in `ci.yml` run eight shards beside the
JavaScript tests. Each shard publishes its part of the report in
its job summary and as a `self-translation-report-<sha>-<shard>` artifact.
The Links report records the observed commit and the SHA-256 hashes of each module and
its Rust counterpart. Both acceptance suites download all eight shards and run
`--verify-reports <directory> --commit <sha>` before recording the per-module
report assertion. Verification rejects missing or duplicate modules, stale
sources, failed translations, inconsistent measurements and missing Markdown
rows without translating the whole corpus again. A separate `report-artifacts`
JavaScript test group checks these published measurements on the producers'
exact checkout before Rust and delivery stages start. It retains the per-module
source coverage and generic/decorated difference assertions. Standalone runs
of `js.yml` produce the same eight shards and run the same assertion group.

The report resolves relative named imports against the source tree and passes
the providers' checked translated-export signatures and each module's directory
to both measurements. It caches source reads, signatures and dependency contexts
within a shard. Missing providers, unsupported exports and unresolved cycles
retain the translator's carried-item diagnostics.

The same measurements retain each item's exact range, concrete syntax term and
actual translation error. Report-only diagnostic capture preserves the ordinary
library output and adds no translation pass. Analyze existing shard artifacts
without translating their source again:

```sh
node js/scripts/self-translation-report-blockers.mjs --report shard-1.lino --report shard-2.lino --out blockers.lino
node js/scripts/self-translation-report-blockers.mjs --report shard-1.lino --report shard-2.lino --group GROUP_KEY --out replay.lino
node js/scripts/generate-self-translation-report.mjs --replay-plan replay.lino --out-dir selected-report
```

Groups rank distinct affected modules before carried source bytes and retain
every original failure and range. Older reports expose missing diagnostics
explicitly. A replay plan keeps its original commit, source hashes, diagnostics
and denominator; the existing worker measures only its selected unchanged
sources and records the current translator commit separately. Changed sources,
missing modules or incompatible language/mode selections refuse. This report
runner measures JavaScript to Rust; its ranking is not reverse translation or
successful execution evidence.

Decorators edit emitted lines; they cannot add what the translator carries
untranslated (imports, classes, exports the emitters do not cover), so for
most modules the report shows the difference that is left honestly rather
than hiding it.

## Decorated translations

`selfTranslate(source, from, to, { decorators })` (JavaScript) and
`self_translate_decorated(source, from, to, &decorators)` (Rust) apply emitter
decorators to the code they write before the provenance marker is hashed, so
the decorated translation still translates back to its source. The corpus case
`arithmetic-decorated-to-rust` translates `sources/arithmetic.mjs` with
`decorators.lino` and is checked, function by function, against
`hand-written/arithmetic.rs`; removing the decorators gives the generic
translation again.

Sibling functions can also read immutable number, string and boolean literal
constants from their module. Local declarations and parameters take precedence
over those bindings, including a local declaration in its temporal dead zone.
This does not bind mutable values, array or object identities, or imported values.
Rust constant storage decisions and declaration rendering are generated from
the same JavaScript rules.

Literal Number arithmetic constants also provide module ownership facts, from
the complete CST. For example, `const BYTE_LIMIT = 128 * 1024 * 1024` need not
prevent an unrelated checked function from constructing the standard Map.
This classification accepts Number arithmetic without evaluating it, preserves
the actual binding name and refuses identifiers, calls, BigInt and coercion from
other types. The initializer retains its own translation obligation.
When that complete initializer independently translates, its actual checked
Number static signature may bind sibling functions. The existing static emitter
retains the expression, including rounding, NaN and signed zero. Unknown owners,
eager sibling effects and operators the frontend cannot translate still refuse.

A declaration-only module with carried groups is also retried as one checked
scope, so JSDoc data types can bind across declarations. A failed retry keeps
its earlier decisions. Imports, executable statements and stored provenance
blocks are excluded. The frontend decision module itself is checked for zero
carried items, generated Rust execution and exact restoration in both runtimes.
Provenance records definitions, including attributes and documentation that
accompany a declaration. The provenance reader consumes attributes together
with the definition they annotate.

Bracketed JSDoc parameter names (`[value]` and `[value=99]`) now bind the
same declared type as an ordinary `value` parameter. Their default text is
documentation metadata: only the JavaScript function's actual default supplies
an omitted argument. The shared frontend decision is generated into Rust;
checks cover mismatched names, required arguments, execution and restoration.
This does not implement nullable values, tuple or record types, or Map types.

JavaScript `typeof` now translates for typed bindings and primitive literals,
with the JavaScript names `number`, `bigint`, `boolean`, `string` and `object`.
Both runtimes use generated decisions for eligible operands and type names.
Calls, field reads and other operand expressions remain refused: a type query
must not discard their evaluation, effects or exceptions. This is a bounded
extension of #212, not support for dynamic type unions or unbound names.

Homogeneous array `concat` calls lower to the existing checked array spread
representation. Checks cover argument order, one-level spreading for nested
arrays and a zero-argument copy. Scalar arguments and non-array receivers
retain diagnostics; this does not complete the remaining array operations.

A nonempty, childless parser `ERROR` root is retained as a carried syntax item
with its entire byte span. Parser budgets remain unchanged. Reverse translation
can restore an original source prefix when the header's byte length, language
and SHA-256 digest match and only formatting whitespace follows it. Appended
executable code prevents that restoration. Targeted checks cover the repository's
executor module and a source without a final newline. These changes preserve
source coverage; carrying a syntax error is not a semantic translation.

## Items that use other items

From JavaScript or TypeScript into Rust, a module is one scope. Each item is
translated after the items it names, and the signatures of the ones that
translated are bound for it: a function may call a function declared before or
after it (`const f = (x) => …` included) and read a top-level constant. The
Rust calls the function by the name its own translation gives it (`square(x)`,
`capped_square(x)`) and reads the constant by its name (`LIMIT`, `*COMPUTED`
for a `static`). A name whose item is carried, or that a cycle of items leaves
untranslated, stays unbound, so its callers are carried with the checker's
diagnostic. The corpus case `siblings-to-rust` runs such calls in both
runtimes. Both packages translate top-level constants into Rust.

A relative named import of another module of the crate,
`import { a, b as c } from './m.mjs'`, becomes `use crate::m::{a, b as c};`.
The module path comes from the specifier: `.mjs`, `.js` and the TypeScript
extensions are dropped, each segment is a Rust module name in snake case, and
`..` climbs from the module's directory, which the `moduleDirectory` option
(`module_directory` in Rust) gives as path segments; it is the crate root by
default, above which an import is refused. The imported names are bound when
the `imports` option maps the specifier to the signatures
`selfTranslationSignatures(source, language)` (`self_translation_signatures`
in Rust) gives for that module, its exported items that translate:

```js
const math = selfTranslationSignatures(mathSource, 'JavaScript');
selfTranslate(quadSource, 'JavaScript', 'Rust', { imports: { './math.mjs': math } });
```

```rust
let options = SelfTranslationOptions {
    imports: [("./math.mjs".to_owned(), signatures)].into(),
    ..SelfTranslationOptions::default()
};
self_translate_with(&quad_source, "JavaScript", "Rust", &options)?;
```

The `utf16Strings` option (`utf16_strings` in Rust) selects the retained
UTF-16 representation for the module. Exact source member names can also
select it automatically, including `slice` on a numeric array. Both packages
share the selector and the complete emitted prelude as Links data. Native
self-translation currently admits this selection when its checked values and
bound signatures do not require a String representation; other selected items
remain carried with the explicit Text-lowering obligation. This internal
context does not add a field to saved typed-program graphs.

A bound function is named in snake case on both sides of `as`, as its own
translation names it. Without signatures the import still translates and the
items that use its names are carried. Default and namespace imports, Node.js
built-in modules and packages are refused with their own diagnostics; the
corpus case `imports-to-rust` lists them.

The dependency binder runs before the declaration-scope retry. Signatures from
successful retries are returned for the module's eligible exported functions;
imported aliases use the same signature lookup. Cross-module signatures still
exclude algebraic data types and guarded natural-number parameters. Nonliteral
constant captures stay unbound: lazy initialization does not establish eager
JavaScript initialization order or effects. The shared generated eligibility
rule is used by both binders.

Implicit module reference selection uses the complete frontend's checked owned
array rewrites before forcing a reference representation from method syntax.
Explicit reference requests and imported reference signatures still select that
representation. A refused complete parse retains lexical admission. The complete
owned outcome worklist exercises this path with zero carried items and exact
source restoration; its native counterpart runs in CI.

The complete source-type edge validator also exercises this path. All sixteen
declarations translate together and restore the original JavaScript bytes.
Seven native control declarations remain iterative. The shared schema retains
every captured source owner, field and variant; structural edge admission stays
separate from scalar semantics, actual native ownership and execution evidence.

Whole-string `startsWith`, `endsWith` and `includes` predicates use shared
generated decisions in both translators. Focused checks cover Unicode text,
empty searches, missing matches, four-target emission parity and Rust method
spellings (`starts_with`, `ends_with`, `contains`). Positional search arguments
and non-string operands remain explicit refusals. These checks do not establish
complete JavaScript string support or the full native execution matrix.

The Rust translator also handles Unicode lower/upper case mapping and
JavaScript `trim`, `trimStart` and `trimEnd`. Shared generated render decisions
retain U+0085 and trim U+FEFF. The shared binding examples check expanded case
mappings and both whitespace cases against JavaScript execution. Lean and Rocq
keep explicit refusals for these Unicode transformations.

Type queries also accept reads of eligible captured primitive literal constants.
The checked reference is identified from its external constant signature; source
calls and unsafe captures keep their existing refusals.

Sources containing carriage returns, a leading prefix, a noncanonical trailing
suffix or inter-item gaps that emitted separators cannot reproduce retain an
escaped JSON source envelope in the translation header. Restoring that envelope requires matching the source
language, UTF-8 byte length and source digest, and the digest of the complete
translated body, including preludes and layout. This preserves CRLF, mixed line
endings, leading and trailing whitespace and Unicode separators when the
translation is unedited. An edited body or damaged envelope falls back to the
ordinary per-item provenance rules; it cannot restore the stale envelope.
The restoration gate is generated from the JavaScript frontend module. A
complete shared JavaScript rule also generates the native layout selection.
The envelope preserves source bytes; carried items still require executable
translation support.

Dense immutable arrays also support zero-argument `slice()`, one-array
`Array.from()` without a mapper, and homogeneous `Array.of()` construction.
Their frontend forms and argument decisions are generated from JavaScript into
Rust, then bound to the existing checked array and spread representation. Both
runtimes test order, nested arrays, empty arrays, strings, booleans and a receiver
that prints once. Indexed slicing, non-array iterators, array-like objects and
mapping functions remain explicit diagnostics until their behavior is modeled.

The fixed integer cast source draft adds Rust's truncating and sign-reinterpreting
casts across all four emitters without changing exact widening. Its modular
definition, decimal powers of two, rendering decisions and JavaScript source
construction plan are shared JavaScript translated automatically into Rust.
Both packages ship the complete typed definition graph as Links data.
Independent JavaScript parsing admits unshadowed `BigInt.asIntN`/`asUintN` at
the represented constant widths and lowers to ordinary unbounded integer terms
with fresh local bindings and one operand evaluation. Non-BigInt coercions and
dynamic or other widths remain explicit gaps. This batch has not been tested.

The regular grammar discovery draft is another complete JavaScript-first
algorithm library. Its persistent numeric worklists and explicit failure values
translate into native source together, and both packages ship the complete typed
Links program. Source drafting also exposed two general frontend gaps: a leading
throw branch can now borrow the actual returning branch's inferred type, and an
array spread reads a reference payload while retaining fresh array identity.
Both adapters retain the original runtime evaluation order. Source formatting
of the new inference library currently triggers a rustfmt internal whitespace
error; its author preserves the emitted draft and retains all existing CI gates.
No behavioral execution or validation has been performed for these changes.

The next source draft connects shared array, Map and Set methods to checked
method nodes and the JavaScript/Rust emitters. Receiver and argument evaluation
are explicit before native storage borrowing; aliases, insertion order, NaN
membership, numeric zero keys and each declared return kind remain represented.
The complete shared constructor and admission library is generated from
JavaScript and shipped as typed Links data. Checked-wire admission uses an
additive actual-receiver profile. Live collection iterators and other missing
state behavior retain explicit obligations. Future controls compare independent
builtins, checked text/binary reload, both translators and one native compiled
product in CI. These controls have only been authored, never executed locally.

The following parity draft fills native immutable collection read/size nodes,
their actual key/value inference and their four target constructors. All target
text decisions come from one automatically translated JavaScript library.
The typed Links emission API also reaches the existing Lean and Rocq backends,
while retaining model-specific option boundaries and full static rechecking.
Future nine-case controls compare actual JavaScript behavior, exact native
graphs and all four target emissions. Their product compilation remains CI-only.

The RDF address draft generates IPv6 and IPvFuture authority admission from a
single JavaScript definition, including embedded IPv4 tails. Both graph and
N-Triples boundaries compose this with the preceding IRI grammar. The validation
projection never replaces an identifier, original token, source spelling or
semantic value. Both packages retain the complete typed Links definition.
Future controls cover eleven valid and twenty-four invalid identifiers, source
replay, canonical emission and paired graph bytes. They have not run. Generated
source is retained while a rustfmt internal whitespace failure is deferred;
formatting gates and earlier unsupported-domain assertions remain unchanged.

Shared own-record field assignment now has a complete JavaScript-first draft:
source admission, actual type owners, both target constructors, automatic native
rules, editable typed definitions and text/binary replay controls. Qualified
method operations preserve existing checked algebra identities and undergo full
rechecking against the actual record. Direct assignments and Number compound
statement assignments preserve aliases, order and the preceding value. Native
borrows end before the right operand; generated names reserve complete operands.
Future fourteen-case and three-refusal controls remain unexecuted. Dynamic
properties, slots, prototype behavior and coercing compound writes still need
their implementation drafts.

BigInt bitwise operations and signed shifts now have one complete arithmetic
source draft. The checker selects actual Number or integer owners; native
library and runtime generation preserve the source algorithm, all supporting
preludes and the existing integer representation. Relocation is token based
and preserves literals, with complete edits retained as Links data. All four
target constructors use that same shared rule library. Public factories expose
editable typed definitions in both packages. Future controls retain full signed
boundary products, compound locals, large right-shift saturation, both codecs,
paired complete emissions and native CI execution. None has run. Mixed-domain
error encodings, coercions, resource-error equivalence and independent Rust
syntax still require the remaining implementation drafts.

The Rust primitive bitwise draft adds three source precedence levels and
integer complement, preserving actual fixed widths and contextual literals.
Eager Boolean arguments remain distinct from short-circuit operators. Both
checkers and all four emitters use one automatically translated admission and
construction library; typed Links reload retains the actual primitive owners.
Future controls compare every Boolean/integer-width/operator product with
original Rust, translated JavaScript and emitted Rust, plus both codecs and
complete four-target parity. Precedence, contextual literals and eager Boolean
failure have independent controls. None ran. Shift overflow, overloaded traits
and compound Rust local state retain their implementation obligations.

The dense array range draft adds shared `splice`, `fill` and `copyWithin`
checking, result ownership and target construction. JavaScript generates native
rules and complete typed Links definitions. Bound actual operands precede
storage borrows; their type annotations preserve nullable generic owners.
Missing/undefined/null/NaN boundaries, overlap direction, element identities and
the fresh removed-array identity remain explicit. New linked operation rows
are additive. Future twenty-four-case controls and two refusals compare actual
builtins, both codecs, complete paired source translation and one native CI
product. They have not run. Sparse/described arrays, additional coercion domains,
missing element widening and live callback mutation remain to draft.

Rust checked shifts now have a separate total optional-result definition and
an ordinary explicit overflow-abort continuation. Source precedence, actual
left/count owners, range guards before narrowing, bit truncation and sign
extension survive both checker adapters and four shared target constructors.
Complete arithmetic and constructor source generates native code and editable
typed Links data. Future controls compare all declared values/counts against
original Rust with overflow checks enabled, emitted Rust and JavaScript, plus
both codecs and complete target emission parity. These controls have not run.
Native output retains its full unstylized source because the existing numeric
style producer refuses duplicate formatter contexts. Its original guard and
all CI gates remain; mechanical style repair is still required. Unchecked
release behavior and overloaded shift traits remain separate drafts to supply.

The nominal Rust record draft reuses existing datatype/constructor/match terms.
Both frontends retain actual source owners, named fields, tuple positions and
unit construction forms. Source-order bindings precede declared-order argument
placement and reserve all source identifiers. The complete shared ordering,
admission and binding definition generates native source and typed Links assets
automatically. Host code transports tokens, source origins and graph ownership.
Sixteen future positive/abort cases and eight exact refusals retain aliases,
forward declarations, nested/borrowed fields and first-failure order. Whole
graph/codec/four-target and original/emitted-Rust controls are authored for CI;
none ran. Record patterns, generic/trait/layout/destructor behavior, updates and
remaining ownership profiles still need implementation. Static source review
also adds empty mains required by earlier primitive/shift control programs,
retaining all existing inputs and assertions.

The next record source draft adds named enum variants and constructor patterns
with source field puns, rest, nested fields and actual tuple positions. The
shared ordering/absence plan remains JavaScript-first and generates both native
source and complete typed Links definitions. New source wildcard spans retain
their own indirection evidence across field reordering. Construction locals now
carry actual field types, with nominal names qualified at their declaration;
this preserves contextual fixed-width literals and tuple fields. Thirty-eight
future value/abort cases and twelve exact refusals retain all earlier controls.
None ran. Irrefutable let patterns, further ownership/reference/alias forms,
functional update, generic/trait/layout behavior and other project requirements
still need their source drafts.

The inherent Rust method draft uses ordinary owner modules and functions with
an explicit receiver, so the existing four emitters remain the single semantic
path. Host adapters transport source owners, `Self`, receiver tokens and actual
module context. Shared admission, borrowing and parent-path decisions generate
native source from JavaScript and ship as complete editable typed Links data.
Twenty-two future values/aborts and eleven exact refusals retain forward and
aliased owners, multiple implementation blocks, enum methods, field contexts,
source module lookup and receiver/argument order. The preceding record matrix
keeps every input and assertion through a common future product control. None
ran. Trait/generic and mutable receiver behavior, additional dereference/region
profiles, cross-module placement, associated constants/types/macros and closed
profile composition still need their implementation drafts.

Explicit immutable inherent receivers now carry their complete reference/Box
layers into one shared candidate planner. The same JavaScript definition
generates native source and typed Links; host adapters only transport actual
types, lifetimes, nominal owners and type-context `&&` tokens. Forty-three
future values/aborts and eleven refusals retain every former input, including
three corrected refusals with exact historical expectations and independent
original-Rust value controls. Complete paired graphs, codecs and all four
emissions remain mandatory. No control executed during this draft.

Irrefutable nominal let bindings reuse the same ordinary match and product
terms. One shared source rule requires actual single-constructor owners and
irrefutable children; the native rule is generated automatically. Host adapters
retain source patterns, field ownership and fresh temporary names. Twenty
future values/aborts and four refusals add complete paired graph, codec,
four-emission and independent original-Rust controls. None executed.

Tuple patterns expand through a shared rest/position plan before the existing
match compiler. The native planner and editable typed Links are generated from
JavaScript. Hosts transport actual product types, columns and borrowed field
origins; checked terms and all target runtimes stay shared. Twenty future
values/aborts, five refusals and two corrected historical input products retain
complete paired graph, codec, emission and original-Rust controls. None ran.

Tuple-local and constructor rest adapters now consume that same generated
position plan. Actual constructor fields supply widths; omitted fields retain
explicit source wildcard intervals. Existing tuple ownership guards remain.
Thirty-eight future nominal-binding values/aborts and eight refusals include
raw Boolean-keyword identifiers and all earlier controls. No control ran.

Value-returning Rust conditional patterns reuse the ordinary checked match
algebra and all four emitters. Source adapters retain successful pattern scopes,
the outer fallback scope and result origins. Fifteen future values/aborts and
three refusals add full paired graph/codec/emission/original-Rust controls.
None ran; chained conditions and the remaining control/effect forms still need
complete scope-preserving source lowering.

Let-else transports refutable patterns through the existing one-initializer
match/binding path, retaining an outer-scope diverging fallback. Its initializer
grammar guard is shared JavaScript automatically generated to native source and
typed Links. Fifteen future values/aborts and three exact refusals add full
paired graph/codec/emission/original-Rust controls. None executed. Remaining
early-return, effect/ownership and divergence inference work stays explicit.

JavaScript property-call owner selection now has one shared source definition,
generated native source and complete typed Links. Callable record fields infer
their actual signatures before primitive defaults; proven primitive receivers
retain the preceding canonical surface forms. A new future paired matrix keeps
twenty original/emitted values/aborts, two refusals, exact program/definition
graphs, both codecs and four emissions, plus CI-only native Rust execution.
All earlier Rust-source products remain unchanged in scope. None executed.

Concatenation now reuses checked lets, reference reads, appends and primitive
text terms after actual receiver dispatch. All operand values precede payload
reads; the result gets a distinct array identity. Shared role/default decisions
generate Rust and complete typed Links. The expanded future property-call
matrix has thirty-eight values/aborts and three refusals with all original and
paired native/graph/codec/emission controls. None ran. Sparse/species/Symbol,
widening, length limits and general coercions remain explicit obligations.

The native allocation endpoint draft uses shared JavaScript scalar-word
capacity rules, automatically generated into Rust and complete typed Links.
Host glue retains the exact u64 domain and a separate exhausted cursor across
real graph imports and allocation paths. Sixteen future decisions, three
actual endpoint graph inputs, atomic insertion/storage controls and full paired
definition graphs/codecs/four emissions are authored. None ran. The existing
JavaScript safe-Number and shared binary/archive profiles remain bounded.

The functional record-update source draft reuses shared partial-field and
actual-owner decisions. JavaScript generates native source and the full typed
definition. Ordered field bindings precede one typed base binding and one
ordinary constructor match, preserving evaluation order without repeated base
projections. Named/tuple/empty and qualified owners, contextual fields, failures
and hygiene expand the record matrix to fifty-one values/aborts and sixteen
exact refusals. The exact previous refusal input remains in the complete source
with an additive caller and active graph, codec and original-Rust controls;
the correction records its preceding expectation and authoritative source basis.
None ran. Full ownership/drop/resource proofs, borrowed bases, mutation,
generics and the remaining language scope still need their drafts.

The named-constructor emission draft uses one generated shared renderer for
declarations, applications, patterns and bounded value enumeration. Nonempty
ordinary ASCII Rust field names survive in the target syntax; actual field
types and boxing stay in the existing checked graph. A program-wide cached
decision supplies consistent mixed-layout metadata. Full typed definitions ship
through the existing record-rule factories. A separate independent layout
fixture reuses every paired graph, codec, four-target and original/native value
control, adding literal syntax and encoding assertions. None ran. Raw/Unicode
identifiers, zero-field syntax provenance and full emitted-source reimport still
need their implementations; no complete round-trip claim follows from this draft.

The raw identifier draft normalizes ASCII `r#` names while retaining complete
source spelling and coordinates. Both lexer cursors distinguish them from
keywords, and raw patterns preserve their actual binder/constructor meaning.
Reserved-name errors, keyword selection and escaped constructor field rendering
come from the same automatically translated shared definition. Fourteen future
values/aborts, four token/cursor cases, five exact lexical errors and five
program refusals reuse the paired definition/graph/codec/four-target and native
product controls. No control ran. Unicode/NFC, other edition profiles, further
source forms and complete generated-source reimport remain to draft.

The struct namespace draft prevents a same-name associated item from overwriting
the normalized constructor. One shared fresh-name definition generates native
source and the complete typed Links program; host adapters move actual catalog
entries and preserve the source owners. Every discovery pass derives the same
constructor name from the completed method register. The inherent matrix gains
seven value cases and one exact refusal, retaining all preceding controls.
Independent values require actual method execution, including occupied fresh
names, forward blocks and unit/tuple owners. None ran. Enum homographs and
remaining type/value namespace composition still need implementation.

The Unicode scalar draft generates native source and complete typed Links from
one JavaScript definition with reference-array semantics. Host adapters retain
UTF-16 source intervals, transport official pinned Links tables and map shared
errors; actual XID/ID membership and NFC decisions use generated functions.
The importer converts official Unicode 17 property/normalization declarations
automatically and preserves their license, source URLs and hashes. Future Rust
and JavaScript programs, lexical controls, canonical normalization cases and
full official corpus runners remain unexecuted. Older Unicode/edition profiles,
escaped JavaScript identifiers, Unicode lifetimes and the inherited broader
translation requirements still need source work. Metadata and CI integration
remain deferred until the complete requirements draft exists.

Shared-array source reads capture receiver and key in existing typed lets
before loading the payload. One generated definition also selects safe native
borrowed length/index projections and excludes effectful typed keys. Both
emitter adapters transport actual shapes, operands and unchanged prelude flags;
the shared renderer owns emitted source and hygiene. Regenerating the Unicode
kernel applies these projections automatically. Twenty-two future values/aborts
and full graph/codec/four-target controls use the common original-Node product
oracle, preserving its previous fixtures. No controls ran; arbitrary typed
snapshot parity and the inherited full-language/resource obligations remain.

The escaped-identifier draft generates its full decoder, strict module role,
keyword and lexical-goal decisions as Rust and typed Links. Host adapters retain
actual raw tokens, source units, source positions and binding owners; no native
replacement classifier is added. Array/record plans transport the actual
rejected BoundName before choosing an escaped-name syntax error. Unicode
lifetime/character selection extends the shared Unicode program. Future paired
controls include independent Node module syntax rejection and every original
value, exact lexical assertion, graph, codec and emission; none ran. Source
header Unicode profiles, source symbol hygiene and inherited requirements remain.

The source identifier definition now generates native/Links namespace selection,
generated binder/cell spelling and target escaping. Host adapters census actual
source/surface/typed names and transport UTF-16 values, sorted ordinal columns
and existing target naming maps. No native replacement naming policy is added.
Parser temporaries, shadows and mutable parameter cells use separate families;
checker and natural-recursion binders avoid every actual declaration name.
Fifteen future original-Node/native/graph/codec/four-emission source controls,
two retained refusal inputs with corrected positive expectations, and namespace
boundary controls are drafted without execution. Generated snapshots and
historical assertions still need reconciliation alongside closed source profiles
and the inherited requirements. No tests, gates or Rust compilation ran.

The source-only author now captures complete relative named-function dependency
closures automatically. Exact source/hash/import intervals and actual exported
owners remain recorded; aliases and other unrepresented imports refuse. The
record-parameter kernel uses this path for all four actual modules, generates
Rust plus a public editable program and retains its historical closed operations.
Ordinary array/record source adapters select the additional hygienic BoundName
operations. Five new future cases and the preserved original rest-prefix input
add positive controls; all previous inputs and closed-policy checks remain.
No control ran; old generator metadata and the broader module/source obligations
still need reconciliation after the complete draft exists.

Scalar assignment expressions use a shared topological source-term plan that
materializes only existing lets, reads, writes, arithmetic and conditionals.
Source capture order, logical write selection, numeric update conversion and
returned values belong to the JavaScript definition generated as Rust/Links.
The complete callable definition also records expression assignment/capture
facts through lexical scopes; source liveness shares its mechanical adapter.
Both definitions have public editable program factories. Forty-one future
source value/abort controls, full paired graphs/codecs/four emissions and native
CI runners are drafted without execution. General coercions/errors, constant
logical targets, properties, module state and other source requirements remain.

Module lexical roles now derive from actual CST field relations rather than
identifier spellings. The shared definition generates native/Links import,
export, reexport and rename prefix/suffix decisions; hosts transport syntax
owners and their own coordinate widths. Both public definition factories and
twelve future original/renamed Node/native/source/codec controls are drafted.
No control ran. The frozen formatter reported an internal trailing-whitespace
error during authoring; the source writer explicitly retains complete output
with formatting preserved. Its guard and all CI gates stay unchanged. Broader
alias composition, semantic token profiles and module state remain required.

The optional source alias compiler now uses resolved CST binding renames to
combine actual named-function dependencies automatically. Aliased/colliding
dependency owners receive fresh names while root APIs stay stable; original
bytes/imports, transformed sources, declaration mappings and rename hashes
remain in the typed Links provenance. The scalar assignment library adopts the
path for its real bitwise dependency. Existing scalar source/native/codec
controls cover the resulting library but remain unexecuted. Public source
factory, Unicode and allocation signatures are also drafted in TypeScript.
Broader module/state and semantic token obligations remain explicit.

Primitive numeric conversion now has one source algorithm for exact
StringNumericLiteral parsing, positive rational rounding and explicit BigInt
conversion, with shared whitespace and UTF-8/UTF-16 transport. Its author emits
complete Rust, Lean, Rocq and typed Links definitions; formal namespace aliases
come from actual binding mappings. Checkers lower optional conversion through
existing typed lets, absence comparisons and unwrapping, while formal output
threading retains implicit BigInt errors and operand evaluation. No host decimal
parser or handwritten native replacement supplies numeric meaning.

Future source, direct-unit, native, original-Node, codec and emission controls are
declared but unexecuted. The source writer retains complete unformatted output
after an internal formatter failure; formatter and metadata debt remains. Whole
source surrogate transport, objects, Symbols, spreads, error classes and every
other unmet requirement remain required. No tests begin before the complete
requirements implementation draft exists.

Primitive coercion reuses that numeric definition for unary plus, negation,
arithmetic, bitwise operations and unspread supported Math calls. Shared admission
and inference keep BigInt identity for ToNumeric and errors for ToNumber; argument
capture keeps later source evaluation before conversion. Nullable string lexical
branches retain their separate obligations. Original obsolete Math refusal inputs
and diagnostics remain recorded beside corrected, sourced expectations and new
future controls. No fixture behavior or test was executed.

The reusable source-runtime author now derives borrowed array interfaces from
actual emitted declarations/calls before embedding complete Rust runtime bodies.
It also emits namespaced Lean/Rocq bodies and actual entry aliases. Both exact
numeric and BigInt conversion definitions use it. BigInt algorithms parse UTF-16
directly and decompose integral binary64 values exactly; shared digit and white
space definitions are captured once as dependencies. Native public adapters
transport the generated integer value and exact UTF-16 source errors, with no
handwritten replacement conversion. API, definition and independent future
controls are drafted; source builtin lowering and dynamic exceptions remain
active implementation work. No code behavior or tests ran.

The next source batch lowers primitive BigInt constructors through their exact
shared result columns and generated complete runtimes in all four targets.
One decoder transports UTF-8 input for both numeric definitions. Source exception
constructors normalize arguments once and retain message own-property presence
independently of empty text. Native transport retains class and UTF-16 units;
implicit BigInt ToNumber errors use the same typed payload. The existing math
wire container also holds these checked intrinsic operations, with distinct raw
constructor identities so complete codec rechecking preserves normalization.
Future independent controls cover class, units, property presence, argument
evaluation and original source. Full formal exception payloads, uncaught native
process behavior, arbitrary thrown objects and source isolated-surrogate
transport remain required. No source behavior or tests have run.

Source composition now captures actual JSDoc datatype declarations and their
complete owned comments. It keeps datatype source ownership distinct from
generated callable products. The shared JavaScript UTF-16 escape source generates
its native enum and decisions automatically; lexer hosts project actual unit
columns. Isolated literal/template values lower through a checked unit intrinsic
instead of a scalar host string. JavaScript and Rust retain every unit through
codecs and emission. Formal unit representations, non-value unit names and the
remaining source operations still need implementation; future controls remain
unexecuted. Earlier source inputs and wrong refusal diagnostics are retained
with sourced corrected expectations and bounded active runtime profiles.

The UTF-16 operation source now generates complete runtime bodies in all four
targets. Full formal String representations keep unit columns across parameters,
results, nested types, optional values and output threading; Rocq units use
binary N values. Shared bridges reach the numeric kernels and only the final
scalar display replaces isolated units. A shared bound function also implements
generic formal array slicing. Source String.fromCharCode/fromCodePoint adapters
capture arguments first, then preserve each conversion/range-check step and
error order through existing typed operations. Native host adapters construct
actual IR; source policies, operations and rendering rules generate Rust.
Complete formal exception payloads, the native default String parameter ABI,
other String/state operations and all earlier obligations remain required.
Future controls remain unexecuted; no behavioral or validation gate ran.

Numeric source exponentiation now has a draft complete source closure for
BigInt repeated squaring and canonical dyadic Number interval evaluation. The
existing runtime author retains every dependency, actual binding and generated
body across all four targets. Formal array membership is an actual shared
emission rule, including optional primitive SameValueZero and relative bounds.
These additions are unexecuted drafts; exact V8 approximation parity, proofs
and formal general-recursion budgets remain required.

The primitive comparison source draft also admits the actual unshadowed
Object.is builtin. A shared operator plan preserves binary64 NaN/signed-zero
SameValue, primitive type distinctions, nullable tags and the common BigInt
domain. Both actual checker hosts capture all arguments before selecting the
first two, preserve ignored argument effects and supply missing undefined
values. Complete rule source and Links are generated automatically; prepared
source/codec/emission/native controls remain unexecuted. Object/Symbol identity
and argument iterator effects retain their scope.

Binding defaults now have a complete shared inference/selection draft for
represented primitive and structured domains, with null retained and only
undefined selecting the initializer. Actual term endpoints survive nested
optional inference; existing typed conditionals, absence tags, unwraps and
wrappers preserve lazy effects. Shared source generates native rules and the
public complete ArrayBindingProgramLinks graph. Two obsolete refusal
expectations retain original inputs and assertions beside source-qualified
replacement results. Future controls are declared, not executed. Iterator
protocols and heterogeneous present-value unions retain their full scope.

The String/search source draft captures all raw arguments before primitive
text and position conversion. It admits primitive String() calls, nullable
String concat values and positions for includes/startsWith/endsWith/indexOf/
lastIndexOf, retaining UTF-16 indexing and missing/undefined/NaN distinctions.
Both checker/inference hosts select the same source-derived rules; ignored
String map extras still evaluate. Existing own record methods keep their
actual meanings. Generated source and future full graph/emission/native
controls are drafts, with no behavior executed. Objects, Symbols/RegExp,
generic receiver/lookup semantics and all previous scope remain required.

String range source lowering now uses one generated checked-operator plan for
slice, substring and the Node Annex B substr profile. Captured arguments retain
sequential numeric conversion and undefined defaults; NaN normalization, swapped
substring endpoints and relative/count substr bounds select the exact existing
UTF-16 slice operation. Complete public source graphs and both runtime adapters
are drafted, with original/native/codec/emission controls left unexecuted.

Source inspection found the complete builtin-owner classifier omitted BigInt;
it now admits actual unshadowed BigInt and Boolean while retaining all lexical/
module/context shadows. Boolean conversion uses existing shared truth nodes
over represented values, with ordered raw argument capture, missing false and
ignored extra effects. Complete optional-value source rules generate native
code and public editable graphs. All future original/native/graph controls
remain unexecuted; host exotic values and iterator/wrapper behavior retain scope.

Ordinary Rust source drafts now retain implicit Unit results and empty blocks,
terminal Unit returns, inert semicolons and sequences of proven direct Unit
calls. Collecting passes defer call ownership to the actual declaration catalog;
the final source pass requires its real direct Unit result. Hygienic ordinary
bindings preserve call order in functions and main, with Unit tails admitted.
Former default refusal inputs/assertions remain recorded beside stronger
source/graph/call controls. The standard environment wrapper and closed source
profiles keep their remaining obligations. General nonlocal returns and
discarded values/destructors still need implementation; no control ran.

Array.isArray source calls now retain missing inputs and every extra-argument
effect, while reusing actual represented-kind and optional-presence decisions.
Both source hosts preserve the first captured value and the standard owner
guards. Complete rules and future source/native/codec controls are drafted
without execution; proxy/exotic and dynamic union behavior remains required.

Written record-parameter bindings now initialize through the ordinary local
mutable-binding path. Actual lexical write facts select let/const prefixes
before the original body, preserving ordinary parameter writes, captures and
caller-property independence. The historical read-only predicate remains
unchanged. Three former refusal sources/assertions remain beside stronger
original/graph/native controls. Full source rules generate native code and
editable graphs; no controls executed. Parameter/var/accessor generality and
formal mutable-cell execution retain their full remaining scope.

Comma source expressions reuse the complete shared scalar construction plan
schema and existing Let terms. Both parser adapters transport actual left/right
source terms in evaluation order, using the whole-module hygienic name supply.
Expression roles retain list separator grammar. Ordered statement/for lists
reuse ordinary lowering, including ignored values and calls. Complete source
rules generate native definitions and Links; future source/native/codec/target
controls are declared but remain unexecuted. General references, arbitrary
completions and every preceding obligation remain.

Void is another source-owned term plan over the same actual operand and Let
transport, followed by explicit undefined. The complete source closure generates
both native rules and editable Links. Ordinary const-for bindings reuse lexical
declaration construction with header punctuation owned by the loop. The exact
former **= plan refusal remains recorded beside its normative correction and
stronger original-input/source controls. No behavior or checks executed.

Full Unicode 16 casing is now a complete shared source algorithm plus compact
array declarations automatically converted from official Unicode data. The same
checked closure emits native library rules, standalone JavaScript/Rust/Lean/Rocq
runtimes and editable Links graphs. Target bridges convert strings to actual
unit columns and back, retaining the profile in contracts. Context reads original
points and possessively ignores Case_Ignorable, including Cased overlap. No
host case conversion implements JavaScript-source mappings. Alternate Unicode/
locale and complete Rust standard-library version profiles remain required.
Future source/native/codec/context/scalar controls are declared without execution;
the exhaustive independent Node comparison belongs only in CI.

Ordinary Rust terminal return values now use a complete source-owned delimiter/
continuation decision. Both adapters grow actual punctuation windows only as
needed, then retain existing value ownership checks and ordinary tail lowering.
No nested/earlier return is converted to an initializer value or closed proof.
The exact former refusal row remains historical beside current same-input
value controls. Complete decisions generate native code and public editable
graphs; all behavior and metadata reconciliation remain deferred.

Ordinary Rust println now transports Print terms through the existing output
algebra, preserving lexical context and Unit statement/tail values. Formatting
grammar, occurrence selection and ordered raw operand captures are complete
shared source plans, emitted automatically to native code and editable Links.
Both source adapters bind every explicit argument once before reusing its value,
keep independent implicit/explicit cursors and reuse actual source name grammar.
The former native handwritten grammar is replaced by source-plan transport.
Future complete original/native traces, source AST parity, both archives and
four emissions remain unexecuted; all general formatting/macro/borrow and
preceding requirements remain.
