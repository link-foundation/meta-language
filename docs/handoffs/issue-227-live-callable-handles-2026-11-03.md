# Live callable handle and source driver draft

This continues [bound construction](issue-227-bound-construction-2026-11-03.md)
under the sole architecture in [vision](../vision.md). Source edits, textual
review and primary specification reading only. No test, represented program,
generator, formatter, validation gate, compilation or CI feedback ran or was read.
The complete requirements code draft is still unfinished; the full active goal
and every broader obligation remain unchanged.

## Actual source paths drafted

`javascript-live-handles.js` replaces synthetic copied callable metadata for the
source-function embedding with stable live Node Proxy handles. Ordinary records,
Arrays and callable entities share the same heap-backed property adapter. Public
descriptors/prototypes use public handles consistently, retaining cycles, aliases
and exact semantic object identity when encoded again. The internal authorized
native-operation decoder keeps its actual protected native references; public
source handles do not expose original declaration bodies as callbacks.

Constructibility comes from actual target constructor authority, source declaration
facts or the actual supplied Function prototype identity. Nonconstructor method
carriers and constructible bound carriers have no invented own prototype or
private own name/length properties. The latter use protected native bind only to
allocate an inert Node carrier, never as represented bind semantics. Actual source
Call and Construct remain the preceding portable algorithms. Function.prototype
also has its protected nonconstructor empty Call body in the shared catalog.

The adapter reads actual ordinary Get/Set plans, own keys/descriptors and shared
Define/Delete/SetPrototype/PreventExtensions operations. It retains partial
descriptor field presence and exact primitive values. Imported keys, values,
receivers and descriptor values are encoded before selecting the current heap,
avoiding stale snapshots when encoding adds actual global identities. Pure data
reads/writes complete directly; getters, setters and Array length writes enter
real shared VM instructions. Accessor execution never invokes an original body.

Carrier synchronization installs irreversible own descriptors and the selected
prototype before reporting them, mirrors every own property before publishing
nonextensibility, and preserves Array index-before-readonly-length order. It does
not eagerly decode configurable property values on an extensible carrier.
Conflicting irreversible snapshots refuse rather than silently changing meaning.
Host Proxy invariants follow the
[primary contract](https://tc39.es/ecma262/2025/multipage/ordinary-and-exotic-objects-behaviours.html#sec-proxy-object-internal-methods-and-internal-slots).

`javascript-live-handle-call-source.js` drafts four private boundary coordinators:
Call, Construct, Get and Set. Call/Construct reuse the complete existing frame
argument-List compiler. Construct checks actual target/newTarget before collecting
arguments and uses the ordinary Construct instruction path. Get/Set reuse their
existing opcodes; a resumed setter therefore returns Boolean success rather than
its arbitrary source return value, and Array length uses its actual repeated
numeric conversion and truncation protocol. Fresh coordinator owners never replace
source declarations; their inert original carrier bodies are never called.

`javascript-source-driver.js` contains the existing Node source driver moved from
`node-source-modules.js`; the public entry point reexports that exact implementation.
The live embedding uses the same allocator, intrinsic/qualified port, constructor
authority, Symbol metadata and ordinary machine resumption loop. `liveHandleSteps`
selects an explicit bounded host work slice. Public `beginHandleOperation` and
`runHandleOperation` expose data transitions. A host invocation that suspends
throws a private adapter error retaining the actual transition, available through
`readHandleSuspension` and `resumeHandleSuspension`. Retry rejects stale heaps,
consumes the old private token and retains another exact boundary if still needed.
Adapter errors never become represented source throws. Genuine program throws
decode to their actual represented/opaque identity.

Untranslated native callback stacks cannot be recreated by starting a second VM.
The current guard refuses such source reentry before its body, preserving the
previous explicit boundary. Full portable callback/iterator/async ports remain
required. The guard is not a completion claim or a final supported-language limit.

## Independent authored controls, all unexecuted

Eleven literal originals compare ordinary native observations against actual
portable execution plus public handles. They cover bound metadata/calls,
nonconstructor arrows, bound/base-class construction, live getters/setters,
partial descriptors/deletion/order, readonly self links/freezing, nested aliases
and sparse Array shape, exact throw identity, primitive SameValue bits and the
actual callable Function prototype. Node assertion ranges are explicit.

Additional controls cover exact common driver identity, genuine carrier
constructibility, later VM metadata/prototype writes visible through an existing
handle, imported object/Symbol identities, resource retry without replay, stale
retry refusal, resumed setter success, native callback reentry refusal, source-free
immutable bound-call snapshots and actual Array length conversion. The snapshot
control executes only retained program/heap/frame data with no native handle cache
or original body. None ran; program/control counts are not requirement counts.

The automatic source recipe includes the new common coordinator library, both
Node adapter files, literal originals and their consumer. No generated Rust,
schema/model publication or native transport was edited by hand or executed.

## Remaining actual code work

1. Complete reflective Array DefineOwnProperty through the existing resumable
   Array descriptor conversion protocol. The current direct descriptor adapter
   handles completed primitive descriptors; object-valued length conversions,
   RangeError versus ordinary incompatibility and resource suspension need their
   real shared continuation. Do not turn an incompatible Reflect definition into
   an adapter failure, nor suppress a conversion throw.
2. Complete metadata authority for every native callable reachable through public
   prototype reflection, including the actual Function constructor, intrinsic
   toString and restricted-property accessors. Use protected identity owners and
   supplied versions; host callable shape or a native-code-looking string proves
   neither constructibility nor portable execution. Complete source text and
   native brand/slot reflection through actual source/port witnesses.
3. Draft full native callback, iterator and asynchronous continuation ports so
   external stacks can resume actual source effects instead of nested VM replay.
   Keep current refusal identity and add exact once-only side-effect, throw,
   cancellation and resource controls as each real protocol is drafted.
4. Continue derived/super/uninitialized-this, Proxy/private-field and mutable
   environment/effect protocols. Bound derived/Proxy targets retain their actual
   pending specialization requests until those target algorithms exist.
5. Continue Rust borrowed callback/iterator interfaces, storage and loans,
   partial moves, generics/traits/macros/modules and full own-project CST,
   emission and both translation directions. Keep all broader shared concept,
   recursive grammar, ontology/taxonomy, RDF/OWL, dynamic reconstruction, inference,
   formal and delivery requirements from vision.
6. After every obligation has its complete actual code/control draft, audit live
   sources before ledger access, publish all automatically translated assets,
   execute authorized JavaScript and CI-only gates, fix failures and record
   observations at their executed commit. Never compile Rust locally.
