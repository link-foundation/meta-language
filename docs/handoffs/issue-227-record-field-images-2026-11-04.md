# Record component ownership image draft

Continues [Box receiver operations](issue-227-box-receiver-operations-2026-11-04.md)
under the sole architecture in [vision](../vision.md). Source edits, textual
inspection, Git metadata and primary specification reads only. No test,
represented program, generator, formatter, validation gate, compilation or CI
feedback ran or was reviewed. All controls remain unexecuted. The full drafting
goal remains active; there is no audited drafted/total count or completion rate.

## Code paths and retained controls

`rust-source-field-images.js` derives private component roots from actual closed
single-constructor declaration inventories, using existing ownership/read and
resource policies. Actual components have independent availability and containing
lifetimes. Root images are prepared at binding, parameter referent initialization
and storage publication; branch snapshots therefore include owned components.
Canonical Box wrappers retain their original public positions while actual
record layouts share the same field inventory. No new serialized shape, mask or
trusted epoch was added. The module is registered as a source authoring consumer;
automatic native publication remains deferred.

`readRustRecordPlaceRegions` uses the independently checked original projection
graph to inspect its actual variable/container or nested projection without
promoting a partially moved whole into an ordinary value. Type/owner/wrapper/
lifetime/qualifier agreement is checked before variable inspection; other owner
expressions retain normal whole reads. Selecting a field retains its own storage
root and containing lifetime roots. Owning transfer consumes the component and
clears its former coordinates; direct siblings remain initialized. Whole reads
still inspect the complete component tree.

`SourceRegions` now separates referent loan roots from containing lifetime roots.
Whole shared references keep whole-image restrictions; direct field loans retain
their actual selected component. Opaque readonly helper references may expose a
different declared record type under an existing loan root; these get a separate
readonly field view, preserve inherited whole loans, and cannot relabel or move
the original owner's storage. Field-region lifetimes derive from actual parent
regions. Returned regions must be actual declaration/view descendants of allowed
parameter roots, rather than unrelated regions with matching lifetime spelling.

Branch storage joins rebase only the returned value's real component/loan/lifetime
coordinates onto the new current root. Original aliases retain stale coordinates.
Joined reference alternatives retain all referent roots. Borrowed vector/index
operations keep selected referent roots and containing lifetimes; scalar copying
and fresh String materialization clear old coordinates. Complete masked state
transport/reinitialization is not supplied by ordinary full-value publication.

Sixteen positive and twelve invalid originals are appended under existing
explicit Rust/Node ranges. They cover direct/nested/boxed transfers, sibling
String/scalar access, field escape into fresh owned storage, conditional moves,
direct/helper/vector field loans, repeated field transfers, current-root reference
joins, alternative loans and scalar copies outliving a partial owner. Invalid
inputs retain moved-field/whole-owner reads, whole borrowing, duplicate/nested
moves, overlapping and escaping loans, conditional availability and containing
record lifetime for indexed references. Independent controls preserve unchanged
public facet keys, absence of archived image coordinates, a forged same-type
sibling projection that passes the local declaration proof but must fail ownership,
the three preceding Vec/sibling inputs unchanged, and Links text/binary/four-target
transport. These counts are authored inputs, not requirement or pass counts.

The three receiver originals identified in the preceding handoff are retained
with their sibling reads after `let values = *value.values;`. This batch supplies
the independent component-transfer code path they require; no execution has
established that they pass or that broader partial ownership is complete.

Primary contracts: [Rust places and moves](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions)
and [partial moves](https://doc.rust-lang.org/rust-by-example/scope/move/partial_move.html).
Existing custom Drop, stored-reference, recursive state and resource restrictions
remain. No assertion, prior input, oracle or gate was removed or weakened.

## Remaining actual code, in implementation order

1. Draft ordinary selected-field reinitialization and masked state publication.
   `rust-owned-record-state.js` already tracks parser field identities and
   reinitialization; its source effect lowering still reconstructs an ordinary
   whole match. After a component moves, that whole read is correctly refused by
   the new independent image checker. Add an explicit checked field-assignment
   extraction/rebuild plan in `rust-source-place-replacements.js` (or a separately
   defined source-place model with generator input updates), retaining actual
   RHS-before-place order, complete field positions and original binders. Prove
   that only the selected target is overwritten and no stale moved component is
   read as an owning sibling.
2. Carry actual partial images across that checked operation. Its region adapter
   in `rust-source-place-transform-regions.js` must inspect only the actual
   original container lifetime and selected path, allow an uninitialized target
   when ancestors are initialized, retain all other component availability, and
   initialize exactly the target's new image. The resulting private state carrier
   may transport a partial record only through the independently checked plan and
   matching actual storage identity. Ordinary source positions, constructors,
   tuple fields, caller-supplied masks or a transport spelling cannot create a
   fully owned whole from stale payloads. Whole reads/calls/returns remain blocked
   until all fields are initialized. Integrate complete/partial state packets,
   projections, binding, branch joins and scope expiry together.
3. Finish field writes/calls on partial containing roots and disjoint surviving
   field loan revisions. Current call/intrinsic controllers require complete root
   input and publish a new whole epoch. Preserve exact selected targets and
   untouched partial siblings while transferring only actual changed components;
   stale overlapping references stay invalid. Whole-root overwrite remains a
   distinct operation. Retain source-free wrong-path, forged carrier, omitted
   sibling, stale revision and incomplete whole-use controls.
4. Finish mixed move/ref constructor patterns, tuple/product and multi-constructor
   partial images. The current ordinary owned match path still consumes its whole
   scrutinee when any bound field owns a value. Actual per-binding source modes,
   declaration indices and variant witnesses must select transfer versus loan;
   pattern names alone cannot grant Copy or ownership. Preserve custom Drop
   restrictions and exact match guard/fallthrough observations.
5. Complete temporary/fresh projection/receiver storage and lifetime extension,
   general per-layer lifetimes, reference-bearing/finite recursive state, actual
   source alias/reexport identities, custom Deref/traits/arbitrary self receivers,
   full last-use ending and implicit function-reborrow phases. Opaque readonly
   views deliberately retain their original whole loans; precise result-path
   summaries require independently checked actual bodies.
6. Complete actual low-level payload context APIs, Vec<Box>, generalized iterators,
   allocator/Drop/unsafe observations, borrowed/escaping/field callback environments
   and Box captures/results/state, finite helper borrowed captures/results/residuals,
   zero-iteration state and iterator short-circuit/drop. Do not infer these from
   direct single-constructor component images.
7. Complete own-project default CST parsing/emission and both JavaScript/Rust
   translation directions, including modules/generics/traits/macros. Preserve all
   JavaScript dynamic Function, callback/async, derived/super, Proxy/private-field,
   mutable-cell/effect and live-root obligations in preceding handoffs.
8. Complete full grammars/global concepts/deep formal definitions/equivalence,
   recursive any/all parsing, ontology/taxonomy, RDF/OWL import/conversion/merge,
   dynamic Links/link-cli reconstruction, grammar inference, formal systems and
   distribution. Every broader obligation in vision remains binding.
9. Only after every code/control draft: reconcile registered sources online before
   ledger use, publish automatic source/schema/naming/native assets, run authorized
   JavaScript and CI-only native gates, repair actual failures and record observed
   assertions at their executed commits. Never compile Rust locally.
