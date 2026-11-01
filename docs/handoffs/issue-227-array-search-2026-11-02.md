# Issue #227 Array search source draft — November 2, 2026

Architecture remains solely in [vision.md](../vision.md). This file records
source edits and remaining integration, not execution evidence or completion.
The source register and ledger remain untouched because their required online
audit is deferred under the current no-validation drafting instruction.

## Drafted source

- `javascript-array-index-rules.js` shares ToLength and ordinary HasProperty with
  callback traversal and adds relative search index selection. The previous
  callback presence entry point remains a wrapper with all previous guards.
- `javascript-array-search.js` drafts includes/indexOf/lastIndexOf/at. Length,
  index conversion, getters and current effects remain explicit continuations.
  Missing versus explicit undefined fromIndex is retained. includes visits holes
  with SameValueZero; indexed searches skip missing properties and use strict
  equality. at uses relative indexing and performs conversion even on empty
  input; empty searches skip fromIndex conversion. Infinity never becomes a
  property coordinate. Negative indices truncate toward zero.
- The real standard-method registry captures/verifies four callable identities.
  The directory compiler, instruction register accounting, machine instruction
  admission, pending validation, child calls, step/resume and final completion
  adapters are drafted together. A common selected-call frame validator retains
  the prior Array callback identity/receiver/argument checks for both families.
- Automatic native source-library generation registrations and deferred consumer
  inputs are added. No native semantic implementation is handwritten, and no
  generated Rust/codec/model/Links output is manually edited.

## Authored controls awaiting execution

Twelve original-source cases cover NaN/zero equality, holes, identity, forward
and reverse selection, omitted/undefined arguments, fractional/infinite/negative
indices, lazy empty search conversion, mandatory empty-at conversion and length
mutation by index conversion. Six continuation controls cover those differences,
inherited getters, exact held conversion receivers, current effects on throws
and retry of a selected unstarted call. These are declarations, not observations.

The normative source is ECMAScript 2026 Array.prototype.at/includes/indexOf/
lastIndexOf:
https://tc39.es/ecma262/2026/multipage/indexed-collections.html#sec-array.prototype.at
and the adjacent method sections. Original runtime comparisons must use the
project's selected active runtime/dependency ranges and report uncovered ranges;
no engine-specific message mapping or broader runtime support is claimed here.

## Remaining work for this batch

After the full requirements code draft permits generation, regenerate the source
libraries, typed model codecs, native modules/publication and Links fixture data
through the existing authoring pipeline. Execute every retained original-source
assertion and JS/native replay at the exact recorded revision in CI. Preserve
classification of malformed state, unsupported host/boxing protocols and resource
outcomes. Do not claim completion from these source declarations.

Primitive receiver boxing, opaque HasProperty/Proxy behavior, full diagnostics,
Array species/result construction for map/filter/flatMap, general constructor
execution, IteratorClose/for-of and lexical cells/TDZ remain explicit obligations.
All earlier requirements and controls remain binding; this file is not a new
architecture specification or a replacement ledger.


## Species selection foundation drafted in the same continuation

`javascript-array-species.js` now drafts ArraySpeciesCreate selection through the
actual constructor Get, constructability/realm classification, foreign intrinsic
normalization and species Get. It preserves null-versus-undefined distinctions
and permits a nonconstructor Object to provide a genuine species constructor.
Its outputs are explicit ArrayCreate or Construct requests, not completed result
Objects. Unknown function authority produces `arraySpeciesNeedAuthority` with the
held candidate/phase so adding registry facts never repeats a completed getter.
The directory binds stable current-realm roots, globally identified Symbols,
ordered classifications and one intrinsic Array constructor identity per realm.

Node metadata capture seeds protected builtin classifications and checked source
declaration classifications. The latter are copied privately before publishing
mutable module records; returned facts are copies. An authenticated ordinary
source unit extends those private facts after successful source-unit publication.
An own prototype property or function name is not constructability proof. The
actual protected Array species getter is separately admitted to the portable
standard directory and compiled as `copy this; return`, preserving its real
callable identity without invoking an original host body as translated semantics.

Eleven deferred controls cover non-Array bypass, actual getter/receiver requests,
foreign intrinsic normalization, nonconstructor species containers, missing
facts/dynamic extension, null distinctions, nonconstructor rejection, current
abrupt effects, ArrayCreate-versus-custom length boundaries, the species getter
VM code and private/dynamic source facts. They are not observations. The shared
source library and test consumer are registered for later automatic generation.

Normative source: ECMAScript 2026 ArraySpeciesCreate and Array species getter,
https://tc39.es/ecma262/2026/multipage/ordinary-and-exotic-objects-behaviours.html#sec-arrayspeciescreate
and https://tc39.es/ecma262/2026/multipage/indexed-collections.html#sec-get-array-%symbol.species%.
Engine-specific diagnostics, Proxy classification and bound/proxy constructor
realm traversal still need their explicit authorities and version profiles.

### Concrete next map/filter composition work

1. Carry protected per-realm species directories through portable program/model
   composition and native codecs. Bind each operation's directory to its actual
   intrinsic owner realm and preserve dynamically added source-unit facts.
2. Add shared map/filter frames in order: ToObject, captured LengthOfArrayLike,
   callback IsCallable, species selection, actual target creation, HasProperty,
   Get, callback and CreateDataPropertyOrThrow. Preserve callback arguments,
   thisArg, getter effects and every abrupt completion.
3. Drive ArrayCreate requests through the existing external global identity
   allocator and sparse Array kernel. Enforce its 32-bit length RangeError there,
   without rejecting a custom constructor's larger valid requested length.
4. Drive Construct requests through actual portable constructor metadata,
   prototype acquisition and constructor return/receiver rules. Extend the VM's
   class/new instruction proof to these explicitly owned constructor frames;
   implement ordinary/derived/proxy construction rather than host-body fallback.
5. Retain completed map values or filter predicates across allocation/property
   resource limits. Map keeps original sparse result positions; filter advances
   its output index only after successful selected property creation. Custom
   species results may be ordinary Objects and need not have an Array brand.
6. Author source and raw-state controls for species getter ordering, thrown or
   nonconstructable species, foreign realm Arrays, custom result objects, holes,
   callback mutation, length limits and resume-without-replay. Regenerate the
   model/native/Links products automatically once the full drafting phase ends,
   then run all retained assertion cells at one recorded commit in CI.

This selection foundation does not claim completed map/filter execution. Earlier
iterator/IteratorClose, lexical cells/TDZ, source import/decoder, full grammar and
architecture obligations remain binding and keep their prior detailed plans.


