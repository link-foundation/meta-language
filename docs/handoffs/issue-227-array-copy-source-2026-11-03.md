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
