# Generic base Construct source batch — 2026-11-02

This is an integration handoff supporting the [vision](../vision.md). Authored
source and future controls are not execution evidence or requirement completion.

## Source contract

`javascript-construction.js` drafts shared base construction for an actual target,
independent actual newTarget and an ordered argument vector. It reuses the
existing Array species constructor authority, ordinary heap Get/allocation,
source scalar conversion, primitive-wrapper allocator and Array descriptor rules.
No represented native constructor or carried source body is its fallback.

Actual private constructor facts retain constructability, realm and an optional
`intrinsicKind`. Recognized protected identities are Array, Number, Boolean,
String and Object. Existing `intrinsicArray` remains the Array compatibility fact.
An undefined intrinsicKind plus an independently constructable source declaration
and actual executable strict/base-class mode admits source body construction.
Function names and prototype resemblance do not establish intrinsic identity or
constructability. Unknown targets/newTargets request authority. Unsupported
derived, bound, Proxy and other constructor kinds request implementation,
including unsupported newTarget prototype/realm protocols; they do not become
invented nonconstructor errors or incomplete empty instances.

Both constructor classifications precede implementation work. Source functions
and base classes read newTarget.prototype, use that actual Object when supplied,
and otherwise select the default Object prototype from newTarget's realm. Body
Call carries the actual newTarget separately, receives the newly allocated
receiver and every original argument, and accepts an Object return as the result.
A primitive return keeps the receiver. The body can change its receiver prototype;
pending-state validation must not reject that legal mutation. Original body or
prototype-getter throws preserve their value and completed heap effects.

Number and String convert the first supplied argument before prototype Get;
missing arguments remain distinguishable from explicit undefined. Number's
explicit conversion accepts BigInt. Boolean uses truthiness without object
coercion. Wrapper allocation creates actual typed internal data; String's exact
UTF16 data produces its own immutable indices and length through the existing
atomic wrapper allocator. String construction rejects Symbol implicit conversion,
instead of applying the distinct descriptive String Call operation.

Object with identical target/newTarget returns an Object argument unchanged,
boxes a primitive using its actual Object-constructor realm roots, or creates an
empty ordinary Object for nullish input. A different newTarget instead causes
ordinary creation through that newTarget's prototype and ignores the input.

Array reads newTarget.prototype first. Zero arguments creates an empty Array;
one non-Number creates one actual element without coercing it; one Number creates
the Array at length zero before validating and writing its requested unsigned
length. A bad numeric length therefore reports RangeError only after the actual
Array identity allocation. Multiple arguments create the initial length and
publish each element in order. All initial element definitions are own data
properties and preserve selected prototypes and resource retry positions.

Constructor conversion and Array length errors include the target intrinsic's
realm, independent of the calling/newTarget realms. Target/newTarget
nonconstructor errors omit that optional override and belong to the invoking
Construct context. Getter/body throws already carry actual realm Error identity.

