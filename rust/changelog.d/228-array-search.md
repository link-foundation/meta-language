Draft shared Array includes/indexOf/lastIndexOf/at with exact search equality,
hole handling, argument presence and resumable getter/index conversion effects.
Protected callable linkage and automatic native generation inputs accompany the
draft; original/native execution and verification remain deferred.

Draft generic iterable Map/Set construction using shared ordinary Get, Call,
iterator-region and completion instructions, with ordered Map entry reads and
actual IteratorClose effects. Arrays, Strings and source-defined iterables have
authored controls; execution and generated native publication remain deferred.

Draft colon-form literal prototype installation with decoded-key recognition,
duplicate-setter rejection, null/Object prototype handling and primitive no-op
semantics; computed/shorthand keys retain own data semantics.

Draft fixed object-literal methods and accessor definitions with genuine runtime
callable allocation, strict receiver binding, captured values, name/length and
ordinary getter/setter descriptor merging. Source and native execution deferred.

Draft iterable Call/constructor argument spread with internal frame argument
Lists, selected callee/receiver snapshots, actual iterator traversal, abrupt scope
release and base-class receiver allocation after arguments. Execution deferred.

Draft computed object methods/accessors and runtime anonymous Function names
from converted property keys, including authenticated Symbol description
metadata and replay through translated shared semantics. Execution deferred.

Draft captured local let bindings through the existing private shared cell
representation, preserving sibling closure updates and independent declaration
evaluations. General TDZ/parameter/loop-head environments remain separate.

Draft shared simple required/default/rest parameter cells for functions, methods
and Arrows, preserving ordered initialization and the original Call arguments.

Draft shared captured mutable leaves/rest in recursive Object/Array binding
patterns with actual source binding identity directories and immutable cell
holder registers. Execution and generated native publication remain deferred.

Draft shared catch parameter cells, including recursive patterns and fresh
storage per handler activation for closures retained beyond catch scope.

Draft captured mutable for-of heads with fresh cells per received iterator value,
including recursive patterns and continue paths. Execution remains deferred.

Draft normal lexical for-head environment cloning before the first test and
each update, preserving initializer/condition/body closure environments and
continue/finally control flow. Execution remains deferred.

Draft immutable lexical PutValue failures after RHS/numeric/compound effects,
preserving logical short-circuiting and constant captures through failed writes.
The prior refusal input remains as an authored TypeError original.

Draft protected Array push/pop bodies using actual Call arguments, observable
length conversion, strict ordinary property writes/deletion and held results.
Object receivers and inherited accessors have deferred original controls.

Draft shift/unshift through shared presence/Get/Set/delete bytecode, retaining
sparse holes, inherited elements, ordered effects and bounded resumable loops.

Draft reverse/fill/copyWithin shared bodies with preserved hole presence,
ordered bound conversion, overlap-safe copying and receiver identity returns.

Draft source delete/in/void with held property References, strict deletion,
accessor-free presence, optional deletion and actual operand evaluation order.

Draft Object keys/values/entries with shared own-property snapshots, actual
getter continuations, UTF16 String keys and genuine entry/result Array creation.

Draft Object.hasOwn/is and resumable freeze/seal/extensibility operations,
preserving accessor functions, primitive behavior and collection internal slots.

Draft Object prototype operations with ordinary cycle/extensibility behavior
and authenticated primitive realm-root metadata replayed by shared semantics.

Draft public Object.defineProperty through shared descriptor conversion and
ArraySetLength continuation, preserving both conversions and partial error heaps.

Draft Array species selection with real getter effects, foreign-realm rules,
private declaration-based constructor facts and explicit construction/allocation
requests. Authenticated source units extend facts dynamically; map/filter target
creation and traversal composition remain to be drafted.

Draft excluded String/Symbol keys in shared CopyDataProperties, including exact
instruction/register binding and skipping excluded getters. General source rest
pattern binding and execution checks remain deferred.

Draft ordered identifier declaration groups and uninitialized local let in
ordinary statements/for heads, preserving binding identities and initializer
order while retaining explicit switch TDZ refusal.

Draft shared map/filter traversal and default species Array allocation, preserving
holes, callback effects, compact filter outputs and held results across resource
limits. Custom construction and missing authority remain explicit VM requests;
portable constructor execution and generated parity remain separate work.

Draft actual source species Construct through the shared call stack, including
prototype getter continuations, realm fallback, receiver allocation and constructor
return selection. Preserve current effects without replaying original host bodies.

Draft Node driver responses to authenticated constructor metadata additions,
retaining suspended effects and publishing monotonic shared program updates.

Draft shared object binding/rest initialization for parameters and lexical
declarations, including empty-pattern coercibility, defaults and own-key exclusions.

