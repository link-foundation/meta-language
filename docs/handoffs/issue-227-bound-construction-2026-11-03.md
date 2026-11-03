# Bound construction and instanceof source draft

This continues [binding and ordinary Calls](issue-227-function-binding-calls-2026-11-03.md)
under the sole architecture in [vision](../vision.md). Source edits, textual review
and primary specification reading only. No test, represented program, generator,
formatter, validation gate or compilation ran; no CI feedback was read. Full
requirements drafting remains incomplete; the original active goal stays intact.

## Actual drafted paths

Constructor classification follows actual immutable bound target links to the
independently registered leaf. Derived facts keep the bound identity and target
realm/constructability, with no intrinsic Array identity or intrinsic kind.
Directories reject invented independent bound constructor facts. Unknown authority
requests the actual leaf, preserving the original source operand in its pending
operation. Constructor fallback roots use that leaf realm, including an
independently supplied bound newTarget, and request that actual leaf if roots
are missing. Prototype mutation and mutable names do not change this ownership.

The generic Construct kernel now drafts bound delegation with one outer
continuation and one normalized non-bound child. It collects every actual bound
prefix once and concatenates them in reverse target traversal order, then the
supplied arguments. Each matching bound newTarget becomes its actual target;
independent newTargets are retained. The derivation performs no property Get,
body execution or receiver allocation. Actual target semantics then use the same
generic kernel: base function/class bodies, actual Array/wrapper/Object and
branded collection allocation/initialization. The bound this slot is unused by
construction. Existing work/storage limits retain exact operands and operation.

Saved-state inspection independently derives the child operands from actual
slots, disallows nested delegated children or a bound normalized target, checks
the complete child and mirrored selected call, and retains unchanged original
source target/newTarget/arguments on the outer envelope. Parent construction
context readers expose only the active child to constructor frame and collection
initializer reuse checks. Array creation and map/filter/slice/concat species
validation use derived bound facts and the actual selected newTarget. Authority
extension checks its actual pending request instead of assuming one phase name.
The old persisted species model retains its preceding separate interpretation.
Shared species specialization requests also retain the actual delegated target,
newTarget, prefix-inclusive arguments, operation/reason and target fact rather
than reusing the outer bound fact or synthesizing a length-only request.

OrdinaryHasInstance now drafts actual target delegation before primitive-left
shortcuts. The state retains its original left/right operands and well-known key,
plus a checked bounded path of actual bound target links and the current candidate.
It then performs full target InstanceofOperator: actual Symbol.hasInstance Get,
selected getter or method, receiver, original left value, Boolean result and
abrupt values. Nested delegation appends actual links. Prototype containment
keeps its original distinct operation. No source operand is overwritten to make
the delegated call pass parent frame inspection.

Normative contracts: [bound Construct](https://tc39.es/ecma262/2025/multipage/ordinary-and-exotic-objects-behaviours.html#sec-bound-function-exotic-objects-construct-argumentslist-newtarget),
[GetFunctionRealm](https://tc39.es/ecma262/2025/multipage/abstract-operations.html#sec-getfunctionrealm)
and [OrdinaryHasInstance](https://tc39.es/ecma262/2025/multipage/abstract-operations.html#sec-ordinaryhasinstance).
This textual correspondence is not execution evidence.

## Independent controls authored, all unexecuted

Fourteen constructor originals cover stored-this bypass, ordered nested prefixes,
matching and independent newTarget, own bound prototype bypass versus actual
independent prototype reads, replacement returns, base classes, nonconstructor
arrows, intrinsic Array/wrapper/Object/Set construction, Array.of/from, bound
species and original throw identity/effects. Eight instanceof originals cover
actual prototypes, nested binding, custom methods/getters, primitive inputs,
method receiver/result conversion, direct OrdinaryHasInstance versus an own bound
method override, null/noncallable methods and arrows. Node assertion ranges are
explicit. These are program counts, never a requirements percentage.

Kernel controls separately inspect normalized operands, absence of extra
allocation, actual body/newTarget correspondence, foreign realm fallback and
getter receiver, exact missing leaf authority, nonconstructor closures, forbidden
independent bound facts, modified names, forged saved child operands/calls and
one-time prototype observation across resource retry. A foreign bound Array
must reach its species Get rather than normalize as an intrinsic identity.
Instanceof controls retain outer source operands and actual inner children,
reject forged target paths/receivers/arguments and distinguish prototype contains.
No old input, assertion or version range was removed.

The automatic source recipe includes both new literal original files and their
consumer. No generated Rust/model/schema publication has run or been edited by
hand. Full automatic self-translation, native transport/publication and eventual
runtime equivalence controls remain required.

## Next actual code work

1. Complete public represented callable handles. Current `decode` in
   `javascript-error-realm.js` uses a host function with an inappropriate own
   prototype and refuses unknown names. Draft a live handle whose ordinary
   metadata reads/definitions/deletions/reflection consult actual heap descriptors,
   preserve identity and aliasing, and suspend accessor/source execution through
   the real protocol. Do not invoke an original body or infer constructability
   from a host wrapper. Extend source-free transport and retained strong edges
   through the actual slots and complete normalized child states.
2. Draft remaining derived/super/uninitialized-this, Proxy/private-field and
   mutable environment/effect protocols. A bound derived/Proxy target delegates
   into that target's pending implementation until its real protocol is drafted;
   this is an outstanding whole-language obligation, not a supported subset.
3. Continue remaining Rust source ownership/borrowed callback and iterator
   interfaces, partial moves/disjoint loans, reference-bearing storage,
   generics/traits/macros/modules and whole own-project bidirectional translation.
4. Keep every broader grammar/CST-default/AST-option, shared concept/equivalence,
   recursively nested language, ontology/taxonomy, RDF/OWL import/merge, dynamic
   reconstruction, grammar inference, formal proof and delivery obligation in
   vision. No feature count here substitutes for full requirement accounting.
5. Only after every requirement has a full actual code/control draft, reconcile
   live sources online before ledger use, run automatic publication and authorized
   JavaScript/CI-only gates, repair failures and record executed observations.
   Never compile Rust locally.
