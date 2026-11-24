# Shared ordered BTree source loops draft

Source drafting only. No represented behavior imports, tests, source generation,
validation, builds, compilation, native runs or ledger observations occurred.
Expected stdout literals and future controls are declarations, not evidence.

This batch adds genuine readonly standard BTreeSet/BTreeMap for traversal under
the existing actual projected-library profile. The admitted source heads are
shared `&set` / `&map`, shared formal collection references, Set `.iter()`, and
Map `.iter()` / `.keys()` / `.values()`. Map pairs retain both actual key/value
positions. Item binders may retain a shared scalar reference, destructure a pair
of shared references, or use `&name` to copy an actual primitive Copy referent.
String referents remain borrowed and require their real explicit owned Clone
operation before transfer. Patterns cannot introduce refutable constants,
constructors, duplicate binders or invented String Copy semantics.

`executeRustStandardCollectionOrderedBorrow` supplies actual checked ordered
entry storage through the shared value-packet body. Its source-owner/body edge
is required by the complete projected-library admission validator. Borrowed
entry transport is a distinct result variant and projection, separate from the
previous owned copied/cloned Vec snapshot. The private physical array receives
no owned Vec source owner and cannot be used as a source ownership or mutation
authority. Source views carry receiver-derived actual source regions and their
real registered root/place leases. No bare iterator value is published to user
source; private view metadata is accepted only from its actual builder and
rechecked against the current real origin and consumed head tokens. An unrelated
method cannot overwrite an origin and keep its previous iterator authority.

Mutable and immutable actual owner roots are registered for shared loan tracking
without granting mutation permission. The actual containing root is used for
record fields, preserving precise field dependencies. A hidden source-scope
borrow remains live throughout the loop even when the visible item uses a Copy
pattern. Overlapping collection mutation, field replacement, exclusive call
operands and owned root moves therefore refuse. Disjoint sibling fields remain
available. The root lease expires on normal/break completion; an inner readonly
loop preserves its outer loop's still-live lease. Hidden source names and item
shadowing do not replace the real lexical/root identity used for writeback.

`rust-standard-collection-source-loops.js` consumes actual ordinary loop
completion trees. Continuing paths preserve all outer ownership states,
including lazily discovered descendant fields. Break paths join their true
remaining-owner states with the zero-iteration path. Return and question-mark
residual branches retain their actual source ownership snapshots and complete
changed-root packets. The existing eight-level nested finite-loop source depth
bound and bounded inspection budgets apply. Returned iterator references remain
an explicit lifetime/result-region frontier; owned scalar/String results and
ordinary Option/Result residual completion use the existing completion algebra.
Main return inside the loop remains refused until enclosing main completion is
implemented.

The private loop marker has an authenticated parser-reader admission witness.
The common state effect transport lowers it to an ordinary checked fold over the
actual ordered input values. Its accumulator carries an explicit stopped flag,
actual ordinary completion control and every initialized outer state root.
Completed body writes publish before continue, break, owned return or residual
return. The inactive branch never evaluates later loop bodies. Loop input
expressions execute once before traversal. Actual lexical immutable captures
are derived from the lowered closure body, while mutable roots remain explicit
state parameters; a callback descriptor never supplies replacement semantics.
Nested ordered loops consume only their nearest break/continue signals.

Narrow hooks in rust-ordinary-completions.js and rust-owned-record-effects.js
reuse existing ordinary completion and state recipes. Companion owns the shared
rust.js commit containing the collection-loop import, block/main reader hooks
and explicit final private-marker refusal. Effect finishing independently
recognizes and refuses any unconsumed private loop marker. No unknown math
operation may escape as successful public parse output.

The dedicated fixture file exports `rustStandardCollectionLoopOriginals` (32),
`rustStandardCollectionLoopRefusals` (18) and
`rustStandardCollectionLoopVersions`. Source bounds are Rust >=1.90.0 <1.100.0;
source target64; future JavaScript controls select Node >=22.15 <23 or >=24 <25.
Every positive is authored for real parse/check/emitJavaScript/stdout comparison
at source 1.90 and 1.99, with an assertion that no private loop marker is
published. Independent originals cover ordered references, String Unicode order,
Map pairs/key/value association, precise Scope loans, repeated owners, changed
roots on all exits, nested loops, head evaluation once, shadowing, owned returns
and first Option residual behavior. Negative originals retain the actual unsafe
or unsupported syntax, including iterator-origin overwrite and head-local observation attempts.

Earlier borrowed iterator-local and pair-collect refusal inputs remain unchanged:
this batch admits a real for consumer, not unrestricted iterator storage or
reference-vector collection. The earlier raw BTreeMap/iter arity input is retained
and corrected from unsupported to zero arguments because its real borrowed pair
consumer now exists; the correction is documented in that control. Earlier Hash
iteration and custom domain refusals remain in place.

Remaining frontiers include escaping borrowed iterator results, stored iterator
state/next, owned or mutable IntoIterator, labels and break values, reference Vec
collection, additional iterator adapters, arbitrary custom iterator/Ord behavior,
Hash enumeration, source target32 and actual generated/native verification.
Parent owns native-positive/automatic source registrations and root CI wiring;
source/body provenance is not presented as generated-body equivalence evidence.
