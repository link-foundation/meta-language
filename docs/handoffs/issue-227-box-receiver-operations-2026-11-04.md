# Box receiver operation draft

Continues [Box payload calls](issue-227-box-payload-calls-2026-11-04.md)
under the sole architecture in [vision](../vision.md). Source edits, textual
review, Git metadata and primary specification reads only. No test, represented
program, generator, formatter, validation gate, compilation or CI feedback ran or
was reviewed. The full drafting goal stays active. These authored controls are
unexecuted and are not evidence, audited requirements or a completion percentage.

## Actual source paths

`captureRustExclusiveSourceReceiver` now selects standard String/Vec methods from
the actual canonical payload while preserving the complete original mutable
place. Actual original lexical tokens/binders are checked by
`requireRustExclusiveSourceReceiverRoot`, preventing shared aliases into an owned
mutable root from acquiring receiver authority. Inherent calls derive projection
from their actual self formal, rather than stripping Box layers unconditionally.
Vec pop/truncate/clear build the existing actual direct Vec helper using the
payload view and publish through the checked complete Box call/rebuild route.

String push_str/clear and Vec push retain their actual direct operation while
keeping complete original target positions in source writeback. The actual
peeled referent is private construction metadata. The existing intrinsic facet
retains its shape; `rust-source-place-replacements.js` checks a payload qualifier
on the operation expression against the actual selected Box field, standard
String/Vec/element/suffix contracts and complete extraction/reconstruction graph.
An empty field path requires genuine canonical Box projection. Ordinary unboxed
root intrinsics retain their preceding path. Source effect lowering handles
boxed root intrinsics alongside existing field intrinsics. Vec push validates an
already transferred owning argument without reading its consumed binding again.

Implicit mutable method autoref acquisition now reserves its actual place during
arguments. The source read guard allows readonly observations before activation,
while write/acquisition overlap guards remain active. Activation checks root/
ancestor/selected/descendant initialization, surviving scope loans and actual
argument-carried references. Actual disjoint exclusive operands keep their own
selected-place proof. Private reservation flags are not serialized or accepted
as archived authority. Explicit &mut arguments retain active acquisition. General
implicit function reborrow phases and full last-use inference remain separate.

Twenty-three positive and twelve invalid literal originals are appended under
existing explicit Rust/Node ranges. They cover String append/clear/shared suffix,
Vec push/pop/truncate/clear/owning String elements, inherent boxed receivers,
fields and nested roots, disjoint RHS effects, readonly reservation operands,
shared/immutable/moved storage, surviving loans, wrong argument owners and owning
argument reuse. Independent controls inspect real payload/root positions, strict
intrinsic facet keys, forged projections, complete checked Vec helper bodies,
argument-before-operation ordering and Links text/binary/four-target transport.
All remain unexecuted; no original input or assertion was removed or weakened.

Source review identified a remaining dependency in three new positive controls:
`boxed Vec push operands publish disjoint sibling effects before actual replacement`,
`boxed Vec field pop reconstructs the latest containing root and its untouched sibling`,
and `boxed Vec field truncate retains the actual standard vector payload and containing sibling`.
They move an owning boxed Vec field and then read a containing sibling. Current
region transfer consumes the containing root, so the general partial-move code
below remains necessary. Retain these inputs and assertions; do not reclassify
them as invalid Rust or remove the sibling observations to hide the gap. This is
a source inspection finding, not an observed test failure or executed result.

All changed consumers are already registered in the source authoring recipe.
No generated source/schema/native bytes were edited, and no handwritten Rust
replacement or native dispatch path was added. Automatic publication stays
deferred until the complete code/control draft.

Primary contracts: [Rust method receiver resolution](https://doc.rust-lang.org/reference/expressions/method-call-expr.html),
[two-phase method borrows](https://rustc-dev-guide.rust-lang.org/borrow-check/two-phase-borrows.html),
and the existing [Box ownership contract](https://doc.rust-lang.org/std/boxed/index.html).

## Remaining actual code, in implementation order

1. Draft genuine selected-field ownership transfer in
   `rust-record-place-regions.js` and `SourceRegions` in
   `rust-borrowed-source-regions.js`. `readRustRecordPlaceRegions` currently returns
   the containing owner's root/storageRoot for an owning field, and `read(...,
   'move')` consumes that whole root. Track actual field paths and initialized/
   moved components independently of the containing storage lifetime/revision;
   transfer only the selected owning field into fresh result storage. Permit
   actual untouched sibling projections while rejecting moved-field reads and
   whole-owner use until every component is initialized. Reinitialization and
   field/call/intrinsic reconstruction must retain the exact private partial image
   and update only actual selected paths; a constructor must not claim that
   stale moved payloads have become owned values again. Ordinary whole reads,
   moves, calls, joins and scope expiry must use actual complete availability.
2. Make projection-owner access explicit and independently checked. A source field
   projection may inspect its real owner image to select an initialized sibling;
   this cannot become a general bypass for borrowing/calling with a partially
   moved whole value. Preserve exact declaration indices, source binders and
   original roots across nested Box/record fields. Add retained native originals
   for moved-field/whole reads, disjoint siblings, reinitialization, branches,
   nested scopes and source-free forged partial-image operations. Integrate with
   parser field identities in `rust-owned-record-updates.js` rather than adding an
   unrelated ownership notion or upgrading Copy evidence.
3. Finish per-field surviving loan revisions and reference-bearing/recursive
   state. Existing region epochs still conservatively cover containing roots.
   Preserve whole-owner loans, exact field loans, initialized-path checks and
   actual parameter/result lifetimes. Complete per-layer lifetime substitution,
   temporary lifetime extension and finite constructive recursive witnesses.
4. Finish general readonly explicit payload autoref before owning dereference,
   temporary/fresh receiver places, custom Deref/traits/arbitrary self receivers
   and canonical source alias/reexport proofs. Complete last-use loan ending and
   implicit mutable function reborrow reservation/activation; the new source
   phase currently applies to method autoref, not every two-phase compiler case.
5. Extend actual low-level context places/calls to the same Box payload protocols.
   Complete Vec<Box>, generalized iterators, allocator/Drop/unsafe observations,
   borrowed/escaping/field callback environments, Box capture/result/state,
   finite helper borrowed captures/results/residuals, zero-iteration state and
   actual iterator short-circuit/drop. Previous handoffs retain concrete entry
   points; receiver examples do not reduce any of this scope.
6. Finish full own-project default CST parsing/emission and bidirectional
   JavaScript/Rust translation, including modules/generics/traits/macros. Retain
   JavaScript dynamic Function, callback/async, derived/super, Proxy/private-field,
   mutable-cell/effect and live-root requirements in the preceding handoffs.
7. Finish full grammars/global concepts/deep formal definitions/equivalence,
   recursive any/all parsing, ontology/taxonomy, RDF/OWL import/conversion/merge,
   dynamic text/binary/link-cli reconstruction, grammar inference, formal systems
   and distribution. Every broader requirement in vision stays binding.
8. Only after all code/control drafts: reconcile registered sources online before
   ledger use; publish automatic source/schema/naming/native assets; run authorized
   JavaScript and CI-only native checks; repair actual failures and record all
   observed assertions at executed commits. Never compile Rust locally.

The [component image successor](issue-227-record-field-images-2026-11-04.md)
drafts direct field transfer and sibling selection, retaining the three inputs
above. It records remaining reinitialization/publication work; no controls ran.
