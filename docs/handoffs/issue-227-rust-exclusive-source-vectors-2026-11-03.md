# Closed Vec pop, clear and truncate source packets

Unexecuted JavaScript source draft only. No tests, imports/behavior execution,
source generation, validation, compilation, native parity or ledger work ran.
The existing explicit exclusive source model covers Rust >=1.90.0 <1.100.0;
deferred edition-2024 originals select Node >=22.15 <23 or >=24 <25. Native
ParseOptions/frontend translation and subsequent execution remain deferred.

Concrete own-code uses include Vec pop before chunk formatting in
`rust/src/translation/emit_rust.rs:386`, and many own-source worklist pops.
This batch does not claim the surrounding iterator/control-flow bodies are
fully admitted. Unbounded loops, iterator mutations/callbacks and generic
FnMut residual traversal remain separate source families.

`rust-exclusive-source-vector-rules.js` admits only actual direct standard
owned Vec receiver origins and selected `pop`, `clear`, `truncate` arities.
The real source state inventory already excludes custom Drop, pointer/raw
observations and stored references. The new source helper additionally checks
the actual complete owned/Copy element origin. It does not grant source Clone
to a nominal record merely because the target value representation uses
internal cloning. Capacity/reserve observations remain unsupported, including
the native fact that clear/truncate/pop can retain allocation capacity.

The new `rust-exclusive-source-vectors.js` exposes:

- `readRustExclusiveSourceVectorMethod(parser,origin,method,path)` returns
  actual selected method/parameter metadata for the existing receiver frame.
- `createRustExclusiveSourceVectorPacket(parser,place,selected,args,path,span)`
  registers a real generated helper body and its actual complete source
  interface; it returns `{callee,parameters,result}` for the existing packet
  effect writer. The helper returns `(original result,updated Vec)`.
- `finishRustExclusiveSourceVectorHelpers(parser,program)` appends those real
  helpers after original body/state lowering. The collection agent owns the
  narrow parser import/finalization hook during its concurrent Eq integration.

Pop uses existing checked array reduce, tuples, array spread and the actual
standard Option constructor inventory. Its accumulator holds the latest
element as an Option and the preceding Vec prefix. Each next element moves the
previous held value into that prefix; the final packet publishes the actual
last element or None and the remaining prefix. No numerical last-index
calculation, source Clone call or host/native pop fallback is introduced.

Truncate uses an exact usize remaining counter with checked subtraction only
while positive. It folds retained elements into the prefix and discards later
elements. Counts beyond the length, including usize::MAX, leave the original
sequence of values. This deliberately avoids the existing generic slice's
floating-point argument interface. Clear publishes unit and an empty Vec.
These are immutable value algorithms; they can allocate more intermediate
prefixes than a native in-place Vec operation. Runtime resource/performance
equivalence is not asserted by this unexecuted source draft.

Receiver capture retains the actual exclusive lexical slot and field layout
before arguments. The packet uses those real layouts to update every ancestor
from the latest root, preserving sibling changes after earlier argument effects.
Truncate only infers a genuine unsuffixed integer literal in its actual usize
position; an independently typed u8/i64 value is not silently recast. Its
operand remains exact fixed-width usize throughout the body. Actual element,
packet and helper types are qualified to their source declaration paths so a
helper emitted at root cannot accidentally resolve a module-local record to a
different declaration.

`rust-exclusive-source-vectors.mjs` contains 12 positive literal originals and
eight independently compiler-classified source refusals. Controls include
empty/successive pop, exact truncate and usize::MAX, owned Unicode Strings,
a record without source Clone, Scope-shaped field pop, sibling writes, clear
then push, parent reborrow and a foreign-root bound producer. Refusals retain
shared/immutable roots, arity/domain errors, stored references/custom Drop,
capacity observations and conservative two-phase borrowing boundaries.

Deferred tests cover policy facts, complete Rust source parse/check to emitted
JavaScript output, absence of floating slice/index casts in generated helpers,
concrete source text/binary transport, typed Links transport and every emitter.
They declare future controls and are not test observations or support evidence.
