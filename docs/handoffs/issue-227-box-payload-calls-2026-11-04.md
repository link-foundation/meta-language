# Box payload call projection draft

Continues [Box payload assignments](issue-227-box-payload-assignments-2026-11-04.md)
under the sole architecture in [vision](../vision.md). Source edits, textual
review, Git metadata and primary specification reads only. No test, represented
program, generator, formatter, validation gate, compilation or CI feedback ran or
was reviewed. The full drafting goal stays active. Source input counts below are
not audited requirement counts, observed passes or a completion percentage.

## Code and independent controls

`rust-exclusive-source-compound-rules.js` uses its actual ordered parser for call
borrow operands, distinguishing enclosing call delimiters from grouped-place
parentheses. `rust-exclusive-source-places.js` supplies the same token/resolution
path to borrows, assignments and compounds. Mode selection does not reserve a
source identifier named `borrow`. Original mutable roots and field paths remain
separate from explicit payload projection.

`rust-record-place-owner-rules.js` supplies a bounded canonical Box projection
depth policy. It preserves complete remaining owner trees and refuses invented
owners/layers, stored references and borrowed element positions. Core type
agreement remains the actual expression checker's responsibility. The actual
selected mutable formal can request further standard Box dereference coercion in
`selectRustExclusiveSourceCallReferent`. Equal-width distinct source owners grant
no coercion. This path is restricted to actual selected mutable call acquisition;
it is not ordinary Box value conversion or a custom Deref implementation.

`rust-exclusive-source-calls.js` acquires the original place and exposes the real
referent to its selected callee, preserving all old initialization, same-callable,
pending-loan, surviving-reference and overlap checks. The private borrow retains
both the full selected field and optional projected referent/depth. Call result/
state extraction in `rust-owned-record-effects.js` uses the projected controller
for actual payload calls even when the original field path is empty.

`rust-source-place-transforms.js` preserves the original complete root positions
and existing facet shape. Only the actual selected extraction argument receives
the retained payload position. Independent checking proves its exact canonical
projection to the complete actual callee referent, checks the real extraction and
packet/rebuild graph, and preserves untouched siblings/current roots. A root-only
transformation needs genuine Box projection; unrelated ordinary empty-path calls
remain refused. Hidden callback capture slots retain their whole original binder
and cannot be reinterpreted as projected payload captures. The existing region
controller already publishes one fresh current revision per distinct checked
root, even for multiple disjoint payload fields. No new native dispatcher,
serialized facet field, trusted epoch or handwritten Rust semantics was added.

`rust-borrowed-source-regions.js` establishes actual standard Option/Result owner
positions before canonical boxed sum construction, following the real variant
declarations and payload fields. This does not establish general nominal Copy,
recursive/reference-bearing state, escaping loans or allocator/Drop behavior.

Twenty-one appended positive originals cover root/nested/partial payload calls,
explicit and implicit reborrow/coercion, fields/disjoint fields/boxed containing
roots, grouping, String/Vec/Option/Result replacement, scalar results, early
completion, ended loans, ordinary binder names and explicit callback payload plus
separate captured state. Eleven invalid originals retain immutable/shared/moved
roots, overlapping whole/payload and duplicate places, surviving loans, wrong
remaining depth/width/owner and later operands reading an acquired payload.
Independent controls cover call delimiters, exact owner/depth decisions, actual
root versus payload qualifiers, unchanged facet keys, missing/forged projections,
same-core-type wrong packet slots, invented empty-path transformations, actual
callback capture binders and source-free Links text/binary/four-target transport.
All are unexecuted. Previous inputs, assertions and explicit version ranges are
preserved. Automatic source/native publication remains deferred; the changed
policy libraries and consumers are already in the existing recipe.

Primary contracts: [Rust borrowing/dereferencing](https://doc.rust-lang.org/reference/expressions/operator-expr.html#dereference-operator),
[mutable dereference coercion at call sites](https://doc.rust-lang.org/reference/type-coercions.html#coercion-types),
and [standard Box](https://doc.rust-lang.org/std/boxed/index.html).

## Remaining actual code, in implementation order

1. Finish Box receiver selection for inherent and standard String/Vec operations
   in `captureRustExclusiveSourceReceiver`. Retain an actual peeled receiver view
   and original place; acquire the real selected method formal rather than an
   erased field position. `rust-exclusive-source-strings.js` and
   `rust-exclusive-source-vectors.js` must build operations on that view and
   preserve complete caller reconstruction. Actual Vec pop/truncate/clear helper
   packets can now reuse the projected payload call route; pass the real referent
   alongside the complete original place and prove helper interfaces/bodies.
2. Finish boxed intrinsic replacements in
   `rust-source-place-replacements.js`, argument evaluation and region operands.
   String push_str/clear and Vec push still need a complete independently checked
   payload operation/rebuild graph, including root-only boxed leaves. The current
   projected call extension does not silently broaden the intrinsic facet.
   Readonly explicit payload receivers also need actual place/autoref selection
   before owning transfer. Retain iterator invalidation and exact operand order.
3. Extend low-level context call places to the same genuine payload protocol where
   their actual formal differs from a full original Box place; source frontend
   projection does not automatically establish a new low-level context API.
   Complete actual source namespace/alias identity proofs wherever canonical
   reexports have different retained spellings, rather than accepting equal core
   widths or relabeling a remaining Box owner by hand.
4. Complete recursive state with constructive finite witnesses, stored-reference
   fields/payloads, per-layer lifetime substitution, temporary lifetime extension,
   general partial moves and disjoint surviving field revisions. Complete
   Vec<Box>, generalized iterators and actual Drop/allocator/unsafe observations.
   Current acyclic/destructor-free and closed state restrictions remain.
5. Complete borrowed/escaping/field callback environments, Box capture/result/
   state interfaces, finite helper borrowed captures/results/residuals,
   zero-iteration state and actual iterator short-circuit/drop. Explicit scalar
   payload arguments do not establish these broader environment/result domains.
6. Complete full own-project default CST parsing/emission and both JavaScript/
   Rust translation directions, including modules/generics/traits/macros. Retain
   all JavaScript dynamic Function, callback/async, derived/super, Proxy/private-
   field, mutable-cell/effect and live-root requirements in preceding handoffs.
7. Complete full grammars, global concepts, recursive formal definitions/provable
   equivalence, recursive any/all parsing, ontology/taxonomy, RDF/OWL import/
   conversion/merge, dynamic Links/link-cli reconstruction, grammar inference,
   formal-system and distribution requirements. No broader requirement is removed.
8. Only after every code/control draft: reconcile source register online before
   ledger use, publish automatic source/schema/naming/native assets, run authorized
   JavaScript and CI-only native checks, repair actual failures, and record all
   observed assertions at their executed commits. Never compile Rust locally.
