# Opt-in standard collection source admission draft

This records unexecuted JavaScript source work. The architecture remains in
`docs/vision.md`. No original, generated library or native compilation was run.

The new source profile is `standardCollectionSourceModel:
'Rust closed standard collections 1'`. It requires
`standardCollectionSourceLibrary` containing the complete actual supplied
library graph and `standardCollectionPointerWidth: 64`. The scalar/state kernel
also supports 32-bit pointer domains, but the existing Rust source frontend's
usize/isize core types are 64-bit; source selection explicitly refuses 32-bit
targets rather than changing those public types. Source versions are bounded
to Rust `>=1.90.0 <1.100.0`, including empty programs selected under the profile.

New helper modules are `rust-standard-collection-source.js`, `types.js`,
`calls.js` and `signatures.js` under `js/src/translation/`. The parser integration
is owned by the companion exclusive-source batch. Hooks initialize all parser
passes after the active version is set, handle standard imports, parse concrete
types before the generic refusal, handle constructors and eager methods before
ordinary nominal clone dispatch, and append real helper/library bodies at exit.
Legacy source guards retain their existing path when the profile is absent.

The initializer reconstructs/checks the actual supplied source graph through the
prior packet adapter. It installs complete actual data, constructor and function
source interfaces from that graph. An arbitrary callable descriptor cannot pass.
The initial graph correspondence supports exact identity symbol mapping and
refuses unsupported target remapping. Public source calls additionally require
distinct actual protocol/resource source continuations in
`library.sourceHandlers.invalid` and `.resource`; no native fallback or carry-
source path substitutes for absent bodies.

Concrete annotated HashMap/HashSet types retain actual unshadowed library owners,
exact key/value child positions, canonical primitive/String identities and
target width. Both canonical and original scalar source positions remain
available; type aliases/import aliases do not grant a local lookalike authority.
The origin retains `sourceCollectionTypeIdentity` and
`sourceStandardCollection`, with complete closed source facts. A real registered
kernel state datatype supplies its underlying carrier. The actual graph/type
validator gates mutable storage and owned recognition. The companion source
origin hooks compare concrete collection identities during arguments and joins,
so sharing an internal carrier never erases different key/value types.

Constructors require an explicit matching expected annotation. Their source
bodies call the real projected `createRustStandardCollectionState`, matching its
actual Created/Invalid/Resource constructors. Method wrappers call the actual
`executeRustStandardCollectionValuePacket`, match complete actual transitions,
decode concrete primitives/standard Option results through actual constructor
inventories, and return an ordinary `(result, state)` tuple. They never evaluate
collections during parsing or use mutable core map operations.

String encoding/decoding are two additional executable JavaScript functions in
`rust-standard-collection-value-packet.js`; the source wrappers invoke those
real projected bodies. Integer/Boolean scalar wrapping is boundary constructor
transport selected by actual checked fields. Integer casts retain the complete
source domain and kernel range validation. Direct unsuffixed numeric literals
receive their actual protected method-position type; independently typed
variables are never recast.

The drafted path covers new/default, len/is_empty, contains_key/contains, get
with immediate copied/cloned, insert/remove/clear and clone on direct owned
receivers. String insertion retains actual ownership transfers. Mutation through
borrowed receivers, nested-reference Clone dispatch and escaping get references
remain explicit refusals pending their separate composition. Queries can retain
actual ordinary shared receivers. Other old source profiles and exclusive-source
syntax remain outside this initial public composition; whole-carrier runtime
packet/loan interoperability from the prior batch remains available separately.

Mutations require an actual live mutable state slot. Later argument mutations
of the same reserved receiver root refuse instead of overwriting their effects
with an earlier snapshot. Real generated wrapper functions return result zero
and updated state one. `recordRustExclusiveSourcePacketEffect` in the companion
batch orders the call and whole-root writeback before the existing completion
continuation, preserving branches, source returns and residual paths. Reads
retain their previous state. Clone produces an independent owned result.

The exact helper API includes:

- `initializeRustStandardCollectionSource(parser, model, library, pointerWidth)`
- `readRustStandardCollectionSourceImport(parser, segments, path)`
- `readRustStandardCollectionSourceType(parser, segments, resolved, path, start)`
- `readRustStandardCollectionSourceConstructor(parser, segments, arguments_, path, span)`
- `readRustStandardCollectionSourceMethod(parser, receiver, method, arguments_, span, path)`
- `readRustStandardCollectionSourceStorage(parser, origin)`
- `readRustStandardCollectionSourcePositionError(parser, actual, expected)`
- `refuseRustStandardCollectionSourcePrivateName(parser, segments, path, span)`
- `finishRustStandardCollectionSource(parser, program)`

The private-name hook rejects original user type/expression references to emitted
implementation entries; generated AST calls keep their full qualifications.
Declared library collisions also refuse. Shared owned recognition changes in
`rust-owned-bindings.js` and `rust-standard-sum-ownership.js` require the exact
profile and the actual graph-backed storage callback, never a marker alone.

Actual supplied graph availability is still necessary. Automatic source
projection may expose translator gaps; those require translator improvements,
not hand-written Rust semantics or body-availability promises. Header, body,
resource continuation and native parity behavior remain unobserved. The prior
30 original programs and nine refusal inputs remain independent obligations;
their exclusive-parameter cases are not claimed admitted by this standalone
profile. Deferred source controls were authored but not run. No ledger/source
register was used, and no test, behavior import, generation, validation or
compilation ran for this batch.
