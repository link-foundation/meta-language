# Readonly regions beside closed source FnMut packets

This batch is a JavaScript source draft. No tests, original execution, imports,
generation, validation, compilation or native commands were run. It is not
runtime evidence or a claim that the full repository translates.

The explicit successor selects all three existing options together:
`sourceSignatureModel: 'Rust source function signature 3'`,
`exclusiveSourceModel: 'Rust exclusive source loans 1'`, and
`fnMutSourceModel: 'Rust closed source FnMut specializations 1'`.
The Rust range remains >=1.90.0 <1.100.0; Node controls cover >=22.15 <23 or
>=24 <25. Selecting signature3 and exclusive without FnMut retains the old
composition refusal. Existing independent profiles retain their own checks.

Actual readonly borrowed-result bodies retain signature3, parameter lifetimes,
outlives auditing and returned-region interpretation. Actual closed owned and
FnMut bodies retain their separate signature2/exclusive/FnMut interfaces and
their genuine existing body lowering. This does not relabel immutable source
signatures or replace any original body with handwritten native semantics.

`rust-source-region-composition.js` scopes original callable and immediate
callback bodies on authenticated parser objects. Closed bodies cannot erase
borrowed-vector values into their packet ABI. Readonly/main exclusive arguments
are whole roots only; projected field arguments require a separate carrier.
Readonly bodies with mutable value formals are refused pending an initial
parameter storage carrier. Closed bodies preserve existing genuine field loans.

`rust-source-state-transitions.js` exports
`attachRustSourceStateTransition(parser, node, slot, operation)` and
`checkRustSourceStateTransition(facet, type, language, resolveType, resolveData)`.
The additive expression facet is exactly:

```js
{ model: 'Rust source state transition 1',
  operation: 'initialize' /* or 'write' */,
  binder: { name, identity, start, end }, position }
```

It can decorate any checked expression whose type is the complete direct
referent type. Parser attachment checks the real slot table and original binder
token. Checker normalization delegates complete type/source-owner checks to the
existing source signature normalizer. The facet stores source facts; local proof
revision numbers are never serialized in it or in TYPE positions. Root owns
the coordinated checked projection/schema/typed Links retention and validation.

The region interpreter accepts actual normalized closed call interfaces via
`readRustBorrowedSourceClosedCallInterface(entry)`, returning
`{parameters,modes,result,indices,callback,packet,outputs}`. Result slot zero and
the complete ordered exclusive-state tails are preserved. FnMut always returns
a packet, including the tuple of one result when there are no mutable captures.
Ordinary source tuples and escaping function/reference carriers are not admitted
as arbitrary state packets. Full closed nominal field graphs are checked.

Mutable initialization, whole-root writes and call publication retain the actual
source binder and allocate local storage revisions. Copied scalar values do not
inherit mutable storage identity. Every packet state tail must name an initialized
current root. Calls reject overlapping whole roots, validate actual callback
capture binder spans/identities, and publish writes before continuation. Branch
joins rename regions of actual returned values to joined current storage only;
earlier saved aliases retain old epochs. New owned result matching transfers
closed constructor fields while independent readonly matching retains old rules.

Deferred controls are in `rust-source-region-composition.mjs` and
`translation-rust-source-region-composition.test.js`: eight complete literal
positive originals and seven retained refusals, with native validity explicitly
distinguished from unsupported source boundaries. The source capture factory
`createRustTypeRegionCompositionOriginals({typesSource,typesNetwork,spanSource,spanNetwork})`
adds three complete actual Type Tuple/Function/Array compositions. It captures
unchanged `children`, `map_children`, `is_reference`, Type/TypeField and Span,
with actual CST coordinates and hashes; serde-only declaration projection is
explicit. Native sources retain serde; own-crate originals import the actual
library rather than defining replacement methods. Capture/builders were not run.
Deferred controls include checked execution, stale-alias refusals, removed
initialization metadata, exact binder ranges, typed Links text/binary round trips
and all four emitters. They are declarations, not observations.

Remaining boundaries include internal borrowed-vector values in closed/FnMut
bodies, escaping callback references, mutable readonly formals, projected-field
exclusive arguments in readonly/main regions, moved mutable-owner
reinitialization in this mixed proof, and standard collection/profile3
composition. Opaque generic E restrictions and unused generic-definition
validation remain unchanged. Complete actual methods are captured; this does
not establish whole-crate support. Existing project source composition already
handles supplied out-of-line modules, path attributes and archives; it is not a
missing module linker. Native generated frontend parity remains CI/source
authoring work, not handwritten Rust parser changes.

Root's coordinated copied-reference index import/dispatch and operation rule
are retained in the shared region files. Its new index module, array reader,
children order/value originals and public/source-author graph changes are
root-owned. This agent finishes and releases the current batch here; no further
frontier is started.