The dated primary constructor sources inspected for ordering were
[Number](https://tc39.es/ecma262/2025/multipage/numbers-and-dates.html#sec-number-constructor-number-value),
[String](https://tc39.es/ecma262/2025/multipage/text-processing.html#sec-string-constructor-string-value),
[Object](https://tc39.es/ecma262/2025/multipage/fundamental-objects.html#sec-object-constructor)
and [Array](https://tc39.es/ecma262/2025/multipage/indexed-collections.html#sec-array).
They establish separate Call/Construct conversion rules, Object's active-function
special case and Array's creation-before-length-error order. No specification
text was copied into generated artifacts.

## Types and public kernel API

The kernel accepts `JavaScriptArraySpeciesDirectory`. Shared authority extends
`JavaScriptArrayConstructorFact` with optional `intrinsicKind:string`, and
`JavaScriptConstructorRealm` with optional numberPrototype, booleanPrototype,
stringPrototype, integerPrototype and symbolPrototype values. These are actual
protected realm identities, not source names. Kind/root updates must preserve
previously established facts when extending authority.

Exports used by shared integration:

- `beginJavaScriptConstruction(program, heap, directory, target, newTarget, arguments_)`.
- `stepJavaScriptConstruction(program, heap, directory, state)`.
- `resumeJavaScriptConstructionCall(program, heap, directory, state, completion)`.
- `resumeJavaScriptConstructionAllocation(program, heap, directory, state, identity)`.
- `readJavaScriptConstructionStateError(program, heap, directory, state)`.

State is `{$:'constructionState',target,newTarget,arguments,phase,mode,prototype,
receiver,primitive,arrayIndex,allocationKind,conversion?,call?}`. A selected call
is `{$:'constructionCallRecord',callee,receiver,arguments,constructing,newTarget}`.
The newTarget value is semantic undefined for ordinary setup calls, and the actual
selected constructor for body Construct calls. Arguments are copied as an
immutable ordered vector; referenced Object values remain live shared identities.

Every transition includes `heap`:

| Discriminant | Other fields |
| --- | --- |
| constructionContinue | state |
| constructionCall | state, callee, receiver, arguments, constructing, newTarget |
| constructionAllocate | state, prototype |
| constructionNeedAuthority | state, constructor |
| constructionNeedImplementation | state, constructor, newTarget, arguments, operation, reason |
| constructionCompleted | value |
| constructionThrow | value |
| constructionError | errorClass, reason, optional realm |
| constructionInvalid | reason |
| constructionResourceLimit | state, reason |

Source work limits are explicit resource outcomes: 4096 arguments, the shared
Object/property limits, existing numeric conversion limits and atomic wrapper
bounds. State retains completed conversions, selected prototype getter results,
held argument values, allocated receiver and Array publication cursor. Resource
retry must never repeat a completed getter, conversion or element creation.
Wrong-kind allocation responses are refused before advancing any other request.

## Shared VM/frontend/realm integration owned by root

1. Add a program-level construction directory, initially from actual protected
   realm/declaration metadata. Maintain its actual intrinsic kinds and realm
   wrapper roots through existing authority requests and monotonic extensions.
   No string/function spelling or hand-created default realm may substitute.
2. Add ordinary and variadic Construct operations with explicit target/newTarget
   operands and actual argument vectors. Hold callee/newTarget expressions before
   later argument effects can change source bindings. Generic `new` uses the
   actual target as newTarget. Reflect.construct has its own specified operand/
   argument-list order and must validate its constructor operands in the correct
   order before observable list reads where the standard requires that.
3. Pending construction stores destination, invoking realm and kernel state.
   Validate the actual instruction operands, immutable argument vector, directory,
   selected call, and all child receiver/newTarget/argument metadata. Normal setup
   Calls use shared Call. `constructing:true` must enter the actual source body as
   a constructor with the allocated receiver and explicit newTarget; it must not
   invoke a class through normal Call or assume target.prototype equals the
   selected newTarget prototype. Both normal and abrupt returns feed the kernel.
4. Add actual newTarget to constructor frames and expose direct source new.target.
   Normal ordinary function Call uses undefined. Arrow new.target is lexical and
   requires closure capture; it must not read undefined merely because an arrow
   is invoked through Call. That closure extension remains independent work if
   it is not included in root's shared integration batch.
5. Allocation and authority events use existing global identity/context providers
   and the dedicated kernel resumers. Preserve global identities in Array failure
   heaps and newTarget realm fallbacks. NeedImplementation remains an explicit
   unsatisfied operation request, never a successful placeholder receiver.
6. ConstructionError uses its optional target realm override where present;
   otherwise allocate in the invoking realm. Original throws and unsupported
   operations retain their proper distinction. Generic base construction has no
   special IteratorClose cleanup to fabricate on errors.
7. Keep existing specialized new Map/Set constructor paths until their iterable
   construction bodies are composed with explicit newTarget. This kernel does
   not silently drop those existing adder/iterator semantics: dynamic aliases or
   Reflect.construct targeting currently unsupported Map/Set kinds request their
   implementation explicitly. The same applies to derived classes/bound/Proxy
   targets and unrelated exotic builtin bodies.
8. Register the module, fixture/control file, producer group and native replay
   roots with automatic policy authoring. Automatically translate the complete
   shared JavaScript bodies to Rust together. Generated native files were not
   edited and source authoring/generation did not run in this phase.

Generic local constructor support also changes the rationale for the retained
same-spelled-local-Map refusal. Preserve its exact source input and coverage when
moving it into positive capture controls: a genuine local Map declaration now
uses its actual ordinary Construct semantics, not the protected native Map body.
An executable wrapper should demonstrate the allocated local instance and ignored
primitive return. Identity protection remains mandatory.

## Deferred controls

`javascript-construction-source.mjs` records twenty-one original inputs under the
existing closed-source profile and selected Node ranges `>=22.15.0 <23.0.0` and
`>=24.0.0 <25.0.0`. The test runner errors on uncovered versions rather than skips.
`translation-javascript-construction.test.js` adds eighteen kernel controls plus
all original-versus-product oracles. Coverage includes independent newTarget,
full arguments, original returns/throws/effects, body prototype mutation, raw and
fallback prototype selection, constructor realm differences, missing authority,
wrapper slot types, conversion ordering, Symbol rejection, Object special cases,
Array creation/length/elements/holes, actual new.target, identity-vs-name,
implementation requests, resource retries and substitution refusal. Exact native
Error message wording remains a separately version-sensitive contract; original
error fixtures preserve inputs and assert classes and original identity/effects.

No represented program, test, import-time oracle, generation, validation gate or
compilation ran. No observations are claimed. Small source files use the shared
checkout; no dependencies, new worktrees or compiler caches were created. The
ledger/source register, naming and feature inventory updates remain deferred to
the required live source-audit/generation phase. Unrelated journals are preserved.
