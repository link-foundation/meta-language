# Issue 227: own-source concrete baseline source draft

This handoff records source inspection and deferred controls under the existing
[vision](../vision.md#concrete-and-abstract-syntax-trees). It introduces no
architecture specification and supplies no execution or completion evidence.
The requirement ledger and its source register were not read or changed: their
online prerequisite is deferred with runtime execution in this drafting phase.

## Disjoint collection iteration source batch

The own-source anchor is `EmitState.assign` in `emit-common.js`: it reads
`program.declarations.values()` while writing its separate naming maps. The
new source path wraps only actual eager `for…of receiver.values()` in a closed
known source scope. Existing general live collection iterator refusals remain.
`disjoint-collection-iteration-rules.js` and
`disjoint-collection-projection-rules.js` project the final checked graph into
actual lexical slots, allocation sites, record fields, call edges and effects.
Reference names are lookup keys, never distinctness proofs. A final bounded
fixed point must prove the complete reachable lowered loop cannot mutate the
captured collection. Unknown container contents, callable bodies, operations
and effects refuse. Cell replacement remains refused because this first
profile does not model temporal cell versions. Array index/lookup results have
unknown provenance; an Array-typed method result is never by itself a fresh
allocation certificate. Actual `referenceAllocate` nodes establish outer
identities. Primitive-column reads use the existing required-fact functions.

`source-collection-iterations.js`, the JavaScript parser/inference/checker
seams and `disjoint-collection-iterations.js` connect this policy to actual
source consumers. Private pending markers are proof-checked and removed before
checked IR returns. The root-authored native source-wrapper/checker seams
consume the same decisions; `disjoint_collection_iterations.rs` only transports
actual serialized checked terms and the generated numeric plans. The additive
callback consumers in `collection-copies.js`, `emit-rust.js`, native
`collection_copies.rs` and `emit_rust/reference_callbacks.rs` retain the existing
readonly/factory/arity restrictions and permit additional disjoint effects only
when every relevant original callback passes the same final-program proof.

The independent fixture `disjoint-collection-iterations.mjs` retains two exact
own-source anchors, 24 literal decisions, seven complete positive originals
and nine complete refusal originals with literal stdout. Controls include
distinct fields and helper arguments, an own-shaped closed EmitState class,
nested Array callbacks, actual Map/Set aliases, aliases recovered through
array indexing/reduce, opaque Math operations and mutable cell redirection.
`translation-disjoint-collection-iterations.test.js` and the CI-only producer
`produce-disjoint-collection-iterations.mjs` reuse the complete physical CST,
exact source bytes, explicit AST projection without graph mutation, source
text/binary reload, typed three-transport/four-target and stdout assertions.
The native `translation_disjoint_collection_iterations.rs` uses the same full
CI consumer factored from `translation_collection_copies.rs`; the original
collection-copy test invocation and every prior assertion remain represented.

The initial proof bounds are 4,096 checked expression nodes, 32,768 edges,
65,536 accumulated facts, 32 aliases per node and 128 fixed-point passes, with
an additional explicit work budget. Larger/opaque programs and full live
iterator behavior remain obligations. Generated native policy sources, tests,
compilation, formatting, validation and runtime observations were not run in
this batch. These source drafts provide no parity or completion evidence.

The additive keys/entries batch uses the same proof and private-marker path.
`readDisjointCollectionIterationItemForm` delegates to the established
collection-access result roles and preserves the original values-only decision
and fixture unchanged. `collection-items.js` factors the existing JavaScript
checked key/value/pair construction for both consumers; native transport reuses
`checked_collection_access` directly. Map entries preserve independent key and
value types; Set entries retain the same element owner in both pair positions.
Existing snapshot runtime operations retain insertion order and nested handles.
`disjoint-collection-item-iterations.mjs` adds ten literal decisions, five
complete positives and five complete refusal inputs, including pair
destructuring, nested handle mutation, actual source aliases, nonzero argument
calls and escaping stored iterators. Its JS test, CI-only producer and native
test reuse all existing complete source/CST/transport/target assertions. This
extension was also authored without execution or generation.

The subsequent eager-spread source batch addresses the exact own-source
`return [...this.assumptions.values()]` and encoding counterpart. It adds an
`eager snapshot` consumer role to the same shared projection/solver. This role
requires actual standard collection allocation provenance, with synchronous
consumption at the existing array-spread operand; it neither fabricates a
callable loop scope nor ignores mutations that run during a loop body. Earlier
and later array elements retain their existing left-to-right append structure.
Later sibling mutations therefore run after the earlier snapshot, and a later
spread captures at its own later position. A separate private marker prefix is
registered and erased by both actual final checker consumers. Only closed
owned name/field receivers and zero-argument standard item operations enter
this path; stored iterators, factory/index receiver expressions and nonstandard
source ownership remain outside it.

The new `disjoint-collection-spreads.mjs` fixture contains two exact own-source
anchors, 12 literal decisions, ten complete positives and four complete
refusals. It retains literal evaluation traces, an actual single-call producer
assertion, nested handle aliases and all Map/Set item families. Its JS/native
tests and CI-only producer use the same complete source/CST/typed/target
assertions; the native shared consumer now also explicitly counts every
declared single-call owner in the actual checked native body. No existing
inputs or assertion cells were removed. Source generation, tests, compilation
and observations remain deferred.

## Actual closed Scope and composition source batch

Inspection corrected the proposed inheritance anchor: current `RustEmitter`
does not extend `Emitter`; it composes an EmitState. No speculative inheritance
implementation was authored. Actual `Scope` in `ir.js` instead exposed two
concrete gaps: its `reserved = new Set()` constructor default and its `child()`
method returning a fresh Scope while forwarding the ordinary `ident` function
field. The literal-only class parameter census remains an exact old policy;
an additive collection-default census forwards actual zero-argument Map/Set
expressions to the existing ordinary default parser. Actual lexical/function
shadows are still checked there, and class-name collisions refuse before
class rewriting can hide the original constructor owner. Shared class plans
now carry actual brand/declared-own-field pairs, allowing ordinary stored
callable fields to travel without admitting detached prototype method slots.

The additive prototype profile is selected when a method constructs an actual
known closed class. It emits finite instance records containing data fields
and the private brand, with prototype methods as module helpers taking the
actual receiver first. The old closure-slot model stays the separate earlier
profile. The source-only method table has brand/method/helper triples. Actual
constructor return types seed receiver constraints; ordinary call/default
constraints select results, and the checker validates actual local helper
definitions and branded receiver signatures. Receiver capture precedes every
argument exactly once. The table is consumed into ordinary Core calls and is
not copied into checked Program or typed archives. Native class/inference/
checker adapters transport the same decisions and reuse ordinary calls rather
than implementing an independent object runtime.

`javascript-class-observation-rules.js` and actual JS/native type/declaration
traversal preserve direct and nested branded aggregate observation refusals
after prototype Function slots disappear. Scalar field/method observations
remain ordinary operations. Printing/String conversion, aggregate identity
comparisons and structural proof operands cannot expose hidden brands. The
existing raw reflection, method replacement, detached method and unproved
module-context gates remain. The subsequent persistent owner batch below
revalidates archived ownership using actual constructor/type edges; no emitter
guesses ownership from a generated field spelling.

Independent defaults controls live in `closed-class-collection-defaults.mjs`
(nine decisions, five complete positives, five complete refusals). Prototype
controls live in `closed-class-prototype-methods.mjs` (two exact source anchors,
nine method decisions, five observation decisions, five complete positives and
five complete refusals). These include the complete actual Scope class body,
Set copy independence through child(), own-shaped backend/state composition,
call-once/effect order, helper lexical scope, nested handle aliasing and hidden
brand observations. Each family has a future JS/native full-matrix test and
CI-only producer using the existing complete CST, byte, source codec, typed
transport, four-target and stdout assertions. Actual module import closure
composition remains a separate required frontier. No test, source generation,
formatting, compiler, validation or product execution ran in this source batch.

The persistent-owner source batch adds `javascript-class-owner-rules.js`,
`javascript-class-owners.js` and native `class_owners.rs`. Source lowering
captures actual constructor, brand, representation, declared data fields and
method/helper ownership. Inference preserves a nominal `sourceClassOwner`
record type attribute; its type identity and typed Links constructor reference
are root-authored structural plumbing. Checked programs retain the versioned
`sourceClassOwners` witness, whereas the private prototype dispatch table still
disappears into actual Core calls. Shared validation checks the witness against
actual constructor fresh-reference return flow, helper receiver signatures,
complete branded record fields and ordinary type identities. It checks direct
and nested nominal/structural observations at checker, typed archive and all
four emitter boundaries. The old closure-slot representation retains reference
identity; the data-only prototype representation retains its observation
refusals. Missing, contradictory, malformed and orphaned owner data refuse.

`closed-class-owner-witnesses.mjs` adds ten independent policy decisions, four
complete originals, 16 direct archive edits and three semantic graph edits.
Controls retain alias/fresh/empty-class identity positives, malformed/null/empty
owner metadata, dropped witnesses, wrong constructor/helper edges, retagged
plain records, direct/nested debug and prototype identity observations.
Restored graph edits run through original/text/binary transports and every
target; native consumers retain each exact JSON input even when serde itself
rejects malformed types. Producer, JS tests, support and native unit sources
are drafted but unexecuted. Formal Lean/Rocq reference allocation/state lowering
remains required target work: existing four-target positive assertions are
retained and are not presented as passing. No source generation, tests,
compilation, validation or runtime observation ran for this batch.

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

## Direct Copy standard sum combinators

`rust-standard-sum-combinator-rules.js` now supplies the shared decisions for
Option `and`, `or`, `xor`, `or_else` and Result `and`, `or`, `or_else`.
`rust-standard-sum-combinators.js` and native `standard_sum_combinators.rs`
transport actual source origins into those decisions through the parent-owned
postfix dispatchers. The receiver is captured before the eager right operand
or callback factory, and each is captured once. Only the selected failure
branch invokes `or_else`; Result passes its actual error payload and Option
passes no arguments. Complete retained payload/error identities are checked
against actual source owners before constructing the output datatype branches.
The nominal Copy `Option<Span>.or` use in native copy-closure parsing is a
concrete source dependency covered by a separate complete original.

A direct Result `and(Ok(value))` obtains only its missing error argument from
the actual receiver, and direct `or(Err(error))` obtains only its missing
successful argument there. The new payload/error must have its own complete
immutable Copy origin. Constructor recognition uses actual value-namespace
resolution, including imports and local shadows. The constraint is limited to
the entire direct right operand; it cannot cross a following method or infer
an unsuffixed scalar type. Other generic-changing right operands and Result
fallback callbacks must establish their complete output from actual syntax.
An unestablished Err-only closure remains an explicit inference obligation.

`rust-standard-sum-combinators.mjs` retains 35 independent rule decisions,
13 complete originals with literal stdout and 13 complete refusal inputs.
The originals cover eager evaluation on unselected operands, callback-factory
order, selected lazy invocation, changed Result error families, all four xor
presence combinations, Unit presence, imports and the actual Span owner.
The version-sensitive source range is Rust `>=1.90.0 <1.100.0`. The coordinator
owns appending these rows to the common default-CST/frontend/typed/four-target
controls without moving existing fixture coordinates. The standalone policy
test and all integration controls remain unexecuted source drafts; native
shared-policy generation and every subsequent check are deferred.

## Nonconsuming standard sum observers

`rust-owned-standard-sum-observer-rules.js` and the JavaScript/native
`rust-owned-standard-sum-observers.js` / `owned_standard_sum_observers.rs`
adapters add actual `is_some`, `is_none`, `is_ok` and `is_err` consumers for
owned String/vector/nested sum origins. They inspect the original root/loan
state before using the declaration-owned constructor paths. The generated
surface term matches the original receiver once and uses only wildcard fields:
it creates no receiver binding, payload move or receiver Copy evidence. The
receiver remains available to subsequent observations and consuming operations.

Immutable reference layers retain their complete lifetime metadata, including
checked elided parameter lifetimes, while local references continue to check
all actual root/loan states. Referenced Copy sums use this same observer path
because a Copy field projected from an owned enclosing constructor still has
its real transitive root loan. Direct Copy sums retain the existing dispatcher.
Unknown ownership, missing provenance, moved/dropped roots and illegal later
moves from borrowed places keep their explicit refusals.

`rust-owned-standard-sum-observers.mjs` contains 33 independent rule controls,
nine complete original programs and nine complete refusal programs. The
originals retain repeated observations before consuming String/vector/nested
matches, shared parameters and locals, dereferenced borrowed places, a
single-evaluation producer trace, and a borrowed `Option<u8>` field of an owned
`Result<Option<u8>,String>`. Refusals retain direct and transitive use after
move, an expired root loan, illegal shared payload extraction and arity/family
errors. Every original and refusal is bounded to Rust `>=1.90.0 <1.100.0`.
The coordinator owns appending these inputs to the common full source-CST,
frontend and target controls. All controls and generated native policy work
remain deferred; only source reads and edits were performed.

## Delayed owned pattern transfers in guarded arms

`rust-standard-sum-guard-rules.js`, `rust-standard-sum-guards.js` and native
`standard_sum_guards.rs` supply the deferred-mode source decisions and actual
pattern shape transport. The existing immediate-mode guarded-move refusal
policy remains intact. Root match readers select deferred mode for a guarded
owned standard sum, build a readonly pattern scope without owner allocation,
check guard reads and preserve the original root states, then reconstruct the
ordinary case scope to commit moves only for the body path. Every following
arm starts from the original ownership snapshot. Constructor wildcards remain
nonmoving. Native readonly preview restores its parser-owned wildcard-reference
queue so the second committing traversal uses the same source records.

The implementation uses actual source binding/loan and owned-producer facts,
including readonly inspection of fresh nested sum producers with no fabricated
owner identities. The initial restricted alias/or admission policy remains in
the literal boundary controls; the expanded scope model is described below.
The native constructor projection also corrects a misplaced
`standard_sum` declaration that referred to a constructor-only type entry from
the tuple arm; its actual declaration now belongs to the constructor arm.

`rust-standard-sum-guards.mjs` retains ten guard decisions and four owned
binding registration decisions, ten complete original programs with literal
stdout and eight complete refusals, all bounded
to Rust `>=1.90.0 <1.100.0`. One positive repeats the exact historical guarded
input with its independently declared original compiler expectation and retained
previous translation expectation. Coordinator routing retains the former owned
guard refusal as a complete previous boundary and admits its identical source
with the original compiler expectation. The new originals include false-guard fallthrough consuming
the original root inside the same match, actual true-body moves, repeated
readonly reads, nonbinding guards and nominal Copy behavior. Controls remain
source drafts; no verification or runtime observation has occurred. Nested
guard block scope entry uses the new shared owned-registration rule to leave a
borrowed place unchanged; it cannot manufacture an ownership identity for a
readonly payload projected from a fresh temporary. Positive readonly block and
negative attempted-move originals retain that boundary explicitly.

## Guarded alternatives and safe whole-value aliases

The expanded guarded source path compares complete canonical source type
identities and the original root/place/loan/lifetime/producer records for each
binding before joining alternatives in the readonly guard scope. The body
scope still uses ordinary case transfers after the guard is checked. This
admits disjoint `Ok(text)|Err(text)` on `Result<String,String>` and overlapping
`Some(text)|Some(text)` without discarding ownership. The existing guarded
lowering already attempts each matching alternative separately: a false guard
can run again on the next overlapping alternative; a true guard enters its
body immediately. No replacement evaluation or move semantics were added to
the native adapter.

Whole-value aliases are admitted when each alternative binds at most one owned
value. Wildcard fields add no move, and independent Copy fields remain usable
after the whole value is transferred. A whole alias combined with an owned
field binding retains an explicit partial-move agreement diagnostic. The first
profile's restrictive shape policy and controls are retained separately from
the new actual-consumer admission policies.

`rust-standard-sum-guard-alternatives.mjs` adds 13 shared rule controls, eight
complete original programs with literal stdout and six complete refusals.
Originals retain disjoint and overlapping guard traces, safe whole aliases,
actual temporary production, nested owned aliases and Copy field use after
whole transfer. The exact formerly refused OR input is repeated with its
previous translation expectation; coordinator routing owns retaining the old
slot as history and admitting that complete source. Missing names, conflicting
actual payload types, owned alias partial moves and attempted guard moves
retain explicit negative originals. All are bounded to Rust
`>=1.90.0 <1.100.0`. Source drafts alone have been read and edited; generation,
execution, validation and compiler controls remain deferred.

## Same-kind JavaScript Map and Set copies

Source reads identified `new Map(env)`, `new Set(reserved)` and
`new Set(this.used)` in `js/src/translation/ir.js` as concrete dependencies
that the array-only constructor admission could not represent. The broader
closed-class/module boundary remains intact: ordinary import/export linkage,
external class consumers and instance escape still need complete source proofs.
The new closed Scope-shaped original is a local class control, not a claim
that imported or exported classes now translate.

`collection-copy-rules.js` and `collection-copies.js` add shared decisions and
actual solver/checked-graph transport. Unknown constructor inputs wait for
ordinary call and method facts. Only an actual resolved same-kind Map/Set
selects a copy; the established unresolved array contract is settled only at
the ordinary solver fixed point. Known arrays retain their original immediate
path and constructor oracles. The native inference adapter uses those same
shared decisions, retaining actual term owners and updating pointer-keyed
expression/method observations when an original operand root moves into its
existing `ArrayFrom` wrapper. Full source spans and the original concrete
source graph remain independent of that surface normalization.

Proven copies snapshot the input once and allocate a new outer collection
through the established constructor/reference model. Checked `collectionItems`
preserves insertion order, primitive SameValueZero key admission and nested
reference handles. The focused `Array.from` checker correction reads an actual
reference Map/Set payload before making its snapshot; immutable and array
paths keep their previous behavior. No new iterator, typed IR or target runtime
operation was introduced. Native transport changes are in the existing
collection/Array.from checkers and JavaScript inference seams.

Fully checked snapshot mappers reuse the established readonly callback graph
proof. The native emitter delegates to the extracted common proof without
changing its decisions. A writing mapper needs live ordered iterator slots;
an unknown mapper factory also needs the original argument-before-iteration
effect order. Those cases keep explicit refusals, complete original programs,
literal original stdout and history. One-argument constructor snapshots do not
have this mapper factory problem. General/custom iterables, structured keys,
cross-kind inputs and inconsistent monomorphic caller domains remain explicit
source boundaries.

`collection-copies.mjs` retains 21 scalar shared decisions, 12 complete original
programs, nine complete refusals and three exact own-source anchor strings.
The JavaScript test, bounded producer recipe and native CI test retain default
JavaScript CST parsing, every physical ordered field/span/flag, exact source
bytes, explicit AST projection without mutation, source and typed text/binary
replay, all four emissions and independent literal stdout. The native control
also compares actual frontend surfaces and graphs, checks each transport/target
cell and compiles/runs each emitted Rust program only in CI. Source controls
select ECMAScript 2024-2025 on Node `>=22.15 <23 || >=24 <25`, and report
uncovered Node versions; native target controls select Rust `>=1.90 <1.100`.
These are authored controls and producer recipes, not observed results. No
test, generator, validator, compilation, runtime execution or Git command ran.
