# Guarded source place match draft

Continues [irrefutable place bindings](issue-227-irrefutable-place-bindings-2026-11-04.md).
The sole architecture remains [vision](../vision.md). Source edits and textual
inspection only: no test, represented original, import, generator, formatter,
validation gate, compilation or CI feedback was executed or reviewed. All controls
below are unexecuted. No audited drafted/total requirement count is available.

## Actual code draft

`rust-source-place-matches.js` plans bounded single-constructor attempt chains:
guarded irrefutable arms and one final unguarded exhaustive arm, with explicit
arm separators. It retains the actual original source place, or evaluates an
arbitrary scrutinee once into real hidden storage. Actual field bindings reuse
`rust-source-place-bindings.js`, checked constructor/owner/field positions and the
existing partial field-image protocol. Copy/ref/owning bindings are distinct;
wildcards perform no transfer. `rust.js` establishes separate guard/body scopes,
actual per-arm mutable declaration frames and existing ownership outcome joins.

Guard bindings are captured separately from successful body bindings. Owning
guard bindings retain their original owning type through an executable readonly
place view. It is a real identity let over an existing lexical place or checked
actual field projection, not an archived ownership assertion. After a successful
guard the body reselects its actual fields and performs the real transfers.
False guards preserve original ownership for fallthrough or whole-source moves.
Shared/Copy guards use their actual existing observation behavior. All selected
fields are captured before user names, including unused names. Independent alpha
renaming and distinct success/failure holes prevent same-typed user shadows from
capturing source owners, body extraction or outer fallthrough. Mutable body names
initialize ordinary real source storage only after success.

`rust-source-match-guards.js` checks two exact operation models transported through
`sourcePlaceTransform`: readonly Boolean guard and readonly pattern place. Their
only facet fields are model, empty slots, empty places and original span. The
view checks exact let/result binding, full core types, actual projection/lexical
input and optional complete source position. Region interpretation derives actual
allocated source roots, keeps lifetime and inherited whole-place constraints, and
mints a bounded private readonly binding capability. `SourceRegions.bind` accepts
only genuine capabilities, reads rather than moves once, then clears admission.
Later ordinary owning moves still refuse. Proof objects are neither AST data nor
serialized Links. Guard interpretation checks the actual Boolean body and refuses
changes to pre-existing ownership states or source storage revisions. Printing
and reads remain actual ordered operations. Arbitrary foreign mutation/consumption
inside guards requires further effect transport; this is deliberately recorded
as unfinished native-valid code scope, not compiler-invalid input.

The new JavaScript authoring modules are registered in the automatic native
publication policy. No handwritten Rust semantic replacement was added. Native
publication, new model naming registration and any required generated schema or
feature inventory reconciliation remain deferred until the entire draft exists.

## Authored controls

Eighteen positive native originals cover single-arm mixed move/ref/Copy matching,
true/false owning guards, owning String reads, repeated attempts, whole fallthrough,
shared/Copy preservation, mutable success bindings, same-typed name shadows,
once-only effectful initializers, nested partial records, returned actual shared
regions, previously partial owners, released guard loans, unused owning bindings,
whole owning aliases and ordered printing. Twelve compiler-invalid originals
retain selected/whole use after move, guard move/mutation, matched Copy-field
freezing, escaping or overlapping references, duplicate transfers and an attempted
owning result from a shared parent. All remain under the retained Rust
`>=1.90.0 <1.100.0`, edition 2024, and Node ranges; uncovered versions are not
claimed. Existing originals and assertions are unchanged.

Seven independent controls inspect exact facets and executable graphs, absent
archived proof flags, actual true-arm transfers, one initializer call, shadowing,
mutable state, output order and structural forgeries. One same-typed forged
Boolean guard inserts an actual owning field transfer while retaining the valid
guard envelope: local model admission must succeed and independent region
interpretation must reject. Another erases a readonly operation before a failed
guard and later whole-source transfer, which must expose the premature move.
Twelve scenarios retain Links text/binary and all JavaScript/Rust/Lean/Rocq target
bodies. These controls have not established passing behavior.

Contract: [Rust match guards](https://doc.rust-lang.org/reference/expressions/match-expr.html#match-guards).

## Remaining actual code and order

1. Generalize actual variant-indexed field images and match entry to multiple
   constructors. Connect each selected component to the actual active variant,
   constructor inventory and source owner; retain partial moves, shared/exclusive
   loans and branch availability. Draft nested literal/refutable patterns,
   or-pattern binding-mode agreement, aliases and unreachable arms. Retain
   independent wrong-variant, same-typed field and forged selector controls.
2. Extend guard transport to native-valid foreign state effects and owning values
   unrelated to the matched place. Track actual guard-protected selected places
   independently of other source state, preserve both success and failure effects,
   keep exact evaluation/drop/loan order, and compose with existing state packets.
   Draft guard let chains and short-circuit attempts. Do not relax the current
   snapshot check or substitute an archive-selected permission mask.
3. Cover all legal arm separators and expression forms, including omitted commas
   after block expressions. The current lookahead falls back to existing matching
   for forms it cannot fully plan. Cover closed function bodies independently of
   the current source-state retention admission. Preserve legacy paths and inputs.
4. Implement anonymous product places, `ref mut`, explicit reference patterns,
   mutable-reference ergonomics, owning alias/subpattern overlap and explicit
   edition-dependent binding profiles. Use actual place ancestry and source binder
   tokens, not modifier inventories as semantic evidence. Retain native positive
   and negative boundaries, arbitrary nested references/Box and real reborrows.
5. Complete stored-reference fields, temporary lifetime extension, custom Drop,
   destructuring assignments, arbitrary indexed/custom-Deref places and function/
   closure pattern parameters. Preserve actual original storage, drop order,
   escape rules, abrupt completion and resource bounds.
6. Complete reference-bearing call/state ABIs, partial closure captures/results,
   general helper and loop effects, residuals, zero-iteration ownership, allocator/
   unsafe observations and general iterator/container APIs. Prior handoffs retain
   detailed loci and independent-control obligations for these continuations.
7. Complete own-project CST-default parsing/emission and both JavaScript/Rust
   translation directions, modules/generics/traits/macros, outstanding JavaScript
   callback/Function/async/derived/super/Proxy/private fields and live effect cells.
   Preserve full catalog grammar generation, recursive any/all parsing, global
   link concepts with recursive formal definitions and checked equivalence,
   ontology/taxonomy entry points, RDF/OWL import/conversion/merge, dynamic
   Links/link-cli snapshots, grammar inference and package/distribution scope.
8. Only after every actual code/control draft: reconcile online sources before
   ledger use, publish automatic native/schema/naming/features assets, run the
   authorized JavaScript and CI-only native gates, repair actual failures and
   record every assertion at its executed commit. Never compile Rust locally.

The current batch is a concrete source draft, not full language support or a
requirement completion claim. No external blocker prevents continued drafting.
