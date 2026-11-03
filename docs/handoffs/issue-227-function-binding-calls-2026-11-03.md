# Function binding and ordinary Calls source draft

This continues [shared species construction](issue-227-shared-species-construction-2026-11-03.md)
under the sole architecture in [vision](../vision.md). Source edits, source review
and primary specification reading only. No represented program, test, generator,
formatter, validation gate, compilation or CI feedback ran or was reviewed.
Full requirements drafting remains incomplete and the active goal is unchanged.

Successor: [bound construction and instanceof](issue-227-bound-construction-2026-11-03.md)
drafts the target-chain authority, common Construct delegation, Array integration
and OrdinaryHasInstance work described below. The remainder stays binding.

## Actual paths drafted

`javascript-function-binding.js` drafts binding allocation and resumable metadata
reads under the pinned
[ECMAScript bind contract](https://tc39.es/ecma262/2025/multipage/fundamental-objects.html#sec-function.prototype.bind).
The new heap variant stores the callable target, receiver and ordered argument
prefix. Slot validation rejects missing references, target cycles and storage
overflow; completed target paths are shared during graph inspection. Retention
includes those slots as strong edges. Ordinary property operations, reflection,
descriptors, copying, integrity and indexed-property handling admit this genuine
ordinary-method function kind rather than routing it through opaque host hooks.

The protocol captures the actual target prototype, allocates a fresh external
identity and then performs ordered own-length and name observations. Selected
getters execute as real machine children; saved states retain the captured
prototype and original operands. Length remains in its Number domain, with
fractional, infinite and large values retained; nonnumeric length and nontext
name are not coerced. Metadata uses normal descriptors and preserves UTF16.
Name/property work limits retain the already observed value and continuation.

`Function.bind` joins the existing protected callable directory beside call and
apply. The Function namespace is available as an actual protected source
reference for borrowed invocation through Function.prototype; this does not
implement dynamic Function source construction. Source lexical shadowing remains
in the existing binding graph. The earlier assertion calling Function.bind an
unknown operation now requires its actual protocol body for that same input.
Unknown-operation refusal remains covered by Function.unknown. The change is
justified by the pinned contract and the actual new body; no old input, version
range or coverage was removed and no behavioral result is claimed.

Ordinary Calls project a constant two-instruction forwarding body from each
actual bound heap entity and the real target code realm. This avoids rewriting
every source caller's callee identity and avoids unrolling stored arguments into
generated instructions. The forwarding opcode calls the actual target with
stored receiver and prefix plus actual supplied arguments. Child validation
checks this correspondence, and failed resource acquisition retains the original
operation for retry. Existing lexical target and class Call behavior remain
owned by their real bodies. Static and bound identities cannot replace one
another. A target without executable source code receives no host-body fallback.

The generic constructor mode recognizes a bound entity separately from this
ordinary forwarding body. It must not treat the projected strict wrapper as a
base constructor. Its complete construction protocol is not yet drafted; this
guard is not a final supported-language limitation or a completion claim.

The normal source-author recipe includes the new module, literal programs and
consumer controls. Native/model I/O/schema/name/source publication remains
deferred until every requirement has its actual complete code draft. No Rust
replacement semantics or generated native edits were introduced.

## Independent controls authored

Thirteen original programs cover stored receivers/prefixes, repeated binding,
fresh identities and aliases, lexical arrows, descriptors, ordered getter effects,
missing own length, prototype snapshots, noncoercion, numeric boundaries,
original throws, bound getters, Array callbacks and bound class Call errors.
They retain independent native declarations and explicit Node version ranges.
Heap controls inspect actual strong slots and metadata, forge saved operands and
target cycles and preserve exact preallocation state at the heap identity limit.
All controls remain unexecuted; these are input counts, not a requirement count
or percentage. All earlier call/apply, heap and source inputs remain.

## Next code batch: bound construction and related integration

1. In `javascript-array-species.js` and the generic constructor authority path,
   derive a bound entity's constructability from its immutable actual target
   chain and authoritative target facts. A bound Array is not the intrinsic
   Array identity. Missing target facts need the exact actual authority request;
   names, prototype shape and a projected Call body prove nothing here.
2. In `javascript-construction.js`, draft actual bound Construct delegation:
   concatenate stored and supplied arguments, substitute newTarget only when
   it is the same bound identity, and invoke the real target through the common
   kernel without allocating a substitute ordinary receiver. Keep complete
   source arguments, ordering, realm/error and child correspondence. Nested
   binding repeats this rule in order; arrows remain nonconstructors.
3. Complete GetFunctionRealm through bound targets for independent newTarget
   fallback and all Array creation/species paths. Retain actual realm roots,
   monotonic fact updates and caller-versus-constructor Error distinctions.
4. In `javascript-instance-of.js`, preserve outer operands while independently
   checking a delegated InstanceofOperator on the actual bound target, including
   target Symbol.hasInstance effects before primitive-left shortcuts. A simple
   replacement of the outer right operand would lose its source correspondence.
5. Complete public callable handles and source-free snapshot/native transport
   for bound values. Metadata reads must use actual stored descriptors; opaque
   source-handle restrictions cannot fabricate a bound name/prototype or execute
   an original body. Keep code reconstruction from actual heap slots and actual
   source/program definitions, without a hidden per-bound compiled owner table.
6. Draft native-original controls for nested construction prefixes, stored-this
   bypass during new, independent newTarget, derived/class/arrow targets, realm
   fallback, custom hasInstance and exact abrupt/resource effects. All native
   execution and Rust compilation remain CI-only after the full draft gate.

## Remaining whole objective

Continue derived/super/Proxy/private-field/effect/liveness JavaScript protocols,
Rust callback/iterator/storage/generic/trait/module support, full own-project CST
and both translation directions, and every broader grammar/concept/formal,
recursive parsing, ontology/taxonomy, RDF/OWL, dynamic, inference and delivery
requirement in vision. After every requirement has full actual code/control
drafts, reconcile sources online before ledger use, publish automatic assets,
execute authorized JavaScript and CI-only gates, fix actual failures and record
observations at their executed commits. Never compile Rust locally or present
source review as runtime evidence.