## CopyDataProperties exclusions drafted

The common object-copy state now retains excluded actual String/Symbol keys.
Exclusion is tested before own descriptor/Get work and therefore never invokes an
excluded getter. Duplicated exclusions remain legal and Symbols compare by their
global identities. Optional source/instruction exclusion operands preserve the
old no-exclusion form; source lowering converts and snapshots explicit operands
once, with register/operand bounds. Whole-machine pending validation binds every
excluded value to the selected instruction's actual register.

Three continuation/VM controls and one original/compiler/native replay input are
authored. The raw replay input compares data-rest behavior for an explicit supplied
source IR; it does not claim that the CST object-rest binding reader is drafted.
General rest-pattern binding, RequireObjectCoercible/diagnostics, prior field read
order and lexical environment construction remain explicit next work. Automatic
codec/native/Links regeneration and all behavioral assertions remain deferred.


## Ordinary lexical declaration groups drafted

The CST reader now drafts uninitialized local let and multiple identifier
const/let declarators in statements and for initializers. One shared reader
preserves each actual declaration binding, initializer expression and inferred
anonymous arrow name. Initializers lower in source order, so later initializers
see earlier completed declarations. A dedicated declaration-group statement
shares the existing binding lowerer without pretending that the group is a new
syntactic block. Original single-declarator source plan forms remain unchanged.

Switch's unbraced declaration guard also recognizes the new group form, retaining
the requirement for a real shared lexical TDZ environment. Var hoisting,
read-before-initialization, mutable per-iteration captured cells and general
binding patterns remain separate protocols; this change does not silently assign
undefined to those cases. Eight original-source controls and one explicit switch
boundary refusal are authored. Source/native/model generation and all execution
remain deferred. No assertions, existing inputs or gates were removed.


## Map/filter composition source continuation

Added `javascript-array-transform.js`: captured length/getter/ToNumber/ToLength,
callback IsCallable, species selection, actual sparse Array allocation or explicit
custom construction request, live HasProperty/Get, callback and indexed own-data
creation. Map preserves input positions/holes; filter retains original elements
and advances its compact output coordinate only after a successful write. The
shared indexed writer distinguishes ordinary descriptor rejection, Array exotic
index restrictions, malformed/opaque protocols and shared storage exhaustion.
Completed callbacks are held across write-resource suspension.

The protected standard directory now includes map/filter with source-derived
species directories. Standard constructor facts are captured before mutable
publication, from exact protected constructor/method owners. The call instruction
carries the actual realm directory, checked against the global context. VM
program admission, register accounting, per-frame pending invariants, exact
child calls, step/return, result allocation and construction resumption APIs
are drafted together. ArrayCreate length/storage rejection precedes requesting
a new global identity. The allocation request retains its actual prototype.

Custom Construct is exposed as `machineConstructArraySpecies`; missing metadata
is exposed as `machineNeedConstructorAuthority`. The ordinary runner preserves
these requests for actual portable constructor/authority implementations. Generic
source constructor execution, dynamic authority response transport, bound/Proxy
realm/prototype protocols, primitive boxing and version-qualified errors remain
explicit work. No selected constructor is replaced by an original-host-body
fallback. The earlier detailed map/filter checklist records its historical
planning point; this section records newly drafted layers, not verification.

Twelve original-source controls and six continuation controls are authored for
sparse map/filter, callback operands/thisArg, mutations, returned closures, nested
transformations, empty input, custom ordinary result Objects and held writes.
A seventh control checks the explicit resource profile: the Node standard catalog
selects 64 owners; the absent-profile 16-owner boundary remains intact, and an
unregistered profile refuses. No observations or verified support are claimed.

Native/model/codec/Links generation remains deferred with all tests, compilation
and validation. The source register and ledger were not used because their
required online audit is deferred under the active drafting instruction.


### Dynamic constructor authority update drafted

`extendJavaScriptCallArrayAuthority` now drafts an explicit program/metadata
extension at an actual suspended transform species selection. It preserves realm
roots, every prior constructor classification, the executable instruction and
all machine values, while adding newly authenticated classifications. The caller
continues with the returned program and identical held machine. A new deferred
VM control changes the original species property while authority is absent,
then adds the missing fact and checks that selection retains its earlier
constructor rather than rereading the property. Rebinding roots or changing an
established classification refuses with the original program retained.

This is the explicit core response API. Node automatic response handling, trace
transport of program/authority updates and generic portable Construct execution
remain further drafting work. No tests or generation ran.


### Actual source species constructor bridge drafted

The earlier custom Construct boundary now enters the common machine for actual
checked strict ordinary functions and base classes. The new
`javascript-species-construction.js` retains prototype Get/getter completion,
constructor realm fallback, external identity allocation, actual body invocation
and Object-versus-primitive return selection. Intrinsic Array construction uses
the shared Array allocator. Missing realm roots suspend after the completed Get;
authority extension preserves all previous roots. Receiver prototype mutation
during a constructor remains legal. Nested pending validation binds the exact
selected constructor, allocated receiver and length, while initial class entry
keeps its actual receiver proof. External construction completion cannot replace
a supported live source body.

Five original-source fixtures cover custom map/filter class results, the initial
length argument, ordinary function primitive returns, replacement Object returns
and primitive prototype fallback. Five kernel controls cover allocation/body
operands, Object/abrupt results, realm fallback, getter non-repetition and receiver
prototype mutation. Symbol namespace linkage supplies actual Symbol.species.
These controls are authored only; nothing executed. Automatic metadata response
in the Node driver, authenticated update publication and transport remain next.
Derived constructors, bound/Proxy protocols and standard collection/Error
constructor specializations remain explicit requests. Generated Rust/Links and
codec artifacts remain deferred with every test and compilation.


### Node constructor authority response drafted

The collection linker now registers its protected species metadata provider
exactly once before source publication. The embedding re-derives an actual
`machineNeedConstructorAuthority`, checks the selected constructor, requires the
latest adopted heap, and publishes only the shared monotonic program update.
The Node driver refreshes both module and program after that response. Unchanged
authority returns the original request without retry loops or host constructor
execution. Public TypeScript declarations include both constructor boundaries.
Two deferred controls cover authenticated source-unit additions, real portable
constructor execution, unknown constructor suspension, replacement authority
refusal and forged selected-constructor refusal. No control executed.

Portable trace/codec transport of authority updates, exotic constructor protocols
and native artifact regeneration remain further work; these are not claimed as
observations or completed requirements.


