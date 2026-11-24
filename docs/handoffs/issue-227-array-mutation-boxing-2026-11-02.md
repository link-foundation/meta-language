# Array mutation receiver boxing source draft

This handoff describes an unexecuted source draft, not runtime evidence. The
architecture remains defined by `docs/vision.md`; this file records integration
and review details for this batch only.

`compileJavaScriptBoxedArrayMutationFunction(identity, realm, operation, roots)`
is exported from
`js/src/translation/javascript-boxed-array-mutation-call-source.js`. It accepts
the existing seven `Array.push`, `Array.pop`, `Array.shift`, `Array.unshift`,
`Array.reverse`, `Array.fill` and `Array.copyWithin` operation names. It requires
complete noncallable Object primitive wrapper roots through the existing shared
`appendJavaScriptObjectCoercion` API. Unsupported operations or missing/invalid
roots return `undefined`.

The compiler first obtains the complete legacy mutation function, allocates
five registers beyond its register count, and prepends shared ToObject. Its held
Object register replaces original register-zero reads only in explicit opcode
register fields. Every original branch/jump coordinate shifts by the prefix
length. Argument indices and numeric constants retain their meanings. The
legacy require-object guard remains and checks the held Object. No instruction
writes register zero. New unrecognized opcodes and future writes to zero refuse
composition rather than guessing. Existing compiled functions remain unchanged.

The original strict Set/Delete checks, saved length, argument conversion order,
safe append limits, sparse Has/Get behavior, overlap direction and resource
continuations are retained by composition. Receiver-returning mutations return
the held Object. All represented behavior uses existing common instructions;
there is no native Array method fallback.

Deferred controls compare each composed instruction against its legacy input,
undoing only register/coordinate relocation, check every target, and retain
invalid-root coverage. Twenty-five independent expected-true originals cover
Number/Boolean/BigInt/Symbol wrappers, fresh identities and exact internal slots,
String immutable indices and length errors, nullish rejection before argument
conversion, empty conversions, actual boxed getter receivers and ordinary
Object identity/mutation. Original execution is explicitly bounded to Node
`>=22.15 <23 || >=24 <25`; all other versions remain uncovered.

Integration belongs to the parent batch: connect this API into intrinsic catalog
authoring with the actual active wrapper roots, preserve the existing public
legacy API, translate the new authoring module through the translator, and
include any required public exports/source inventory. The new fixture module
must enter the original-source/native replay coverage after draft completion.
These authored fixtures exercise wider source features; admission failures must
lead to implementation or explicit gap reporting, not weaker fixtures.

No tests, behavior imports, generation, validation gates, builds or Rust
compilation were run for this draft. No source register/ledger was used or
modified. Runtime allocation suspension, shared mutation execution and native
parity have not been observed for the composition. No completion claim follows
from the deferred controls or expected-true declarations.
