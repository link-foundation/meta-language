# Irrefutable place binding draft

Continues [disjoint field revisions](issue-227-disjoint-field-revisions-2026-11-04.md)
under the sole architecture in [vision](../vision.md). Source edits, textual
inspection and primary specification reads only. No tests, represented programs,
imports, generators, formatters, validation gates, compilation or CI feedback ran
or was reviewed. Every new control is unexecuted. The full drafting goal remains
active, without an audited drafted/total count or completion percentage.

## Actual source changes

`rust-source-place-bindings.js` reuses exact checked record-field projections for
irrefutable closed nominal patterns. Actual constructors, owner identities, field
order, source annotations, Copy/owning profiles and resource bounds are checked.
Each selected leaf gets its real move/copy or shared-reference operation;
wildcards do nothing. Existing field images and region interpretation continue
to own component availability, containing lifetimes and actual loan constraints.
There is no new archive facet or trusted binding-mode inventory.

All selected values are captured in fresh internal bindings before user names
enter scope. This retains required transfers for unused names and prevents a
shadowing user binding from changing later initializer-field selection. Mutable
owning or Copy leaves initialize ordinary independent source storage, preserving
their original binder token and complete position. Mutating such a copied scalar
does not mutate its source field; moving an owning field leaves its source
component missing.

A syntactic initializer place retains its actual lexical root, parentheses,
field projections and explicit canonical Box layers. Its root is lifetime-checked
without making a partially moved whole usable. Box dereferencing peels the actual
place position without performing whole-payload transfer. Arbitrary initializers
retain ordinary full-value validation and evaluate once into actual hidden
storage before field selection. Shared-parent ergonomics produce shared leaf
bindings, preserving the original whole-parent loans. A bare `ref` binding can
borrow actual existing or hidden initializer storage.

`rust.js` and `rust-records.js` retain `ref`/`mut` syntax only in the active
irrefutable let-binding context. Raw identifier spellings retain their distinction
from modifier keywords. Pattern flags are consumed by lowering, not serialized.
Unsupported modified refutable, anonymous-product, explicit-reference-pattern or
exclusive-reference contexts refuse rather than silently discarding their mode.
Existing general match/let-else handling remains separate.

`rust-source-field-images.js` accepts a canonical owned Box payload view of an
actual lexical projection owner after checking actual type/owner/layers and its
expression qualifier. It inspects that original storage lifetime rather than
moving or copying the whole Box. Shared-reference prefixes cannot acquire this
owning view. The new authoring module is registered for later automatic native
publication; native semantics have not been handwritten.

Twenty-four positive and fourteen invalid originals are appended under the
retained Rust `>=1.90.0 <1.100.0`, edition 2024, and Node ranges. They cover named,
renamed, reordered, nested and tuple-struct patterns; owning/shared/Copy leaves;
wildcards and unused names; fresh initialization; shared-parent/parameter loans;
mutable owning/Copy state; shadows; partial nested places; explicit Box layers;
raw keyword identifiers; and a later pattern on an already partially moved owner. Invalid originals
retain moved-field/whole reuse, duplicate transfer/binding, overlapping and
escaping loans, wrong nominal owner/annotation, edition-specific modifier errors,
implicit owned Box destructuring, custom Drop partial moves and constant shadowing.
Other edition-specific binding contracts are not inferred from these inputs.
These are authored native-control inputs, not executed observations or requirements.

Independent controls check unchanged exact projection facets, absent archived
mode flags, a same-typed ref projection retargeted onto the field that actually
moved, exactly one original initializer call, mutable binder provenance, shadows
and Box places. Twelve scenarios retain Links text/binary and all four target
bodies. Existing inputs, assertions, source-version bounds, oracles and gates are
preserved. Controls have not established passing behavior.

Contracts: [Rust identifier patterns and binding modes](https://doc.rust-lang.org/reference/patterns.html#identifier-patterns)
and [raw identifiers](https://doc.rust-lang.org/reference/identifiers.html#raw-identifiers).

## Remaining actual code, without reducing the full scope

1. Generalize actual match entry binding modes and guarded/refutable patterns.
   `RustParser.match`, `RustSourceOrigins.cases` and the ordinary region `match`
   adapter still use a whole owned scrutinee for an owning constructor binding.
   Retain source binding modes through actual compiler arm graphs, including
   variant identity, nested declaration paths, aliases, guard borrowing, failed
   guards, fallthrough and alternative joins. Check every mode against actual
   executable bindings and positions. No archive mask, generated alias or omitted
   binding may claim that a real move did not happen. Add unused owning fields,
   mixed references, actual or-patterns, guard side effects and wrong-variant/
   path/mode controls before claiming this general protocol drafted.
2. Implement anonymous product places and variant-indexed component images.
   Ordinary source tuples currently overlap the state packet interpretation;
   they need a distinct checked actual product projection/ownership contract.
   Preserve per-field owners, borrows, moves, rest positions, nesting and actual
   tuple allocation lifetime. Multi-constructor patterns/let-else require their
   actual active variant witnesses, not a single-constructor assumption. Retain
   native original inputs and type/variant/layout/coordinate forgeries.
3. Implement `ref mut`, mutable-reference ergonomics, explicit `&` pattern modes,
   aliases involving owning subpatterns, and source edition selection. Derive
   exclusive leases and reborrow phases from actual original places and binder
   tokens; end them according to real scope/last use. Cover same/different-field
   reborrow, parent/child overlap, `@` aliases, nested reference/Box layers and
   edition-dependent positive/negative boundaries. The present controls bind
   only the registered Rust 2024 range; other editions need explicit profiles.
4. Complete general temporary lifetime extension, actual stored-reference fields,
   source functions/closures with pattern parameters, custom Drop, destructuring
   assignment and arbitrary indexed/custom-Deref initializer places. Direct
   hidden storage for an irrefutable ref initializer is not a blanket admission
   for those forms. Preserve initializer/field/drop order, abrupt completion,
   lexical ancestry and effect-aware source aliases.
5. Complete precise reference-bearing call/state ABIs, partial callback captures/
   results, arbitrary helper effects, general partial loop/helper packets,
   residuals, zero-iteration state, allocator/unsafe observations and generalized
   iterators/Box APIs. The preceding handoff retains detailed real code loci and
   controls for these obligations.
6. Complete own-project CST-default parse/emission and both JavaScript/Rust
   translation directions, modules/generics/traits/macros and all outstanding
   JavaScript Function/callback/async, derived/super, Proxy/private-field, mutable
   cell/effect/live-root behavior. Preserve every full grammar/global concept/
   recursive formal definition/equivalence, recursive any/all, ontology/taxonomy,
   RDF/OWL import/conversion/merge, dynamic Links/link-cli, inference and
   distribution obligation in vision. No pattern batch establishes full support.
7. After every actual code/control draft: reconcile sources online before ledger
   use, publish automatic native/schema/naming assets, run authorized JavaScript
   and CI-only native gates, repair actual failures and record all assertions at
   their executed exact commits. Never compile Rust locally.

Successor: [guarded source place matches](issue-227-guarded-place-matches-2026-11-04.md)
drafts single-constructor arm entry and private readonly owning guard views.
Its remaining-work section retains general refutable/effectful matching scope.
