# Owned Box helper call source draft

Continues [canonical Box field regions](issue-227-box-record-field-regions-2026-11-03.md)
under the sole architecture in [vision](../vision.md). Source edits, textual
review and primary specification reads only. No tests, represented programs,
generators, formatters, validation gates, compilation or CI feedback were run or
reviewed. The full requirements code draft remains unfinished and the complete
goal stays active. Authored controls are not execution evidence.

## Actual shared implementation path

Ordinary source helper parsing previously reached the exclusive interface header
adapter even when the helper had no mutable arguments. That adapter refused all
Box positions and erased all wrappers on admitted parameter referents. The source
region call interpreter independently rejected every wrapped output and could
mistake an owned Box wrapper for an already supplied shared-reference operand.
These were separate integration barriers after field projection itself acquired
its actual Box containing-storage proof.

`rust-exclusive-source-signatures.js` now preserves actual owned Box parameter
and result positions. Parameter mode still comes from the actual exclusive source
flag or outer shared-reference wrapper. Header projection removes exactly that
selected outer parameter reference and its lifetime column; canonical Box layers
and complete owners remain. Borrowed text retains its separate preceding shared
representation. Stored references, borrowed vector carriers and escaping result
references still require their own interfaces and cannot enter this owned path.

Independent exclusive signature checking reuses the canonical owner-layer reader
before checking the actual closed payload storage. Nominal field inventories,
scalar owners, standard vector ownership, standard collection carriers and the
existing acyclic/destructor restrictions remain. A Vec element's complete actual
owner supplies its canonical Box layer inventory before recursive storage
checking. No erased primitive/core type or generated name proves an owned Box.
The retained interface model and serialized fields stay the same.

`readRustBorrowedSourceCallParameter` now supplies one common actual shared-formal
view to both ordinary and projected calls. An interface retaining its actual
reference uses it directly. An interface retaining a referent receives one outer
shared reference with every existing Box wrapper preserved. This prevents an
owned Box from being treated as a borrowed call receiver by wrapper count alone,
and prevents a retained reference from acquiring a second artificial borrow.

The common region output check admits actual canonical owned Box chains with no
reference lifetime or borrowed element carrier. Its closed payload proof and
actual value role remain required. Existing source argument evaluation, owner
consumption, new result storage, scalar writeback, early return and projected
outer-record reconstruction continue through their original executable bodies.
No native Box body, handwritten Rust semantics or alternative target emitter is
introduced. The automatic source recipe already includes every changed library,
fixture and consumer; publication remains deferred.

Primary contracts: [Rust Box ownership](https://doc.rust-lang.org/std/boxed/index.html)
and [call expressions](https://doc.rust-lang.org/reference/expressions/call-expr.html).
The source position proof supplies no unsafe pointer/allocator or arbitrary Drop
semantics; those retain their independent obligations.

## Independent controls authored, all unexecuted

Fourteen literal originals cover scalar/record/String Box helper results,
transferred Box arguments, nested Box layers, actual payload transfer into fresh
record storage, repeated shared reads before an owning move, independent scalar
writeback with Box results, new allocations from shared inputs, projected scalar
calls, exact mixed value/shared/exclusive parameter order, conditional allocations
and early return state publication.

Six invalid originals retain use after a helper consumes the Box, an owning
result from a shared Box, unboxed arguments with the same erased record type,
wrong Box depth, moving a Box with a surviving field loan and a mutable argument
derived from a shared payload. Existing inputs/assertions and explicit Rust/Node/
dependency version ranges remain. These are input counts, not requirement rows
or observed results.

Structural controls inspect complete ordinary and exclusive positions/packet
order; forge missing/extra wrappers, library owners, arguments, lifetime columns
and packet indices; and check shared parameter instantiation independently.
Complete Links text/binary reload controls retain ordinary/projected calls,
source interface positions and all four target emissions. No controls ran and
no source/native/schema/naming assets were generated locally.

## Remaining code and concrete next entry points

1. Complete actual mutable Box roots and Box referents. The mutable formal reader
   in `rust-exclusive-source-parameters.js` still refuses them; state registration
   in `rust-owned-record-state.js` still requires direct origins and clears all
   wrappers. Replace that erasure with an independently checked retained Box
   referent view, preserve lexical/root/field identities and every source Box
   layer, and retain allocation/loan effects during whole replacement.
2. Carry those positions through state initialization/write transitions,
   `rust-exclusive-source-call-adapter.js`, the low-level context storage/place
   reader in `rust-exclusive-source-loans.js`, and projected extraction/
   reconstruction in `rust-source-place-transforms.js`. Keep exact selected
   fields, ordered acquisition/reborrow, full original source owners and result/
   state packet positions. Readonly Box arguments and owning results above do
   not establish mutation authority. Add complete actual source and forged-state
   controls before claiming that path drafted.
3. Extend state payload proofs to the actual general nominal/recursive declaration
   graph, including finite constructive witnesses and real Box/Vec indirection.
   Current exclusive payload storage still keeps its original acyclic record or
   standard-sum restriction. Model destructor obligations and stored references
   explicitly rather than dropping those checks. Complete Box/Vec element
   operations and callback result/capture interfaces where direct-only guards
   remain in `rust-fnmut-source-signatures.js` and environment readers.
4. Complete per-layer reference lifetime substitution, temporary lifetime
   extension, stored-reference fields, partial moves and disjoint surviving
   field loans. Preserve original source spans/owners, field paths, initialization
   states and whole-owner restrictions. The containing-root model remains
   conservative; canonical Box ownership does not stand for custom Deref traits.
5. Finish borrowed callback interiors and actual forwarding/escaping/nested/field
   environments; iterator stages and actual short-circuit/drop behavior;
   finite-helper borrowed captures/results/residuals and zero-iteration state;
   full own-project default CST parsing, emission and JavaScript/Rust translation
   in both directions, including modules, traits, generics and macros.
6. Retain all remaining JavaScript dynamic Function, native callbacks/async,
   derived/super/Proxy/private-field and live root/retention work, alongside the
   full grammar/global concepts/deep formal definitions/equivalence, recursive
   any/all parsing, ontology/taxonomy, RDF/OWL import/conversion/merge, dynamic
   reconstruction, grammar inference, formal system and delivery scope in vision.
7. Only after every actual code/control draft is complete, reconcile sources
   online before ledger access, automatically publish source/schema/naming/native
   assets, run authorized JavaScript and CI-only gates, repair actual failures and
   record observations at their executed commits. Never compile Rust locally.
