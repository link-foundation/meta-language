# Closed owned Rust standard collections source draft

This is an unexecuted source handoff. `docs/vision.md` remains the architecture
contract. These files do not establish public Rust source admission or native
parity, and the existing import/type refusals remain in place.

`rust-standard-collection-rules.js` recognizes only actual unshadowed standard
HashMap/HashSet owners, including their `std::collections::hash_map` and
`hash_set` reexports. There is no `core::collections::HashMap` or HashSet owner.
Actual core/std Default and Clone trait dispatch is checked independently.
Concrete source witnesses retain complete library TYPE arguments, their actual
scalar owners, a concrete type identity, source version, pointer width and
explicit default-hasher/primitive-equality context. Identically spelled local
declarations cannot gain authority.

The closed key and value domains are owned String, Boolean, and signed/unsigned
integers of widths 8, 16, 32, 64, 128 and the explicitly selected 32/64-bit target
pointer width. HashSet has only its key domain. String values retain exact valid
Unicode code units, reject lone surrogates, and do not normalize. Integer state
uses exact BigInt scalars. Custom Hash/Eq, custom hashers, floats, arbitrary owned
records/containers, entry APIs and all collection iteration are refused. No
insertion-order observation is exposed by the internal entry vector. No host
Map/Set substitutes for represented behavior.

The executable `rust-standard-collection-state.js` API is:

- `createRustStandardCollectionState(origin, keyDomain, valueDomain, pointerWidth, root, callOrigin)`
- `readRustStandardCollectionStateError(state)`
- `executeRustStandardCollectionOperation(state, method, arguments_, callOrigin)`
- `readRustStandardCollectionReference(state, reference, operation)`
- `projectRustStandardCollectionOption(state, option, operation)`

Construction uses actual `new` or Default call witnesses. Methods implement len,
is_empty, map contains_key/set contains, get, insert, remove, clear and clone.
Map insert/remove return owned Option values; set insert/remove return bool.
Map replacement preserves one equal key. Clone copies owned String storage and
requires its independently identified output root. Queries do not move roots.
Insertion requires actual owned String transfers and exact argument domains;
borrowed String queries may use the standard Borrow-to-str relationship.

Get returns Option of an explicit root/revision/key/domain reference. Copied
and cloned project that actual reference, with copied refusing String even for
None. This runtime representation never grants a Rust loan: the initial source
adapter rejects escaping bare get and requires immediate actual standard
copied/cloned projection. Mutation creates a new snapshot/revision, returns a
writeback flag, and rejects overlapping live-loan witnesses. Old snapshots stay
intact. Explicit storage bounds are 4096 entries, 1048576 total String units and
safe exact revision coordinates; resource refusal retains the prior state.

`rust-standard-collection-value-packet.js` provides a real executable body for
later automatic projection:

`executeRustStandardCollectionValuePacket(state, method, arguments_, callOrigin, projection)`

It executes the kernel and immediate get projection, returning
`rustCollectionPacketNormal { values: [result, state], writeback }` or the actual
Invalid/Resource transition. Result zero and state one use the existing ordinary
tuple recipe, including read packets that retain their previous state. This
module depends only on the shared collection kernels, keeping compiler graph
metadata out of the eventual represented library.

`rust-standard-collection-call-adapter.js` additionally provides:

- `executeRustStandardCollectionSourcePacket(context, loanState, binderIdentity, authority, collection, method, arguments_, callOrigin, projection)`
- `createRustStandardCollectionSourcePacket(context, binderIdentity, callOrigin)`
- `resumeRustStandardCollectionSourcePacket(packet, roots, binderIdentity, continuation)`
- `readRustStandardCollectionProjectedLibraryError(library)`
- `lowerRustStandardCollectionProjectedSourcePacket(parser, library, request, heldOperands, roots, handlers, continuation)`

The executable context adapter checks the actual whole-collection source
registry and lexical loan authority before kernel access. It reuses
`executeRustExclusiveSourceCollectionOperation` for ordinary reads/mutations and
the shared access operation for immediate get projection and independent clone
results. Normal mutation packets publish the new selected caller root before
the supplied continuation. Constructors return a fresh binding initializer;
they do not write back an existing root.

