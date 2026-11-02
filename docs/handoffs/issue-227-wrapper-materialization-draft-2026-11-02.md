# Boxed output data materialization source draft

This handoff supplements [the vision](../vision.md). It describes source changes,
not another architecture specification or execution evidence. No tests, imported
programs, parsers/type-checkers, generation, validation or compilation ran.

The Node data adapter now reifies slot-bearing ordinary rows after portable
Object Call/wrapper semantics have completed. A protected native Object factory
receives only an already primitive metadata value and creates an external data
carrier. It supplies no represented coercion, constructor, getter or method body.
Private Symbol ownership remains authoritative. Number/Boolean/BigInt/Symbol
primitiveData and StringData survive export, including -0, NaN, infinity, exact
integers and isolated UTF16 units. Returned boxes whose prototypes were changed
to null or the supplied Object root retain their internal slots rather than
silently becoming ordinary records.

Supported prototypes are null, the supplied Object root and privately registered
primitive roots. Their native identities and complete prototype descriptor chains
must match the represented snapshots; changed or unknown/custom prototypes refuse
explicitly. Accessors remain refused without executing them. Native String
carriers' inherent immutable index/length descriptors must all match actual heap
properties before reification; absent or incompatible properties cannot be
silently synthesized. Additional data descriptors, Symbol keys, extensibility,
cycles and aliases are preserved. Memoization occurs before recursive data fields.
Plain record and Array paths retain their existing behavior.

`js/tests/fixtures/javascript-wrapper-materialization.mjs` declares
`javascriptWrapperMaterializationVersions`, sixteen
`javascriptWrapperMaterializationOriginals` and three
`javascriptWrapperMaterializationRefusals`. Accepted originals return real boxed
outputs or data graphs containing boxes. Literal expected values are JSON-safe
observer declarations, not observed evidence. Node original-oracle scope is
>=22.15 <23 or >=24 <25; future native scope follows generated Rust 1.90–1.99,
edition 2024. Refusals cover custom prototypes, accessors and changed registered
prototype snapshots; source bodies run only in the portable VM in those controls.

`js/tests/support/wrapper-materialization.js` exports
`observeWrapperMaterialization(value)`. Its data signature is
`{root,objects,symbols}`. Encoded values distinguish graph references, Symbols,
exact UTF16 text units, BigInt decimal text, finite Numbers, explicit NaN/-0/
infinity tags, Booleans, null and undefined. Objects record kind, supplied root,
extensibility, optional internal slot and every own data descriptor. One graph
identity allocator covers objects and Symbols; repeated references/cycles retain
identity and fresh Symbols retain distinctness while normalizing host identities.
Well-known and registered Symbol metadata remains explicit. Only metadata slot
readers inspect native carriers; represented bodies are never executed there.

The dedicated deferred test file compares both native original and portable
materialized observations against each literal expected signature. It also
retains accessor nonexecution, missing/changed String descriptor rejection,
cycles/aliases/fresh snapshots and former sparse Array/plain data behavior.
Root integration must register these literal fixture exports, helper/control
sources, producer observation lane and native replay/projection assertions. The
existing source trace must remain complete; plain JSON serialization of a box is
not evidence of its slot/prototype semantics. Root owns vision/changelog updates.
