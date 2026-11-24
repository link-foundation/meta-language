# Reflective definitions and descriptor operand source draft

This continues [live callable handles](issue-227-live-callable-handles-2026-11-03.md)
under the sole architecture in [vision](../vision.md). Source edits, textual
review and primary specification reading only. No represented program, test,
generator, formatter, validation gate, compilation or CI feedback ran or was read.
The complete code draft remains unfinished and the full active goal is unchanged.

## Actual source paths drafted

`javascript-property-definition.js` shares the complete Object and Reflect
definition sequence: target validation, actual key conversion, ordered descriptor
Get/validation, ordinary definition and resumable Array length definition. An
optional explicit operation retains the Boolean Reflect policy through all
states. Absent operation preserves the old Object target-or-throw interpretation.
Reflect ordinary incompatibility returns false; Array incompatibility retains
actual partial deletion/rollback/readonly effects and returns false. Actual
RangeError and conversion/getter throws remain abrupt with their original values.
The protected Reflect.defineProperty owner is registered and compiled from this
same operation path, retaining target-before-key-before-descriptor ordering.

Selected definition callbacks retain their actual callable, receiver and complete
bounded arguments. Retrying a callback request returns that saved call without
re-reading an already selected getter; actual child frames are checked against
the saved selection. New Reflect suspensions cannot hide an unrelated child.
Preceding saved Object states without the new fields retain their original
interpretation and actual completion entry points.

Ordinary DefineOwnProperty now distinguishes actual storage exhaustion in its
result data. Only actual budget branches set resourceLimit; ordinary incompatibility
still returns false before capacity admission. The definition controller saves a
`propertyDefinitionReady` with the complete converted descriptor and selected key
instead of reporting storage as invalid input or repeating descriptor getters.
Retry reuses that descriptor; a completed descriptor state cannot accept a child
completion. Array numeric continuations keep their preceding separate phase/work
protocol. Bulk Object definitions reject a Reflect policy and retain their exact
selected preconverted key/descriptor, including a storage-ready state.
Its early capacity check now delegates to the common definition result, so an
already incompatible descriptor is not hidden behind an unnecessary resource
outcome. Valid additions still retain actual storage suspension.

`javascript-descriptor-operands.js` drafts a checked projection/reconstruction of
six descriptor presence/payload pairs as twelve semantic frame operands. Present
attribute payloads must be Booleans; value/getter/setter payloads retain semantic
identity, actual accessor callability and partial descriptor kind. Absent payloads
carry no meaning. Descriptor comparison uses SameValue on present values,
including negative zero, NaN and undefined accessors.

The new `ordinaryDefine` opcode consumes that complete internal descriptor and
uses the same Reflect definition controller. Instruction/frame validation keeps
the actual target/key, realm, twelve operand coordinates, descriptor and result
policy. Source Object/Reflect entry points still perform their real
ToPropertyDescriptor. The fifth private live-handle coordinator instead accepts
an already converted host descriptor, preserving field presence without creating
a source attributes Object. This prevents inherited represented prototype fields
from changing a descriptor that the host has already converted, avoids duplicate
getter effects and needs no extra heap identity for the descriptor carrier.

The Node live adapter routes Array length value definitions and actual ordinary
descriptor storage exhaustion through that real coordinator. Numeric conversion,
partial deletion, false results, errors and work suspension use the common driver.
Other completed ordinary descriptor definitions retain their direct shared kernel
path. No original represented body or native Array definition substitutes for
these operations. Native carriers still mirror irreversible descriptors only to
retain their Proxy invariants.

Primary contracts: [Reflect.defineProperty](https://tc39.es/ecma262/2025/multipage/reflection.html#sec-reflect.defineproperty)
and [ArraySetLength](https://tc39.es/ecma262/2025/multipage/ordinary-and-exotic-objects-behaviours.html#sec-arraysetlength).
Textual correspondence is not execution evidence.

## Independent controls authored, all unexecuted

Fifteen native-original programs cover Boolean success/false versus Object
throwing, descriptor defaults and partial masks, nonextensible receivers,
target/key/descriptor order, all descriptor fields before mixed-kind rejection,
actual Symbol keys, readonly Array length, two numeric conversions, RangeError,
latest receiver effects, first/second conversion throw identity, partial deletion
and readonly rollback, incompatible attributes and bulk throwing behavior.
Node assertion ranges are explicit; every preceding input/assertion remains.

Additional controls cover twelve-operand round trips and malformed/mixed masks,
exact SameValue bits, both Array result policies, saved callback retry, converted
descriptor retention across storage exhaustion, incompatibility before capacity,
legacy Object states, real public Array definition effects without an extra
descriptor Object, represented prototype pollution after host conversion, public
work retry and forged pending operation/descriptor/child arguments. Compiler
selection retains the old Object instruction shape and rejects unknown operations.
No controls ran; these input counts are not requirements counts or a percentage.

The automatic source recipe includes the new descriptor library, literal
originals and their consumer. Source/model/native publication and every actual
cross-runtime execution remain deferred. No handwritten Rust behavior or generated
native edit was added.

## Remaining actual code work

1. Complete native callable metadata authority/source text and brand/slot ports,
   including Function constructor/toString and restricted property accessors.
   Extend protected identity catalogs with actual versioned ownership; callable
   shape and source-looking/native-looking text are not authority.
2. Complete native callback/iterator/async continuations. A host operation that
   has not yet reached the live Proxy definition trap still owns its own native
   key/descriptor conversion stack; source getter suspension there cannot replay
   that stack or pretend the entire host call completed. Portable source Reflect
   already retains these conversions in the common controller. Keep exact
   effects/throws/cancellation and do not remove the source reentry guard as a
   substitute for a real external continuation.
3. Complete explicit external root/retention and storage-release admission for
   live handles and suspended machines. Work-slice retry retains the exact heap;
   raw data controllers can supply a checked released heap, while the host token
   rejects intervening writes. Draft the full GC/root/update protocol before
   claiming automatic native storage recovery or full callable liveness.
4. Continue derived/super/uninitialized-this, Proxy/private-field and mutable
   environment/effect protocols; Rust ownership/callback/iterator/storage and
   generic/trait/macro/module support; full own-project CST/default parsing,
   emission and both translation directions; and all broader shared concept,
   recursive grammar, ontology/taxonomy, RDF/OWL, dynamic, inference, formal and
   delivery requirements in vision.
5. Only after all obligations have complete actual code/control drafts, reconcile
   sources online before ledger access, publish automatically generated assets,
   execute authorized JavaScript and CI-only gates, fix actual failures and record
   observations at the executed commits. Never compile Rust locally.