### Shared fixed object binding/rest source draft

Parameters and const/let declarators now use one object binding reader and one
ordered lowerer. Fixed keys, aliases and defaults precede fresh rest copying;
original source keys become shared CopyDataProperties exclusions. The source
initializer is retained once. Required object coercibility precedes all field
work, including empty patterns and rest-only patterns. The scalar operation
returns unused/undefined, rejects nullish inputs and never calls a getter.
Unbraced switch object bindings preserve the existing TDZ refusal.

Eight new originals cover rest parameters, aliases/defaults, independent rest
Objects, Array own indices, lexical const/let patterns, empty lexical patterns and
default effects preceding rest copy. The previously retained empty-arrow-pattern
source moved from refusal to the original-source oracle inventory with its exact
source/name retained. This expectation changes because the missing coercibility
operation is now drafted, consistent with ECMAScript 2026 RequireObjectCoercible
and RestBindingInitialization (linked in the vision). Its input and successful
result assertions are now explicit. A scalar control covers null/undefined, all
other value categories and wrong arity. No assertion executed.

Computed/nested object patterns, array/rest iterators, general TDZ cells and exact
engine nullish-pattern diagnostics remain further work. Generated products and
source-register reconciliation remain deferred, as do all tests and compilation.


### Computed object binding key draft

The shared reader/lowerer now admits actual computed binding keys with explicit
aliases. Each key expression completes the existing ToPropertyKey protocol once;
an immutable internal binding retains the converted key for both Get and rest
exclusion. Four further originals cover earlier-parameter keys, later defaults
mutating the key binding, genuine well-known Symbol identities, and a counted
key expression. These are authored inputs only. Nested patterns, array iterators
and exact engine diagnostics still require further drafting; no tests ran.


### Constructor metadata trace/native replay draft

The source trace helper drafts explicit `extendConstructorAuthority` events with
the exact selected constructor, species directory, resulting portable program
and continuation. Native replay calls the translated common authority extension,
compares its error, complete unchanged machine and resulting program, and then
continues using that program. Existing budget accounting includes these events.
The generator root list includes species directories and authority updates;
generated codecs themselves remain untouched until automatic regeneration. A
deferred Node trace control expects one metadata addition and complete source
execution. Neither that control nor Rust replay was executed or compiled.


### Recursive object binding source draft

Nested object patterns now recurse through the same reader and initialization
lowerer, retaining each parent field/default value in its synthetic actual
binding before nested coercibility and field work. Computed keys and rest
exclusions stay at their original level. Both reader and lowerer bound nesting
at 128. Outer default expressions are read before marking nested parameter
bindings initialized, retaining the existing parameter TDZ refusal. Four new
originals cover nested parameter defaults with inner/outer rest, lexical nested
bindings, computed nested keys with earlier defaults, and nested empty patterns.
These inputs and all generated/native products remain unexecuted. Array binding
iterators, mutable lexical cells and exact nullish diagnostic compatibility still
need their further drafts. No ledger was used and no validation gate ran.


### Synchronous iterator protocol draft

`javascript-iterator-protocol.js` now drafts shared acquisition, IteratorStepValue
and IteratorClose continuations with actual heap Get/getter/call effects. The
record caches next once; callability is checked at invocation. Done precedes
value, exhaustion skips value, and ECMAScript 2026 next/done/value abrupt paths
mark the record done. Primitive acquisition emits an explicit original-receiver
Get request. No fabricated boxing, iterator method or host body executes.

Cleanup preserves original throw precedence across return getter/call failures
and non-Object results, while retaining normal/return/break/continue completion
values and actual numeric source control targets. Return is still acquired when
this exact close operation is explicitly requested for a done record; consumer
algorithms decide whether closing is required. State validation binds actual
selected callback receivers and cached next method.

Six deferred controls cover acquisition/callability timing, cached next and done
truthiness, skipped value getters, getter throws/current effects, cleanup throw
precedence, return/break preservation, primitive cleanup rejection and primitive
receiver requests. Automatic native policy/library generation inputs are
registered; no generation or control execution ran. Array/String iterator
builtin adapters, source/VM iterator regions and abrupt-unwinding consumers for
for-of, spread and array patterns remain the next required connections. Older
engine done-slot behavior needs explicit version-qualified controls rather than
an unqualified assertion. These are draft semantics, not recorded observations.


### Live Array iterator slot and next draft

`javascript-array-iteration.js` drafts genuine common-heap Array iterator
allocation and next continuations. The shared iterator Object gains a distinct
Array family and executing slot; Map/Set keep their existing 65,536 insertion
bound and reject Array receivers. Array cursors use full safe ToLength indices.
Live length Get/ToNumber precedes each index; holes yield undefined; keys omit
element Get. Suspended getter/conversion work rejects reentry. Abrupt completion
and normal exhaustion permanently release the source. Scalar resource refusal
retains its completed primitive without rereading length/getter effects.

A shared result-allocation entry point consumes completed Array components,
reusing the existing entry Array and iterator-result Object allocator. Six
deferred controls cover holes/live growth/permanent exhaustion, keys and family
brand, reentry/throw closure, real pair/result allocations, primitive boxing
boundaries and length conversion effects. Automatic native source/control inputs
are registered. Nothing executed or compiled; generated products remain stale
until the later regeneration phase. Actual Array builtin owner registration and
VM next continuations, String iteration, for-of/array-spread/pattern consumer
unwinding and version-qualified diagnostic controls still need drafting.


### Actual Array iterator builtin/VM source integration draft

The existing iterator instruction family now admits the distinct Array brand.
Creation preflights the actual receiver before requesting identity; primitives
remain `machineNeedArrayIteratorBoxing`. Array next composes its resumable
getter/conversion state with the existing pair/result allocation pending state.
Frame admission binds the actual opcode, receiver, conversion Symbol and child
callee/receiver/arguments. Independent result prototypes validate before any
cursor effects. Return/step and native generator inputs use the common path.

The Node linker records actual keys/values/entries owners, verifies the true
Symbol.iterator alias, captures only native empty-probe prototype witnesses,
and links actual Array Iterator next/shared-self owners to portable code. These
probes are future linkage code; none executed during drafting. Six original
source fixtures cover holes, live growth, permanent exhaustion, keys, genuine
entries and Symbol/self aliases. A deferred VM control throws from an actual
selected getter and checks that the original throw propagates after iterator
source/executing state release.

