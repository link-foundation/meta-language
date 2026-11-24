# Standard BTree collection source draft

This batch authors JavaScript kernels, actual source lowering, independent
original literals and deferred controls. No behavioral import, tests, generation,
validation, build or compilation ran. Expected fixture output is a declaration,
not evidence. No ledger rows or source-register entries were touched.

`resolveRustStandardCollectionOwner` now authenticates actual std BTreeSet and
BTreeMap declarations and their btree_set/btree_map reexports independently of
Hash owners. Complete source owners, primitive domain witnesses, source versions,
concrete nominal carriers and graph-backed caller packet writeback are retained.
The exact existing profile remains `Rust closed standard collections 1` with
an actual projected shared library and a 64-bit source target. Supported Rust
versions remain >=1.90.0 <1.100.0; raw 32-bit scalar kernels remain independent.

Existing membership operations apply to both BTree families: new/default,
len/is_empty, contains/contains_key, insert/remove/clear, immediate get copied or
cloned, clone and equality. Map results remain owned old-value Options; Set
results remain Boolean. BTree insertion maintains strictly ascending keys and
the state validator rejects unsorted or duplicate actual states. Equal-key map
replacement preserves the stored key, and duplicate Set insert preserves its
actual stored member. No host Map/Set or handwritten native semantics are used.

`rust-standard-collection-order.js` implements standard closed primitive Ord.
Integers compare exactly, including large signed/unsigned 128-bit values;
Booleans place false before true. Valid Unicode String values compare by scalar
order, equivalent to unsigned UTF8 byte lexicographic order. UTF16 code-unit order
is deliberately not used: U+E000 precedes astral characters in Rust String Ord.
Prefixes, embedded zero and canonically equivalent but distinct Unicode strings
retain their exact ordering and equality.

The genuine immediate iterator source paths are:

- `set.iter().copied().collect::<Vec<T>>()` for primitive Copy members;
- `set.iter().cloned().collect::<Vec<T>>()` for owned cloned members;
- `map.keys()` or `map.values()` with the same immediate projection/collection.

The complete chain is parsed from actual tokens, including the standard Vec
owner and exact element domain. A live borrowed iterator cannot escape. The
shared `executeRustStandardCollectionOrderedSnapshot` callable validates the
actual BTree receiver/call and returns owned scalars in its actual sorted key
order. Map value snapshots keep key association and do not sort values.
The source adapter decodes this private value through actual projected bodies
and returns an ordinary owned Vec with a fresh constructor origin. Source
mutation after collection does not affect that owned snapshot. String copied
is refused even for an empty iterator. Hash iteration remains refused.

`rust-standard-collection-ordering.js` implements lexicographic BTreeSet/BTreeMap
collection Ord. Keys precede their corresponding map values; a first unequal
entry precedes the collection-length tiebreaker. Begin/step/run/execute retain
explicit invalid/resource transitions, complete actual operands, bounded
primitive/String work and externally rechecked equal-prefix cursors. Direct
`<`, `<=`, `>` and `>=` source operators invoke the actual distinct
`compareRustStandardCollectionOrderedValuePackets` body through the existing
source comparison hook. Hash collection ordering is refused. The callable has
its own `ordering` source-library owner role; Eq origin metadata is not reused.
The projected library still has the three explicit packet/kernel/equality
bindings; strong admission additionally derives/rechecks the actual ordered
snapshot and ordering callable owners and their real body edges from the complete
source graph. Unavailable bodies or unsupported remapping still refuse.

Canonical source carrier names now retain all four actual collection families.
The authenticated source signature owner branch admits BTree owners only through
the same real checked carrier/domain facet and exact recursive owner matching
used for Hash collections. Existing Scope/Env field and exclusive call packet
transport therefore applies without a generic ownership bypass.

Deferred controls include 35 independent complete positive originals and nine
explicit refusal inputs in rust-standard-ordered-collections.mjs. The positives
cover Scope.names, source aliases, ownership, UTF8 ordering, concrete ordered Vec
snapshots, replacement/removal/clone and collection operators. Future controls
parse/check/emit and compare exact emitted JavaScript stdout at both Rust 1.90
and 1.99 source boundaries, under Node >=22.15 <23 or >=24 <25. Raw controls
cover strict sorted-state invariants, independent clone storage, exact domains,
resource resumption and forged cursors. Prior Eq controls now also compare emitted
stdout. The former BTreeMap resolver refusal input is retained as a positive
expectation because its distinct Ord kernel and authentic source path are now
authored; all earlier Hash iteration/custom implementation refusal inputs remain.

Still explicit: custom key Ord/Eq/Hash, float or nominal domains, live borrowed
iterators, BTreeMap pair/reference iterator materialization, direct for iteration,
into_iter/iterator mutation, collection cmp/PartialOrd methods, inferred Vec<_>
collection targets, recursive aggregate ordering/Debug, source target32 and
actual generated/native verification. Parent owns catalog/native/fixture wiring
and the generic CI-only native-original/generated-product driver.
