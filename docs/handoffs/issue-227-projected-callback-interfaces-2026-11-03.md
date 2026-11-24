# Projected callback call interfaces source draft

This continues [Vec helper interfaces](issue-227-vector-mutation-helper-interfaces-2026-11-03.md)
and [projected source calls](issue-227-projected-source-call-regions-2026-11-03.md)
under the sole architecture in [vision](../vision.md). Only source edits and
review were performed. No represented program, test, generator, formatter,
validation gate, compilation or CI-feedback review ran. The complete issue
objective and all preceding obligations remain active.

Continuation: [direct record field places](issue-227-record-field-place-regions-2026-11-03.md)
adds actual containing-storage loans for the initial closed-field profile. The
remaining callback capture and whole-project obligations below remain binding.

## Actual source/compiler path drafted

The generic callback reader previously collected hidden capture borrows but did
not acquire its original explicit mutable formals through a selected source
argument frame. `rust-exclusive-source-calls.js` now exposes the same actual
bounded acquisition frame for that reader, including current formal selection,
implicit reborrow of a genuine exclusive formal and ordered frame exit. Existing
place, owner, overlap, initialization and surviving-loan checks remain. Hidden
capture acquisition also retains the existing frame-depth bound.

`rust-fnmut-source-functions.js` keeps that frame live through parsing, actual
source specialization and hidden capture acquisition. The call retains both
original explicit borrows and hidden captured-root borrows in the actual checked
specialization packet order. The catalog packet's field inventory follows that
same complete order, rather than describing only hidden captures. Free functions
and the existing immutable receiver method path use the same controller. Original
source declarations and callback bodies are still actually parsed and admitted.

Ordered lowering keeps an actual selected inline lambda in ordinary and projected
calls instead of wrapping it in a generic function-value alias. Existing internal
formal forwarding keeps its previous path. Projected call validation accepts the
actual typed callback only through its genuine source FnMut specialization. Every
hidden capture writeback must retain the exact original binder identity, name and
token span, and select a whole captured root. Explicit projected fields and hidden
state remain disjoint actual places with complete root reconstruction.

`rust-closed-callback-regions.js` derives every free binding from the checked
lexical body, handling actual let, pattern and nested lambda scopes under the
existing work/depth limits. It does not trust an omitted or supplied capture list.
An optional supplied list must agree with the derived graph. Free immutable
captures must be live direct Copy values with no loans or mutable storage
identity. Mutable captures use actual hidden state parameters/results. Ordinary
and projected closed calls share these checks. Callback source positions may not
be erased to pass the closed-body guard; actual borrowed callback interiors still
require their separate interface.

The existing source FnMut header validator retains parameter/result/capture
positions, original declaration/body ranges, versions and packet correspondence.
The normal checker still type-checks the real callback and selected specialization
bodies; this batch does not substitute a native dispatcher or fabricated header.
Existing Links projection and datatype edges retain the callback expression and
both source interfaces. Automatic source-author inputs include the new checker
module. Generated receipts/native policies remain deferred to automatic
publication after every requirement has its full code draft.

## Independent controls drafted

Thirteen positive originals cover whole and projected explicit mutable formals,
immutable Copy captures, actual mutable scalar/record captures, two disjoint
fields of one root, nested ancestors, owning results, repeated callback state,
conditional aborts, implicit reborrow through an actual exclusive relay,
independent specializations and observable callback invocation order. Six invalid
originals preserve overlapping formals/captures, old loans, FnOnce-only ownership
transfer, incorrect result domains and immutable receivers. No input or assertion
was removed; all retain the existing explicit language/dependency/target ranges.

Structural controls inspect the actual inline callback and `[explicit, hidden]`
packet order, exact root/capture binder correspondence and real typed lambda
parameters. They forge capture roots, replace callbacks with aliases, duplicate
parameters and inject owning or mutable-root free reads despite an omitted/empty
capture list. Immutable captures are also checked with no declaration present.
Complete Links text/binary reload controls retain same-type distinct roots,
callbacks and all four target emissions. These controls remain unexecuted.

Latest authored inventory: 503 positives in nineteen groups, 439 literal
originals plus 64 actual-source scenarios. The mixed composition group has 138
positives and 48 refusals. These count inputs, not observed passes, drafted
requirement counts or a requirement completion percentage.

## Remaining full-scope work

1. Add exact field-granular callback captures. Current capture state uses actual
   whole roots; a callback capturing a disjoint field of the same root as an
   explicit mutable argument still needs a new real field capture/return ABI.
   Preserve overlap checks and original callback/source binder correspondence.
2. Complete borrowed/owned escaping and nested mutable callback environments,
   reference-bearing callback interiors, function aliases and general mutable
   receiver methods. Their actual creation-time transfer, lifetime, receiver and
   invocation contracts cannot be inferred from this immediate closed profile.
3. Add full compiler interfaces/body proofs for iterator-generated callback
   helpers and other mutations. Existing internal formal forwarding is retained;
   this batch does not claim every generated iterator helper now has an external
   projected-call ABI. Continue general field-reference regions and storage.
4. Complete finite-helper borrowed capture/result/residual interfaces, loop
   initialization-state carriers, borrowed iteration and shared collection
   composition. Continue full own-project JavaScript/Rust CST, emission and both
   translation directions plus constructor, grammar/concept, ontology/taxonomy,
   RDF/OWL and dynamic inference obligations in vision.
5. After all requirements have full code drafts, reconcile sources online before
   ledger access, automatically publish pending source receipts/schemas/naming
   changes/native translations, and run authorized JavaScript plus CI-only
   gates. Record observations only afterward at the exact executed commit.
   Never compile Rust locally or treat authored controls as execution evidence.