Source review also corrected two related invariants: machine throw unwinding
releases an abandoned Array next, and intrinsic TypeError requests release it
before replacing the pending state. Constructor frame validation now retains
receiver identity/kind rather than reasserting its initial prototype forever;
initial constructor entry proof remains. No checks executed, generated products
were not edited, and no CI feedback was reviewed. Primitive boxing, String
iterator builtins and structured iterator consumer unwinding for spread, array
patterns and for-of remain required further source connections.


### Shared Array spread accumulation kernel draft

`javascript-array-spread.js` composes the common synchronous iterator protocol
with shared indexed own-data creation. It holds each completed value across
property storage refusal and resumes writing before another next call. Cached
next, actual receiver/key identity, direct iteration throws and primitive Get
requests remain intact. It does not inject IteratorClose where the normative
ArrayAccumulation algorithm propagates IteratorStepValue failure directly.
Independent target/index validation precedes getter/call effects. Five authored
controls cover actual accumulation/index completion, cached next mutation, throw
without return, primitive/target boundaries and held-write resource retry. Source
library/native control inputs are registered. VM instructions and source literal
composition are the next connections; no generation or tests ran.


### Array literal spread source/VM composition draft

Spreading literals now retain ordered value/hole/spread parts in source plans.
The shared source compiler emits actual Array allocation, an internal Number
coordinate, own-data writes, explicit hole length writes and the common
`arraySpread` opcode. A protected intrinsic namespace supplies the true iterator
key independently of lexical Symbol shadowing. Static literals keep their
previous shape. The opcode/pending state validate actual registers, source realm,
target/input/key, starting coordinate and selected child operands. Step/return,
throw/resource paths and primitive Get suspension/resumption are drafted together.

Five originals cover sparse spread values, dynamic offsets/trailing holes,
multiple-spread evaluation order, nested iterator operands and lexical Symbol
shadowing. A source compiler control asserts shared opcode/counter composition
and explicit hole writes. Automatic native generation consumes the common source
model; no generated output was edited and no check ran. Primitive String
iteration authority/builtins, spread call arguments, array binding IteratorClose
and for-of control unwinding remain further required source connections.


### Exact String iterator slot/next draft

The common iterator slot representation now includes the distinct String family
with an actual semantic text slot and code-unit cursor. Non-String families
cannot acquire that slot. `javascript-string-iteration.js` drafts allocation
after completed ToString, code-point advancement with exact lone-surrogate
preservation, permanent exhaustion, and shared value/done result allocation.
Both text and compact sourceText representations use the same UTF16 accessors.
Map/Set direct/brand advancement explicitly refuses String/Array families.

Five deferred controls compare full exact outputs with the original String
iterator for both representations, cover surrogate pairs/lone units, empty and
permanent exhaustion, family refusal, genuine result allocation and allocation
operand/identity refusal. Automatic library/native-control generation inputs
are registered. No generation, tests, compilation or CI feedback review ran.
String builtin ToString/allocation continuations, protected owner linkage and
primitive iterator property authority are the next required connections before
source String spread/for-of can consume this foundation.


### Primitive iterator authority and String source integration draft

The common iterator protocol can now Get the actual iterator property from an
independently authenticated primitive realm prototype while retaining the
original primitive as getter/method receiver. The spread and VM resumers
re-derive the pending request. Node roots are copied and registered once before
publication; later root replacement and substituted receiver/key operands
refuse. The driver resumes this boundary without host getter/body fallback.
Trace/native replay records and applies the exact primitive prototype input
through the common translated VM resumer. Metadata events consume the shared
transition budget.

Protected String iterator/next owners now compile to portable instructions.
Creation performs shared coercibility and ToString, then allocates from the
held converted text using the iterator instruction's explicit input register.
Other iterator families retain their existing receiver operand convention.
String next advances the genuine slots and reuses common result allocation.
Five complete originals cover Unicode code points, lone-surrogate escaped source,
explicit iterator next, empty spread/hole behavior and nested/multiple spreads.
Three deferred controls cover original primitive getter receiver/selection,
prototype trace witnesses and authority operand/root refusal.

All are source drafts. Generation, behavioral controls, native compilation and
CI feedback review remain deferred. Exact nullish/String-method diagnostic text,
Array method primitive boxing, iterable call-argument spread and array-pattern/
for-of unwinding still require their further source work; no passing observation
or completion percentage is claimed.


### Actual source try/catch composition draft

The CST source reader now captures complete try/catch blocks, identifier catch
bindings with actual lexical identities and omitted binding clauses. Lowering
reuses common sourceTry/VM handlers. Protected body and handler are lowered
separately from the following statement continuation, preserving catch scope and
mutable catch values. Existing loop/control handler-depth unwinding applies.
Seven originals cover thrown identity/rethrow, optional and mutable catch
bindings, later statements outside protection, handler failure to the outer
catch, loop break unwinding and a real intrinsic TypeError name. No original or
portable body executed. Catch binding patterns and finally completion semantics
remain required further source connections; generated products were untouched.


### Recursive catch object binding draft

Catch heads now admit the same fixed/computed/recursive object binding reader
and initializer as parameters and lexical declarations. The caught value binds
to the actual synthetic pattern owner; coercibility/default/field/rest work runs
after the catch handler is removed. Initialization failures reach the enclosing
catch. Temporary parameter initialization sets enforce the existing later/self
TDZ refusal while reading pattern defaults. Five originals cover mutable alias
defaults/rest, nested rest levels, enclosing computed keys, nullish pattern
failure to an outer catch and UTF16 own String index/rest behavior. No code or
assertion executed. Catch array bindings and finally/control completion machinery
remain further drafting work; source/native generation and all gates are deferred.


### Shared finally/completion source and VM draft

Finally now uses ordered actual handler records and explicit deferred
return/throw/jump completions in the same source frame. Common unwinding enters
crossed finalizers, preserves primitive values/Object identities and current
heap effects, resumes normal finalizer exits and discards overridden completions
on abrupt finalizer exit. Catch handlers nested inside a finalizer retain the
deferred outer completion. Constructor return normalization follows finalizers.
Array next abandonment still releases its source while throwing across frames.
Completion values, source entries, destinations and handler depths validate.

Source finally lowering emits pushFinally, completion jumps and finishFinally.
Existing source loop/control transfers now retain their scope handler depth and
use common completion jumps when crossing handlers. The CST reader/lowerer
composes try/finally and try/catch/finally with the same machinery; following
statements remain outside the completed protected region. Eleven originals cover
return snapshots/overrides, original throw identity/current effects, throw
override, catch/finally order, loop exits, nested finalizers, internal catch,
finalizer break/continue overrides and constructor normalization. Three deferred
VM/source controls cover actual return override, finalizer-local catch and
malformed completion coordinates. Native generation inputs include the controls;
no generated files were edited and no test or compilation ran.

