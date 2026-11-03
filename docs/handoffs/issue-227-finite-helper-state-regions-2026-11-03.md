# Finite helper state regions source draft

This follows [loop-aware overwrite liveness](issue-227-loop-aware-overwrite-liveness-2026-11-03.md)
and the [owned fold interface](issue-227-owned-loop-region-interface-2026-11-03.md),
under the sole architecture in [vision](../vision.md). No represented code,
tests, generation, formatters, validation gates or compilation ran. No CI
feedback was inspected. The active objective retains the complete prior scope.

## Actual source path drafted

`rust-source-state-transforms.js` attaches a distinct `sourceStateTransform`
interface to actually admitted finite-loop chunks before the existing helper
seal captures them. It retains ordered original source binders and closed
positions, actual Unit/nominal completion and payload, and the original loop
span. It is a compiler function ABI, not an invented Rust source signature.
Shape checking validates the real generated function, each input/output state
type, distinct binder inventory, complete control layout and exact fields. It
rejects competing source signatures, unrepresented containers and stored proof
revisions. Existing source counter/progress, ownership and helper admission
checks remain independent and unchanged in strength.

`rust-source-state-transform-regions.js` checks each actual helper body, even
when the caller has already used its summary. Parameters seed that function's
local closed storage; the body must publish every current state slot and cannot
export local storage. Calls acquire exact current source binders/revisions and
publish new state snapshots through the existing packet interpreter. Completion
matches reuse the explicit compiler control roles. Closed parameters contain
no reference regions: any borrowed completion payload must be static, so a
call summary cannot invent a loan from the caller. Payload lifetime names are
abstract compiler type parameters, not authority to create a runtime region.

The helper graph walk derives actual static edges and rejects reachable cycles,
dynamic application and missing callees. An encoded `recursive: false` or
`mutual: false` cannot replace that derivation. Proof-local roots, lifetimes,
binders, temporaries and inference constraints now reset between independent
function/main scopes; cumulative visit, loan, position and branch budgets remain.
This avoids retaining irrelevant prior-function coordinates in later branch
snapshots without weakening the cumulative resource gates.

The normal checker, typed Links validator, datatype-reference walker and checked
declaration projection preserve the interface. Projection also retains the
existing finite-helper marker. The source schema author adds their vocabulary;
generated schemas and native policies remain untouched during this phase. New
source modules enter the existing automatic source-author receipt inventory.

## Record projection prerequisite drafted

`rust-records.js` now emits wild patterns for unselected projection fields. The
selected field has the same original position and value; unused String/record
payloads are no longer bound merely to read a primitive counter. The finite
counter-place recognizer accepts exact bind/wild layouts, still requiring the
real source state read, constructor identity, field arity and selected binder.
Old all-bound layouts remain recognizable, and duplicate/other patterns refuse.

In the source-region visitor, constructor-arm consumption follows actual bound
field positions. Copy and immutable-reference bindings alone preserve the owner;
wild fields do not move. A named non-Copy payload still consumes its owner even
when unused, and whole-value bind patterns consume through their actual lexical
binder within that arm. All branches retain independent ownership joins. This
does not infer nominal Copy traits from scalar-looking layouts or allow arbitrary
partial moved-field access.

Ordinary Copy transfer also detaches the resulting value from its lexical alias
root. The original scalar/reference place remains available; references retain
all actual referent loans. A copied scalar or shared reference may outlive its
inner alias binder, while a reference to an expired local referent still fails.
Mutable-state packet transport retains its independent exact storage identity.

## Independent controls drafted

Ten appended mixed-profile originals cover Copy/wild record patterns, real
record reconstruction, zero/nonzero finite loops, nested state binders, readonly
results after finite loops, break completion and scalar/reference alias scope
exit. Three invalid-Rust originals retain unused named String moves, whole-value
bind moves and an escaping local referent despite reference copying. Existing originals,
their inputs, earlier frontier history and other refusal assertions remain.

Additional graph controls inspect all 32 actual u8 chunks, exact source spans
and binders, ordered state signatures, malformed interfaces and removed facets.
A forged self-call with false recursion flags must fail the independent actual
graph check. Links text/binary controls retain interfaces and compare all four
emitter products. All controls remain unexecuted and retain explicit versions.

Inventory at this checkpoint: 438 positive inputs in nineteen groups, 374 literal
originals plus 64 actual-source scenarios. The mixed group has 73 positives and
25 refusals. These count authoring inputs, not passes or completed requirements.
The existing original/product CI driver imports them automatically; it did not
run locally. Generated asset publication remains deferred with testing.

## Remaining full-scope work

- Extend finite-helper result/abort/residual paths against actual enclosing source
  interfaces, including source bodies that need nonlocal borrowed parameters.
  Existing closed-capture admission still applies. Closed parameters cannot
  supply nonstatic result loans; adding real borrowed captures requires an
  explicit argument/return-region ABI, not a fabricated source header.
- Exact source iteration boundaries were subsequently drafted in
  [finite iteration local scopes](issue-227-finite-iteration-local-scopes-2026-11-03.md),
  including body packet transport and controls. General borrowed completion and
  field-place paths below remain open; this successor has no execution evidence.
- Generalize record place regions for borrowed/non-Copy projections, partial
  moves, disjoint field loans and projected exclusive calls. Primitive field
  reads do not establish complete field-reference or arbitrary nominal Copy
  semantics. Preserve invalid named-binding and whole-owner move controls.
- Extend loop initialization-state carriers and source returns across loop-aware
  overwrite analysis, retaining zero iterations, all continuing paths and every
  actual scope/loan exit. Shared collection/signature-3 composition and borrowed
  callback values still require their complete genuine interfaces.
- Automatically publish the pending schema, source receipts, naming/register
  changes and translated native policies once every requirement has a full code
  draft. Preserve all predecessor source/fixture/alias coverage. Never handwrite
  replacement Rust behavior or edit generated output to simulate translation.
- Continue full own-project parsing/emission/translation, JavaScript constructor
  frontiers and the broader grammar/concept/ontology obligations in vision. This
  source batch does not replace the full objective with a finite-loop milestone.
