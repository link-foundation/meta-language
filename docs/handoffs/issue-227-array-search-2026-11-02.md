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
