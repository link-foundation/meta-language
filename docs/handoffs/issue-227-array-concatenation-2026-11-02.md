# Array concatenation shared-kernel source draft

This is an unexecuted implementation handoff. `docs/vision.md` remains the only
architecture specification; declarations and deferred tests are not evidence.

The new `js/src/translation/javascript-array-concatenation.js` exports:

- `beginJavaScriptArrayConcatenation(heap, directory, receiver, arguments_, spreadableKey, exoticKey)`
- `readJavaScriptArrayConcatenationStateError(heap, directory, state)`
- `stepJavaScriptArrayConcatenation(heap, directory, state)`
- `resumeJavaScriptArrayConcatenationCall(heap, directory, state, completion)`
- `resumeJavaScriptArrayConcatenationConstruction(heap, directory, state, completion)`
- `resumeJavaScriptArrayConcatenationAllocation(heap, directory, state, identity)`

The receiver is the actual shared ToObject result supplied by the compiler/VM.
Arguments are the actual complete Call argument List, copied into held state.
The two Symbol operands are independently authenticated isConcatSpreadable and
toPrimitive identities. The species directory supplies actual realm authority.
Transitions use the `arrayConcatenation` prefix with Continue, Call, Allocate,
Construct, NeedAuthority, Completed, Throw, Error, Invalid and ResourceLimit.

Species selection starts with length zero and completes before receiver or
argument spreadability reads. Default/intrinsic allocation requests length zero;
custom Construct receives `[0]`. The receiver is item zero, followed by every
argument, retaining aliases and repeated reads. Objects Get spreadability once;
undefined alone falls back to the actual Array brand. Other values use shared
ToBoolean without conversion. Primitive arguments are scalar items.

Spreading reads and converts one saved length, checks the full span against the
remaining safe output coordinate range, then Has/Gets each coordinate in order.
Holes advance output without a definition; inherited present values create own
data properties. Scalar writes check their bound independently. Completed
getter/conversion values survive property/resource suspension without replay.
Strict final length Set composes the objectSpread assignment child used by
slice, retaining Array length rules and actual custom setters. No host method,
proxy trap or represented constructor body is executed as fallback.

Deferred controls cover species ordering, hole/inherited/scalar behavior,
spreadability overrides, length conversion, retained getter values, custom
construction/setters, abrupt values, authority extension, output bounds and
malformed continuations. Large-coordinate controls seed stored boundary
coordinates rather than performing unbounded prefix loops. They are controls of
the declared state contract, not observations of a completed huge concat.

`js/tests/fixtures/javascript-array-concatenation-source.mjs` exports
`javascriptArrayConcatenationOriginals`, `javascriptArrayConcatenationVersions`
and `javascriptArrayConcatenationAssertionPolicy`. Thirty-one independent
expected-true originals cover species, spreadability/getter order, sparse and
inherited values, primitive receiver/arguments, shared aliases, strict writes,
original abrupt identities and count bounds. The policy selects Node
`>=22.15 <23 || >=24 <25`; other versions remain uncovered. The Proxy original
is a retained obligation: the current ordinary Get/Has/definition helpers refuse
opaque/Proxy behavior, and this draft supplies no proxy implementation.

Parent integration must connect the protected concat owner, actual Symbol and
species metadata, boxed compiler, VM pending state and shared construction/
allocation resumers. Public export/source inventory and translator/native
replay integration remain parent-owned. No source register or ledger was read
or changed. No test, behavior import, generation, validation, build or Rust
compilation was run. Native observations, original admission and runtime parity
remain outstanding; no completion claim follows from this source draft.