This provides completion machinery for the still-required iterator-region
connections in for-of and array binding/default evaluation. Those consumer
connections, general shared lexical environments and remaining language surfaces
continue to require source drafting. No ledger/evidence status was changed.


### Iterator consumer region ownership foundation draft

`javascript-iterator-regions.js` drafts ordered source slot ownership for actual
internal iterator records: bounded opening, cached identity-preserving updates,
monotonic exhaustion and explicit release. Three deferred controls cover ordering
and duplicate opening, identity/exhaustion invariants, nesting and malformed
coordinates. Library/native-control generation inputs are registered. Array
spread source lowering additionally holds its operand and iterator key in
private registers before callbacks; its existing compiler control now asserts
the actual operand copy without removing prior assertions.

The next source/VM connection should use one iteratorProtocol instruction with
acquire/step/close operations and a per-frame optional region directory. Acquire
belongs outside a generated sourceFinally; step binds actual value/done registers
in the source loop; close uses the nearest deferred completion for throw
precedence and skips already-done records. Continue targeting this loop remains
inside its region; exits crossing the region use the common completion unwinder.
Getter/method throws during step must update done before the generated finalizer
checks the record. Throws from selected close getters/calls must resume the
shared close protocol before ordinary handler unwinding, preserving an original
throw while letting new cleanup failures override returns/control jumps. Close
completion/error paths release the actual region. Primitive prototype resumption
and trace/native replay should extend the existing common API rather than
introduce host iterator execution. Then connect for-of source heads/body/bindings
and Array binding/default/IteratorClose consumers through the same primitives.
No tests, generation or compilation ran; these connections remain drafting work.


### Source for-of region/VM integration drafted

The region ownership foundation is now connected through the iteratorProtocol
instruction's acquire/step/close operations and actual optional per-frame region
directories. Pending/frame validation retains source slot, realm, distinct
private result coordinates, operand/key, cached record and deferred cleanup
completion. Primitive prototype response and source/native replay reuse the
existing checked API. Return/step compose the common iterator continuation with
region opening/update/release; step failures mark done before finalization.
Selected callback throws are resumed through the protocol before ordinary
unwinding. Close preserves an original body throw across return getter/call
failures; intrinsic call failures follow the same precedence. Source regions
release on normal, exhausted and failed closing paths.

Source for-of admits const/let identifiers and recursive object binding heads.
The compiler holds input/key before callbacks, opens outside a generated
sourceFinally, steps each actual yielded value/done pair, and closes only an
unfinished record. Own continues target the next step without closing; exits
crossing a region use common deferred completion unwinding. Object pattern
default failures occur inside the protected region and therefore close it.

Fourteen complete originals cover sparse Array and String iteration, recursive
head binding/defaults/rest, mutable local heads, nested and labelled loop exits,
user iterator break/continue/return, body-vs-cleanup throw precedence, cleanup
throw replacing break, step failure without return, pattern failure with close,
and natural exhaustion without return. Two deferred compiler/control tests cover
actual acquire/step/close/finalizer composition, held operands and nearest
deferred completion selection. Automatic native generation inputs are updated.
No source program, original oracle, test, validation, generation or compilation
executed. Array binding heads/parameters, iterable call-argument spread, full
shared lexical environments, version-qualified diagnostics and the remaining
recorded language/library surfaces still require further source drafts.


### Recursive Array binding source/VM draft

Array patterns now use actual iterator regions across parameter/default, lexical
const/let, catch and for-of heads. Identifier and recursively mixed Object/Array
targets retain actual binding identities, undefined-only defaults and named
arrow defaults. Whole-parameter Array defaults participate in Function length.
Elisions retain a checked step choice that reads done without reading value.
Rest uses the opened cached record and ordinary indexed own-data writes into a
fresh realm Array, with no iterator reacquisition or species selection. Nested
rest patterns initialize from that actual Array.

A shared binding output scope preallocates private registers, initializes actual
identities in order, finishes iterator closing, then compiles the following body
with original constant/mutable flags. It avoids extending the iterator lifetime
through following statements or the function body. Internal initialization writes
constant outputs without allowing ordinary constant assignment. Static markers
refuse uninitialized reads/writes and missing initialization; conditional/catch
paths retain only definite initialization. General runtime TDZ/shared-cell
semantics remain separate work. The Object initializer shares this internal mode
when recursively nested inside Array patterns.

Seventeen originals cover default/elision/rest semantics, Arrow length, null vs
undefined, String code points, dense sparse rest results, nested/rest patterns,
catch/for-of/Map-entry heads, captured field values, close before later code,
empty patterns without next, nested close ordering, close before function body
and original default throw precedence over cleanup failure. Four deferred scope/
compiler controls cover initialization/refusal and branch paths, close-before-body
and cached-record rest collection; an iterator control distinguishes elision
from value getter acquisition. Nothing executed.

Source review showed mutable binding reads already produce a snapshot before
the extra explicit protocol operand copy. Two existing authored operand checks
now trace the copy chain to the same original parameter, retaining their input
and provenance assertion without requiring one physical copy instruction. This
is source inspection, not passing execution evidence. Automatic generation is
still deferred. Remaining source work includes formal rest parameters, iterable
call-argument spread, general lexical environments/cells, version-qualified
diagnostics and the other recorded library/language surfaces.


### Formal rest parameter source/VM draft

`javascript-argument-rest.js` drafts fresh Array creation and indexed copying
from the actual immutable frame Call argument list. Named parameter binding
writes do not replace that list. Allocation and storage pauses retain formal
offset/count, target/prototype and next copy coordinate. The argumentRest
instruction/pending state bind the actual formal register, source realm, original
argument range and prototype; no selected callback is hidden. Step, identity
allocation, result publication and argument-context validation are drafted as
one batch.

The source reader/lowerer now admits one final rest identifier or recursive
Object/Array target for functions, methods and arrows, without initializer or
trailing comma. Function length stops at rest, and nested targets initialize
from the genuine collected Array. Ten originals cover all supplied arguments,
Arrow length, omitted/undefined distinction, fresh Array identity, constructor/
method rest, recursive targets, preceding defaults, returned closures and
mutable rest binding assignment. Four deferred controls cover exact copied
values/Object identities, empty/rest range behavior, copy storage suspension and
formal-register source emission. Generation inputs are registered. Nothing ran.

