# Object assignment target coercion source handoff

This batch is an unexecuted source draft. No test, import behavior, source
generation, validation gate, native compilation or Rust operation ran. No result
here establishes behavioral evidence or requirement completion. The source
register and requirement ledger were deliberately left unread and unchanged.

`compileJavaScriptObjectAssignmentFunction(identity, realm, roots)` in
`js/src/translation/javascript-object-assignment-call-source.js` returns the
protected Call body or `undefined` when complete actual primitive wrapper roots
are unavailable. It uses `appendJavaScriptObjectCoercion` before examining the
source argument list. Existing Objects retain their identity; primitive inputs
use the existing shared wrapper allocator and actual realm roots. Nullish input
uses the shared coercibility failure before source enumeration or Get.

The saved boxed target remains in register 2 throughout the actual argument
loop. Sources begin at argument index 1, use `argumentCount`/`argumentAt`, and
flow individually through `objectSpread` with `operation: 'assignment'` and the
actual realm. Thus key enumeration, Get, strict Set, setter dispatch, failure
ordering, completed writes and resumable resources remain the responsibility
of the existing shared assignment kernel. There is no native assignment call,
replacement record allocation or primitive-slot reconstruction in this body.

The deferred direct-VM controls in
`js/tests/translation-javascript-object-assignment-call-source.test.js` cover
Object identity, all primitive target brands and prototypes, nullish failure
ordering, allocation suspension/resumption, immutable String indices, UTF16
units, complete argument consumption, Symbol keys, source failures, inherited
setter receivers and primitive String sources. They have not run. The literal
fixture `js/tests/fixtures/javascript-object-assignment-coercion.mjs` supplies
17 original programs, each declaring `expected: true`, scoped to Node
`>=22.15 <23 || >=24 <25`. Its policy text states algorithm-derived assertions,
not observations. Other Node versions are uncovered by that fixture policy.

Integration still needs the callable catalog to dispatch `Object.assign` to
this compiler with the actual wrapper roots, and later native/source fixture
registration, source reconciliation, generated obligations, translations and
deferred execution after the full draft. The catalog is owned by the integrating
batch and was not edited here. No helper API gap was found by source inspection.
