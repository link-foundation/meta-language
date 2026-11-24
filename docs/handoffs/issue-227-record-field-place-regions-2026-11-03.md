# Direct record field places source draft

This continues [projected callbacks](issue-227-projected-callback-interfaces-2026-11-03.md)
under the sole architecture in [vision](../vision.md). Source edits and source
review only: no program, test, generator, formatter, validation gate, compilation
or CI feedback was executed or reviewed. The overall code draft is incomplete;
the active goal and every preceding requirement remain binding.

Continuation: [fallible iterator helper interfaces](issue-227-fallible-iterator-helper-interfaces-2026-11-03.md)
drafts the existing contiguous Vec map/Result collect helper's complete compiler
ABI and body proof. Other iterator, storage and frontend obligations below remain.

[Canonical Box field regions](issue-227-box-record-field-regions-2026-11-03.md)
subsequently extends this same place facet to canonical Box parents and owned Box
fields. Partial moves, disjoint surviving loans, stored references and the other
full-scope requirements below remain; all controls are still unexecuted.

## Actual source/compiler path drafted

`rust-records.js` retains the actual parent before Box projection and attaches
`Rust source record place 1` through `rust-record-place-regions.js` for direct
closed fields in the selected source region composition. The facet carries the
qualified parent and declared field positions, actual declaration index and
original field span. It contains no runtime epoch or invented source signature.
Ordered effect lowering preserves the actual projection while evaluating owner
effects first. Existing Box and stored-reference paths are not admitted by this
new facet.

Independent validation derives the actual single-constructor declaration,
complete field inventory, exact selected body binding and optional one-time
owner evaluation wrapper. Every field binding is unique; compiler names for
wildcard fields remain unused by the complete selected body. Source review
identified that the existing pattern compiler names those wildcard positions,
so validation checks their actual lack of use rather than requiring nonexistent
null bindings. Wrong indices, owners, bodies, competing state facets, extra
facet fields and changed spans refuse. A checker-local WeakSet preserves only
the actual field projection inside a known-constructor case; other case
optimization remains available. The set is not serialized authority.

The source-region interpreter evaluates the actual owner and derives the place
regions from that owner's current containing storage or shared referent loans.
Nested field projections retain those original regions rather than borrowing
the short-lived pattern alias. Reading a field does not itself consume the
owner. Taking an actual reference records the original place regions. A Copy
read or owning transfer into a new binding clears the transient place facts;
the new binding has its own storage. A non-Copy field cannot move out of a
borrowed parent. Owned append now consumes its actual operands so concatenation
cannot conceal a copy of such a borrowed field. Whole-root storage writeback
continues through its existing moved-root replacement protocol.

Display classification admits an actual shared standard/alloc String reference
in addition to shared primitive str, retaining exact owner, wrapper and argument
checks. Declaration homographs receive no standard-library authority.

Typed program projection, independent Links validation and reference transport
retain the new facet and both datatype positions before lexical traversal's
early returns. Finite-loop source facts share the exact immutable facet by
identity. The schema/source-author inputs include the new concept and consumer.
Generated schemas, source receipts and native translations are deliberately
unpublished until the complete requirements code draft, as required by the
current phase; this batch makes no end-to-end passing claim.

## Independent controls drafted

Eleven positive originals cover scalar and String field borrows, returned
references, copied aliases of shared parents, nested records, copying through
borrowed Vec fields, Copy values outliving an inner record, owned fields moved
into fresh storage, ending a borrow before actual writeback, and projection
inside a known-constructor case. Four invalid originals preserve escaping local
field loans, use after moving a borrowed record, non-Copy moves from a shared
parent and fabricated static lifetimes. Existing version profiles and all
preceding inputs remain.

Structural controls inspect real parent/field positions and forge owner, field
index, field owner, model shape, actual body binding and omitted place authority.
Another control erases the outer reference result to attempt a non-Copy move
from a borrowed field. Complete text/binary Links reload controls retain nested
field ownership, borrowed Vec scalar copies and fresh storage for transferred
String fields, alongside JavaScript/Rust/Lean/Rocq emissions. Shared String
Display decisions are added to the original rule inventory. None ran.

Authored inventory: 514 positives in nineteen groups, consisting of 450 literal
originals and 64 actual-source scenarios. The mixed composition group has 149
positives and 52 refusals. These count inputs only, not requirement rows,
observed passes or a requirement completion percentage. No audited requirement
count or denominator has been established in this draft-only phase.

## Remaining work and resumption order

1. Extend actual storage proof to partial field moves, disjoint surviving loans
   and their independently acquired revisions. Current loans cover the whole
   containing root, and an owned field move conservatively consumes that root.
   Introduce exact field-path state instead of declaring these cases complete
   from the disjoint mutable call protocol.
2. Add explicit Box/multiple-reference projections, reference-bearing fields and
   tuple/product storage with complete source positions. Model temporary
   lifetime extension at its actual Rust scope; do not extend a producer's life
   from a textual projection or strip a reference to enter the closed profile.
3. Complete field-granular callback capture/writeback, escaping and nested
   callback environments, borrowed callback interiors, function aliases and
   mutable generic receiver methods. Preserve all original acquisition frames,
   overlap checks and explicit/hidden packet order.
4. Supply complete compiler interfaces and independently checked bodies for
   iterator-generated helpers and other mutations. Complete finite-helper
   borrowed capture/result/residual interfaces, zero-iteration initialization
   carriers, borrowed iteration and shared collection composition.
5. Finish whole-project JavaScript/Rust CST, semantic emission and translation
   in both directions using actual own-codebase modules. Retain constructor,
   Proxy/bound constructor, derived receiver and weak-liveness obligations.
   Complete the remaining shared concept definitions/equivalence, full default
   CST grammars, recursive any/all parsing, ontology/taxonomy, RDF/OWL import,
   conversion/merge, dynamic reconstruction and grammar inference work in vision.
6. Only after every requirement has an actual full code draft: reconcile live
   sources before using the ledger; automatically publish schema/naming/source
   receipts and native translations; run authorized JavaScript checks and
   CI-only Rust/full gates. Then fix actual failures, record observations at the
   executed commit and claim completion only when every requirement passes.
   Never compile Rust locally or present authored controls as evidence.
