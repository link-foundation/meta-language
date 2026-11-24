# Concrete standard collection carrier integrity draft

This source-only follow-up supersedes the shared-carrier limitation in the
initial source admission handoff. Nothing here is execution or native evidence.
`docs/vision.md` remains the architecture contract.

Each actual closed HashMap/HashSet owner/key/value/target specialization now
gets a real private nominal datatype. Its single `CollectionState` constructor
has one `state` field containing the actual generic shared kernel state. The
core named types therefore distinguish, for example, `HashMap<u8, bool>` and
`HashMap<u16, bool>` even after source origin objects are erased or archived.
Fresh names avoid actual source/library declarations and stay stable through
parser passes. Complete source domain identity remains separately retained.

Constructors wrap actual shared Created results. Methods unwrap the nominal
carrier only at the actual protected packet body call, then wrap its updated
state. Clone also wraps its actual independent returned state. These are
boundary constructors/projections; no map operation is reimplemented in a
wrapper and no handwritten Rust body is introduced.

Every emitted carrier datatype retains `sourceStandardCollectionCarrier`:

```
{
  model: 'Rust closed standard collections 1',
  origin: <complete actual RustStandardCollectionOrigin>,
  keyDomain, valueDomain, pointerWidth,
  concreteIdentity: origin.typeIdentity,
  kernelType: <actual generic shared state core type>,
  kernelLibrary: {
    entryPath, sourceSha256,
    kernelDeclaration, packetDeclaration
  }
}
```

The actual generic state datatype, packet function and kernel function receive
`sourceStandardCollectionLibraryOwner` with exact fields `model`, `entryPath`,
`sourceSha256`, `declaration`, `role`. Its model is
`Rust standard collection projected library owner 1`; roles are state, packet
and kernel. The digest comes from the independently reconstructed actual supplied
closure. This is a provenance binding, not an executable body certificate or a
declaration that an absent generated asset exists. Complete source graph/body
correspondence remains a separate mandatory admission check.

`rust-standard-collection-signatures.js` exports:

- `checkRustStandardCollectionLibraryOwner(entry, language)`
- `checkRustStandardCollectionCarrier(data, language, resolveType, resolveData)`
- `checkRustStandardCollectionObservation(type, resolveData, operation, span)`

The first two return normalized facets or null for an absent facet, and throw
on invalid metadata. They check exact fields, source/version/domain rules,
canonical concrete identity, the actual one-constructor/one-field shape,
resolved generic-state identity and matching actual state owner provenance.
State and callable owners retain their complete actual declaration interfaces.
The current public source carrier profile covers 64-bit targets; raw kernel
32-bit domains remain independently available. The companion checker and root
archive/schema work retain and recheck these fields after graph projection.

Observation guards refuse internal structural equality, ordering and Display,
including actual nested aggregate fields, until their actual Rust semantics
are supplied. Generic field projection blocks direct carrier internals but
allows an actual user Scope/Env field to reach its protected collection method.
Typed checker guards can use the same complete datatype traversal, preventing a
typed graph edit from exposing insertion-vector/root/revision metadata as Rust
collection observations.

The method source adapter now selects actual mutable places with
`selectRustExclusiveSourcePlace`: `{slot, layouts, fields, field}`. It validates
the real collection field's storage, actual live source loans and disjoint
field coordinates. A source packet writes back through genuine parent record
layouts before continuing. Mutating a shared receiver refuses; an actual
exclusive parameter under the companion profile retains exclusive authority.
Argument effect checks reject overlapping later writes and permit disjoint
places. Kernel work still executes only in its actual supplied shared bodies.

`readRustStandardCollectionSourceArguments` checks each actual argument and
performs an owned String transfer before parsing the following argument. This
prevents a later argument from reading a key already moved into insert. The
actual transfer WeakSet prevents duplicate consumption at method lowering.
Direct unsuffixed numeric literals receive their protected method-position
domain; independently typed operands never acquire a substitute domain.

Deferred controls cover nominal type separation, malformed specialization
facets, actual state owner linkage, missing provenance, private source names,
graph immutability and data-only graph correspondence. Provenance-shape control
fixtures do not stand in for actual generated bodies. No tests, behavior
imports, generation, validation, builds, Rust compilation or ledger operations
were run. Source graph availability, executable receipt projection and native
parity still require their deferred checks. Standalone source Hash iteration,
custom Eq/Hash and unrestricted collection observations remain explicit gaps.
