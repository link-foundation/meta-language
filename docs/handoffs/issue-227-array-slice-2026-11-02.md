# Generic Array slice source handoff

This is an unexecuted source draft. No tests, imports, generation, validation,
compilation or Rust operations ran. The source register and requirement ledger
were left unread and unchanged. These files declare controls and expected
behavior; they contain no execution evidence or completion claim.

The new `js/src/translation/javascript-array-slice.js` exposes:

- `beginJavaScriptArraySlice(heap, directory, receiver, start, end, endPresent, exoticKey)`
- `stepJavaScriptArraySlice(heap, directory, state)`
- `readJavaScriptArraySliceStateError(heap, directory, state)`
- `resumeJavaScriptArraySliceCall(heap, directory, state, completion)`
- `resumeJavaScriptArraySliceAllocation(heap, directory, state, identity)`
- `resumeJavaScriptArraySliceConstruction(heap, directory, state, completion)`
- `readJavaScriptArraySliceRelativeIndex(number, savedLength)`

`receiver` must be the genuine Object returned by the caller's shared ToObject
adapter. `directory` is the existing `JavaScriptArraySpeciesDirectory`.
`endPresent` retains actual argument presence; false requires an undefined end.
Missing and explicitly undefined end both select the captured length without
end conversion. Start still performs ordinary ToNumber/ToIntegerOrInfinity.

The exported data types are `JavaScriptArraySliceState`,
`JavaScriptArraySliceCall` and `JavaScriptArraySliceTransition`. Transition tags
are `arraySliceContinue`, `arraySliceCall`, `arraySliceAllocate`,
`arraySliceConstruct`, `arraySliceNeedAuthority`, `arraySliceCompleted`,
`arraySliceThrow`, `arraySliceError`, `arraySliceInvalid`, and
`arraySliceResourceLimit`. Call, Construct, allocation, authority and completion
payloads follow the existing Array transformation transition convention.
Construct receives exactly `[{$: 'number', value: state.count}]` with its actual
constructor fact. Allocation retains actual prototype and count before identity
resumption. Shared species construction must apply actual receiver/return rules
before passing an Object result to the construction resumer.

The state retains original arguments, actual receiver, saved ToLength, normalized
start/end indices, selected count, corresponding input/output coordinates and
held output value. Shared scalar continuations enforce length/start/end ordering;
species selection precedes indexed access. Shared Has/Get observes inherited
indices and live mutation while the saved interval stays fixed. Missing source
properties advance the output coordinate without definitions. Present values
use the existing `createJavaScriptIndexedDataProperty` helper. Resource outcomes
retain completed getter values rather than replaying effects.

Final length uses the existing Object assignment strict Set path with one saved
length key and count. Its nested `JavaScriptObjectSpreadState` is retained as
`state.assignment`, including existing Array assignment numeric/definition
continuations. Setter callbacks, writable checks, ArraySetLength, abrupt values,
partial writes and resources therefore reuse the existing shared protocol.
Validation constrains this child to its actual target/key/value and write or
completed phase; it cannot restart source enumeration or Get.

Deferred controls in `js/tests/translation-javascript-array-slice.test.js` cover
15 kernel scenarios, including holes, inheritance, ordered conversions,
allocation, custom species, setter dispatch, Array length truncation, actual
throws, authority extension, retained resource values and forged responses.
`js/tests/fixtures/javascript-array-slice-source.mjs` exports
`javascriptArraySliceOriginals` (24 literal originals),
`javascriptArraySliceSourceVersions`, and `javascriptArraySliceAssertionPolicy`.
Every original declares `expected: true`; Node scope is
`>=22.15 <23 || >=24 <25`. Other versions are explicitly uncovered.

The root batch owns the compiler adapter, callable catalog, VM opcode/pending
integration and later source registration, generation, translation and execution.
This draft implements no native slice body or handwritten species constructor.
