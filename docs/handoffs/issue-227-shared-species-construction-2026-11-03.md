# Shared Array species construction source draft

This continues [generic collection construction](issue-227-generic-collection-construction-2026-11-03.md)
under the sole architecture in [vision](../vision.md). Source edits and review
only: no represented program, test, generator, formatter, validation gate,
compilation or CI feedback ran or was reviewed. Full requirements drafting
remains incomplete and the active goal remains unchanged.

## Actual implementation path drafted

`javascript-species-construction.js` now starts every new species request through
`beginJavaScriptConstruction` with the actual selected constructor as target and
newTarget and the one actual numeric length argument. The explicit
`shared Construct species continuation 1` state retains the full shared operation.
The existing species envelope keeps its constructor fact, length and mirrored
phase, mode, prototype, receiver and child call. Independent validation checks
that every mirror agrees, that both targets retain the selected identity, that
the complete argument List is exactly the selected length and that the immutable
fact retains its actual intrinsic kind, realm and constructability. The common
constructor validator then checks the real operation.

Stepping, allocation and call completion delegate to the common kernel and adapt
its actual transition. The adapter carries target-realm errors, real receiver
replacement, original abrupt values and resource/authority continuations. It
contains no new wrapper, Array, collection or iterator algorithms. Preceding
persisted states without the explicit new model retain the exact prior protocol;
new requests always carry the shared operation and cannot fall back after a
failure. The earlier state interpreter remains for that compatibility boundary.

`javascript-construction-parents.js` selects the actual shared operation from a
generic Construct, Array creation or current species parent. Constructor frame
and newTarget validation, collection receiver reuse and species body entry share
that selection. The surrounding Array transform/slice/concat continuation still
checks its actual species and length, so the nested operation cannot replace its
outer selection. External specialization completion now requires the actual
pending implementation request rather than inferring it from an old phase name.

This connects all newly emitted species requests to supported scalar wrappers,
Object/Array, actual source functions/classes and collection constructors in the
generic kernel. String exotic own indices retain their real readonly behavior;
Map/Set/weak species receive a numeric iterable argument even when the length is
zero. Array transformation property writes, callback order and optional final
length Set remain owned by their original algorithms. Unsupported derived,
bound, Proxy and other exotic constructor behavior remains an explicit full-scope
obligation, not a host execution fallback.

Source review also added Map/Set prototype roots to the existing monotonic
authority extension comparison. These new optional roots cannot replace or
remove established roots during later metadata extension. The new parent module
and updated species dependency recipe are registered for automatic source
publication and self-translation. Generated native models/I/O and source assets
remain deferred until all requirements have full code drafts.

## Independent controls authored

Thirteen original programs cover Number/Boolean/Object map species, zero-length
Number filter construction, String readonly writes and empty String filtering,
collection constructor failures before source reads/callbacks/spreadability,
Boolean slice and Number concat final lengths, actual source-class newTarget and
a source factory returning a nested constructed Set. Each compares the real
original declarations with the portable product under explicit Node version
ranges. These controls have not executed.

Additional structural controls retain the actual shared operation and forge
its model, selected fact, length, target/newTarget, mirrored phase/prototype and
child callee/receiver/arguments. Removing either half of the explicit model
refuses. An exact preceding base continuation without new fields remains an
independent compatibility input. Every earlier species construction assertion
and source input is retained unchanged. The normal source-author recipe includes
the new literal originals and consumer controls; later native/codec/target checks
remain required and no passing result or requirement percentage is claimed.

## Remaining full-scope work

1. Complete actual derived/super/uninitialized-this, bound and Proxy constructor
   protocols, fields/private names and escaping lexical constructor environments.
   General species support inherits those same missing constructor protocols;
   selecting a species does not establish them by declaration shape or spelling.
2. Complete general cross-realm, exotic receiver and lifecycle/effect interfaces,
   full own-project JavaScript frontend and dynamic heap/liveness behavior.
   Preserve original objects, aliasing, exact abrupt values, descriptors and
   source/target observations through actual edited typed Links.
3. Continue Rust borrowed callback interiors, forwarding/escaping environments,
   field-granular captures, iterator/finite-helper ABIs, partial moves/disjoint
   loans/reference-bearing storage, generics/traits/modules and whole-project
   emission/translation in both directions. Earlier handoffs retain details.
4. Complete every remaining grammar/CST/global-concept/recursive formal meaning,
   nested any/all parsing, ontology/taxonomy, RDF/OWL, dynamic reconstruction,
   inference, formal-language and delivery requirement in vision. This shared
   runtime integration does not establish those broader scopes.
5. After every requirement has a complete actual code draft, reconcile online
   sources before ledger use, publish generated schemas/names/receipts and native
   translations, then execute authorized JavaScript and CI-only full gates.
   Fix actual failures and record observations afterward at the executed commit.
   Never compile Rust locally or present authored controls as evidence.
