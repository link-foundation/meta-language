# Explicit shared-reference pattern draft

Continues [guarded source place matches](issue-227-guarded-place-matches-2026-11-04.md)
under [vision](../vision.md). Source editing, primary reference reads and textual
inspection only; no behavioral test, import, represented program, generator,
formatter, validation gate, compilation or CI feedback ran or was reviewed.
Every new control is authored and unexecuted. No audited drafted/total count or
completion percentage is available.

## Actual source changes

`rust-source-reference-places.js` adds a checked shared-referent identity operation
to the existing place transport. Its exact facet retains only model, empty slots,
empty places and the original span. The actual let/value/body types and identity
binder must agree; complete input/result source positions must remove exactly one
real shared reference and its corresponding lifetime while retaining the actual
owner and all other layers. Every nested source `&` has its own actual operation.
Region interpretation reads the actual shared operand, validates its inner
position and keeps its allocated referent roots, live regions and inherited whole
borrow constraints. An exposed owning-looking referent remains a borrowed place:
no owning move, new storage lifetime or archived authority is granted.

`rust-source-place-bindings.js` consumes the parser's existing private reference
pattern inventory. It checks the actual reference layers, refuses exclusive
references, validates the Rust 2024 default binding mode and reuses real field
projection lowering afterward. Copy fields become independent values; explicit
`ref` fields retain source loans. Nested constructor patterns keep the actual
borrowed-place restriction without inventing an implicit binding mode after an
explicit dereference. Whole owning bindings through the view remain invalid.
Wildcards validate reference layers without transferring an ignored field.

`rust-records.js` admits explicit `&`/`&&` let patterns through that same path.
`rust-source-place-matches.js` admits the same shapes in the existing guarded
attempt protocol. `rust.js` stops treating `&mut` as a shared pattern followed by
an ordinary mutable binder; it requests the still-required exclusive protocol.
Explicit wildcard match patterns retain their actual layer checks. The source
region dispatcher accepts expected positions for independently checked value
operations while preserving ordinary state-packet restrictions. The new module
is registered for deferred automatic native publication; no replacement Rust
semantics were handwritten.

## Authored independent controls

Fourteen native-positive originals cover shared String fields and Copy scalars,
mutable independent Copy state, nested reference layers, tuple structs, nested
records, user-name shadows, failed guards, returned parameter field regions,
scalar value/ref bindings, whole transfer after a Copy read, raw keyword names,
canonical Box fields and wildcards. Ten compiler-invalid originals retain owning
moves through shared storage, absent/excess reference layers, escaping references,
overlapping selected/whole writes, nominal mismatches, duplicate names, a Rust
2024 inherited-mode reference subpattern and a mismatched source annotation.
Retained Rust `>=1.90.0 <1.100.0`, edition 2024, and Node ranges apply; no other
versions are claimed.

Six independent controls inspect actual one-layer identity graphs, nested actual
operations, exact facet fields, complete source positions and layer/owner/span/
binding forgeries. Removing a shared-referent operation must fail ordinary
qualifier interpretation. Dropping a selected field's reference qualifier keeps
its core type and locally valid field projection but must fail the independent
owning-transfer analysis. Twelve scenarios retain Links text/binary and every
JavaScript/Rust/Lean/Rocq target body. No observations have been recorded.

Contract: [reference patterns](https://doc.rust-lang.org/reference/patterns.html#reference-patterns)
and [binding modes](https://doc.rust-lang.org/reference/patterns.html#binding-modes).

## Remaining actual code

1. Generalize these views to reference-bearing stored fields and anonymous
   products. Preserve actual separate reference-layer ancestry rather than
   conflating tuple source data with compiler packets. Add source-parameter and
   closure pattern lowering, and independent wrong-referent/escaping controls.
2. Draft exclusive `&mut`/`ref mut` patterns and mutable-reference ergonomics with
   actual lease identity, place ancestry and activation/reborrow/last-use phases.
   Supply explicit source edition profiles and their native positive/negative
   boundaries; do not infer older edition rules from the current Rust 2024 range.
3. Complete general multi-constructor/refutable/or/alias matching, guard let
   chains, arbitrary legal arm syntax, foreign guard effects and closed-body
   internal region composition. The preceding handoff specifies the actual code
   protocols and controls required, including variant-indexed component images.
4. Complete temporary lifetime extension, custom Drop, destructuring assignments,
   indexed/custom-Deref places and arbitrary stored-reference ownership. These
   source views are not a blanket admission for those forms or whole Copy
   nominal-value transfers in an independently owning region domain.
5. Preserve all prior remaining call/state/helper/closure/loop, complete own-code
   CST-default bidirectional JavaScript/Rust translation and JavaScript runtime
   obligations. Full grammars, recursively shared formal link concepts,
   ontology/taxonomy, any/all embedding, RDF/OWL, dynamic snapshots, inference,
   distribution and every source obligation remain in vision and the prior
   detailed handoffs. No language or fixture scope was removed by this batch.
6. After every actual draft: reconcile sources online before ledger use, publish
   native/schema/naming/features assets automatically, execute authorized
   JavaScript and CI-only native gates, repair failures and record every actual
   assertion at its executed commit. Never compile Rust locally.

The entire requirements draft remains active. There is no external drafting
blocker, and this batch claims neither executed behavior nor full support.
