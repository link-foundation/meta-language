# Object.fromEntries source batch — 2026-11-02

This handoff supports [the authoritative vision](../vision.md). Source drafts and
future controls provide no behavioral execution evidence.

## Shared source and observable ordering

`javascript-object-from-entries.js` composes the existing synchronous iterator,
primitive ToPropertyKey, ordinary Get, property-definition and heap-budget
algorithms. It creates a genuine result using the supplied actual realm Object
prototype after checking nullish input and before iterator acquisition. The
protected body ignores thisArg and retains actual well-known iterator/conversion
Symbols. Every selected getter/method executes through shared Call.

For each yielded Object, entry `0` is acquired before entry `1`; only afterward
is the held key converted with the string hint. Publication creates ordinary own
data properties, including Symbols and `__proto__`. Last duplicate values win
without changing the original property's position or prototype. All standard
creation flags are true. Inherited result setters are not invoked. There is no
carried original body, host fromEntries fallback or private native collection.

The primary source checks were the dated
[2024 AddEntriesFromIterable algorithm](https://tc39.es/ecma262/2024/multipage/keyed-collections.html#sec-add-entries-from-iterable)
and [2025 Object.fromEntries algorithm](https://tc39.es/ecma262/2025/multipage/fundamental-objects.html#sec-object.fromentries).
These specify direct IteratorStepValue propagation, closing after entry/adder
abrupts, entry key/value Get before the adder, and actual intrinsic TypeError
creation before cleanup. The draft uses existing protocol done flags; next,
done-getter and value-getter failures propagate without closing an already-done
record. Entry getters and key coercion failures close the unfinished iterator.
Cleanup errors cannot replace the original thrown value. Intrinsic errors are
materialized through the actual realm before cleanup, rather than delaying their
allocation or substituting undefined as a fake exception. No source text from
these specifications was copied into generated policies or runtime artifacts.

## Kernel API and shared integration

The root agent owns shared integration files. The new module alone is not public
runtime support until the opcode, pending state, realm owner and automatic source
translation are connected.

Exports:

- `testJavaScriptObjectFromEntriesOperation(operation)`.
- `compileJavaScriptObjectFromEntriesFunction(identity, realm, prototype, iteratorKey)`.
- `beginJavaScriptObjectFromEntries(heap, input, prototype, iteratorKey, exoticKey)`.
- `stepJavaScriptObjectFromEntries(heap, state)`.
- `resumeJavaScriptObjectFromEntriesAllocation(heap, state, identity)`.
- `resumeJavaScriptObjectFromEntriesCall(heap, state, completion)`.
- `provideJavaScriptObjectFromEntriesPrimitivePrototype(heap, state, prototype)`.
- `resumeJavaScriptObjectFromEntriesError(heap, state, actualErrorValue)`.
- `readJavaScriptObjectFromEntriesStateError(heap, state)`.

The opcode is
`{$:'objectFromEntries', destination, input, prototype, iteratorKey}`. The program
supplies the actual exotic conversion Symbol. Its protected function uses five
registers and obtains only argument zero; the result is returned normally.

Every transition has `heap`. The discriminants/other fields are:

| Discriminant | Additional fields |
| --- | --- |
| objectFromEntriesContinue | state |
| objectFromEntriesCall | state, callee, receiver, arguments |
| objectFromEntriesAllocate | state, prototype |
| objectFromEntriesPrimitiveGet | state, receiver, key |
| objectFromEntriesNeedError | state, errorClass, reason |
| objectFromEntriesCompleted | value |
| objectFromEntriesThrow | value |
| objectFromEntriesError | errorClass, reason |
| objectFromEntriesInvalid | reason |
| objectFromEntriesResourceLimit | state, reason |

The VM needs a `waitObjectFromEntries` state with destination, source realm and
kernel state. Program validation checks the primitive-intrinsic frame and operand
coordinates. Pending validation checks the preceding instruction, destination,
realm, actual input/prototype/iterator-Symbol registers, program conversion
Symbol, kernel state and selected child callee/receiver/arguments. The receiver
of the protected global function does not participate. Completed writes the
result register and clears pending; Throw unwinds the original value. Every
transition adopts its latest heap.

Allocate uses the common global identity allocator, then the allocation resumer.
PrimitiveGet uses the existing authenticated primitive-iterator prototype metadata
boundary, then the dedicated provider. Neither resumer executes original user
bodies or duplicates previous cursor movement.

NeedError retains pending state and requests actual source-realm Error allocation.
The Error-allocation resumer must return the actual Error into
`resumeJavaScriptObjectFromEntriesError`, not throw/unwind immediately or erase
this pending operation. The kernel then begins IteratorClose with that original
Error. Existing error-realm validation remains responsible for authenticating the
allocated Error's class/realm identity.

A child getter/key method throw must enter `resumeJavaScriptObjectFromEntriesCall`
with throwCompletion before ordinary unwinding crosses this operation. That may
return Continue to begin cleanup or a final Throw. The unwinder can store an
intermediate state for normal VM stepping when no program argument is available.
A selected callable class can fail during shared Call setup before a child frame
exists: its actual TypeError must likewise retain pending, be allocated in the
actual source realm, and resume that selected call as a throw. During cleanup,
return getter/method errors are suppressed by the existing IteratorClose
precedence when an original throw is already present.

Object and property resource bounds retain current input, cursor/record, held
entry key/value and converted key. Retry must not repeat a getter, coercion or
completed property definition. Resource suspension is not a language throw and
does not close an otherwise resumable iterator.

The root must register actual Object.fromEntries identity/prototype/iterator
Symbol metadata in the protected realm directory, authenticate it on publication
and later verification, classify its callable as nonconstructible, admit it in
the collection directory, and register source/control/fixture roots with the
deferred automatic policy author. Rust bodies must come from the complete
JavaScript source automatically. No generated/native semantics were hand-edited.

## Deferred controls and boundaries

The dedicated fixture file contains seventeen original programs; its runner
selects explicit active Node ranges `>=22.15.0 <23.0.0` and `>=24.0.0 <25.0.0`.
Uncovered versions error rather than skip. The ten kernel/compiler controls and
original-versus-product runner cover allocation order, default Object prototype,
duplicates/order, Symbol and own `__proto__` keys, entry Get/ToPropertyKey ordering,
live mutations, cached next, cleanup precedence, actual Error allocation before
cleanup, done/step failures, ordinary data attributes, nullish inputs, inherited
entry fields, resource retry and protected metadata. Native original bodies run
only inside deferred oracle callbacks; product execution uses the portable VM.
The first fixture preserves order through exact key count/index assertions,
avoiding an unrelated Array.join implementation dependency.

As in existing collection algorithms, intrinsic reasons flow through the shared
source Error boundary. Exact native diagnostic wording remains a separately
version-sensitive requirement; these new error fixtures retain their inputs and
assert error class, actual throw identity and cleanup/effect order. Opaque Proxy
entry/property traps remain governed by their separate generic heap protocols;
unsupported ordinary access is Invalid, never a false missing property or fake
language TypeError.

No tests, behavioral imports, compilation, generation, validators or CI feedback
were executed/read. Only small source files were created in the shared checkout;
no worktrees, caches or dependencies were created. Journals and unrelated files
were preserved. Ledger/source-register, naming and runtime feature mappings await
the mandated source-audit/generation phase.
