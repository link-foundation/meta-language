# Stable opaque vector sort source draft

Authored source and deferred controls only. No represented imports, tests,
generation, validation, builds, compilation or native execution ran. No ledger
observations, passing evidence or feature completion claim is made.

The complete JavaScript algorithm is stable-vector-sort-template.js. It performs
stable insertion of declared KeyedValue entries, comparing only each String key.
Only a strictly smaller incoming key inserts before a previous entry, so equal
keys retain their original opaque payload order. The input array and its entry
objects remain unmodified. Payload values remain wrapped and are never inspected,
coerced or used to break ties. There is no host sort call, callback comparator or
handwritten native implementation.

`compareStableVectorKeys` uses ordinary JavaScript String `<` and `>` expressions.
The ordinary checker retains the existing explicit unicodeCodeUnit stringTest
operation. This is UTF16 lexicographic order, including prefix order; on actual
valid Rust owned String values it matches encode_utf16().cmp(). It deliberately
differs from Rust UTF8/scalar String order for astral versus high BMP characters.
Root owns the shared native/formal UTF16 emission adapters, not this algorithm.

`specializeStableVectorSort(element,namespace)` reads and parses the actual full
JavaScript source during normal compiler operation, replaces exactly the declared
OpaqueValue.value field type and runs the ordinary checker. Every original body
remains present. The result contains checked `library`, payload wrapper `type`
and `constructor`, `entryType` and `entryConstructor`, sorter `entry`, key
`keyComparison` and the exact model string
`complete stable opaque UTF16 keyed vector insertion sort 1`.

A nominal caller payload remains its real checked kind:data reference. No dummy
owner, placeholder declaration or replacement payload semantics are manufactured.
The algorithm never projects that field, so only its actual wrapper and key-entry
layouts are inspected by this library. The root checker adapter merges the real
caller declarations with all actual specialized declarations/items before graph
publication. Deferred controls include that complete nominal merge and Links
recheck, in addition to primitive/optional/product payload specializations.

`lowerStableVectorSort(node,value,keyLambda,specialization,fresh)` returns checked
IR. It captures the input and actual key function once, computes one key for each
original element, wraps the opaque payload, calls the genuine source sorter and
unwraps the resulting payload sequence. This boundary performs representation
conversion only. Root's strict syntax/owner/place reader authenticates the actual
Rust comparator and applies caller state writeback. Arbitrary source comparator
captures/effects are not authorized by this lower-level checked helper.

Root owns rust-source-vector-sort.js, checker/module registration and parser
integration. No rust.js edit belongs to this batch. The normal compiler reads
real source directly, so specialization does not require a not-yet-generated
asset. The separate future author command
`node js/scripts/draft-stable-vector-sort-template.mjs --draft` captures complete
source/syntax/surface/opaque-slot metadata, then uses draftSourceLibrary to author
module stable_vector_sort and stable-vector-sort-program from actual JavaScript.
No generated data or native files were edited or produced during this draft.

The independent raw fixture file contains 24 complete literal stable-order
vectors and two explicitly JavaScript-only lone-surrogate boundary vectors.
Those extra JavaScript cases do not broaden Rust String source admission.
Deferred controls preserve opaque nullish/signed-zero/NaN/BigInt/aliased-object
values; compare exact emitted product stdout; check single input/key-function
capture and per-value key evaluation order; and retain Lino/binary/all-target
projection coverage. Node controls are bounded to >=22.15 <23 or >=24 <25.
Root retains the separate full Rust source originals and refusals.

The algorithm has quadratic insertion work and returns an ordinary fresh array;
the source adapter supplies the actual mutating Vec state result. Allocation
identity, custom Drop, arbitrary comparators and broader source sort families
are outside this closed comparator admission. Automatic source generation and
native/formal/product verification remain deferred with the complete batch.