Draft computed object binding keys with held String/Symbol keys shared between
field access and rest exclusion, preserving key and default evaluation order.

Draft explicit constructor metadata events in source traces and native replay
through the common authority extension, with automatic codec generation inputs.

Draft recursive object bindings using the shared initialization protocol,
including nested defaults, per-level rest exclusions and explicit nesting bounds.

Draft shared synchronous iterator acquisition, advancement and closing protocols
with cached methods, original receivers and exact cleanup completion precedence.

Draft genuine live Array iterator slots, reentry protection and getter/conversion
continuations, reusing shared entry/result allocation and native generation inputs.

Draft protected Array iterator builtin linkage and resumable VM integration,
including result allocation and source release during exception unwinding.

Draft Array spread accumulation through shared iterator continuations and own
indexed writes, retaining completed values across storage resource suspension.

Draft source Array spread literals through ordered accumulation, shared VM
continuations, private coordinates and explicit hole length writes.

Draft exact UTF16 String iterator slots, code-point advancement and shared
result allocation, preserving surrogate pairs and unpaired code units.

Draft String iterator builtin/VM composition and authenticated primitive
prototype lookup, preserving original receivers in source/native replay.

Draft complete source try/catch composition through shared handlers, retaining
catch identities, mutable values and the boundary before following statements.

Draft recursive catch object bindings through shared coercibility, default and
rest initialization, preserving failures to the enclosing handler.

Draft source finally and common deferred return/throw/control completions,
including loop exit unwinding and constructor normalization after finalizers.

Draft bounded per-frame iterator region ownership and preserve Array spread
operand/key snapshots before callbacks.

Draft complete source for-of composition through actual per-frame iterator
regions, shared callbacks and finalizer-based closing with throw precedence.

Draft recursive Array binding patterns across parameters, declarations, catch
and for-of, with elisions, defaults, cached iterator rest and closing before body.

Draft formal rest parameters through original Call argument collection, actual
Array allocation, recursive binding targets and Function length boundaries.

- Draft shared `instanceof` and `OrdinaryHasInstance` semantics, selected getter/
  method continuations, actual Error namespace roots and protected Function
  intrinsic linkage for automatic translation. Verification remains deferred.

- Draft public Object/Reflect own-key and descriptor reflection with genuine
  result Objects/Arrays, Symbol/nonenumerable keys and primitive String flags.
  The JavaScript source remains the automatic Rust translation input.

- Draft Object.defineProperties and Object.create by composing ordered descriptor
  conversion and the shared property/Array definition kernels, retaining partial
  effects and actual prototypes for automatic cross-runtime translation.

- Draft Object.assign using shared enumerable-copy Set semantics, actual getter/
  setter Calls, Array-length continuations and portable legacy prototype accessors.

- Draft Function.call/apply and Reflect.apply through frame-owned argument Lists,
  ordered array-like Get/ToLength and shared receiver-aware source Calls.

- Draft borrowed Object.prototype own-property/enumerability and prototype-chain
  observations with their distinct conversion order and primitive boundaries.

- Preserve actual Object.prototype immutable-prototype metadata in the shared
  heap while allowing same-prototype updates and ordinary descendant mutation.

- Draft shared lexical cells for recursive/later closures, runtime TDZ and const
  writes, ordered destructuring/formal defaults, normal for cloning, separate
  for-of RHS/iteration environments and shared switch-case scope. Preserve all
  prior inputs and add deferred original/native parity controls.

- Draft function-scoped var hoisting, canonical redeclarations, parameter/body
  environment copies, destructuring and shared var loop cells using the common
  source storage model. Keep catch-shadowed assignment targets explicit.

- Draft separate catch-shadowed var declaration and assignment-reference facts
  for identifiers, patterns/rest and for-of, including captured catch cells.

- Draft twenty-one protected String method bodies using shared coercion/UTF16
  kernels, ordered IsRegExp checks, padding early return and actual argument Lists.
  Preserve boxed String/RegExp input slot metadata and legacy catalog profiles.

- Draft Number Call/noncoercive predicates and common Math bodies using shared
  numeric conversion/IEEE754 primitives and actual variadic argument Lists.
  Preserve Number constructor classification separately from Call support.

Draft protected BigInt/Boolean Calls and exact BigInt.asIntN/asUintN conversion
order, width bounds and signed modular wrapping through shared source kernels.

Draft UTF16 parseInt/parseFloat prefix grammars with exact shared numeric rounding,
protected parser aliases/global predicates, and exact Math.imul/clz32 kernels.

Draft shared primitive prototype reads, own Number/Boolean/BigInt slot extraction,
Boolean stringification and exact BigInt radix stringification with ordered coercion.
