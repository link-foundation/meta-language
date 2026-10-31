# Issue 227: own-source concrete baseline source draft

This handoff records source inspection and deferred controls under the existing
[vision](../vision.md#concrete-and-abstract-syntax-trees). It introduces no
architecture specification and supplies no execution or completion evidence.
The requirement ledger and its source register were not read or changed: their
online prerequisite is deferred with runtime execution in this drafting phase.

## Drafted files

- `rust/src/link_network.rs`: corrected the documentation of `LinkNetwork::parse`
  to describe its actual registered-grammar concrete-tree dispatch. No executable
  Rust statement changed and no handwritten native semantics were introduced.
- `js/tests/issue-227-own-source-concrete-baseline.test.js`: future controls for
  complete current `js/src/network-projection-rules.js` and
  `rust/src/network_projection.rs`, independently bounded to 16 KiB each.
- `js/src/translation/rust-standard-sum-completions.js`: actual question-mark
  postfix transport, ordinary completion branch construction and final private
  marker refusal. This consumes the concurrently drafted standard-sum adapter's
  validated origins and complete argument identities.
- `js/src/translation/rust-ordinary-completions.js`: direct initialized-binding
  and tail/terminal-return propagation into the existing ordinary callable
  completion tree, including joins through its current result/continuation
  carrier. Private expressions are refused after lowering.
- `js/src/translation/rust-unit-value-rules.js`: shared source decisions select
  the actual question-mark token for ordinary callable completion.
  Automatically generated native artifacts remain deferred with the full batch.
- `js/src/translation/rust-expression-completion-rules.js` and
  `rust-expression-completions.js`: shared ordered child/branch roles and actual
  source continuation transport. Calls, constructors, arithmetic, products,
  selected branches, matches, guards and conditional packets now consume nested
  propagation without lifting a lambda body into an outer callable.
- `rust/src/translation/rust/standard_sum_completions.rs`,
  `expression_completions.rs` and `ordinary_completions.rs`: native structural
  term/origin transport invokes the same shared source policies. Private Math
  markers index actual parser-owned attempts; fresh private continuation holes
  are removed before surface admission. No public semantic-node enum changed.
- `js/tests/fixtures/rust-standard-sum-integration.mjs` and
  `js/tests/translation-rust-standard-sum-integration.test.js`: complete original
  Rust programs, independent literal stdout, retained existing originals and
  refusal inputs, and deferred actual frontend/checker/typed-Links controls.
- `js/scripts/produce-rust-standard-sum-integration.mjs` and
  `rust/tests/unit/translation_rust_standard_sum_integration.rs`: deferred actual
  JavaScript/native producer and native integration matrix over those same
  original inputs, selected source snapshots, all graph transports and targets.
- `js/src/translation/rust-array-lookup-options.js`, shared
  `rust-array-lookup-option-rules.js`, and native
  `rust/src/translation/rust/expressions/array_lookup_options.rs`: stored direct
  scalar Copy results for existing vector/slice `get`/`first`/`last` followed by
  `copied`. Existing admitted immediate consumers retain their original route.
  Original native array plan/selection/dimension/origin helpers gained only
  sibling visibility so the new adapter reuses their unchanged constructions.
- `js/tests/fixtures/rust-array-lookup-options.mjs`,
  `translation-rust-array-lookup-options.test.js` and
  `support/rust-standard-sum-concrete.js`: deferred stored/copied originals,
  old-route refusal/preservation controls and complete original physical source
  CST replay controls for the main producer/test matrix.

The controls explicitly select Node `>=22.15.0 <23.0.0` or `>=24.0.0 <25.0.0`,
the package's declared web-tree-sitter `0.27.0` dependency, and the repository's
Rust edition `2024`/declared minimum `1.90`. Rust source is parsed as text by the
JavaScript package; this is not Rust toolchain execution or native equivalence.
Uncovered Node/dependency/source-manifest versions fail the future control
rather than silently skipping an assertion. Grammar implementations remain the
actual selected checkout's catalog/lock inputs; the controls do not assign an
unverified independent grammar version to them.

Every selected file must retain clean recovery, nested grammar structure,
actual named declarations, fields, trivia, exact physical token intervals,
distinct identities and spans for repeated same-text occurrences, and exact
original bytes. The tests exercise omitted concrete selection before and after
explicit abstract selection, complete text/binary graph reload while source
parsing is trapped, subsequent source reconstruction and deterministic reparse.
Binary replay additionally compares its complete archive bytes. Assertions use
the actual current complete file and original byte intervals; declarations are
independently named in the control, not discovered as their own expected values.

These two files are a bounded control pair, not the repository denominator.
There is no source hash baseline, whole-source inventory claim, full independent
CST-row oracle, native parity fixture, or semantic behavior certificate here.
The complete own-source inventory and original independent tree expectations
remain necessary. No observation recorder or status field is added.

## Inspected production path

`js/src/programming-language-parser.js:parseProgrammingLanguage` selects the
registered grammar. `parseGrammarCst` retains its tree and physical source gaps;
`js/src/network.js:LinkNetwork._insertProgrammingLanguage` stores actual tokens,
and `_insertProgrammingTree` stores syntax hierarchy and field relationships.
`LinkNetwork.parse` does not request abstract syntax. `projectedLinks()` defaults
to `ConcreteSyntax`, with shared membership supplied by
`js/src/network-projection-rules.js`; explicit selection leaves the source graph
intact. A lexical fallback for an unknown language is a separate existing path,
not proof of a complete registered-language tree.

`rust/src/link_network.rs:LinkNetwork::parse` delegates to the built-in language
parser and retains recovery/provenance. `reconstruct_text` selects physical
nonmissing token links with spans and terms, sorts by start byte then identity,
and suppresses already-covered intervals. Its default CST documentation now
agrees with this registered-parser boundary.

## Source gaps requiring coherent follow-up

1. **Spanless constructed source tokens.** JavaScript
   `LinkNetwork.reconstructText` retains nonmissing source tokens without spans,
   using the existing `sourceOrder` comparator. Native `reconstruct_text`
   filters them out through `span()?`; `rust/src/source_generation.rs:render_source`
   instead supports constructed roots and, when physical tokens exist, places
   spanless tokens after physical tokens. JavaScript `renderSource` also places
   spanless tokens last. These are observably distinct public operations.
   A follow-up must inventory callers and retain the existing JavaScript
   constructed-token behavior before drafting a shared explicit source-defined
   ordering/ownership policy and translating it automatically. Future controls
   must include a purely spanless graph, mixtures inserted in both orders,
   absent terms, zero-width spans, missing recovery tokens, separate documents,
   and equal-text distinct occurrences. Dropping tokens to align the runtimes
   would conceal this obligation. No source policy or adapter semantics changed
   in this batch.
   There is also a static ordering issue in JavaScript's `sourceOrder`: it
   compares byte starts only when both tokens have spans, and compares IDs
   otherwise. Consider A(id 1, start 20), B(id 2, no span), C(id 3, start 10).
   The comparator gives A before B, B before C, and C before A. This is a
   source-derived counterexample to transitivity, not an executed observation.
   A coherent shared total order and its compatibility controls must cover this
   mixed case; a platform's incidental sort result cannot define the oracle.
2. **Embedded overlays and editable ownership.**
   `LinkNetwork._attachEmbeddedRegions` inserts the host physical tree before
   child trees, preserves absolute offset spans, and keeps actual Region links
   plus shared budget/ancestor decisions. `reconstructText` merges all token
   links by start then identity. Earlier host tokens therefore suppress child
   tokens covered by the same interval. This preserves a passive original host
   stream; it does not define how editing an embedded child rewrites that host
   token, escapes it, or updates derived source planes. `renderSource(language)`
   narrows language membership but does not resolve an edited child against its
   physical owner. Do not reverse token priority: that could change unchanged
   host bytes and select unrelated overlapping alternatives. Follow-up controls
   must independently specify physical and decoded source planes, duplicate
   nested-region boundaries, same-language embeddings, edited/deleted child
   leaves, ancestor cycles, and text/binary reload. Shared Links data must choose
   ownership and supported edit propagation before public emission integration;
   unsupported ambiguous edits require an explicit refusal, not cached original
   text presented as edited emission.
3. **Full own-source acceptance.** The bounded pair cannot replace every real
   tracked JavaScript/Rust module, complete function/type/effect/ownership
   inventory, whole-graph exact identity support, independent concrete rows,
   native replay or behavioral translation. Keep the complete denominator and
   original refusal cases when joining this draft to that work. The public
   exact-ID source adapters and the existing full replay controls require their
   own full inventory and native execution before any closure claim.

## Actual standard-sum propagation integration draft

The parent frontend integrates `readRustStandardSumTryExpression` at the actual
`?` postfix token and maps owned question-mark punctuation to the `try` marker
for ordinary completion selection. This handler requires the actual validated
standard sum source origin, consumes exactly that token, and marks a private
expression with its complete successful payload origin. Ordinary initialized
bindings, leaves and conditions call the expression continuation adapter before
constructing their existing completion terms. Each successful payload resumes
its actual expression context; failure retains the nearest callable return.
The postfix handler also captures the actual nearest callable result origin;
lowering compares its complete identity with the receiving callable and refuses
borrowed sum results or a changed callable owner.

`lowerRustStandardSumTryCompletion` revalidates both operand and actual enclosing
function result owners. It uses the existing shared question-context admission
rule: direct Copy sums, identical Option/Result families, and identical complete
Result error identities. It captures the operand in exactly one `let` before a
match against both real source constructor paths. Only the successful branch
enters the original continuation. The failure branch constructs the enclosing
result's real `None` or `Err` and delegates to the existing ordinary function
return lowering; joins use its existing `FunctionResult` and
`StatementContinuation` datatype. There is no invented `Try` semantic IR and no
handwritten replacement native implementation.

The handler depends on `readRustStandardSumSourceOrigin` and
`readRustStandardSumSourceIdentity` from the separate standard-sum adapter draft.
Actual frontend hooks and nearest-callable context ownership are parent-owned;
they must compose with the adapter's expected-type and source-origin work before
execution. The source declaration catalog and collecting passes must preserve
actual known owners; an unknown owner cannot be inferred from an Option/Result
spelling or manufactured as a proof.

`refuseRustStandardSumPrivateCompletions` enforces finite final admission and
explicitly rejects any unconsumed private expression before semantic checking.
Stacked propagation, nested call/constructor arguments, strict arithmetic,
unary/cast/products, selected conditions, matches and existing conditional
success/failure packets now have real ordered context transport. Earlier
effectful children are captured once before a later propagation; the source
origin must supply that capture's complete type. Static immutable leaves keep
their original contextual typing. `&&` and `||` keep their right-hand
continuation in the selected branch. Guard fallbacks remain lazy through the
existing shared conditional packet vocabulary. Actual array value wrappers and
spread flags remain present. Shared finite work/inspection/depth bounds refuse
exhaustion. Cross-lambda lifting, opaque source forms, early explicit-return
continuations and general trait/error conversion semantics retain their
obligations and explicit admission boundaries. Full generic ownership, mutable payloads, custom `Try`,
`FromResidual`, destructor observations and distinct-error `From` conversion
remain refused. No source program, generated semantic graph or completion
fixture has executed during this drafting pass.

The integration controls retain all four separate standard-sum originals and
their refusals, and append single-evaluation operands, two sequential operands,
nested-sum tail propagation and a conditional continuation join. Exact literal
stdout includes `before`, operand traces and `after`, so a duplicate evaluation,
wrong order or execution of a failed continuation cannot match the oracle.
Actual checked call nodes additionally retain exactly one represented call to
each independently named operand producer. Every positive input runs the actual
frontend/checker and all four emissions after original/text/binary typed-graph
transport; JavaScript emission compares the same stdout each time. Every refusal
runs both direct frontend/checker and typed import. Nearest-callable boundaries
retain explicit refusals. The exact former argument/arithmetic/stacked refusal
inputs and former draft reasons remain in a historical array, while their same
unaltered inputs now have positive controls. Additional actual originals cover
earlier argument effects before a failed later operand, both short-circuit
branches, selected match arms/guards, typed/inferred tuples, if-let and let-else.
A separate closure original retains its own explicit standard-sum result and
literal inner/outer stdout: an inner failure returns from that closure while
the outer callable still runs its subsequent statement. A nonsum closure
refusal preserves the nearest-callable boundary in the other direction.
The selected future snapshot endpoints are `1.90.0` and `1.99.0`; interior
source snapshots and native original/emitted compilation remain future CI
matrix work, not results. The fixture retains explicit native control work
items for original stdout, real native/JavaScript graph comparison, all target
cells and original refusals. No native observation is fabricated from those
declarations and no handwritten replacement interpreter is introduced.
Four additional namespace originals retain grouped standard import aliases,
a forward user callable named `Some`, a qualified local `crate::Option`, and
an actual local `std::option::Option`. The nominal controls require their real
decoded qualified datatype and constructor relationships and declaration order. The qualified
local type deliberately preserves the existing distinct bare reserved-name
boundary; these controls do not reassign source identities by spelling.

The native adapter carries actual typed sum identities and constructor paths;
Result error equality is checked structurally before its complete diagnostic
text is transported into the shared string policy. Ordinary result templates
use an impossible source-identifier namespace and
never consume user-visible source fresh identifiers. Internal native expression
frames now defer ordered-child, successful propagation, branch, match and packet
continuations until their actual source values are available. Earlier effectful
children therefore choose capture type and reserve names at the same callback
point as the JavaScript adapter; an original conditional is no longer prematurely
captured as though its successful payload had not been selected. Ordinary
JavaScript carrier/body construction follows the native adapter's source-only
construction order while preserving actual runtime evaluation order. The final
native guard refuses both unconsumed attempts and escaped holes. Full generated
graph identity remains a required actual matrix assertion, not a source-review
conclusion. New shared policy modules and every changed shared definition still
require automatic native generation.

The future producer preserves every actual original, refusal, prior input,
selected version, surface, typed root, complete graph bytes and emission fields;
it computes stdout only after executing the emitted JavaScript and compares the
independent literal. The native test compares real native surfaces, roots, Links,
binary snapshots and every emitted field with that actual producer. It compiles
each complete original Rust program under each exact declared installed CI
snapshot, checks
its literal stdout, compiles the exact emitted Rust code and executes that code
for every associated graph transport. Equal code bytes explicitly associate the
reused compiled executable with each actual emission; no graph cell is skipped.
JavaScript code from each native graph is executed against the same literal.
Native refusal categories, full reasons and spans compare with real JavaScript
diagnostics. Source snapshots `1.90.0` and `1.99.0` are both import interpretations
and explicit original/emitted native controls. The selected native test alone
provisions both exact toolchains under its CI-only guard, retains setup stdout
and stderr, and fails setup instead of skipping cells or substituting the active
compiler. Other native test shards do not pay that provisioning cost. All these
are unexecuted test sources; no toolchain installation ran during authoring.

Both additive method-fixture files feed this same matrix: all three sum-adapter
method originals and all ten separate source-method originals, plus four
source-method refusal inputs. The small adapter originals receive an explicit
empty single-call-owner array without removing any original field or assertion.
No method fixture is replaced by another contributor's version.

## Stored copied lookup source consumers

The actual frontend hooks run the new copied-result adapter immediately before
the existing complete array lookup pipeline. Its shared selector peeks the
actual empty `copied()` suffix and next method. Every already-admitted immediate
consumer remains selected by `readRustArrayLookupConsumerArity`; the new adapter
returns without consuming tokens, reading receiver origins or changing the old
AST. New stored/boundary/question/map/and-then results reuse the original
receiver/element-owner/usize/Copy/owned-read checks, condition/index plans and
actual standard-sum instantiator. They capture receiver then index exactly once
and put both the selected element read and empty-last subtraction only in the
guarded present branch. The result carries actual standard constructor paths
and complete owned scalar payload origins, independently of a source spelling
named `Option`.

Eight complete copied lookup originals retain hit/miss and empty/nonempty
branches, receiver/index ordering, repeated Copy use, result and index
propagation, actual new mapper payloads and present Unit versus absent. Five
refusal inputs preserve usize identity, general reference ownership, non-Copy
String, stored cloned String and wrong concrete result owners. Separate routing
controls retain all eleven prior immediate consumers on all three lookups.
These controls append to the same complete frontend/typed/four-target native
matrix; none has executed. General lookup references, cloned owned String,
non-Copy payload lifetimes and broader collection methods retain their full
models and original unsupported boundaries.

## Complete physical source control alongside typed translation

The main future producer also parses every original/refusal source through the
public default Rust CST path. It verifies actual clean nested grammar structure,
every physical syntax node's ordered field/term/span/flags, exact source/token
bytes, full own-graph text and binary replay and explicit abstract projection
without mutation. The native control independently performs the same original
and replay checks. Native graphs use parent edges while JavaScript graphs use
child edges, so only normalized original physical tree rows compare across
runtimes; complete identities, references and metadata stay exact within each
runtime's codec replays. Typed graph parity remains separately exact. The CST
control performs no target emission or source-program execution, and returns
only a bounded normalized witness so the producer does not retain another full
set of concrete networks. Panic originals keep their separate actual runtime
control path instead of being misclassified as source-parser refusals.

No tests, source programs, generators, validators, compilation, package install,
network requests, CI inspection, ledger operation or Git mutation ran for this
subtask. Only targeted source reads and file edits were performed. The next
phase must review the whole drafted batch before executing its required checks.

Coordinator integration adds a deferred aggregate policy-control file at
`js/tests/rust-baseline-source-policies.test.js` for the separate alias/sum
policies. It does not exercise or resolve the CST emission gaps above.