Iterable argument spread/constructor spread, full lexical environments/cells,
version-qualified diagnostics and remaining standard/library/language surfaces
remain further source drafting work. Generated artifacts and all verification
continue to be deferred; no ledger/evidence status or closing directive changed.

### Generic iterable Map/Set constructor draft

The standard callable directory now emits ordinary Get/Call/iterator-region/
finally bytecode for both constructors. This replaces its restrictive certified
same-kind collection copy path; the old explicit copy kernel/opcode remains
available for existing controls. Nullish input exits before adder access. A
non-nullish constructor reads and checks its actual adder once before acquiring
the input iterator. Map requires an Object entry and reads properties zero then
one in order. Normal exhaustion releases the done region without return;
advancement failure retains the protocol done marker. Entry/adder throws cross
the finalizer and use existing IteratorClose original-throw precedence.

Ten original sources cover sparse Array/Unicode String Set inputs, duplicate Map
keys, entry getters, custom iteration, primitive-entry cleanup, throwing entry
getter versus throwing return, step failure without close, missing entry values,
and nullish inputs. Three authored controls cover both instruction graphs and
primitive guard boundaries. Their generation input is registered. No tests,
behavioral imports, generation, compilation or validation ran. The latest live
ECMA page labels itself 2027; its constructor algorithms were read as supporting
design context, not as proof of Node version parity. Exact engine diagnostic
wording, mutable lexical cells, argument/constructor spread and the other
previously recorded source surfaces still need their own drafts.

### Literal prototype setter source/VM draft

The source reader distinguishes colon-form decoded `__proto__` from computed
keys/shorthand, rejects duplicate literal setters, and preserves value evaluation
order without anonymous name inference. A dedicated source property/instruction
uses the existing ordinary prototype kernel for Object/null values and ignores
other primitives without creating a field. Opaque prototype traversal remains
an explicit unsupported protocol, never native Object.setPrototypeOf fallback.

The former prototype-setter refusal is retained byte-for-byte as a positive
original with primitive input and an empty own-property result; the assertion
changes because source and VM installation are now drafted. Five more originals
cover inherited reads, null prototype, computed own data coexistence, quoted/
escaped key decoding and source-order effects. One deferred compiler control
checks allocation/data/prototype/spread ordering and register provenance.
Nothing executed; generated codecs/native implementations remain deferred.

### Fixed object-literal methods and accessors draft

Runtime templates now optionally declare strict receiver binding; omitted mode
preserves existing lexical arrows. Strict method creation stores no outer this
capture, method invocation uses its actual receiver, and frame validation checks
lexical receiver equality only for lexical functions. Source capture uses actual
method CST intervals, shared formal initialization, immutable capture proofs,
fixed name/length metadata and per-evaluation external callable allocation.
Getter/setter descriptor instructions preserve an existing opposite accessor
and ordinary data/accessor replacement behavior. Fixed methods named get/set/
__proto__ remain ordinary methods. All behavior stays in JavaScript source for
later automatic native translation.

Eight original sources cover shared methods on different receivers, capture
values, accessors and data replacement, name/length/identity, detached strict
calls, arrows created inside methods, get/set spellings and __proto__ methods.
Two authored controls cover template mode compatibility and ordered getter/
setter source instructions. No execution/generation/compilation occurred.
Computed method naming (including Symbol description authority), super/home
objects, async/generators and mutable lexical capture still need their separate
source extensions; this batch does not claim those surfaces are covered.

### Call and constructor argument spread draft

Frame-owned abstract argument Lists now support opening, ordered immutable append,
single consumption by Call/collection/Error/base-class constructor operations,
and handler-depth-based abandonment. They allocate no language Array. Validation
and all Call stack admission paths include retained List storage in the shared
argument budget. A resource refusal at invocation returns the original List and
instruction so retry cannot lose arguments. Nested catches preserve outer Lists;
crossed catch/finally boundaries release abandoned inner evaluation.

Source argument lists recognize spread and lower fixed and iterable segments in
order after holding callee/receiver. Iterable segments reuse the iterator-region
instructions and finally cleanup of done records. Base-class construction lowers
receiver allocation after all argument segments. Eleven originals cover ordered,
String/sparse/nested/empty/multiple spreads, receiver/callee/value snapshots,
class/collection constructors and catch recovery. Four deferred controls cover
immutable List state, handler scope release, source instruction provenance and
receiver allocation order. Kernel/control generation inputs are registered.
Nothing executed; generated artifacts and native validation remain deferred.

### Computed methods and runtime Function names draft

Computed method/accessor syntax now uses the actual method CST site and the
completed property key. Anonymous computed Arrow fields no longer require a
literal key. Before property publication, the shared functionName instruction
installs exact text names or suspends for actual Symbol description metadata.
The name kernel preserves lone UTF16 surrogates, distinguishes absent/empty
Symbol descriptions and assembles accessor prefixes within its metadata budget.
The private Node response verifies the live request, latest adopted heap and
actual Symbol identity before using the protected intrinsic description getter.
It never invokes a represented method or a user description accessor. Unknown
symbols remain explicit requests for another embedding. Driver declarations,
source replay events and native replay orchestration are authored together.

The former dynamic-computed-name refusal remains byte-for-byte as a positive
original with key pick and expected Function name pick. Five further originals
cover computed receiver/name, Symbol.iterator name, paired computed accessors,
single Object key conversion and computed __proto__ alongside null prototype.
Three deferred controls cover name assembly/metadata boundaries and converted
key/function provenance. This also connects previously drafted custom iterable
method syntax to the generic collection/for-of/spread paths. Generation inputs
are registered; generated code, all execution and validation remain deferred.

### Captured mutable local let cells draft

The source reader identifies simple local let declarations captured by actual
Arrow/method CST sites. It marks stable storage identities as captures rather
than taking immutable value snapshots. Lowering reuses the existing private
module-cell representation, allocating a genuine null-prototype value holder
per declaration evaluation. Reads and every existing module-cell assignment/
update protocol share that holder. The pointer binding is immutable; user value
writes change only its value field. Ordinary unshared locals keep registers.
No Rust replacement semantics or host closure fallback was authored.

Two previous mutable-capture refusals retain their exact source text as positive
originals expecting 9. Six additional originals cover sibling returned closures,
updates, distinct loop-body declarations, shared method/Arrow access, undefined
initialization and independent parent invocations. Two deferred controls compare
shared-cell versus ordinary register declaration lowering. Their expected IR
tags were selected by reading the existing local-binding constructor, without
executing it. Generation inputs are registered and no execution occurred.

