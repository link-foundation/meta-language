# Generic collection construction source draft

This continues [fallible iterator helper interfaces](issue-227-fallible-iterator-helper-interfaces-2026-11-03.md)
under the sole architecture in [vision](../vision.md). Source edits and review
only: no represented code, test, generator, formatting, validation gate,
compilation or CI feedback ran or was reviewed. All prior requirements remain
binding and the complete code draft remains unfinished.

Continuation: [shared Array species construction](issue-227-shared-species-construction-2026-11-03.md)
connects new species requests to the same actual constructor operation. The
remaining broader constructor/frontend scope below remains binding.

## Actual frontend/runtime path drafted

The common generic Construct kernel previously admitted weak collection targets
while excluding Map and Set targets. Map/Set source aliases and Reflect.construct
therefore did not have the same complete receiver/prototype path as direct
specialized collection construction. `javascript-construction.js` now selects
the actual strong or weak allocation from the protected constructor fact, retains
that brand through its state checks and invokes the existing registered iterable
initializer body with the actual receiver and newTarget. No collection body or
iteration semantics are copied into a host/native fallback.

`javascript-collection-construction-rules.js` shares kind/storage selection.
`javascript-collection-construction.js` checks reuse of the one real allocated
receiver against the actual suspended generic Construct or Array creation parent.
It independently checks the construction state, target, newTarget, exact ordered
arguments, actual current allocation instruction, selected prototype, receiver
brand and empty internal slots. Other allocation instructions retain their
ordinary allocation route. The previous weak helper remains an exported weak-only
compatibility adapter to the shared check.

Source review also identified an existing frame admission condition which,
despite entering a block for classes and collections, required a base class in
the generic-parent branch. The branch now admits an actual matching collection
constructor kind while preserving the same target, parent, receiver and newTarget
checks. This is a source-level correction, not a response to executed CI. Generic
body entry likewise requires actual registered ordinary, class or collection
code. A kind fact with a missing initializer still requests implementation.

The ordinary constructor realm model retains optional Map/Set prototype roots,
and the protected realm adapter captures the actual roots. Fallback selects the
appropriate prototype from newTarget's actual realm, not the target's realm or
Object.prototype. Missing roots retain the existing authority request and saved
continuation. New fields are model source changes; automatic I/O and native
publication are deferred, rather than manually editing generated native code.

The existing iterable initializer still selects the actual receiver's adder once,
skips adder/iterator access for nullish input, gets real Map entry properties,
preserves key/value and insertion semantics, closes actual iterators on relevant
abrupt completions and retains original throws. A selected adder can change the
instance prototype without changing its internal collection slots. Its return
value does not replace the constructor receiver. Existing scalar/Array/base
construction and weak-source controls retain their original inputs.

## Independent controls drafted

Fourteen positive original programs append to the existing generic construction
fixture. They cover Map/Set aliases and key equality, actual function/class
newTarget prototypes, primitive/null fallback, complete Reflect argument order,
single adder lookup, nullish bypass, noncallable-adder order, abrupt adder and
malformed entry closing, prototype mutation, strong/weak brand separation and
Array.from's generic Set construction path. Existing explicit Node version
ranges and native-original comparisons remain selected by the existing policy.

New structural controls check one source argument array plus exactly one actual
collection receiver for all four kinds; metadata with missing executable bodies;
and receiver reuse with a removed parent, changed kind/storage/newTarget,
arguments, instruction position, receiver brand, prototype or nonempty storage.
Independent VM realm controls compare Map/Set fallback against real native
Reflect.construct without calling the foreign newTarget body. These controls
remain unexecuted. They are not added to the separate Rust source-composition
inventory or presented as a requirement-completion percentage.

The normal source-author recipe includes both new shared modules and the new
consumer controls. Existing literal fixture publication automatically retains
the appended original programs. Source-free codecs, native generated operations
and target emissions remain required later checks; no new passing claim is made.

## Remaining work and resumption

1. Finish genuine derived constructor/super/uninitialized-this cells, bound
   functions and Proxy construction with actual GetFunctionRealm/trap behavior.
   Preserve the exact existing source refusal controls until their actual
   protocols exist; ordinary collection brands or prototypes cannot grant them.
2. Complete general Array species construction through the same actual authority.
   Legacy species body admission is still separate from the generic Construct
   and Array creation paths covered here. Preserve actual allocation, callback,
   prototype and error ordering across all receiver kinds.
3. Complete the remaining JavaScript whole-project frontend, callable/effect and
   shared heap/liveness interfaces, and the Rust callback, iterator, storage,
   module/generic/trait and full bidirectional translation work in preceding
   handoffs. Actual own-code project coverage remains the first baseline target.
4. Preserve every broader grammar/CST/global-concept/formal-definition/equivalence,
   recursive parsing, ontology/taxonomy, RDF/OWL, dynamic and inference obligation
   in vision. These constructor changes do not discharge those requirements.
5. After all requirements have actual complete code drafts, reconcile sources
   online before ledger access, publish generated schemas/names/source receipts
   and self-translated Rust, and execute authorized JavaScript plus CI-only full
   gates. Fix actual failures and record observations afterward at the executed
   commit. Never compile Rust locally or treat source review as test evidence.
