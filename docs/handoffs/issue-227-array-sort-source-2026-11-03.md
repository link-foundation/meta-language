# Shared Array sorting draft

This records source work under [the authoritative vision](../vision.md), with no
execution evidence. `javascript-array-sort-call-source.js` drafts Array.sort and
Array.toSorted through existing shared bytecode. It introduces no new VM opcode
or handwritten native sort implementation.

The actual comparator is checked before receiver conversion. The receiver is
boxed and its ToLength captured once. Mutating sort selects HasProperty/Get
values in ascending order, excluding absent entries. toSorted validates ordinary
Array length before indexed Gets and includes undefined for every hole. The
hidden dense working Array never goes to a user callback. Shared ordinary data
property creation bypasses prototype setters for working/output entries.

The initial stable insertion algorithm holds each original item, reads its
preceding dense neighbor and moves that neighbor only for a positive comparison.
Undefined ordering bypasses callbacks and String conversion. Actual comparators
receive undefined as thisArgument and their returned values use the existing
ToNumber protocol; NaN behaves as stable zero. Default comparisons convert left
then right to String and use the shared UTF16 comparison operation. Calls,
getters, scalar conversion and resource interruptions retain normal VM frames.

After all comparisons, mutating sort strictly Sets selected values in ascending
order and deletes the captured remaining tail. Partial writes and failed strict
deletions remain observable. It returns the original boxed receiver, without a
final length Set. toSorted returns its dense fresh realm Array, never reading
source constructor/species. Captured source values are preserved across later
mutations; there is no carried-source execution fallback.

The comparison sequence is implementation selected, as permitted by the
[ECMAScript sorting contracts](https://tc39.es/ecma262/multipage/indexed-collections.html#sec-sortindexedproperties).
It does not claim V8's comparator count/order, output for an inconsistent
comparator, or unstable user String conversions. The literal controls assert
stable consistent results and specification-mandated effects rather than one
engine's comparison trace. The initial algorithm has quadratic worst-case
comparison cost; actual work is bounded/resumable through the shared VM step
budget. A faster shared stable algorithm can replace it without weakening
semantic controls. Opaque Proxy/host operations retain their separate protocol.

Twenty-four literal originals cover numeric/default ordering, stable object
identity, undefined/holes, inherited values, capture-before-comparison, original
throws, comparator-result conversion, Number-only results, NaN, strict partial
writes/deletes, UTF16 order and dense copying. Node assertions explicitly cover
>=22.15 <23 or >=24 <25. The complete source producer/native replay inventory,
automatic source author and Links fixture conversion include these originals.
Deferred bytecode controls retain branch bounds, actual callable invocation and
copy-versus-mutating access operations. Generation, behavior, tests, compilation
and validation remain deferred; broader language requirements remain open.

The Node source embedding additionally drafts `resumeJavaScriptSourceTransition`.
It accepts an actual retained machineResourceLimit transition and delegates a new
bounded slice to the existing checked driver. It does not reinterpret completion,
silence protocol/storage limits or call an original host method. Deferred controls
exercise one-step exhaustion, a zero-step retry, repeated bounded slices and
actual sorting/copy results; invalid boundaries and budgets refuse. Public Node
entry declarations include the exact resource-transition input. No retry or
source body was executed while drafting.