Admission still refuses capture during initialization/later binding access;
mutable parameters, destructuring bindings, for-head per-iteration environments
and general runtime TDZ remain separate undrafted source work. The private cell
representation is shared with existing module storage, not a claim that full
ECMAScript declarative environments are implemented.

### Shared simple formal parameter cells draft

The same shared-storage admission now includes captured simple formal parameters
for functions, methods and Arrows, including one-parameter Arrow syntax. Source
parameter initialization installs a cell after its required/default/rest value
is ready and before later parameters initialize. Defaults can capture an earlier
initialized parameter; rest binding reassignment leaves the actual immutable
Call argument list unchanged. Own/later-parameter TDZ and recursive binding
patterns remain explicitly outside this admission.

Five originals cover returned parameter closures, later default capture,
rest-binding replacement, single-parameter Arrow and method parameter updates.
An additional deferred lowering control checks default/rest-before-cell ordering.
No execution, generation or validation occurred. Remaining environment work is
runtime TDZ, initialization-time recursion, shared destructuring bindings,
catch cells, loop-head cloning and full declarative environment behavior.

### Recursive destructuring shared cells draft

Source plans now retain the ordered unique actual shared-binding identity
directory. Both ordinary and template lowering carry it into recursive Object/
Array binding initialization. Shared captured leaves/rest bindings initialize
private value holders; Array output declarations keep their holder identities
immutable while user assignments continue through shared field operations.
Nested synthetic pattern roots are not captured as user bindings. Unshared
leaves retain existing mutability. The plan validator bounds and rejects invalid
or duplicate shared IDs; source capture still authenticates the actual full plan.

Six originals cover Object/Array lexical patterns, mixed nested patterns, Object/
Array rest binding replacement and destructured formal parameter captures. One
additional deferred control checks output mutability and cell initialization.
No execution or generation occurred. This drafts shared pattern storage after
initialization; runtime TDZ, capture during initialization, catch-cell storage,
and for-head environment cloning remain further work.

### Shared catch binding cells draft

Shared source binding discovery now recognizes actual catch parameter CST fields.
Simple catches wrap their received thrown value before handler body execution;
recursive catch patterns reuse shared leaf initialization. Allocation occurs on
each actual catch activation so closures retained from repeated handler entries
keep separate mutable storage. Three originals cover simple returned catch
closures, nested destructured catches and independent loop catch activations.
One additional deferred control checks handler-register cell initialization.
Nothing executed. Runtime TDZ, initialization-time recursive captures and
for-head environment cloning remain further source work.

### Captured mutable for-of heads draft

For-of simple and recursive mutable heads now participate in shared binding
discovery. The simple-head body installs a fresh holder after each received
iterator value; pattern heads use the shared leaf initializer. Captured closures
therefore observe writes during their own iteration without sharing a later
iteration's binding. Continue preserves its current holder through existing
iterator-region control flow. Capture initialization proof now checks that the
actual binding lies in a variable declarator's name field; an unrelated outer
Arrow initializer cannot misclassify an inner loop binding's initialization.

Three originals cover simple/pattern heads and continue. One authored lowering
control checks per-body wrapping of the actual loop register. No execution or
generation occurred. Normal for-head environment cloning, runtime TDZ and
initialization-time recursive captures remain further drafting work.

### Normal lexical for per-iteration cell cloning draft

Captured mutable normal for-head bindings now receive shared cells and retain
their actual source identity list on the loop statement. Source lowering drafts
the same clone expression before the initial test and before every update.
Cloning copies current values into new private holders and replaces only the
internal holder registers. Initializer closures keep the original environment;
condition/body closures keep each iteration's environment. Continue targets the
clone/update region even when no update expression exists, and existing
completion machinery routes finally before it. Break exits before cloning.
Recursive binding heads participate through their shared leaf identity directory.

Six originals cover normal capture, initializer capture, continue/finally,
destructured heads, multiple bindings and final-false-condition capture. One
authored compiler control checks two clone sites and the continue/back-edge
targets. No execution/generation/validation occurred. Runtime TDZ and recursive
capture during initialization remain further source drafting work.

### Abrupt immutable lexical PutValue draft

Constant bindings now retain an explicit immutable Reference policy in source
plans. Assignment lowers RHS evaluation before an actual source TypeError
request. Update/compound operations retain the existing GetValue/conversion
ordering before attempting that write; logical operations retain short-circuit
branches without inventing a write. Captured constants remain immutable values
after failed assignments, so syntactic attempted writes no longer incorrectly
disqualify their capture-by-value proof. Runtime TDZ is still a separate need.

The former constant-update refusal is preserved byte-for-byte as a positive
original requiring TypeError. Six more originals cover RHS/conversion effects,
short-circuit behavior, compound ordering, preserved captures and conversion
error precedence. Two deferred controls inspect the operand-before-raise
instructions and immutable Reference policy. Diagnostic text is drafted for the
existing Node >=22.15 <23 || >=24 <25 source profile; it is not evidence for
uncovered engines. No execution/generation/validation occurred.

### Shared push/pop standard bodies draft

Protected Array push/pop owners now compile to shared property bytecode. New
argumentCount/argumentAt instructions read the actual immutable Call List;
ordinaryDelete uses the existing heap deletion kernel. Shared primitives retain
completed ToLength, check safe append limits before writes and enforce strict
delete failure. Push Sets every selected element and length even for zero
arguments. Pop holds the selected value across own deletion and length update.
No native Array body or represented host collection implements the mutations.

Nine originals cover full arguments/new length, Object argument identity, sparse
pop, generic receivers, zero-argument Get/Set, inherited setters/elements, held
getter values and actual length conversion. Three deferred controls inspect
bytecode ordering and primitive boundaries. Source/policy inputs are registered.
No execution/generation/validation occurred. Primitive receiver boxing remains
an explicit unsupported protocol, and exact write/delete/overflow diagnostics
need engine-version policies. Shift/unshift/splice/slice/concat and remaining
standard library surfaces still need their own drafts.

### Shared shift/unshift standard bodies draft

The protected mutation directory now includes shift/unshift. Both lower to the
same inspectable property instructions. A new ordinaryHas opcode uses the shared
prototype presence kernel with actual converted-key validation. Ascending shift
retains the first result; descending unshift moves existing properties before
inserting actual arguments. Missing source properties delete the corresponding
own targets strictly; inherited fields remain actual Get/HasProperty inputs.
Zero-argument unshift skips traversal but Sets length. Safe-length checking
precedes any unshift move/write; source loops remain step-budget resumable.

