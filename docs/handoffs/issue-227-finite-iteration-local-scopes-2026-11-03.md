# Finite iteration local scopes source draft

This continues [finite helper state regions](issue-227-finite-helper-state-regions-2026-11-03.md)
under the sole architecture in [vision](../vision.md). Only source edits and
source review were performed. No represented program, test, generator,
formatter, validation gate or compilation ran, and no CI feedback was read.
The full issue objective remains active; this is not a completion report.

## Source behavior drafted

`rust-owned-record-loops.js` compiles each admitted finite body into its own
result/state packet. The outer generated let receives that packet only after
the source body's local let scopes end. It projects the actual selected outer
state into fresh SSA bindings, then starts the next iteration. The previous CPS
continuation placed later iterations inside a body's mutable local scope, so
the next initialization of the same source binder could encounter storage from
the previous iteration. The new packet boundary retains the existing source
binder and creates the next storage epoch after the old one expires.

The first field retains the actual Unit or completion result. Body observations
are evaluated even when the result is Unit. Existing FunctionResult,
StatementContinuation, LoopBreak and LoopContinue dispositions are processed
outside body-local scopes. A false guard returns current outer state without
evaluating local initializers. The exact u8 decrement/progress derivation,
255-iteration domain, eight-iteration chunks and independent body/region checks
are unchanged. No extra source state slot, signature or loan authority is added.

The expression copier retains actual expression origin links and shares exact
immutable type, span, sourcePosition, sourceStateTransition and sourceLoopFold
objects by identity. Other syntax objects are copied with a bounded WeakMap
walk. This avoids copying the same deep formal source fact into every unrolled
expression. It does not merge independently defined facts or source binders.

Preflight measures the syntax that is actually copied. Before sealing each
helper, an additional cumulative materialization walk charges all retained
syntax, property keys, text and immutable facts, with shared objects charged
once. It uses the existing 100,000-node and 16,777,216-text caps. The independent
helper witness still checks its complete unfolded serialization and original
limits. The policy formulas, progress gate and witness checks were not removed
or relaxed. Source drafts still need deferred execution to establish whether
each complete fixture fits every independent budget.

## Independent controls drafted

Eight appended native-original inputs cover fresh mutable scalar and owned
record locals, local borrowed vectors, continue and break, distinct same-name
block binders, zero iterations and nine iterations crossing the actual chunk
boundary. Two invalid originals retain an escaping local reference and a
same-iteration use after move. Existing inputs and refusals are preserved.

Structural controls require all 255 local initializations across 32 helpers,
with neither a later initialization of that source binder nor any finite helper
call inside its lexical body. A separate control checks exact shared position
identity among distinct copied expressions and the retained helper graph bound.
Both retain normal checking and JavaScript output assertions for later use.

The authored inventory is now 446 positive inputs in nineteen groups: 382
literal originals and 64 actual-source scenarios. The mixed composition group
contains 81 positives and 27 refusals. These are input counts only, not passing
requirements, observed behavior or a completion percentage. Native original and
all translated products remain governed by the existing version-bounded CI
driver; no runner or version range changed.

## Remaining work and next implementation paths

1. Extend finite-helper completions using actual enclosing source return and
   loan ABIs. Closed-state helpers currently cannot supply a nonstatic result
   loan. Add explicit real borrowed capture arguments and return-region mapping
   before admitting those values. Cover ordinary return, question-mark residuals
   and diverging/abort paths; a closed helper summary must never invent a region.
2. Extend actual record-place regions for borrowed/non-Copy field projections,
   partial moves, disjoint loans and exclusive projected calls. Start from the
   real field path, lexical storage and current epoch; reconstruct only actual
   mutated paths. Preserve old-alias, whole-owner and named-binding move controls.
3. Carry actual initialization/move state through loop boundaries, including
   zero iterations. An iteration that initializes a root does not prove it was
   initialized before the first guard or after a zero-iteration exit. Model
   continuing, break, return and residual states separately rather than
   manufacturing an initialized carrier.
4. Complete shared collection/signature-3 composition and borrowed iteration
   interfaces. Establish real source owner, input/item, accumulator, callback
   capture and result-region contracts before removing their existing guards.
   Closed-body borrowed values and escaping callback references need the same
   complete interfaces and actual body checks.
5. Continue complete own-project JavaScript/Rust parsing, emission and both-way
   translation, including module and constructor frontiers. Preserve the full
   grammar, concept-space, recursive definition, ontology/taxonomy, RDF/OWL,
   dynamic reconstruction and inference scope in vision and prior handoffs.
6. After every requirement has its full code draft, perform mandatory online
   source reconciliation before using the ledger; automatically publish source
   receipts, schema/naming changes and translated native policies. Then execute
   the permitted JavaScript checks and CI-only native/whole-project gates.
   Record observations only after execution at the exact recorded commit.

These paths continue, rather than replace, the detailed remaining-scope case
study and preceding handoffs. Full drafting and verification remain unfinished.
