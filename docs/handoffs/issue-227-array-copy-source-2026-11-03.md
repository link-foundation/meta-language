# Array copying source draft

This supplements [the vision](../vision.md); it records unexecuted source work.
`javascript-array-copy-call-source.js` composes Array.prototype.toReversed and
Array.prototype.with from existing shared VM operations. No new opcode, saved
original execution or handwritten native semantic replacement is introduced.

Both box their actual receiver, save ToLength, allocate through the active realm
Array root and create each dense output element as a full own data property.
toReversed Gets source indices in descending order. with converts its actual
index, rejects bounds before allocation and Gets all other coordinates in
ascending order, retaining the replacement value without coercion. Holes become
own undefined values. Source constructors and species are never read. String
boxing retains individual UTF16 units. A shared checked-length scalar rule
establishes ArrayCreate RangeError before resource requests; allocation uses the
existing fresh zero-length Array instruction followed by its unobservable own
length initialization, before any source element getter.

The index scalar rule accepts only completed Number operands and genuine
ToLength coordinates, normalizes NaN/signed zero, truncates finite fractions,
resolves negative positions and reports RangeError for both infinities or
out-of-range positions. Scalar conversion itself reuses the original continuation
protocol, retaining BigInt/Symbol TypeError and thrown values.

Twenty-five complete independent originals cover fresh identity, dense holes,
inherited properties, getters and mutation order, exact throws, frozen sources,
length bounds, scalar coercion and missing arguments. Node assertions explicitly
cover >=22.15 <23 or >=24 <25; uncovered versions fail selection. Deferred bytecode
controls retain branch bounds, actual realm/prototype/wrapper roots and scalar
boundaries. The source producer and native original replay include every row.
The automatic policy author registers the implementation and literal Links
conversion. Generated libraries/codecs/snapshots remain deferred.

Contracts were read from the primary ECMAScript algorithms:
[toReversed](https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.toreversed)
and [with](https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.with).
No original, test, parser/generator, compilation or validation gate executed.
Sorting, splicing, flattening, asynchronous execution and other full-language
requirements remain independent work; this batch does not establish completion.

The continuation adds `javascript-array-spliced-copy-call-source.js`, composed
through the same catalog and realm roots as the preceding copying methods.
Array.toSpliced captures length, converts/clamps start, distinguishes missing
start/missing skip from explicit undefined, then clamps actual skip count. It
subtracts deletion before checked insertion addition to keep safe mathematical
results exact, reports TypeError above the safe-integer maximum and RangeError
above ordinary Array capacity, before source element Gets. Prefix and suffix
Gets occur in ascending order; the removed region is never read. Inserted values
come from the actual already evaluated Call argument List, without coercion or
iterator reinterpretation. Result fields are dense own full data properties;
source constructors/species and prototype setters remain irrelevant.

Thirteen added originals preserve every preceding input and coordinate: missing
versus explicit skip count, ordered conversion/Get, deleted getters, dense holes,
infinite/fractional bounds, String boxing, inserted identity, huge deletion to a
small result, safe-length overflow and ordinary Array RangeError. The fixture now
contains 38 originals. Existing producer/native replay consumes the complete
fixture automatically. Additional deferred scalar controls cover count clamping.
The primary contract is
[Array.prototype.toSpliced](https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.tospliced).
Mutating splice/species, flattening and other families remain independent work.
No tests, imports, source generation, compilation or validation executed.
