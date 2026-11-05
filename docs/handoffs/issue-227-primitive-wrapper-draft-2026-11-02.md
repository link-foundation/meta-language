# Object Call primitive wrapper source draft

This source handoff supplements [the vision](../vision.md); it is not another
architecture specification or execution evidence. No represented program, test,
validation gate, source generator, parser/type-checker or compiler executed.

## Shared transition API

`js/src/translation/javascript-primitive-wrappers.js` exports:

- `beginJavaScriptPrimitiveWrapper(heap,input,prototype)` validates the complete
  current heap/input, passes actual Object identities through unchanged without
  allocating or consulting a prototype, or requests one fresh global identity.
  The VM supplies and authenticates the selected actual realm prototype.
- `readJavaScriptPrimitiveWrapperStateError(heap,state)` validates a pending
  allocation's original primitive/nullish input and supplied prototype. State is
  `{$:'primitiveWrapperState',input,prototype}`. The VM must additionally compare
  both fields with its actual instruction operands, preventing valid-but-swapped
  state inputs/prototypes from replacing the original operation.
- `resumeJavaScriptPrimitiveWrapper(heap,state,identity)` accepts an externally
  allocated, unoccupied unsigned64 identity, rechecks current resource bounds,
  and atomically publishes the completed wrapper heap. It never invokes a native
  represented Object, Number, Boolean, BigInt, Symbol or String constructor.

Transitions are `primitiveWrapperAllocate {heap,state,prototype}`,
`primitiveWrapperCompleted {heap,value}`, `primitiveWrapperInvalid {heap,reason}`,
and `primitiveWrapperResourceLimit {heap,state,reason}`. These fields match the
planned VM use through its existing Object-allocation metadata event. Invalid
input/identity failures leave the original heap; completed rows appear only after
all properties and slots are ready. Resource checks happen before the allocation
request and are repeated when accepting the identity against the latest heap.

## Slots, properties and bounds

Number/Boolean/BigInt/Symbol inputs retain exact own `primitiveData`, including
Number -0/NaN and actual globally registered Symbol identity. String/sourceText
inputs retain exact own `stringData`. Each UTF16 code unit gets a complete own
index descriptor: nonwritable, enumerable, nonconfigurable. The own length data
descriptor is nonwritable, nonenumerable and nonconfigurable. Units are not Unicode
code points: supplementary pairs produce two indexed properties and isolated
surrogates remain exact. String key creation uses a bounded source decimal-digit
algorithm, not a native represented property-conversion fallback.

All new wrappers are noncallable, extensible ordinary Objects with the supplied
actual prototype. Internal slots are derived only from the actual input value;
a prototype lookalike never acquires a slot. Object inputs, including Arrays and
callables, retain identity. Null/undefined are Object Call's fresh empty-Object
special case. General ToObject's nullish TypeError remains the responsibility of
callers requesting that separate operation before using this allocation kernel.

The existing heap profile permits 4096 objects, 4096 own properties per object and
16384 own properties globally. String wrappers consequently require at most 4095
input units in this selected profile, including their length descriptor. Requests
above these bounds return resources, not invented language errors, truncated
wrappers, partial success or consumed identities from an internal allocator.
Broader String wrapper input/resource profiles remain independently required.

## Deferred controls and root integration

Twelve literal allocation inputs cover nullish empty Objects, signed zero/NaN/
infinity, Booleans, exact BigInt, actual Symbol IDs, empty text, surrogate units
and compact sourceText. Deferred controls inspect all slots/prototypes/descriptors,
functional heap preservation, genuine Array/callable passthrough, no prototype
slot inference, fresh/colliding identities, forged states, string limits, object
limits and global property limits. Thirteen original source cases compare actual
Object Call behavior with the portable VM when root integration is available.
Their native-oracle range is Node >=22.15 <23 or >=24 <25; the future generated
native kernel retains the established Rust 1.90–1.99/edition 2024 scope.

Root integration still must select Object/String/Number/Boolean/BigInt/Symbol
realm roots in a protected Object Call body, introduce the wrapper opcode and its
pending-state authentication, route Allocate through the existing fresh global-ID
request/resumer, preserve slot data in source materialization, register both new
fixture exports and controls with automatic authoring and the native replay
producer/driver, and update vision/feature/changelog/source mappings in the proper
phase. Native semantics must be translated automatically from this source; no
handwritten Rust wrapper algorithm should replace it. No observations are claimed.