Seven originals cover sparse movement, inherited fields, complete getter/setter
ordering, zero-argument traversal omission and safe-limit refusal. Two deferred
controls inspect presence branches, common target keys and direction. Inputs
flow through the already registered mutation source/control files. Nothing
executed. Remaining mutation/library surfaces and primitive boxing/opaque
protocols still require their own source extensions.

### Reverse/fill/copyWithin shared body draft

The same protected directory now emits reverse/fill/copyWithin bytecode. Reverse
retains lower/upper presence separately, gets lower before inspecting upper and
selects the correct strict Set/Delete combination. Fill converts relative bounds
in order, then writes actual values without presence tests. CopyWithin converts
target/start/end in order, moves backward for overlap and otherwise forward,
preserving holes through strict target deletion. All return the actual receiver
without explicit length Set or result allocation. The shared relative-index
primitive consumes completed numeric conversions, including infinities/NaN.

Ten originals cover holes/identity, getter/setter order, length-one omission,
negative fill bounds, filled Object identity, empty-range conversions, both
copy directions, sparse deletion and target/start/end conversion order. Two
additional deferred controls inspect bytecode ordering/direction and relative
index boundaries. No execution/generation/validation occurred. Remaining source
library, primitive boxing, opaque protocols and full lexical TDZ/recursion
continue as separate drafting work.

### Source delete/in/void operator draft

Source capture/lowering now routes delete and in to the shared property
instructions. Delete retains the actual receiver/key Reference without reading
the selected value, applies strict success checking and preserves optional
nullish short-circuiting. Non-Reference operands evaluate before returning true;
unqualified strict identifier deletion (including parenthesized undefined) is
rejected as an early error. In holds key then Object operands, checks the Object
before key conversion and uses prototype presence without invoking getters.
Void evaluates then discards its operand value without conversion.

Ten originals cover Array holes/length, accessor deletion, inherited fields,
strict nonconfigurable deletion, accessor presence, in error ordering, one key
conversion, optional deletion, void and non-Reference effects. Two new early
error refusals and three deferred source/provenance controls accompany them.
No execution/generation/validation occurred. Primitive property boxing and
opaque/Proxy protocols remain explicit unsupported boundaries.

### Shared Object keys/values/entries source/VM draft

`javascript-own-enumeration.js` drafts own-key snapshot ordering, current
descriptor enumerability, Symbol exclusion and actual selected Get continuations.
Key-only enumeration never invokes getters. Entry pairs are genuine fresh Arrays
created during enumeration; the final Array is allocated from the completed
internal List after all getters finish. String primitives expose their UTF16
indexed own properties; other non-nullish primitives contribute no own fields.
Nullish inputs raise actual TypeError. Held keys/values/pairs/write coordinates
survive resource suspension, without host Object method fallback.

The shared call machine drafts the opcode, pending validation, selected-child
binding, result allocation, getter completion and error paths together. The
private Node directory verifies actual Object static method owners, supplies its
actual realm Array prototype and publishes the Object namespace. Source capture
admits that namespace through the existing qualified capability path.

Ten originals cover numeric/text order, Symbols, accessor omission/order,
getter mutations, fresh pair identity, prototype/Array-length exclusion, UTF16
String values, primitive empty results, nullish errors and thrown getter values.
Four deferred kernel controls inspect key-only behavior, selected getter
completion, pair-before-result allocation and UTF16/nullish behavior. Generation
inputs are registered. No execution/generation/validation occurred.

The existing compiler adapter `javascript-object-enumeration.js` is preserved;
the runtime kernel lives separately in `javascript-own-enumeration.js`. Source
diff review caught and corrected a working-tree filename collision before any
commit or push of the new kernel.

Resource care: the required project-owned cleanup ran with both inherited Cargo
target variables unset and --project-owned-only --no-preflight --no-docker.
It reported 510.2 KiB before/after, zero removed and zero reclaimed; its receipt
is retained under .git/meta-language-cache. No dependencies, worktrees, workload
clones, builds or generated artifacts were created. Disk last showed 71 GiB free.

### Object own-property/SameValue/integrity source draft

`javascript-object-integrity.js` drafts Object.hasOwn/is and the six integrity
operations through the shared heap. HasOwn performs coercibility before key
conversion, supports actual own descriptors and String indices/length, and never
substitutes prototype presence. SameValue preserves NaN/negative-zero/Object
identity behavior. Modern primitive integrity behavior returns primitives
unchanged and reports their nonextensible/sealed/frozen state. Ordinary mutations
prevent extension, snapshot keys and change one descriptor per step. Seal keeps
data writable; freeze clears data writability while retaining accessor functions
and mutable Map/Set internal slots. Opaque protocols remain explicit boundaries.

The protected Object directory, bytecode, pending/source validation, step/result/
error paths and automatic source policy inputs are authored together. Ten
originals cover own versus inherited fields, String metadata, nullish/key order,
SameValue, frozen/sealed writes, retained setters, Array deletion, Map internals
and primitive integrity. Five deferred controls cover descriptor step ordering,
primitive behavior, own keys/SameValue, Map state and hasOwn key ordering.

Source review also corrected enumeration validation/control code to use the
existing readJavaScriptArrayLength descriptor helper: Array length is an own
descriptor, not a separate record field. All fixture inputs/expectations remain.
No execution, compilation, generation or validation ran.

### Object prototype operations and realm metadata draft

The protected Object operation directory now includes getPrototypeOf and
setPrototypeOf. Ordinary inputs use their actual heap prototype and the existing
prototype mutation kernel. Cycles/nonextensibility reject; an unchanged prototype
on a nonextensible Object succeeds. Primitive setPrototypeOf validates its
prototype then returns the original primitive. Primitive getPrototypeOf suspends
for actual realm-root metadata; the private Node response authenticates the
request/latest heap and selects only roots registered for this embedding realm.
Unknown realms remain explicit requests. Driver declarations, source trace and
native replay orchestration call the same translated shared resumer.

Eight originals cover identity/inheritance, null roots, cycle/nonextensibility,
primitive realm roots and prototype validation, and bypassing own proto setters.
Two deferred controls inspect ordinary transitions and source-bound VM metadata
resumption. No execution/generation/compilation/validation occurred. Exact engine
error wording and opaque/Proxy prototype protocols remain explicit extensions.
