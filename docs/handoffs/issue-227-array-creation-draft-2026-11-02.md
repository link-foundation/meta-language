# Protected Array creation source draft

This source handoff supplements [the vision](../vision.md); it is not another
architecture specification or execution evidence. No tests, represented imports,
generation, validation gates, parsing/type-checking or compilation executed.

## Kernel and boundary API

`js/src/translation/javascript-array-creation.js` drafts Array.of and Array.from
against the actual immutable argument vector and actual Call receiver. The
protected compiler emits `arrayCreation {destination,operation,prototype,
iteratorKey}`; VM integration supplies frame receiver/arguments and program
Symbol.toPrimitive authority. No source Array substitutes for an argument List.

The kernel exports begin/step, readStateError, resumeCall, resumeConstruction,
resumeAllocation, providePrimitivePrototype and resumeError operations, using
`JavaScriptArrayCreationState` and `arrayCreation...` transition tags. The exact
public names follow the existing `JavaScriptArrayCreation` prefix conventions.
Call transitions retain actual selected callee/receiver/arguments in `state.call`.
Generic Construct authority is external to this kernel:

`arrayCreationConstruct {heap,state,constructor,arguments,
fallback:{$:'arrayCreationArrayFallback',prototype,length}}`

The generic bridge must perform actual IsConstructor on the candidate: Construct
with the supplied argument List when true, or actual intrinsic ArrayCreate with
the supplied fallback prototype/length when false. Callability/name/prototype
shape is not constructor evidence. No Symbol.species lookup occurs. Returned
Construct/ArrayCreate values must be actual Objects; original throws propagate.
Intrinsic ArrayCreate lengths beyond uint32 must retain RangeError, while an
ordinary custom constructor may accept the full safe ToLength value.

`arrayCreationAllocate` is exclusively a temporary ToObject wrapper request;
`resumeJavaScriptArrayCreationAllocation` composes the existing wrapper kernel.
Primitive method lookup uses `arrayCreationPrimitiveGet {receiver,key}` and the
existing private primitive-prototype metadata authority. Root must authenticate
that request before calling `provideJavaScriptArrayCreationPrimitivePrototype`.
All new roots/keys/state fields remain tied to actual selected instruction/frame
operands by the root pending-state validator, not this source handoff.

## Observable semantics

Array.of requests Construct(C,[actual count]) or ArrayCreate(count), defines each
index with complete writable/enumerable/configurable data flags, then performs
strict Set of length. Existing inherited setters do not intercept index creation;
the final actual length setter does run, including an empty result.

Array.from validates the mapper before iterator acquisition. It gets @@iterator
once, interpreting null/undefined as absent and rejecting noncallable methods.
An iterable result requests Construct(C,[]) or ArrayCreate(0) **before** calling
the cached iterator method. The common acquisition protocol then caches next.
Each IteratorStepValue result is mapped with exactly (value,index), using the
actual thisArgument, then defined. The maximum safe index is checked before the
next iterator step. Mapper/definition/max-index failures close the unfinished
iterator; intrinsic failures request the actual realm Error before cleanup.
Original throws win when close lookup/call throws or returns a primitive.
IteratorStepValue failures retain the common done flag and do not close. Final
strict length Set happens after exhaustion and does not close an exhausted record.

Absent @@iterator selects ToObject/LengthOfArrayLike before Construct(C,[len]).
Primitive inputs use a real temporary wrapper, so length getters receive an
Object receiver, while the earlier GetV @@iterator getter sees the original
primitive receiver. Length conversion composes the original numeric protocol and
ToLength. Every index is read with Get, including holes/inherited getters, without
HasProperty filtering. Array-like mapper/definition failures have no iterator to
close. Completed scalar values, mapped values, indices, strict ArraySetLength
state and original effects remain retained across resource suspension.

Final Set composes the common ordinary setter/Array assignment protocol, retaining
both Array length conversions and any partial deletions. Existing property,
argument, wrapper and stack profiles stay explicit resource boundaries; they do
not become fabricated language TypeErrors or truncated successful results. Opaque
host/Proxy property protocols and unsupported constructor bodies remain explicit
shared boundary obligations; no native represented methods supply their semantics.

## Deferred controls and integration

The new literal fixture declares twenty-eight original source cases, bounded to
Node >=22.15 <23 or >=24 <25 and the generated Rust 1.90–1.99/edition 2024 scope.
They cover intrinsic/custom/nonconstructible receivers, argument-count forms,
mapper/iterator/constructor/Get ordering, cached methods, sparse/inherited input,
primitive inputs and String code points, strict length setters, Symbol/BigInt
conversion, close precedence, no-close IteratorStepValue failures, large safe
length/custom constructor handling, uint32 fallback failure and ignored species.

Fourteen deferred kernel/body controls exercise real heap transitions and actual
boundary arguments, resource retry without callback replay, Error-before-close,
max-safe-index ordering, primitive boxing receiver identity and mutation-resistant
iterator caches. Full source controls compare the portable VM with original Node
bodies. No control executed, and no completion or evidence status is asserted.

Root still owns the opcode/pending/generic-Construct bridge, callback/throw/Error
resumption, primitive metadata and temporary allocation routing, actual realm
owner/directory registration, native model codecs/replay, producer fixture groups,
automatic source policy author, vision/feature/changelog/source mappings. Selected
Call intrinsic TypeErrors must re-enter resumeCall with an actual Error before
IteratorClose instead of dropping the outer Array creation state. Native semantics
must be translated from this source, never replaced with handwritten Rust.

The state contract also forbids an iterator record before construction/completed
acquisition and forbids publishing a result before Construct returns. Final Set
requires actual exhaustion/completed indices. Temporary allocation replies must
match the actual wrapper Allocate event; a current resource request is retained
without consuming the supplied identity or advancing another phase.