The projected-library seam first independently recomposes all actual supplied
input modules using `readSuppliedSourceLibraryClosure` with aliases enabled,
reparses the complete closure using `parseJavaScript` and runs `checkProgram`.
It also runs `checkProgram` on the supplied sourceProgram and compares both
complete checked graphs with the supplied library program, preserving actual
Map/Set contents, exact scalars, own descriptors and shared-node correspondence.
Different bodies cannot hide behind genuine checked names or ownership rows.
The initial seam supports only identity symbol/path/constructor mappings;
target emitter renaming needs a separate exact projection interpretation and
currently refuses. It does not infer body equivalence across renamed graphs.

It additionally requires complete actual checked library declarations and fresh
source-module ownership, complete source callable/datatype bodies, matching
transition constructors and fields, and parser-registered function/type/
constructor interfaces. The real packet callable must invoke the real projected
kernel. Body names or availability flags alone refuse. The
source lowerer checks the actual held receiver origin, reserves emitted names,
keeps global qualifications and emits ordinary call/match/tuple/let terms.
Handlers decode the concrete source result and preserve distinct Invalid and
Resource continuations. The resulting expression is intended for the existing
owned-record state/completion continuation, including branch/return/residual
publication. No source text is carried as an executable fallback.

Required library linkage data is `entryPath`, `closure`, checked `program`, complete
`sourceProgram`, `functions: [{role, checkedName, sourcePath}]` with packet/kernel
roles, and `packetType: {checkedName, sourcePath, constructors}`. Each constructor
role normal/invalid/resource maps actual `{checkedName, sourceName}`; the current
identity projection requires equality and refuses unsupported target case
conversion rather than guessing it. The request retains actual
origin, keyDomain, valueDomain, pointerWidth, root, path, method and call witness.
Five already evaluated names supply state/method/arguments/call/projection.
Roots retain the actual root identity, origin and current source value.

The companion loan draft now accepts explicit
`context.collectionTypes: [{typeIdentity, carrierType, origin, keyDomain, valueDomain, pointerWidth}]`
and context pointerWidth. Source positions retain the complete original library
owner and arguments plus `collectionTypeIdentity`; parser origins retain
`sourceCollectionTypeIdentity`. The named carrier must have its actual complete
registered declaration layout. An arbitrary record/Vec is never a carrier proof.
Kernel origin metadata contains recursive TYPE-owner trees: a real specialized
generated carrier/interface must account for that representation, potentially
factoring immutable metadata into actual callable parameters. This is not
resolved by setting a registration flag.

The precise remaining source work is:

1. At the Rust use-item standard import rejection, establish actual collection
   owner resolution through existing aliases and namespace shadow checks.
2. Before the generic type refusal, parse the exact closed collection type
   arguments and retain actual complete source origins through headers,
   locals, calls and returns. Register its real projected carrier declarations.
3. Automatically project the supplied JavaScript kernels/value-packet module
   and install their real bodies/types, with a complete checked correspondence
   interpretation for any target symbol remapping. Improve translation for unsupported
   syntax rather than supplying native handwritten collection semantics.
4. Admit actual mutable collection roots into the state slot/effect inventory.
   Existing record field writes reconstruct nominal fields; whole-collection
   replacement needs its explicit carrier update hook and ordered completion.
5. Parse constructor/method/projection calls from their actual origins, hold
   every argument once, select the real projected callable and decode concrete
   Rust Option/primitive results through actual constructor inventories.
6. Connect actual exclusive formals/caller writeback through the registered
   whole carrier; retain get projection/escape rules and source signatures.

These are integration requirements, not authorization to remove the existing
guards ahead of real bodies and source interfaces. The current common core
map/set checker supports immutable reads/items and refuses mutation; it cannot
serve as a replacement insert/remove implementation.

Deferred controls cover owner/type/call substitutions, exact scalar domains,
primitive Eq, old values, owned copies, reference projection, clone independence,
mutation/resource snapshots, packet publication and descriptor-only bridge
refusal. Thirty independent complete originals and nine refusal inputs are in
`js/tests/fixtures/rust-standard-collections.mjs`, exported as
`rustStandardCollectionOriginals`, `rustStandardCollectionRefusals` and
`rustStandardCollectionVersions`. They retain Rust `>=1.90.0 <1.100.0` with
explicit 32/64-bit pointer profiles. Public original admission, automatic source
projection and native execution remain unobserved. No tests, behavioral imports,
generation, validation, builds, Rust compilation or ledger operations ran.
