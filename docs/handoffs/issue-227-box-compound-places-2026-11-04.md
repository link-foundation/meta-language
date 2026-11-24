# Box compound place draft

Continues [mutable Box state](issue-227-mutable-box-source-state-2026-11-04.md)
under the sole architecture in [vision](../vision.md). This batch uses source
edits, textual diffs, Git metadata and Rust Reference reads only. No test,
represented program, generator, formatter, compilation, validation gate or CI
feedback ran or was reviewed. The complete drafting goal remains active. There
is no audited drafted/total count or requirement completion percentage.

## Code and independent controls

`rust-exclusive-source-compound-rules.js` now supplies a bounded ordered place
plan. It retains actual field/dereference precedence and parenthesis grouping,
including `*root.field`, `(*root).field` and nested combinations. It remains in
the existing automatic source-library recipe; no Rust replacement was written
and no generated source was edited. Future source publication must include the
private recursive helper and independent ordered-plan controls.

`rust-owned-containers.js` exposes canonical partial Box place projection using
the actual existing owner/layer/copy/loan metadata and materialization bounds.
Inspection neither reads nor transfers the selected payload. The compound
adapter keeps its original mutable root and field layouts alongside the scalar
view. It distinguishes an actual exclusive reference from each owning Box layer,
retains known primitive admission, evaluates the complete RHS first, reads the
latest actual place, and publishes a complete selected-field value through the
existing ordered writeback. Primitive read/arithmetic positions and full Box
writeback positions use the existing retained source-position protocol.

Fourteen appended positive originals cover owned and exclusive scalar/nested
Boxes, owned parameters, boxed fields, grouped/interleaved places, RHS allocation
replacement and sibling writes, early completion and f64 payload arithmetic.
Six invalid originals preserve immutable/shared mutation, excess/missing
dereferences, surviving payload loans and primitive width mismatches. Existing
inputs and assertions remain preserved under their explicit Rust/Node ranges.
Counts describe authored source inputs, not audited atomic obligations or passes.
Independent controls retain syntactic precedence, malformed/bounded plans,
primitive/source-storage separation, forged owners/widths/lifetimes, source-free
Links text/binary rechecking and all four target emissions. All are unexecuted.

Primary contracts are [Rust compound assignment](https://doc.rust-lang.org/reference/expressions/operator-expr.html#compound-assignment-expressions)
and [expression precedence](https://doc.rust-lang.org/reference/expressions.html#expression-precedence).
Actual standard Box owner admission remains separate from user Deref/assignment
traits, unsafe pointers, indexed-place observations and allocator/Drop behavior.

## Remaining code, in implementation order

1. Share the ordered place resolver with ordinary payload assignment in
   `rust-exclusive-source-calls.js`, whose whole-assignment reader still handles
   only its preceding optional single-star whole referent. Preserve true RHS
   ordering, complete original selected Box/field positions, initialized paths,
   source loans and existing writeback. Complete explicit payload borrows through
   `readRustExclusiveSourceBorrow`, retaining exact mutable formal positions and
   projected call reconstruction rather than erasing Box layers in formals.
2. Complete Box String/Vec receiver autoderef using a separately checked peeled
   payload view and original mutable slot. `rust-exclusive-source-calls.js`,
   `rust-exclusive-source-strings.js` and `rust-exclusive-source-vectors.js`
   must keep actual argument evaluation/acquisition order and latest roots.
   Extend `rust-source-place-replacements.js` for actual boxed intrinsic leaf
   operations; direct primitive/array execution needs its real unboxed operand
   position in `rust-borrowed-source-regions.js`. Pop/truncate/clear must reuse
   actual checked Vec helpers through `rust-source-place-transforms.js` and
   original full Box root reconstruction, retaining result/Option interfaces.
3. Finish recursive state with finite constructive witnesses, stored-reference
   fields/payloads, per-layer lifetime substitution, partial moves, disjoint
   surviving field revisions and actual temporary lifetime extension. Existing
   acyclic/destructor-free restrictions remain in force. Complete Vec<Box>,
   generalized iterators, allocator/Drop and unsafe observations separately.
4. Finish actual borrowed/escaping/field callback environments and hidden state,
   including Box captures/results, finite helper captures/results/residuals,
   zero-iteration state and iterator short-circuit/drop. Concrete entry points
   remain in the preceding handoffs; no callback obligation is reduced here.
5. Complete own-project default CST parsing, emission and both JavaScript/Rust
   translation directions, including modules/generics/traits/macros. Retain all
   JavaScript dynamic Function, callback/async, derived/super, Proxy/private-field
   and live-root/effect requirements in the preceding handoffs and vision.
6. Complete remaining full grammars, global concept space, recursive formal
   definitions/equivalence, recursive any/all parsing, ontology/taxonomy,
   RDF/OWL import/conversion/merge, dynamic Links reconstruction/link-cli,
   grammar inference, formal systems and distribution requirements in vision.
7. Only after every code/control draft is complete: reconcile registered sources
   online before ledger use, publish automatic source/schema/naming/native
   assets, run authorized JavaScript checks and CI-only native work, fix actual
   failures and record observations at the executed commits. Never compile Rust
   locally. No current draft establishes verification or full delivery.
