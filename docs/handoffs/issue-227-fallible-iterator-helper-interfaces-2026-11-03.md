# Fallible iterator helper interfaces source draft

This continues [record field places](issue-227-record-field-place-regions-2026-11-03.md)
and the earlier [Vec helper interfaces](issue-227-vector-mutation-helper-interfaces-2026-11-03.md)
under the sole architecture in [vision](../vision.md). Only source edits and
source review were performed. No represented program, test, generator,
formatter, validation gate, compilation or CI feedback ran or was reviewed.
The complete requirements draft and active goal remain unfinished.

## Actual implementation path drafted

`rust-fnmut-source-iterators.js` already constructs the actual executable fold for
the selected contiguous readonly standard Vec `iter().map(...).collect::<Result>`
path. This batch reuses that algorithm and adds a distinct generated declaration
facet through `rust-fallible-iterator-interfaces.js`. It does not manufacture an
original Rust function header, replace the fold with native semantics, or infer
general iterator behavior from a Vec example.

`Rust fallible iterator compiler interface 1` retains every actual formal source
position, the shared item, mapped Result, collected Result<Vec> and original
span. Header checking normalizes these against the actual checked declaration
and its real standard Result constructors. It checks all input/output/error
owners, exact callback parameter/result state order, complete packet arity and
the selected readonly standard Vec/item domain. Equal erased primitive widths
do not establish equal source owners. State positions are direct closed values;
the generated helper cannot compete with a source callable/state interface.

The independent complete-body check follows actual lexical bindings and typed
operations under a 100,000-node inspection bound. It checks the initialized fold,
empty Ok success vector and ordered initial state. The existing Err branch must
return the entire actual accumulator, without another callback invocation. The
Ok branch invokes the actual mapper exactly once with the current item and every
current state slot. Its new Err must propagate the actual error and all updated
callback slots. Its new Ok must append only the actual mapped payload after the
complete preceding prefix and publish those same updated slots. Duplicate or
shadowing generated binders, changed callback selection/arguments, wrong
constructor/payload/prefix wiring and competing source proof facts refuse.

This preserves observable callback cessation at the first error. The existing
fold can still visit remaining Vec elements without invoking the mapper; it
does not implement physical early iterator termination or arbitrary custom
iterator side effects. The readonly contiguous receiver restriction remains
necessary. General short-circuit iterator/resource protocols remain full-scope
work rather than being claimed from this helper.

The ordinary closed call view exposes the real readonly vector, callback and
ordered exclusive state parameters/results. This compiler callback view includes
its actual item and explicit state formal positions, rather than fabricated
original capture binders. A private parser catalog marker keeps actual inline
compiler callbacks inline during ordered ordinary/projected lowering. It is set
only when the new composition interface was actually attached. Existing internal
formal forwarding retains its route and receives no new generic alias authority.

The normal checker checks the interface before bodies and then the entire real
body. The source-region pass independently checks each helper, including unused
declarations, and checks closed target/output domains. Typed Links projection,
datatype-reference transport and independent validation retain all positions and
the actual executable body through source-free reload. Schema/source-author
inputs include the new concept and consumer. Automatic schema, receipt, naming
and native publication remain deferred until the whole requirements code draft.

## Controls authored, not executed

Six positive original programs cover mixed readonly-region composition,
state publication at the first error, empty input, owning mapped elements with a
different type from input, multiple distinct mutable roots, and actual callback
observation order. Two invalid originals preserve mutable iterator-source overlap
and a reusable callback attempting to move its captured owning value out.
Existing inputs, language/dependency version ranges and four target obligations
are retained.

Structural controls inspect the actual normalized helper, item borrowing and
complete callback packet. Forged variants change the interface shape, item
wrappers, vector owner, callback binders/selection, argument/initial/writeback
order, first-error branch, propagated error and mapped success payload. The
helper remains checked after clearing main effects, so unused code cannot hide
a bad body. Text/binary Links and all four target emissions retain a distinct
owning success element type. All controls remain unexecuted.

Authored inventory: 520 positive inputs in nineteen groups, comprising 456
literal originals and 64 actual-source scenarios. The mixed composition group
has 155 positives and 54 refusals. These are input counts, not audited atomic
requirement counts, observed passes or a completion percentage.

## Remaining work and next entry points

1. Complete the actual borrowed callback-body/lifetime interpreter and explicit
   forwarding environment. The immediate closed inline profile still rejects
   borrowed source qualifiers inside callback bodies. Internal forwarding does
   not prove a general escaping/function-alias ABI. Keep creation-time ownership,
   callback invocation and actual state identities explicit.
2. Add complete interfaces/body proofs for other iterator stages/terminals,
   source loops and mutations. Preserve real lazy or short-circuit effects,
   zero iterations, first error/residual state, owning payload transfers and
   actual iterator close/drop behavior. Custom iterators need their own complete
   receiver/state protocol; do not relabel them as contiguous Vec folds.
3. Finish finite-helper borrowed captures/results/residuals and initialization
   carriers; partial moves, disjoint surviving field loans and reference-bearing
   storage; field-granular/escaping/nested callbacks; and actual library shared
   collection composition. The preceding record place handoff retains details.
4. Complete full own-project JavaScript/Rust CST parsing, semantic emission and
   translation both ways. The full constructor/weak-liveness, all grammar/global
   concept/recursive definition, nested parsing, ontology/taxonomy, RDF/OWL,
   dynamic reconstruction and grammar inference obligations in vision remain.
5. After every requirement has an actual complete code draft, reconcile live
   sources before ledger use, publish automatically generated source/schema/
   naming/native assets, and run authorized JavaScript and CI-only full gates.
   Fix actual failures and record observations afterward at the executed commit.
   Never compile Rust locally or treat these authored controls as evidence.
