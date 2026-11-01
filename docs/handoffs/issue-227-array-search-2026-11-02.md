# Issue #227 Array search source draft — November 2, 2026

Architecture remains solely in [vision.md](../vision.md). This file records
source edits and remaining integration, not execution evidence or completion.
The source register and ledger remain untouched because their required online
audit is deferred under the current no-validation drafting instruction.

## Drafted source

- `javascript-array-index-rules.js` shares ToLength and ordinary HasProperty with
  callback traversal and adds relative search index selection. The previous
  callback presence entry point remains a wrapper with all previous guards.
- `javascript-array-search.js` drafts includes/indexOf/lastIndexOf/at. Length,
  index conversion, getters and current effects remain explicit continuations.
  Missing versus explicit undefined fromIndex is retained. includes visits holes
  with SameValueZero; indexed searches skip missing properties and use strict
  equality. at uses relative indexing and performs conversion even on empty
  input; empty searches skip fromIndex conversion. Infinity never becomes a
  property coordinate. Negative indices truncate toward zero.
- The real standard-method registry captures/verifies four callable identities.
  The directory compiler, instruction register accounting, machine instruction
  admission, pending validation, child calls, step/resume and final completion
  adapters are drafted together. A common selected-call frame validator retains
  the prior Array callback identity/receiver/argument checks for both families.
- Automatic native source-library generation registrations and deferred consumer
  inputs are added. No native semantic implementation is handwritten, and no
  generated Rust/codec/model/Links output is manually edited.

## Authored controls awaiting execution

Twelve original-source cases cover NaN/zero equality, holes, identity, forward
and reverse selection, omitted/undefined arguments, fractional/infinite/negative
indices, lazy empty search conversion, mandatory empty-at conversion and length
mutation by index conversion. Six continuation controls cover those differences,
inherited getters, exact held conversion receivers, current effects on throws
and retry of a selected unstarted call. These are declarations, not observations.

The normative source is ECMAScript 2026 Array.prototype.at/includes/indexOf/
lastIndexOf:
https://tc39.es/ecma262/2026/multipage/indexed-collections.html#sec-array.prototype.at
and the adjacent method sections. Original runtime comparisons must use the
project's selected active runtime/dependency ranges and report uncovered ranges;
no engine-specific message mapping or broader runtime support is claimed here.

## Remaining work for this batch

After the full requirements code draft permits generation, regenerate the source
libraries, typed model codecs, native modules/publication and Links fixture data
through the existing authoring pipeline. Execute every retained original-source
assertion and JS/native replay at the exact recorded revision in CI. Preserve
classification of malformed state, unsupported host/boxing protocols and resource
outcomes. Do not claim completion from these source declarations.

Primitive receiver boxing, opaque HasProperty/Proxy behavior, full diagnostics,
Array species/result construction for map/filter/flatMap, general constructor
execution, IteratorClose/for-of and lexical cells/TDZ remain explicit obligations.
All earlier requirements and controls remain binding; this file is not a new
architecture specification or a replacement ledger.
